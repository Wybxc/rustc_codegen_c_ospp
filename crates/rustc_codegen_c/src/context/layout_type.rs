use rustc_abi::Abi;
use rustc_codegen_c_ast::r#type::CTy;
use rustc_codegen_ssa::traits::LayoutTypeMethods;
use rustc_middle::ty::layout::TyAndLayout;
use rustc_middle::ty::Ty;
use rustc_target::abi::call::FnAbi;
use rustc_type_ir::TyKind;

use crate::context::CodegenCx;

impl<'tcx, 'mx> CodegenCx<'tcx, 'mx> {
    fn get_cty(&self, ty: Ty<'tcx>) -> CTy<'mx> {
        match ty.kind() {
            TyKind::Bool => todo!(),
            TyKind::Char => todo!(),
            TyKind::Int(int) => self.mcx.get_int_type(*int),
            TyKind::Uint(uint) => self.mcx.get_uint_type(*uint),
            TyKind::Float(_) => todo!(),
            TyKind::Adt(_, _) => self.mcx.get_void_type(), // TODO: struct
            TyKind::Foreign(_) => todo!(),
            TyKind::Str => self.mcx.get_char_type(),
            TyKind::Array(_, _) => todo!(),
            TyKind::Pat(_, _) => todo!(),
            TyKind::Slice(_) => todo!(),
            TyKind::RawPtr(_, _) => todo!(),
            TyKind::Ref(_, ty, _) => self.mcx.get_ptr_type(self.get_cty(*ty)),
            TyKind::FnDef(_, _) => todo!(),
            TyKind::FnPtr(_) => todo!(),
            TyKind::Dynamic(_, _, _) => todo!(),
            TyKind::Closure(_, _) => todo!(),
            TyKind::CoroutineClosure(_, _) => todo!(),
            TyKind::Coroutine(_, _) => todo!(),
            TyKind::CoroutineWitness(_, _) => todo!(),
            TyKind::Never => self.mcx.get_void_type(),
            TyKind::Tuple(_) => todo!(),
            TyKind::Alias(_, _) => todo!(),
            TyKind::Param(_) => todo!(),
            TyKind::Bound(_, _) => todo!(),
            TyKind::Placeholder(_) => todo!(),
            TyKind::Infer(_) => todo!(),
            TyKind::Error(_) => todo!(),
        }
    }
}

impl<'tcx, 'mx> LayoutTypeMethods<'tcx> for CodegenCx<'tcx, 'mx> {
    fn backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        self.get_cty(layout.ty)
    }

    fn cast_backend_type(&self, ty: &rustc_target::abi::call::CastTarget) -> Self::Type {
        todo!()
    }

    fn fn_decl_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        todo!()
    }

    fn fn_ptr_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        todo!()
    }

    fn reg_backend_type(&self, ty: &rustc_target::abi::call::Reg) -> Self::Type {
        todo!()
    }

    fn immediate_backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        self.get_cty(layout.ty)
    }

    fn is_backend_immediate(&self, layout: TyAndLayout<'tcx>) -> bool {
        match layout.abi {
            Abi::Scalar(_) | Abi::Vector { .. } => true,
            Abi::ScalarPair(..) | Abi::Uninhabited | Abi::Aggregate { .. } => false,
        }
    }

    fn is_backend_scalar_pair(&self, layout: TyAndLayout<'tcx>) -> bool {
        // match layout.abi {
        //     Abi::ScalarPair(..) => true,
        //     Abi::Uninhabited | Abi::Scalar(_) | Abi::Vector { .. } | Abi::Aggregate { .. } => false,
        // }
        false
    }

    fn scalar_pair_element_backend_type(
        &self,
        layout: TyAndLayout<'tcx>,
        index: usize,
        immediate: bool,
    ) -> Self::Type {
        todo!()
    }
}
