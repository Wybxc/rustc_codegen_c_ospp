use rustc_codegen_ssa::traits::PreDefineMethods;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::mono::{Linkage, Visibility};
use rustc_middle::ty::Instance;

use crate::context::CodegenCx;

impl<'tcx> PreDefineMethods<'tcx> for CodegenCx<'tcx> {
    fn predefine_static(
        &self,
        def_id: DefId,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    ) {
        todo!()
    }

    fn predefine_fn(
        &self,
        instance: Instance<'tcx>,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    ) {
        self.functions.borrow_mut().insert(symbol_name.to_string(), ());
    }
}
