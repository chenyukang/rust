#![deny(unused_must_use)]

use super::utils::SubdiagnosticVariant;
use crate::diagnostics::error::{
    span_err, throw_invalid_attr, throw_span_err, DiagnosticDeriveError,
};
use crate::diagnostics::utils::format_for_variables;
use crate::diagnostics::utils::{
    build_field_mapping, is_doc_comment, report_error_if_not_applied_to_span, report_type_error,
    should_generate_set_arg, type_is_bool, type_is_unit, type_matches_path, FieldInfo,
    FieldInnerTy, FieldMap, HasFieldMap, SetOnce, SpannedOption, SubdiagnosticKind,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use std::collections::HashMap;
use syn::Token;
use syn::{parse_quote, spanned::Spanned, Attribute, Meta, Path, Type};
use synstructure::{BindingInfo, Structure, VariantInfo};

/// What kind of diagnostic is being derived - a fatal/error/warning or a lint?
#[derive(Clone, PartialEq, Eq)]
pub(crate) enum DiagnosticDeriveKind {
    Diagnostic { handler: syn::Ident },
    LintDiagnostic,
}
use syn::LitStr;

/// Tracks persistent information required for the entire type when building up individual calls to
/// diagnostic methods for generated diagnostic derives - both `Diagnostic` for
/// fatal/errors/warnings and `LintDiagnostic` for lints.
pub(crate) struct DiagnosticDeriveBuilder {
    /// The identifier to use for the generated `DiagnosticBuilder` instance.
    pub diag: syn::Ident,
    /// Kind of diagnostic that should be derived.
    pub kind: DiagnosticDeriveKind,
}

/// Tracks persistent information required for a specific variant when building up individual calls
/// to diagnostic methods for generated diagnostic derives - both `Diagnostic` for
/// fatal/errors/warnings and `LintDiagnostic` for lints.
pub(crate) struct DiagnosticDeriveVariantBuilder<'parent> {
    /// The parent builder for the entire type.
    pub parent: &'parent DiagnosticDeriveBuilder,

    /// Initialization of format strings for code suggestions.
    pub formatting_init: TokenStream,

    /// Span of the struct or the enum variant.
    pub span: proc_macro::Span,

    /// Store a map of field name to its corresponding field. This is built on construction of the
    /// derive builder.
    pub field_map: FieldMap,

    /// Slug is a mandatory part of the struct attribute as corresponds to the Fluent message that
    /// has the actual diagnostic message.
    pub slug: SpannedOption<Path>,

    /// Text is a the text embedded in the struct attribute and corresponds to the diagnostic
    pub text: SpannedOption<LitStr>,

    /// Error codes are a optional part of the struct attribute - this is only set to detect
    /// multiple specifications.
    pub code: SpannedOption<()>,

    /// Attributes on the variant.
    pub attrs: HashMap<String, LitStr>,

    /// Bindings
    pub bindings: HashMap<String, String>,
}

impl<'a> HasFieldMap for DiagnosticDeriveVariantBuilder<'a> {
    fn get_field_binding(&self, field: &String) -> Option<&TokenStream> {
        self.field_map.get(field)
    }
}

impl DiagnosticDeriveBuilder {
    /// Call `f` for the struct or for each variant of the enum, returning a `TokenStream` with the
    /// tokens from `f` wrapped in an `match` expression. Emits errors for use of derive on unions
    /// or attributes on the type itself when input is an enum.
    pub fn each_variant<'s, F>(&mut self, structure: &mut Structure<'s>, f: F) -> TokenStream
    where
        F: for<'a, 'v> Fn(DiagnosticDeriveVariantBuilder<'a>, &VariantInfo<'v>) -> TokenStream,
    {
        let ast = structure.ast();
        let span = ast.span().unwrap();
        match ast.data {
            syn::Data::Struct(..) | syn::Data::Enum(..) => (),
            syn::Data::Union(..) => {
                span_err(span, "diagnostic derives can only be used on structs and enums").emit();
            }
        }

        if matches!(ast.data, syn::Data::Enum(..)) {
            for attr in &ast.attrs {
                span_err(
                    attr.span().unwrap(),
                    "unsupported type attribute for diagnostic derive enum",
                )
                .emit();
            }
        }

        structure.bind_with(|_| synstructure::BindStyle::Move);
        let variants = structure.each_variant(|variant| {
            let span = match structure.ast().data {
                syn::Data::Struct(..) => span,
                // There isn't a good way to get the span of the variant, so the variant's
                // name will need to do.
                _ => variant.ast().ident.span().unwrap(),
            };
            let builder = DiagnosticDeriveVariantBuilder {
                parent: self,
                span,
                field_map: build_field_mapping(variant),
                formatting_init: TokenStream::new(),
                slug: None,
                code: None,
                text: None,
                attrs: HashMap::new(),
                bindings: HashMap::new(),
            };
            f(builder, variant)
        });

        quote! {
            match self {
                #variants
            }
        }
    }
}

impl<'a> DiagnosticDeriveVariantBuilder<'a> {
    /// Generates calls to `code` and similar functions based on the attributes on the type or
    /// variant.
    pub fn preamble(&mut self, variant: &VariantInfo<'_>) -> TokenStream {
        let ast = variant.ast();
        let attrs = &ast.attrs;
        for binding in variant.bindings().iter().filter(|bi| should_generate_set_arg(bi.ast())) {
            self.generate_binding_for_attr(binding);
        }

        let preamble = attrs.iter().map(|attr| {
            self.generate_structure_code_for_attr(attr).unwrap_or_else(|v| v.to_compile_error())
        });

        quote! {
            #(#preamble)*;
        }
    }

    /// Generates calls to `span_label` and similar functions based on the attributes on fields or
    /// calls to `set_arg` when no attributes are present.
    pub fn body(&mut self, variant: &VariantInfo<'_>) -> TokenStream {
        let mut body = quote! {};
        // Generate `set_arg` calls first..
        for binding in variant.bindings().iter().filter(|bi| should_generate_set_arg(bi.ast())) {
            body.extend(self.generate_field_code(binding));
        }
        // ..and then subdiagnostic additions.
        for binding in variant.bindings().iter().filter(|bi| !should_generate_set_arg(bi.ast())) {
            body.extend(self.generate_field_attrs_code(binding));
        }
        body
    }

    /// Parse a `SubdiagnosticKind` from an `Attribute`.
    fn parse_subdiag_attribute(
        &self,
        attr: &Attribute,
    ) -> Result<Option<(SubdiagnosticKind, Path, bool, Option<String>)>, DiagnosticDeriveError>
    {
        let Some(subdiag) = SubdiagnosticVariant::from_attr(attr, self)? else {
            // Some attributes aren't errors - like documentation comments - but also aren't
            // subdiagnostics.
            return Ok(None);
        };
        if let SubdiagnosticKind::MultipartSuggestion { .. } = subdiag.kind {
            throw_invalid_attr!(attr, |diag| diag
                .help("consider creating a `Subdiagnostic` instead"));
        }

        let slug = subdiag.slug.unwrap_or_else(|| match subdiag.kind {
            SubdiagnosticKind::Label => parse_quote! { _subdiag::label },
            SubdiagnosticKind::Note => parse_quote! { _subdiag::note },
            SubdiagnosticKind::Help => parse_quote! { _subdiag::help },
            SubdiagnosticKind::Warn => parse_quote! { _subdiag::warn },
            SubdiagnosticKind::Suggestion { .. } => parse_quote! { _subdiag::suggestion },
            SubdiagnosticKind::MultipartSuggestion { .. } => unreachable!(),
        });

        Ok(Some((subdiag.kind, slug, subdiag.no_span, subdiag.text.map(|t| t.value()))))
    }

    /// Establishes state in the `DiagnosticDeriveBuilder` resulting from the struct
    /// attributes like `#[diag(..)]`, such as the slug and error code. Generates
    /// diagnostic builder calls for setting error code and creating note/help messages.
    fn generate_structure_code_for_attr(
        &mut self,
        attr: &Attribute,
    ) -> Result<TokenStream, DiagnosticDeriveError> {
        let diag = &self.parent.diag;

        // Always allow documentation comments.
        if is_doc_comment(attr) {
            return Ok(quote! {});
        }

        let name = attr.path().segments.last().unwrap().ident.to_string();
        let name = name.as_str();

        let mut first = true;

        if name == "diag" {
            let mut tokens = TokenStream::new();
            attr.parse_nested_meta(|nested| {
                let path = &nested.path;
                if first && (nested.input.is_empty() || nested.input.peek(Token![,])) {
                    self.slug.set_once(path.clone(), path.span().unwrap());
                    first = false;
                    return Ok(());
                }

                first = false;

                let Ok(nested) = nested.value() else {
                    span_err(
                        nested.input.span().unwrap(),
                        "diagnostic slug must be the first argument",
                    )
                    .emit();
                    return Ok(());
                };

                if path.is_ident("text") {
                    let value = nested.parse::<syn::LitStr>()?;
                    self.text.set_once(value, path.span().unwrap());
                } else if path.is_ident("code") {
                    self.code.set_once((), path.span().unwrap());
                    let code = nested.parse::<syn::LitStr>()?;
                    tokens.extend(quote! {
                        #diag.code(rustc_errors::DiagnosticId::Error(#code.to_string()));
                    });
                } else {
                    let mut founded = false;
                    let keys = vec!["note", "help", "warning"];
                    for key in keys {
                        if path.is_ident(key) {
                            if let Ok(value) = nested.parse::<syn::LitStr>() {
                                self.attrs.insert(key.to_string(), value.clone());
                                founded = true;
                            }
                        }
                    }
                    if !founded {
                        span_err(path.span().unwrap(), "unknown argument")
                            .note("only the `code` parameter is valid after the slug")
                            .emit();
                        // consume the buffer so we don't have syntax errors from syn
                        let _ = nested.parse::<TokenStream>();
                    }
                }
                Ok(())
            })?;
            return Ok(tokens);
        }

        let Some((subdiag, slug, _no_span, text)) = self.parse_subdiag_attribute(attr)? else {
            // Some attributes aren't errors - like documentation comments - but also aren't
            // subdiagnostics.
            return Ok(quote! {});
        };
        let fn_ident = format_ident!("{}", subdiag);
        match subdiag {
            SubdiagnosticKind::Note | SubdiagnosticKind::Help | SubdiagnosticKind::Warn => {
                Ok(self.add_subdiagnostic(&fn_ident, slug, text))
            }
            SubdiagnosticKind::Label | SubdiagnosticKind::Suggestion { .. } => {
                throw_invalid_attr!(attr, |diag| diag
                    .help("`#[label]` and `#[suggestion]` can only be applied to fields"));
            }
            SubdiagnosticKind::MultipartSuggestion { .. } => unreachable!(),
        }
    }

    fn generate_field_code(&mut self, binding_info: &BindingInfo<'_>) -> TokenStream {
        let diag = &self.parent.diag;

        let field = binding_info.ast();
        let mut field_binding = binding_info.binding.clone();
        field_binding.set_span(field.ty.span());

        let ident = field.ident.as_ref().unwrap();
        let ident = format_ident!("{}", ident); // strip `r#` prefix, if present
        quote! {
            #diag.set_arg(
                stringify!(#ident),
                #field_binding
            );
        }
    }

    fn generate_binding_for_attr(&mut self, binding_info: &BindingInfo<'_>) {
        let field = binding_info.ast();
        let mut field_binding = binding_info.binding.clone();
        field_binding.set_span(field.ty.span());

        let ident = field.ident.as_ref().unwrap();
        let ident = format_ident!("{}", ident); // strip `r#` prefix, if present
        self.bindings.insert(ident.to_string(), field_binding.to_string());
    }

    fn generate_field_attrs_code(&mut self, binding_info: &BindingInfo<'_>) -> TokenStream {
        let field = binding_info.ast();
        let field_binding = &binding_info.binding;

        let inner_ty = FieldInnerTy::from_type(&field.ty);

        field
            .attrs
            .iter()
            .map(move |attr| {
                // Always allow documentation comments.
                if is_doc_comment(attr) {
                    return quote! {};
                }

                let name = attr.path().segments.last().unwrap().ident.to_string();
                let needs_clone =
                    name == "primary_span" && matches!(inner_ty, FieldInnerTy::Vec(_));
                let (binding, needs_destructure) = if needs_clone {
                    // `primary_span` can accept a `Vec<Span>` so don't destructure that.
                    (quote_spanned! {inner_ty.span()=> #field_binding.clone() }, false)
                } else {
                    (quote_spanned! {inner_ty.span()=> #field_binding }, true)
                };

                let generated_code = self
                    .generate_inner_field_code(
                        attr,
                        FieldInfo { binding: binding_info, ty: inner_ty, span: &field.span() },
                        binding,
                    )
                    .unwrap_or_else(|v| v.to_compile_error());

                if needs_destructure {
                    inner_ty.with(field_binding, generated_code)
                } else {
                    generated_code
                }
            })
            .collect()
    }

    fn generate_inner_field_code(
        &mut self,
        attr: &Attribute,
        info: FieldInfo<'_>,
        binding: TokenStream,
    ) -> Result<TokenStream, DiagnosticDeriveError> {
        let diag = &self.parent.diag;

        let ident = &attr.path().segments.last().unwrap().ident;
        let name = ident.to_string();
        match (&attr.meta, name.as_str()) {
            // Don't need to do anything - by virtue of the attribute existing, the
            // `set_arg` call will not be generated.
            (Meta::Path(_), "skip_arg") => return Ok(quote! {}),
            (Meta::Path(_), "primary_span") => {
                match self.parent.kind {
                    DiagnosticDeriveKind::Diagnostic { .. } => {
                        report_error_if_not_applied_to_span(attr, &info)?;

                        return Ok(quote! {
                            #diag.set_span(#binding);
                        });
                    }
                    DiagnosticDeriveKind::LintDiagnostic => {
                        throw_invalid_attr!(attr, |diag| {
                            diag.help("the `primary_span` field attribute is not valid for lint diagnostics")
                        })
                    }
                }
            }
            (Meta::Path(_), "subdiagnostic") => {
                if FieldInnerTy::from_type(&info.binding.ast().ty).will_iterate() {
                    let DiagnosticDeriveKind::Diagnostic { handler } = &self.parent.kind else {
                        // No eager translation for lints.
                        return Ok(quote! { #diag.subdiagnostic(#binding); });
                    };
                    return Ok(quote! { #diag.eager_subdiagnostic(#handler, #binding); });
                } else {
                    return Ok(quote! { #diag.subdiagnostic(#binding); });
                }
            }
            (Meta::List(meta_list), "subdiagnostic") => {
                let err = || {
                    span_err(
                        meta_list.span().unwrap(),
                        "`eager` is the only supported nested attribute for `subdiagnostic`",
                    )
                    .emit();
                };

                let Ok(p): Result<Path, _> = meta_list.parse_args() else {
                    err();
                    return Ok(quote! {});
                };

                if !p.is_ident("eager") {
                    err();
                    return Ok(quote! {});
                }

                let handler = match &self.parent.kind {
                    DiagnosticDeriveKind::Diagnostic { handler } => handler,
                    DiagnosticDeriveKind::LintDiagnostic => {
                        throw_invalid_attr!(attr, |diag| {
                            diag.help("eager subdiagnostics are not supported on lints")
                        })
                    }
                };
                return Ok(quote! { #diag.eager_subdiagnostic(#handler, #binding); });
            }
            _ => (),
        }

        let Some((subdiag, slug, _no_span, text)) = self.parse_subdiag_attribute(attr)? else {
            // Some attributes aren't errors - like documentation comments - but also aren't
            // subdiagnostics.
            return Ok(quote! {});
        };
        let fn_ident = format_ident!("{}", subdiag);
        match subdiag {
            SubdiagnosticKind::Label => {
                report_error_if_not_applied_to_span(attr, &info)?;
                Ok(self.add_spanned_subdiagnostic(binding, &fn_ident, slug, text))
            }
            SubdiagnosticKind::Note | SubdiagnosticKind::Help | SubdiagnosticKind::Warn => {
                let inner = info.ty.inner_type();
                if type_matches_path(inner, &["rustc_span", "Span"])
                    || type_matches_path(inner, &["rustc_span", "MultiSpan"])
                {
                    Ok(self.add_spanned_subdiagnostic(binding, &fn_ident, slug, text))
                } else if type_is_unit(inner)
                    || (matches!(info.ty, FieldInnerTy::Plain(_)) && type_is_bool(inner))
                {
                    Ok(self.add_subdiagnostic(&fn_ident, slug, text))
                } else {
                    report_type_error(attr, "`Span`, `MultiSpan`, `bool` or `()`")?
                }
            }
            SubdiagnosticKind::Suggestion {
                suggestion_kind,
                applicability: static_applicability,
                code_field,
                code_init,
            } => {
                if let FieldInnerTy::Vec(_) = info.ty {
                    throw_invalid_attr!(attr, |diag| {
                        diag
                        .note("`#[suggestion(...)]` applied to `Vec` field is ambiguous")
                        .help("to show a suggestion consisting of multiple parts, use a `Subdiagnostic` annotated with `#[multipart_suggestion(...)]`")
                        .help("to show a variable set of suggestions, use a `Vec` of `Subdiagnostic`s annotated with `#[suggestion(...)]`")
                    });
                }

                let (span_field, mut applicability) = self.span_and_applicability_of_ty(info)?;

                if let Some((static_applicability, span)) = static_applicability {
                    applicability.set_once(quote! { #static_applicability }, span);
                }

                let applicability = applicability
                    .value()
                    .unwrap_or_else(|| quote! { rustc_errors::Applicability::Unspecified });
                let style = suggestion_kind.to_suggestion_style();

                let suggestion_label = if let Some(text) = text {
                    let text = format_for_variables(&text, &self.bindings);
                    quote! {
                        #text
                    }
                } else {
                    quote! {
                        crate::fluent_generated::#slug
                    }
                };

                self.formatting_init.extend(code_init);
                Ok(quote! {
                    #diag.span_suggestions_with_style(
                        #span_field,
                        #suggestion_label,
                        #code_field,
                        #applicability,
                        #style
                    );
                })
            }
            SubdiagnosticKind::MultipartSuggestion { .. } => unreachable!(),
        }
    }

    /// Adds a spanned subdiagnostic by generating a `diag.span_$kind` call with the current slug
    /// and `fluent_attr_identifier`.
    fn add_spanned_subdiagnostic(
        &self,
        field_binding: TokenStream,
        kind: &Ident,
        fluent_attr_identifier: Path,
        text: Option<String>,
    ) -> TokenStream {
        let diag = &self.parent.diag;
        let fn_name = format_ident!("span_{}", kind);
        if let Some(text) = text {
            let text = format_for_variables(&text, &self.bindings);
            return quote! {
                #diag.#fn_name(
                    #field_binding,
                    #text
                );
            };
        }
        if let Some(text) = self.get_attr(kind.to_string().as_str()) {
            quote! {
                #diag.#fn_name(
                    #field_binding,
                    #text
                );
            }
        } else {
            quote! {
                #diag.#fn_name(
                    #field_binding,
                    crate::fluent_generated::#fluent_attr_identifier
                );
            }
        }
    }

    /// Adds a subdiagnostic by generating a `diag.$kind` call with the current slug
    /// and `fluent_attr_identifier`.
    fn add_subdiagnostic(
        &self,
        kind: &Ident,
        fluent_attr_identifier: Path,
        text: Option<String>,
    ) -> TokenStream {
        let diag = &self.parent.diag;
        // eprintln!(
        //     "add_subdiagnostic fluent_attr_identifier: {:?} text: {:?}",
        //     fluent_attr_identifier, text
        // );
        if let Some(text) = text {
            let text = format_for_variables(&text, &self.bindings);
            return quote! {
                #diag.#kind(#text);
            };
        }
        if let Some(text) = self.get_attr(kind.to_string().as_str()) {
            quote! {
                #diag.#kind(#text);
            }
        } else {
            quote! {
                #diag.#kind(crate::fluent_generated::#fluent_attr_identifier);
            }
        }
    }

    fn span_and_applicability_of_ty(
        &self,
        info: FieldInfo<'_>,
    ) -> Result<(TokenStream, SpannedOption<TokenStream>), DiagnosticDeriveError> {
        match &info.ty.inner_type() {
            // If `ty` is `Span` w/out applicability, then use `Applicability::Unspecified`.
            ty @ Type::Path(..) if type_matches_path(ty, &["rustc_span", "Span"]) => {
                let binding = &info.binding.binding;
                Ok((quote!(#binding), None))
            }
            // If `ty` is `(Span, Applicability)` then return tokens accessing those.
            Type::Tuple(tup) => {
                let mut span_idx = None;
                let mut applicability_idx = None;

                fn type_err(span: &Span) -> Result<!, DiagnosticDeriveError> {
                    span_err(span.unwrap(), "wrong types for suggestion")
                        .help(
                            "`#[suggestion(...)]` on a tuple field must be applied to fields \
                             of type `(Span, Applicability)`",
                        )
                        .emit();
                    Err(DiagnosticDeriveError::ErrorHandled)
                }

                for (idx, elem) in tup.elems.iter().enumerate() {
                    if type_matches_path(elem, &["rustc_span", "Span"]) {
                        span_idx.set_once(syn::Index::from(idx), elem.span().unwrap());
                    } else if type_matches_path(elem, &["rustc_errors", "Applicability"]) {
                        applicability_idx.set_once(syn::Index::from(idx), elem.span().unwrap());
                    } else {
                        type_err(&elem.span())?;
                    }
                }

                let Some((span_idx, _)) = span_idx else {
                    type_err(&tup.span())?;
                };
                let Some((applicability_idx, applicability_span)) = applicability_idx else {
                    type_err(&tup.span())?;
                };
                let binding = &info.binding.binding;
                let span = quote!(#binding.#span_idx);
                let applicability = quote!(#binding.#applicability_idx);

                Ok((span, Some((applicability, applicability_span))))
            }
            // If `ty` isn't a `Span` or `(Span, Applicability)` then emit an error.
            _ => throw_span_err!(info.span.unwrap(), "wrong field type for suggestion", |diag| {
                diag.help(
                    "`#[suggestion(...)]` should be applied to fields of type `Span` or \
                     `(Span, Applicability)`",
                )
            }),
        }
    }

    fn get_attr(&self, key: &str) -> Option<TokenStream> {
        self.attrs.get(key).map(|val| {
            let text = format_for_variables(&val.value(), &self.bindings);
            quote! {
                #text
            }
        })
    }
}
