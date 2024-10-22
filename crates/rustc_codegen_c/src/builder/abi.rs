use rustc_codegen_ssa::mir::place::PlaceRef;
use rustc_codegen_ssa::traits::{AbiBuilderMethods, ArgAbiMethods};
use rustc_middle::ty::Ty;
use rustc_target::abi::call::ArgAbi;

use crate::builder::Builder;

impl<'tcx, 'mx> AbiBuilderMethods<'tcx> for Builder<'_, 'tcx, 'mx> {
    fn get_param(&mut self, index: usize) -> Self::Value {
        let val = self.func.params[index];
        let ty = self.func.fn_ptr().args[index];
        (val, ty).into()
    }
}

impl<'tcx, 'mx> ArgAbiMethods<'tcx> for Builder<'_, 'tcx, 'mx> {
    fn store_fn_arg(
        &mut self,
        arg_abi: &ArgAbi<'tcx, Ty<'tcx>>,
        idx: &mut usize,
        dst: PlaceRef<'tcx, Self::Value>,
    ) {
        todo!()
    }

    fn store_arg(
        &mut self,
        arg_abi: &ArgAbi<'tcx, Ty<'tcx>>,
        val: Self::Value,
        dst: PlaceRef<'tcx, Self::Value>,
    ) {
        todo!()
    }

    fn arg_memory_ty(&self, arg_abi: &ArgAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        todo!()
    }
}
