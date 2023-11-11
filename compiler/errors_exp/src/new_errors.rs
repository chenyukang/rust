use rustc_ast::Path;
use rustc_ast::Ty;
use rustc_errors::Applicability;
use rustc_macros::Diagnostic;
use rustc_macros::Subdiagnostic;
use rustc_span::Span;
// #[derive(DiagnosticNew)]
// #[diag("this is the diag message")]
// #[code_error("E0178")]
// pub(crate) struct BadTypePlusNew {
//     pub ty: String,
//     #[primary_span]
//     pub span: Span,
//     #[subdiagnostic]
//     pub sub: BadTypePlusSubNew,
// }

// #[derive(SubdiagnosticNew)]
// pub(crate) enum BadTypePlusSubNew {
//     #[suggestion(
//         label = "try adding parentheses",
//         code = "{sum_with_parens}",
//         applicability = "machine-applicable"
//     )]
//     AddParen {
//         sum_with_parens: String,
//         #[primary_span]
//         span: Span,
//     },
//     #[label("perhaps you forgot parentheses?")]
//     ForgotParen {
//         #[primary_span]
//         span: Span,
//     },
//     #[label("expected a path")]
//     ExpectPath {
//         #[primary_span]
//         span: Span,
//     },
// }

// #[derive(DiagnosticNew)]
// #[diag("missing angle brackets in associated item path")]
// #[suggestion(
//     "types that don't start with an identifier need to be surrounded with angle brackets in qualified paths"
// )]
// pub(crate) struct BadQPathStage2 {
//     #[primary_span]
//     pub span: Span,
//     #[subdiagnostic]
//     pub wrap: WrapType,
// }

// #[derive(SubdiagnosticNew)]
// #[multipart_suggestion(
//     applicability = "machine-applicable",
//     label = "types that don't start with an identifier need to be surrounded with angle brackets in qualified paths"
// )]
// pub(crate) struct WrapType {
//     #[suggestion_part(code = "<")]
//     pub lo: Span,
//     #[suggestion_part(code = ">")]
//     pub hi: Span,
// }

// #[derive(Diagnostic)]
// #[diag(exp_comma_after_base_struct)]
// #[note]
// pub(crate) struct CommaAfterBaseStruct {
//     #[primary_span]
//     pub span: Span,
//     #[suggestion(style = "short", applicability = "machine-applicable", code = "test now")]
//     pub comma: Span,
// }

// #[derive(DiagnosticNew)]
// #[diag("cannot use a comma after the base struct")]
// #[note("the base struct must always be the last field")]
// pub(crate) struct CommaAfterBaseStructNew {
//     #[primary_span]
//     pub span: Span,
//     #[suggestion(
//         style = "short",
//         applicability = "machine-applicable",
//         code = "test now new error",
//         label = "remove this comma"
//     )]
//     pub comma: Span,
// }

// #[derive(Diagnostic)]
// #[diag(exp_maybe_report_ambiguous_plus)]
// pub(crate) struct AmbiguousPlus {
//     pub sum_ty: String,
//     #[primary_span]
//     #[suggestion(code = "({sum_ty})")]
//     pub span: Span,
// }

// #[derive(DiagnosticNew)]
// #[diag("ambiguous `+` in a type")]
// pub(crate) struct AmbiguousPlusNew {
//     pub sum_ty: String,
//     #[primary_span]
//     #[suggestion(code = "({sum_ty})", label = "use parentheses to disambiguate")]
//     pub span: Span,
// }

// #[derive(DiagnosticNew)]
// #[diag(
//     text = "expected item, found `;`",
//     suggestion = "remove this semicolon",
//     help = "{$name} declarations are not followed by a semicolon"
// )]
// pub(crate) struct IncorrectSemicolon<'a> {
//     #[primary_span]
//     #[suggestion(style = "short", code = "", applicability = "machine-applicable")]
//     pub span: Span,
//     #[help]
//     pub opt_help: Option<()>,
//     pub name: &'a str,
// }

// #[derive(Diagnostic)]
// #[diag(text = "expected a path on the left-hand side of `+`, not `{$ty}`", code = "E0178")]
// pub(crate) struct BadTypePlus {
//     pub ty: String,
//     #[primary_span]
//     pub span: Span,
//     #[subdiagnostic]
//     pub sub: BadTypePlusSub,
// }

// #[derive(Subdiagnostic)]
// pub(crate) enum BadTypePlusSub {
//     #[suggestion(
//         label = "try adding parentheses",
//         code = "{sum_with_parens}",
//         applicability = "machine-applicable"
//     )]
//     AddParen {
//         sum_with_parens: String,
//         #[primary_span]
//         span: Span,
//     },
//     #[label("perhaps you forgot parentheses?")]
//     ForgotParen {
//         #[primary_span]
//         span: Span,
//     },
//     #[label("expected a path")]
//     ExpectPath {
//         #[primary_span]
//         span: Span,
//     },
// }

// #[derive(Diagnostic)]
// #[diag(text = "missing angle brackets in associated item path")]
// pub(crate) struct BadQPathStage2 {
//     #[primary_span]
//     pub span: Span,
//     #[subdiagnostic]
//     pub wrap: WrapType,
// }

// #[derive(Subdiagnostic)]
// #[multipart_suggestion(
//     label = "types that don't start with an identifier need to be surrounded with angle brackets in qualified paths",
//     applicability = "machine-applicable"
// )]
// pub(crate) struct WrapType {
//     #[suggestion_part(code = "<")]
//     pub lo: Span,
//     #[suggestion_part(code = ">")]
//     pub hi: Span,
// }

// #[derive(Diagnostic)]
// #[diag(text = "incorrect use of `await`")]
// pub(crate) struct IncorrectUseOfAwait {
//     #[primary_span]
//     #[suggestion(
//         label = "`await` is not a method call, remove the parentheses",
//         code = "",
//         applicability = "machine-applicable"
//     )]
//     pub span: Span,
// }

// #[derive(Diagnostic)]
// #[diag(text = "incorrect use of `await`")]
// pub(crate) struct IncorrectAwait {
//     #[primary_span]
//     pub span: Span,
//     #[suggestion(label = "`await` is a postfix operation", code = "{expr}.await{question_mark}")]
//     pub sugg_span: (Span, Applicability),
//     pub expr: String,
//     pub question_mark: &'static str,
// }

// #[derive(Diagnostic)]
// #[diag(
//     text = "expected item, found `;`",
//     help = "{$name} declarations are not followed by a semicolon"
// )]
// pub(crate) struct IncorrectSemicolon<'a> {
//     #[primary_span]
//     #[suggestion(
//         label = "remove this semicolon",
//         style = "short",
//         code = "",
//         applicability = "machine-applicable"
//     )]
//     pub span: Span,
//     #[help]
//     pub opt_help: Option<()>,
//     pub name: &'a str,
// }

// #[derive(Diagnostic)]
// #[diag(text = "expected a path on the left-hand side of `+`, not `{$ty}`", code = "E0178")]
// pub(crate) struct BadTypePlus {
//     pub ty: String,
//     #[primary_span]
//     pub span: Span,
//     #[subdiagnostic]
//     pub sub: BadTypePlusSub,
// }

// #[derive(Diagnostic)]
// #[diag(
//     text = "type parameter `{$param_ty}` must be covered by another type when it appears before the first local type (`{$local_type}`)",
//     code = "E0210",
//     note = "implementing a foreign trait is only possible if at least one of the types for which it is implemented is local, and no uncovered type parameters appear before that first local type"
// )]
// pub struct TyParamFirstLocal<'a> {
//     #[primary_span]
//     #[label]
//     pub span: Span,
//     #[note(
//         "in this case, 'before' refers to the following order: `impl<..> ForeignTrait<T1, ..., Tn> for T0`, where `T0` is the first and `Tn` is the last"
//     )]
//     pub note: (),
//     pub param_ty: Ty<'a>,
//     pub local_type: Ty<'a>,
// }

// #[derive(Diagnostic)]
// #[diag(
//     text = "`{$incorrect}` is not a logical operator",
//     note = "unlike in e.g., Python and PHP, `&&` and `||` are used for logical operators, {$incorrect}"
// )]
// #[note]
// pub(crate) struct InvalidLogicalOperator {
//     #[primary_span]
//     pub span: Span,
//     pub incorrect: String,
//     #[subdiagnostic]
//     pub sub: InvalidLogicalOperatorSub,
// }

// #[derive(Subdiagnostic)]
// pub(crate) enum InvalidLogicalOperatorSub {
//     #[suggestion(
//         label = "use `&&` to perform logical conjunction",
//         style = "short",
//         applicability = "machine-applicable",
//         code = "&&"
//     )]
//     Conjunction(#[primary_span] Span),
//     #[suggestion(
//         label = "use `||` to perform logical disjunction",
//         style = "short",
//         applicability = "machine-applicable",
//         code = "||"
//     )]
//     Disjunction(#[primary_span] Span),
// }

// #[derive(Diagnostic)]
// #[diag(
//     text = "expected item, found `;`",
//     help = "{$name} declarations are not followed by a semicolon"
// )]
// pub(crate) struct IncorrectSemicolon<'a> {
//     #[primary_span]
//     #[suggestion(
//         label = "remove this semicolon",
//         style = "short",
//         code = "",
//         applicability = "machine-applicable"
//     )]
//     pub span: Span,
//     #[help]
//     pub opt_help: Option<()>,
//     pub name: &'a str,
// }

// #[derive(Diagnostic)]
// #[diag(
//     text = "`<` is interpreted as a start of generic arguments for `{$type}`, not a comparison",
//     suggestion = "try comparing the cast value"
// )]
// pub(crate) struct ComparisonInterpretedAsGeneric {
//     #[primary_span]
//     #[label("not interpreted as comparison")]
//     pub comparison: Span,
//     pub r#type: Path,
//     #[label("interpreted as generic arguments")]
//     pub args: Span,
//     #[subdiagnostic]
//     pub suggestion: ComparisonOrShiftInterpretedAsGenericSugg,
// }

// #[derive(Diagnostic)]
// #[diag(
//     text = "`<<` is interpreted as a start of generic arguments for `{$type}`, not a shift",
//     suggestion = "try shifting the cast value"
// )]
// pub(crate) struct ShiftInterpretedAsGeneric {
//     #[primary_span]
//     #[label("not interpreted as shift")]
//     pub shift: Span,
//     pub r#type: Path,
//     #[label("interpreted as generic arguments")]
//     pub args: Span,
//     #[subdiagnostic]
//     pub suggestion: ComparisonOrShiftInterpretedAsGenericSugg,
// }

// #[derive(Subdiagnostic)]
// #[multipart_suggestion(label = "{?????}", applicability = "machine-applicable")]
// pub(crate) struct ComparisonOrShiftInterpretedAsGenericSugg {
//     #[suggestion_part(code = "(")]
//     pub left: Span,
//     #[suggestion_part(code = ")")]
//     pub right: Span,
// }

// #[derive(Diagnostic)]
// #[diag(text = "unmatched angle {$plural ->
//     [true] brackets
//     *[false] bracket
//     }")]
// pub(crate) struct UnmatchedAngle {
//     #[primary_span]
//     #[suggestion(
//         label = "remove extra angle {$plural ->
//             [true] brackets
//             *[false] bracket
//             }",
//         code = "",
//         applicability = "machine-applicable"
//     )]
//     pub span: Span,
//     pub plural: bool,
// }

#[derive(Diagnostic)]
#[diag(text = "expected found {$first_tok}")]
pub(crate) struct ExpectedElseBlock {
    #[primary_span]
    pub first_tok_span: Span,
    pub first_tok: String,
    #[label("expected an `if` or a block after this `else`")]
    pub else_span: Span,
    #[suggestion(
        label = "add an `if` if this is the condition of a chained `else if` statement",
        applicability = "maybe-incorrect",
        code = "if "
    )]
    pub condition_start: Span,
}
