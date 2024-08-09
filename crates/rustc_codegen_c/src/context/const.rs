use rustc_codegen_c_ast::expr::CValue;
use rustc_codegen_c_ast::r#type::{CTy, CTyKind};
use rustc_codegen_ssa::traits::ConstMethods;
use rustc_const_eval::interpret::{ConstAllocation, GlobalAlloc, Scalar};
use rustc_type_ir::UintTy;

use crate::context::CodegenCx;

impl<'tcx, 'mx> ConstMethods<'tcx> for CodegenCx<'tcx, 'mx> {
    fn const_null(&self, t: Self::Type) -> Self::Value {
        match t {
            CTy::Primitive(_) => todo!(),
            CTy::Ref(tkd) => match tkd.0 {
                CTyKind::Pointer(_) => (CValue::Null, t),
                _ => todo!(),
            },
        }
    }

    fn const_undef(&self, t: Self::Type) -> Self::Value {
        self.const_null(t)
    }

    fn const_poison(&self, t: Self::Type) -> Self::Value {
        todo!()
    }

    fn const_int(&self, t: Self::Type, i: i64) -> Self::Value {
        todo!()
    }

    fn const_uint(&self, t: Self::Type, i: u64) -> Self::Value {
        todo!()
    }

    fn const_uint_big(&self, t: Self::Type, u: u128) -> Self::Value {
        todo!()
    }

    fn const_bool(&self, val: bool) -> Self::Value {
        todo!()
    }

    fn const_i16(&self, i: i16) -> Self::Value {
        todo!()
    }

    fn const_i32(&self, i: i32) -> Self::Value {
        todo!()
    }

    fn const_i8(&self, i: i8) -> Self::Value {
        todo!()
    }

    fn const_u32(&self, i: u32) -> Self::Value {
        todo!()
    }

    fn const_u64(&self, i: u64) -> Self::Value {
        todo!()
    }

    fn const_u128(&self, i: u128) -> Self::Value {
        todo!()
    }

    fn const_usize(&self, i: u64) -> Self::Value {
        (self.mcx.scalar(i as i128), self.mcx.uint(UintTy::Usize))
    }

    fn const_u8(&self, i: u8) -> Self::Value {
        todo!()
    }

    fn const_real(&self, t: Self::Type, val: f64) -> Self::Value {
        todo!()
    }

    fn const_str(&self, s: &str) -> (Self::Value, Self::Value) {
        todo!()
    }

    fn const_struct(&self, elts: &[Self::Value], packed: bool) -> Self::Value {
        todo!()
    }

    fn const_to_opt_uint(&self, v: Self::Value) -> Option<u64> {
        todo!()
    }

    fn const_to_opt_u128(&self, v: Self::Value, sign_ext: bool) -> Option<u128> {
        None // TODO: why?
    }

    fn const_data_from_alloc(&self, alloc: ConstAllocation<'tcx>) -> Self::Value {
        todo!()
    }

    fn scalar_to_backend(
        &self,
        cv: Scalar,
        layout: rustc_target::abi::Scalar,
        ty: Self::Type,
    ) -> Self::Value {
        match cv {
            Scalar::Int(scalar) => (self.mcx.scalar(scalar.to_int(scalar.size())), ty),
            Scalar::Ptr(ptr, _) => {
                let (prov, offset) = ptr.into_parts(); // we know the `offset` is relative
                assert!(offset.bytes() == 0, "TODO");
                let alloc_id = prov.alloc_id();
                let base_addr = match self.tcx.global_alloc(alloc_id) {
                    GlobalAlloc::Function(_) => todo!(),
                    GlobalAlloc::VTable(_, _) => todo!(),
                    GlobalAlloc::Static(_) => todo!(),
                    GlobalAlloc::Memory(alloc) => {
                        let alloc = alloc.inner();
                        assert!(alloc.provenance().ptrs().is_empty(), "TODO");
                        let bytes =
                            alloc.inspect_with_uninit_and_ptr_outside_interpreter(0..alloc.len());

                        let mcx = self.mcx;
                        let var = mcx.next_global_var();
                        mcx.module().push_decl(
                            mcx.var(
                                var,
                                mcx.arr(mcx.char(), alloc.len()),
                                Some(
                                    mcx.init_list(
                                        bytes
                                            .iter()
                                            .map(|&b| mcx.value(mcx.scalar(b as i128)))
                                            .collect::<Box<[_]>>(),
                                    ),
                                ),
                            ),
                        );
                        var
                    }
                };
                (base_addr, ty) // this may create a char* implicitly casted to other pointers
            }
        }
    }

    fn const_ptr_byte_offset(
        &self,
        val: Self::Value,
        offset: rustc_target::abi::Size,
    ) -> Self::Value {
        todo!()
    }
}
