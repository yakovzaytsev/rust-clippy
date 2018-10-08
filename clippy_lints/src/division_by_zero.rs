//! lint when there is an enum with no variants

use crate::rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use crate::rustc::{declare_tool_lint, lint_array};
use crate::rustc::hir::*;
use crate::utils::span_lint_and_then;

/// **What it does:** Checks for `enum`s with no variants.
///
/// **Why is this bad?** Enum's with no variants should be replaced with `!`,
/// the uninhabited type,
/// or a wrapper around it.
///
/// **Known problems:** None.
///
/// **Example:**
/// ```rust
/// enum Test {}
/// ```
declare_clippy_lint! {
    pub DIVISION_BY_ZERO,
    pedantic,
    "division by zero"
}

#[derive(Copy, Clone)]
pub struct DivisionByZero;

impl LintPass for DivisionByZero {
    fn get_lints(&self) -> LintArray {
        lint_array!(DIVISION_BY_ZERO)
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for DivisionByZero {
    fn check_item(&mut self, cx: &LateContext<'_, '_>, item: &Item) {
        let did = cx.tcx.hir.local_def_id(item.id);
        if let ItemKind::Enum(..) = item.node {
            let ty = cx.tcx.type_of(did);
            let adt = ty.ty_adt_def()
                .expect("already checked whether this is an enum");
            if adt.variants.is_empty() {
                span_lint_and_then(cx, DIVISION_BY_ZERO, item.span, "division by zero", |db| {
                    db.span_help(item.span, "consider using the uninhabited type `!` or a wrapper around it");
                });
            }
        }
    }
}
