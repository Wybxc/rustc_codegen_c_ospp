use rustc_codegen_ssa::traits::BaseTypeMethods;
use rustc_type_ir::IntTy;

use crate::context::CodegenCx;

impl<'tcx, 'mx> BaseTypeMethods<'tcx> for CodegenCx<'tcx, 'mx> {
    fn type_i8(&self) -> Self::Type {
        self.mcx.int(IntTy::I8)
    }

    fn type_i16(&self) -> Self::Type {
        self.mcx.int(IntTy::I16)
    }

    fn type_i32(&self) -> Self::Type {
        self.mcx.int(IntTy::I32)
    }

    fn type_i64(&self) -> Self::Type {
        self.mcx.int(IntTy::I64)
    }

    fn type_i128(&self) -> Self::Type {
        self.mcx.int(IntTy::I128)
    }

    fn type_isize(&self) -> Self::Type {
        self.mcx.int(IntTy::Isize)
    }

    fn type_f16(&self) -> Self::Type {
        todo!()
    }

    fn type_f32(&self) -> Self::Type {
        todo!()
    }

    fn type_f64(&self) -> Self::Type {
        todo!()
    }

    fn type_f128(&self) -> Self::Type {
        todo!()
    }

    fn type_array(&self, ty: Self::Type, len: u64) -> Self::Type {
        todo!()
    }

    fn type_func(&self, args: &[Self::Type], ret: Self::Type) -> Self::Type {
        todo!()
    }

    fn type_kind(&self, ty: Self::Type) -> rustc_codegen_ssa::common::TypeKind {
        todo!()
    }

    fn type_ptr(&self) -> Self::Type {
        self.mcx.ptr(self.mcx.void())
    }

    fn type_ptr_ext(&self, address_space: rustc_abi::AddressSpace) -> Self::Type {
        todo!()
    }

    fn element_type(&self, ty: Self::Type) -> Self::Type {
        todo!()
    }

    fn vector_length(&self, ty: Self::Type) -> usize {
        todo!()
    }

    fn float_width(&self, ty: Self::Type) -> usize {
        todo!()
    }

    fn int_width(&self, ty: Self::Type) -> u64 {
        todo!()
    }

    fn val_ty(&self, v: Self::Value) -> Self::Type {
        todo!()
    }
}
