impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
    pub fn report_method_error(
        &self,
        mut span: Span,
        rcvr_ty: Ty<'tcx>,
        item_name: Ident,
        source: SelfSource<'tcx>,
        error: MethodError<'tcx>,
        args: Option<(&'tcx hir::Expr<'tcx>, &'tcx [hir::Expr<'tcx>])>,
    ) -> Option<DiagnosticBuilder<'_, ErrorGuaranteed>> {
        // Avoid suggestions when we don't know what's going on.
        if rcvr_ty.references_error() {
            return None;
        }

        let report_candidates = |span: Span,
                                 err: &mut Diagnostic,
                                 mut sources: Vec<CandidateSource>,
                                 sugg_span: Span|
        {

        };
    }

    fn check_for_field_method(
        &self,
        err: &mut Diagnostic,
        source: SelfSource<'tcx>,
        span: Span,
        actual: Ty<'tcx>,
        item_name: Ident,
    ) {
        if let SelfSource::MethodCall(expr) = source
        && let mod_id = self.tcx.parent_module(expr.hir_id).to_def_id() {
        && let Some((fields, substs)) =
            self.get_field_candidates_considering_privacy(span, actual, mod_id)
        {
        }
    }
}
