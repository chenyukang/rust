use super::{AttrWrapper, Capturing, FnParseMode, ForceCollect, Parser, PathStyle};
use crate::errors::{InvalidMetaItem, SuffixedLiteralInAttribute};
use rustc_ast as ast;
use rustc_ast::attr;
use rustc_ast::token::{self, Delimiter, Nonterminal};
use rustc_errors::fluent_raw;
use rustc_errors::DiagnosticMessage;
use rustc_errors::{error_code, Diagnostic, IntoDiagnostic, PResult};
use rustc_span::{sym, BytePos, Span};
use std::borrow::Cow;
use std::convert::TryInto;
use thin_vec::ThinVec;
use tracing::debug;

// Public for rustfmt usage
#[derive(Debug)]
pub enum InnerAttrPolicy {
    Permitted,
    Forbidden(Option<InnerAttrForbiddenReason>),
}

#[derive(Clone, Copy, Debug)]
pub enum InnerAttrForbiddenReason {
    InCodeBlock,
    AfterOuterDocComment { prev_doc_comment_span: Span },
    AfterOuterAttribute { prev_outer_attr_sp: Span },
}

enum OuterAttributeType {
    DocComment,
    DocBlockComment,
    Attribute,
}

impl<'a> Parser<'a> {
    /// Parses attributes that appear before an item.
    pub(super) fn parse_outer_attributes(&mut self) -> PResult<'a, AttrWrapper> {
        let mut outer_attrs = ast::AttrVec::new();
        let mut just_parsed_doc_comment = false;
        let start_pos = self.num_bump_calls;
        loop {
            let attr = if self.check(&token::Pound) {
                let prev_outer_attr_sp = outer_attrs.last().map(|attr| attr.span);

                let inner_error_reason = if just_parsed_doc_comment {
                    Some(InnerAttrForbiddenReason::AfterOuterDocComment {
                        prev_doc_comment_span: prev_outer_attr_sp.unwrap(),
                    })
                } else {
                    prev_outer_attr_sp.map(|prev_outer_attr_sp| {
                        InnerAttrForbiddenReason::AfterOuterAttribute { prev_outer_attr_sp }
                    })
                };
                let inner_parse_policy = InnerAttrPolicy::Forbidden(inner_error_reason);
                just_parsed_doc_comment = false;
                Some(self.parse_attribute(inner_parse_policy)?)
            } else if let token::DocComment(comment_kind, attr_style, data) = self.token.kind {
                if attr_style != ast::AttrStyle::Outer {
                    let span = self.token.span;
                    let mut err = self.diagnostic().struct_span_err_with_code(
                        span,
                        "expected outer doc comment",
                        error_code!(E0753),
                    );
                    err.set_arg("item_type", "doc comment");
                    if let Some(replacement_span) = self.annotate_following_item_if_applicable(
                        &mut err,
                        span,
                        match comment_kind {
                            token::CommentKind::Line => OuterAttributeType::DocComment,
                            token::CommentKind::Block => OuterAttributeType::DocBlockComment,
                        },
                    ) {
                        err.note("inner doc comments like this (starting with `//!` or `/*!`) can only appear before items");
                        err.span_suggestion_verbose(
                            replacement_span,
                            "you might have meant to write a regular comment",
                            "",
                            rustc_errors::Applicability::MachineApplicable,
                        );
                    }
                    err.emit();
                }
                self.bump();
                just_parsed_doc_comment = true;
                // Always make an outer attribute - this allows us to recover from a misplaced
                // inner attribute.
                Some(attr::mk_doc_comment(
                    &self.sess.attr_id_generator,
                    comment_kind,
                    ast::AttrStyle::Outer,
                    data,
                    self.prev_token.span,
                ))
            } else {
                None
            };

            if let Some(attr) = attr {
                if attr.style == ast::AttrStyle::Outer {
                    outer_attrs.push(attr);
                }
            } else {
                break;
            }
        }
        Ok(AttrWrapper::new(outer_attrs, start_pos))
    }

    /// Matches `attribute = # ! [ meta_item ]`.
    /// `inner_parse_policy` prescribes how to handle inner attributes.
    // Public for rustfmt usage.
    pub fn parse_attribute(
        &mut self,
        inner_parse_policy: InnerAttrPolicy,
    ) -> PResult<'a, ast::Attribute> {
        debug!(
            "parse_attribute: inner_parse_policy={:?} self.token={:?}",
            inner_parse_policy, self.token
        );
        let lo = self.token.span;
        // Attributes can't have attributes of their own [Editor's note: not with that attitude]
        self.collect_tokens_no_attrs(|this| {
            assert!(this.eat(&token::Pound), "parse_attribute called in non-attribute position");

            let style =
                if this.eat(&token::Not) { ast::AttrStyle::Inner } else { ast::AttrStyle::Outer };

            this.expect(&token::OpenDelim(Delimiter::Bracket))?;
            let item = this.parse_attr_item(false)?;
            this.expect(&token::CloseDelim(Delimiter::Bracket))?;
            let attr_sp = lo.to(this.prev_token.span);

            // Emit error if inner attribute is encountered and forbidden.
            if style == ast::AttrStyle::Inner {
                this.error_on_forbidden_inner_attr(attr_sp, inner_parse_policy);
            }

            Ok(attr::mk_attr_from_item(&self.sess.attr_id_generator, item, None, style, attr_sp))
        })
    }

    fn annotate_following_item_if_applicable(
        &self,
        err: &mut Diagnostic,
        span: Span,
        attr_type: OuterAttributeType,
    ) -> Option<Span> {
        let mut snapshot = self.create_snapshot_for_diagnostic();
        let lo = span.lo()
            + BytePos(match attr_type {
                OuterAttributeType::Attribute => 1,
                _ => 2,
            });
        let hi = lo + BytePos(1);
        let replacement_span = span.with_lo(lo).with_hi(hi);
        if let OuterAttributeType::DocBlockComment | OuterAttributeType::DocComment = attr_type {
            snapshot.bump();
        }
        loop {
            // skip any other attributes, we want the item
            if snapshot.token.kind == token::Pound {
                if let Err(err) = snapshot.parse_attribute(InnerAttrPolicy::Permitted) {
                    err.cancel();
                    return Some(replacement_span);
                }
            } else {
                break;
            }
        }
        match snapshot.parse_item_common(
            AttrWrapper::empty(),
            true,
            false,
            FnParseMode { req_name: |_| true, req_body: true },
            ForceCollect::No,
        ) {
            Ok(Some(item)) => {
                // FIXME(#100717)
                err.set_arg("item", item.kind.descr());
                err.span_label(
                    item.span,
                    fluent_raw!("the inner {$item_type} doesn't annotate this {$item}"),
                );
                err.span_suggestion_verbose(
                    replacement_span,
                    fluent_raw!(
                        "to annotate the {$item}, change the {$item_type} from inner to outer style"
                    ),
                    match attr_type {
                        OuterAttributeType::Attribute => "",
                        OuterAttributeType::DocBlockComment => "*",
                        OuterAttributeType::DocComment => "/",
                    },
                    rustc_errors::Applicability::MachineApplicable,
                );
                return None;
            }
            Err(item_err) => {
                item_err.cancel();
            }
            Ok(None) => {}
        }
        Some(replacement_span)
    }

    pub(super) fn error_on_forbidden_inner_attr(&self, attr_sp: Span, policy: InnerAttrPolicy) {
        if let InnerAttrPolicy::Forbidden(reason) = policy {
            let mut diag = match reason.as_ref().copied() {
                Some(InnerAttrForbiddenReason::AfterOuterDocComment { prev_doc_comment_span }) => {
                    let mut diag = self.struct_span_err(
                        attr_sp,
                        "an inner attribute is not permitted following an outer doc comment",
                    );
                    diag.span_label(attr_sp, "not permitted following an outer doc comment")
                        .span_label(prev_doc_comment_span, "previous doc comment");
                    diag
                }
                Some(InnerAttrForbiddenReason::AfterOuterAttribute { prev_outer_attr_sp }) => {
                    let mut diag = self.struct_span_err(
                        attr_sp,
                        "an inner attribute is not permitted following an outer attribute",
                    );
                    diag.span_label(attr_sp, "not permitted following an outer attribute")
                        .span_label(prev_outer_attr_sp, "previous outer attribute");
                    diag
                }
                Some(InnerAttrForbiddenReason::InCodeBlock) | None => self.struct_span_err(
                    attr_sp,
                    "an inner attribute is not permitted in this context",
                ),
            };

            diag.set_arg("item_type", "attribute");
            diag.note("inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files");
            if self
                .annotate_following_item_if_applicable(
                    &mut diag,
                    attr_sp,
                    OuterAttributeType::Attribute,
                )
                .is_some()
            {
                diag.note("outer attributes, like `#[test]`, annotate the item following them");
            };
            diag.emit();
        }
    }

    /// Parses an inner part of an attribute (the path and following tokens).
    /// The tokens must be either a delimited token stream, or empty token stream,
    /// or the "legacy" key-value form.
    ///     PATH `(` TOKEN_STREAM `)`
    ///     PATH `[` TOKEN_STREAM `]`
    ///     PATH `{` TOKEN_STREAM `}`
    ///     PATH
    ///     PATH `=` UNSUFFIXED_LIT
    /// The delimiters or `=` are still put into the resulting token stream.
    pub fn parse_attr_item(&mut self, capture_tokens: bool) -> PResult<'a, ast::AttrItem> {
        let item = match &self.token.kind {
            token::Interpolated(nt) => match &nt.0 {
                Nonterminal::NtMeta(item) => Some(item.clone().into_inner()),
                _ => None,
            },
            _ => None,
        };
        Ok(if let Some(item) = item {
            self.bump();
            item
        } else {
            let do_parse = |this: &mut Self| {
                let path = this.parse_path(PathStyle::Mod)?;
                let args = this.parse_attr_args()?;
                Ok(ast::AttrItem { path, args, tokens: None })
            };
            // Attr items don't have attributes
            if capture_tokens { self.collect_tokens_no_attrs(do_parse) } else { do_parse(self) }?
        })
    }

    /// Parses attributes that appear after the opening of an item. These should
    /// be preceded by an exclamation mark, but we accept and warn about one
    /// terminated by a semicolon.
    ///
    /// Matches `inner_attrs*`.
    pub(crate) fn parse_inner_attributes(&mut self) -> PResult<'a, ast::AttrVec> {
        let mut attrs = ast::AttrVec::new();
        loop {
            let start_pos: u32 = self.num_bump_calls.try_into().unwrap();
            // Only try to parse if it is an inner attribute (has `!`).
            let attr = if self.check(&token::Pound) && self.look_ahead(1, |t| t == &token::Not) {
                Some(self.parse_attribute(InnerAttrPolicy::Permitted)?)
            } else if let token::DocComment(comment_kind, attr_style, data) = self.token.kind {
                if attr_style == ast::AttrStyle::Inner {
                    self.bump();
                    Some(attr::mk_doc_comment(
                        &self.sess.attr_id_generator,
                        comment_kind,
                        attr_style,
                        data,
                        self.prev_token.span,
                    ))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(attr) = attr {
                let end_pos: u32 = self.num_bump_calls.try_into().unwrap();
                // If we are currently capturing tokens, mark the location of this inner attribute.
                // If capturing ends up creating a `LazyAttrTokenStream`, we will include
                // this replace range with it, removing the inner attribute from the final
                // `AttrTokenStream`. Inner attributes are stored in the parsed AST note.
                // During macro expansion, they are selectively inserted back into the
                // token stream (the first inner attribute is removed each time we invoke the
                // corresponding macro).
                let range = start_pos..end_pos;
                if let Capturing::Yes = self.capture_state.capturing {
                    self.capture_state.inner_attr_ranges.insert(attr.id, (range, vec![]));
                }
                attrs.push(attr);
            } else {
                break;
            }
        }
        Ok(attrs)
    }

    // Note: must be unsuffixed.
    pub(crate) fn parse_unsuffixed_meta_item_lit(&mut self) -> PResult<'a, ast::MetaItemLit> {
        let lit = self.parse_meta_item_lit()?;
        debug!("checking if {:?} is unsuffixed", lit);

        if !lit.kind.is_unsuffixed() {
            self.sess.emit_err(SuffixedLiteralInAttribute { span: lit.span });
        }

        Ok(lit)
    }

    /// Parses `cfg_attr(pred, attr_item_list)` where `attr_item_list` is comma-delimited.
    pub fn parse_cfg_attr(&mut self) -> PResult<'a, (ast::MetaItem, Vec<(ast::AttrItem, Span)>)> {
        let cfg_predicate = self.parse_meta_item()?;
        self.expect(&token::Comma)?;

        // Presumably, the majority of the time there will only be one attr.
        let mut expanded_attrs = Vec::with_capacity(1);
        while self.token.kind != token::Eof {
            let lo = self.token.span;
            let item = self.parse_attr_item(true)?;
            expanded_attrs.push((item, lo.to(self.prev_token.span)));
            if !self.eat(&token::Comma) {
                break;
            }
        }

        Ok((cfg_predicate, expanded_attrs))
    }

    /// Matches `COMMASEP(meta_item_inner)`.
    pub(crate) fn parse_meta_seq_top(&mut self) -> PResult<'a, ThinVec<ast::NestedMetaItem>> {
        // Presumably, the majority of the time there will only be one attr.
        let mut nmis = ThinVec::with_capacity(1);
        while self.token.kind != token::Eof {
            nmis.push(self.parse_meta_item_inner()?);
            if !self.eat(&token::Comma) {
                break;
            }
        }
        Ok(nmis)
    }

    /// Matches the following grammar (per RFC 1559).
    /// ```ebnf
    /// meta_item : PATH ( '=' UNSUFFIXED_LIT | '(' meta_item_inner? ')' )? ;
    /// meta_item_inner : (meta_item | UNSUFFIXED_LIT) (',' meta_item_inner)? ;
    /// ```
    pub fn parse_meta_item(&mut self) -> PResult<'a, ast::MetaItem> {
        let nt_meta = match &self.token.kind {
            token::Interpolated(nt) => match &nt.0 {
                token::NtMeta(e) => Some(e.clone()),
                _ => None,
            },
            _ => None,
        };

        if let Some(item) = nt_meta {
            return match item.meta(item.path.span) {
                Some(meta) => {
                    self.bump();
                    Ok(meta)
                }
                None => self.unexpected(),
            };
        }

        let lo = self.token.span;
        let path = self.parse_path(PathStyle::Mod)?;
        let kind = self.parse_meta_item_kind()?;
        let span = lo.to(self.prev_token.span);
        Ok(ast::MetaItem { path, kind, span })
    }

    pub(crate) fn parse_meta_item_kind(&mut self) -> PResult<'a, ast::MetaItemKind> {
        Ok(if self.eat(&token::Eq) {
            ast::MetaItemKind::NameValue(self.parse_unsuffixed_meta_item_lit()?)
        } else if self.check(&token::OpenDelim(Delimiter::Parenthesis)) {
            // Matches `meta_seq = ( COMMASEP(meta_item_inner) )`.
            let (list, _) = self.parse_paren_comma_seq(|p| p.parse_meta_item_inner())?;
            ast::MetaItemKind::List(list)
        } else {
            ast::MetaItemKind::Word
        })
    }

    /// Matches `meta_item_inner : (meta_item | UNSUFFIXED_LIT) ;`.
    fn parse_meta_item_inner(&mut self) -> PResult<'a, ast::NestedMetaItem> {
        match self.parse_unsuffixed_meta_item_lit() {
            Ok(lit) => return Ok(ast::NestedMetaItem::Lit(lit)),
            Err(err) => err.cancel(),
        }

        match self.parse_meta_item() {
            Ok(mi) => return Ok(ast::NestedMetaItem::MetaItem(mi)),
            Err(err) => err.cancel(),
        }

        Err(InvalidMetaItem { span: self.token.span, token: self.token.clone() }
            .into_diagnostic(self.diagnostic()))
    }
}

/// The attributes are complete if all attributes are either a doc comment or a builtin attribute other than `cfg_attr`
pub fn is_complete(attrs: &[ast::Attribute]) -> bool {
    attrs.iter().all(|attr| {
        attr.is_doc_comment()
            || attr.ident().is_some_and(|ident| {
                ident.name != sym::cfg_attr && rustc_feature::is_builtin_attr_name(ident.name)
            })
    })
}
