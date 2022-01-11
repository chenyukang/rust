use crate::{LateContext, LateLintPass, LintContext};
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_hir::def::Res;
use rustc_hir::{GenericArg, PathSegment, QPath, TyKind};
use rustc_middle::ty;
use rustc_span::symbol::sym;

declare_tool_lint! {
    /// The `rustc_pass_by_value` lint marks a type with `#[rustc_pass_by_value]` requiring it to always be passed by value.
    /// This is usually used for types that are thin wrappers around references, so there is no benefit to an extra
    /// layer of indirection. (Example: `Ty` which is a reference to a `TyS`)
    pub rustc::PASS_BY_VALUE,
    Warn,
    "pass by reference of a type flagged as `#[rustc_pass_by_value]`",
    report_in_external_macro: true
}

declare_lint_pass!(PassByValue => [PASS_BY_VALUE]);

impl<'tcx> LateLintPass<'tcx> for PassByValue {
    fn check_ty(&mut self, cx: &LateContext<'_>, ty: &'tcx hir::Ty<'tcx>) {
        match &ty.kind {
            TyKind::Rptr(_, hir::MutTy { ty: inner_ty, mutbl: hir::Mutability::Not }) => {
                if let Some(impl_did) = cx.tcx.impl_of_method(ty.hir_id.owner.to_def_id()) {
                    if cx.tcx.impl_trait_ref(impl_did).is_some() {
                        return;
                    }
                }
                if let Some(t) = path_for_pass_by_value(cx, &inner_ty) {
                    cx.struct_span_lint(PASS_BY_VALUE, ty.span, |lint| {
                        lint.build(&format!("passing `{}` by reference", t))
                            .span_suggestion(
                                ty.span,
                                "try passing by value",
                                t,
                                // Changing type of function argument
                                Applicability::MaybeIncorrect,
                            )
                            .emit();
                    })
                }
            }
            _ => {}
        }
    }
}

fn path_for_pass_by_value(cx: &LateContext<'_>, ty: &hir::Ty<'_>) -> Option<String> {
    if let TyKind::Path(QPath::Resolved(_, path)) = &ty.kind {
        match path.res {
            Res::Def(_, def_id) if cx.tcx.has_attr(def_id, sym::rustc_pass_by_value) => {
                let name = cx.tcx.item_name(def_id).to_ident_string();
                return Some(format!("{}{}", name, gen_args(path.segments.last().unwrap())));
            }
            Res::SelfTy(None, Some((did, _))) => {
                if let ty::Adt(adt, substs) = cx.tcx.type_of(did).kind() {
                    if cx.tcx.has_attr(adt.did, sym::rustc_pass_by_value) {
                        let name = cx.tcx.item_name(adt.did).to_ident_string();
                        let param =
                            substs.first().map(|s| format!("<{}>", s)).unwrap_or("".to_string());
                        return Some(format!("{}{}", name, param));
                    }
                }
            }
            _ => (),
        }
    }

    None
}

fn gen_args(segment: &PathSegment<'_>) -> String {
    if let Some(args) = &segment.args {
        let lifetimes = args
            .args
            .iter()
            .filter_map(|arg| {
                if let GenericArg::Lifetime(lt) = arg {
                    Some(lt.name.ident().to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !lifetimes.is_empty() {
            return format!("<{}>", lifetimes.join(", "));
        }
    }

    String::new()
}
