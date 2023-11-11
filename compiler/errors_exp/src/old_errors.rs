use rustc_ast::Path;
use rustc_ast::Ty;
use rustc_errors::Applicability;
use rustc_macros::Diagnostic;
use rustc_macros::Subdiagnostic;
use rustc_span::Span;
// #[derive(Diagnostic)]
// #[diag(exp_maybe_recover_from_bad_type_plus, code = "E0178")]
// pub(crate) struct BadTypePlus {
//     pub ty: String,
//     #[primary_span]
//     pub span: Span,
//     #[subdiagnostic]
//     pub sub: BadTypePlusSub,
// }

// #[derive(Subdiagnostic)]
// pub(crate) enum BadTypePlusSub {
//     #[suggestion(exp_add_paren, code = "{sum_with_parens}", applicability = "machine-applicable")]
//     AddParen {
//         sum_with_parens: String,
//         #[primary_span]
//         span: Span,
//     },
//     #[label(exp_forgot_paren)]
//     ForgotParen {
//         #[primary_span]
//         span: Span,
//     },
//     #[label(exp_expect_path)]
//     ExpectPath {
//         #[primary_span]
//         span: Span,
//     },
// }

// #[derive(Diagnostic)]
// #[diag(exp_maybe_recover_from_bad_qpath_stage_2)]
// pub(crate) struct BadQPathStage2 {
//     #[primary_span]
//     pub span: Span,
//     #[subdiagnostic]
//     pub wrap: WrapType,
// }

// #[derive(Subdiagnostic)]
// #[multipart_suggestion(exp_suggestion, applicability = "machine-applicable")]
// pub(crate) struct WrapType {
//     #[suggestion_part(code = "<")]
//     pub lo: Span,
//     #[suggestion_part(code = ">")]
//     pub hi: Span,
// }

// #[derive(Diagnostic)]
// #[diag(exp_incorrect_semicolon)]
// pub(crate) struct IncorrectSemicolon<'a> {
//     #[primary_span]
//     #[suggestion(style = "short", code = "", applicability = "machine-applicable")]
//     pub span: Span,
//     #[help]
//     pub opt_help: Option<()>,
//     pub name: &'a str,
// }

// #[derive(Diagnostic)]
// #[diag(exp_maybe_recover_from_bad_type_plus, code = "E0178")]
// pub(crate) struct BadTypePlus {
//     pub ty: String,
//     #[primary_span]
//     pub span: Span,
//     #[subdiagnostic]
//     pub sub: BadTypePlusSub,
// }

// #[derive(Subdiagnostic)]
// pub(crate) enum BadTypePlusSub {
//     #[suggestion(exp_add_paren, code = "{sum_with_parens}", applicability = "machine-applicable")]
//     AddParen {
//         sum_with_parens: String,
//         #[primary_span]
//         span: Span,
//     },
//     #[label(exp_forgot_paren)]
//     ForgotParen {
//         #[primary_span]
//         span: Span,
//     },
//     #[label(exp_expect_path)]
//     ExpectPath {
//         #[primary_span]
//         span: Span,
//     },
// }

// #[derive(Diagnostic)]
// #[diag(exp_maybe_recover_from_bad_qpath_stage_2)]
// pub(crate) struct BadQPathStage2 {
//     #[primary_span]
//     pub span: Span,
//     #[subdiagnostic]
//     pub wrap: WrapType,
// }
// #[derive(Subdiagnostic)]
// #[multipart_suggestion(exp_suggestion, applicability = "machine-applicable")]
// pub(crate) struct WrapType {
//     #[suggestion_part(code = "<")]
//     pub lo: Span,
//     #[suggestion_part(code = ">")]
//     pub hi: Span,
// }

// #[derive(Diagnostic)]
// #[diag(exp_incorrect_use_of_await)]
// pub(crate) struct IncorrectUseOfAwait {
//     #[primary_span]
//     #[suggestion(exp_parentheses_suggestion, code = "", applicability = "machine-applicable")]
//     pub span: Span,
// }

// #[derive(Diagnostic)]
// #[diag(exp_incorrect_use_of_await)]
// pub(crate) struct IncorrectAwait {
//     #[primary_span]
//     pub span: Span,
//     #[suggestion(exp_postfix_suggestion, code = "{expr}.await{question_mark}")]
//     pub sugg_span: (Span, Applicability),
//     pub expr: String,
//     pub question_mark: &'static str,
// }

// #[derive(Diagnostic)]
// #[diag(exp_ty_param_first_local, code = "E0210")]
// #[note]
// pub struct TyParamFirstLocal<'a> {
//     #[primary_span]
//     #[label]
//     pub span: Span,
//     #[note(exp_case_note)]
//     pub note: (),
//     pub param_ty: Ty<'a>,
//     pub local_type: Ty<'a>,
// }

// #[derive(Diagnostic)]
// #[diag(exp_invalid_logical_operator)]
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
//         exp_use_amp_amp_for_conjunction,
//         style = "short",
//         applicability = "machine-applicable",
//         code = "&&"
//     )]
//     Conjunction(#[primary_span] Span),
//     #[suggestion(
//         exp_use_pipe_pipe_for_disjunction,
//         style = "short",
//         applicability = "machine-applicable",
//         code = "||"
//     )]
//     Disjunction(#[primary_span] Span),
// }

// #[derive(Diagnostic)]
// #[diag(exp_comparison_interpreted_as_generic)]
// pub(crate) struct ComparisonInterpretedAsGeneric {
//     #[primary_span]
//     #[label(exp_label_comparison)]
//     pub comparison: Span,
//     pub r#type: Path,
//     #[label(exp_label_args)]
//     pub args: Span,
//     #[subdiagnostic]
//     pub suggestion: ComparisonOrShiftInterpretedAsGenericSugg,
// }

// #[derive(Diagnostic)]
// #[diag(exp_shift_interpreted_as_generic)]
// pub(crate) struct ShiftInterpretedAsGeneric {
//     #[primary_span]
//     #[label(exp_label_comparison)]
//     pub shift: Span,
//     pub r#type: Path,
//     #[label(exp_label_args)]
//     pub args: Span,
//     #[subdiagnostic]
//     pub suggestion: ComparisonOrShiftInterpretedAsGenericSugg,
// }

// #[derive(Subdiagnostic)]
// #[multipart_suggestion(exp_suggestion, applicability = "machine-applicable")]
// pub(crate) struct ComparisonOrShiftInterpretedAsGenericSugg {
//     #[suggestion_part(code = "(")]
//     pub left: Span,
//     #[suggestion_part(code = ")")]
//     pub right: Span,
// }

// #[derive(Diagnostic)]
// #[diag(exp_unmatched_angle)]
// pub(crate) struct UnmatchedAngle {
//     #[primary_span]
//     #[suggestion(code = "", applicability = "machine-applicable")]
//     pub span: Span,
//     pub plural: bool,
// }

#[derive(Diagnostic)]
#[diag(exp_expected_else_block)]
pub(crate) struct ExpectedElseBlock {
    #[primary_span]
    pub first_tok_span: Span,
    pub first_tok: String,
    #[label]
    pub else_span: Span,
    #[suggestion(applicability = "maybe-incorrect", code = "if ")]
    pub condition_start: Span,
}
