use rustc_abi::HasDataLayout;
use rustc_codegen_c_ast::expr::CValue;
use rustc_codegen_c_ast::r#type::{CTy, CTyBase, CTyKind};
use rustc_codegen_ssa::traits::ConstMethods;
use rustc_const_eval::interpret::{ConstAllocation, GlobalAlloc, Scalar};
use rustc_type_ir::UintTy;

use crate::context::CodegenCx;

impl<'tcx, 'mx> ConstMethods<'tcx> for CodegenCx<'tcx, 'mx> {
    fn const_null(&self, t: Self::Type) -> Self::Value {
        match t.resolve().base {
            CTyBase::Primitive(_) => todo!(),
            CTyBase::Ref(tkd) => match tkd.0 {
                CTyKind::Pointer(_) => (CValue::Null, t).into(),
                CTyKind::Struct { .. } => (CValue::Default(t), t).into(),
                _ => todo!(),
            },
        }
    }

    fn const_undef(&self, t: Self::Type) -> Self::Value {
        self.const_null(t)
    }

    fn const_poison(&self, t: Self::Type) -> Self::Value {
        self.const_undef(t)
    }

    fn const_int(&self, t: Self::Type, i: i64) -> Self::Value {
        todo!()
    }

    fn const_uint(&self, t: Self::Type, i: u64) -> Self::Value {
        todo!()
    }

    fn const_uint_big(&self, t: Self::Type, u: u128) -> Self::Value {
        let val = self.mcx.scalar(u as i128); // TODO: overflow check
        (val, t).into()
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
        (self.mcx.scalar(i as i128), self.mcx.uint(UintTy::Usize)).into()
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
            Scalar::Int(scalar) => (self.mcx.scalar(scalar.to_int(scalar.size())), ty).into(),
            Scalar::Ptr(ptr, _) => {
                let (prov, offset) = ptr.into_parts(); // we know the `offset` is relative
                assert!(offset.bytes() == 0, "TODO");
                let alloc_id = prov.alloc_id();
                let (base_addr, ty) = match self.tcx.global_alloc(alloc_id) {
                    GlobalAlloc::Function(_) => todo!(),
                    GlobalAlloc::VTable(_, _) => todo!(),
                    GlobalAlloc::Static(_) => todo!(),
                    GlobalAlloc::Memory(alloc) => self.const_alloc(alloc),
                };
                (base_addr, ty).into() // this may create a char* implicitly casted to other pointers
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

impl<'tcx, 'mx> CodegenCx<'tcx, 'mx> {
    pub fn const_alloc(&self, alloc: ConstAllocation<'tcx>) -> (CValue<'mx>, CTy<'mx>) {
        let alloc = alloc.inner();
        let mut chunks = Vec::with_capacity(alloc.provenance().ptrs().len() + 1);
        let dl = self.data_layout();
        let pointer_size = dl.pointer_size.bytes() as usize;

        let mut next_offset = 0;
        for &(offset, prov) in alloc.provenance().ptrs().iter() {
            let alloc_id = prov.alloc_id();
            let offset = offset.bytes();
            assert_eq!(offset as usize as u64, offset);
            let offset = offset as usize;

            if offset > next_offset {
                let bytes =
                    alloc.inspect_with_uninit_and_ptr_outside_interpreter(next_offset..offset);
                chunks.push(bytes);
            }
            next_offset = offset + pointer_size;
        }
        if alloc.len() >= next_offset {
            let range = next_offset..alloc.len();
            let bytes = alloc.inspect_with_uninit_and_ptr_outside_interpreter(range);
            chunks.push(bytes);
        }

        assert!(chunks.len() == 1, "TODO");
        let mcx = self.mcx;
        let var = mcx.next_global_var();
        let ty = self.mcx.arr(self.mcx.char().to_const_if(alloc.mutability.is_not()), None);
        mcx.module().push_decl(mcx.var(
            var,
            ty,
            Some(mcx.init_list(
                chunks[0].iter().map(|&b| mcx.value(mcx.scalar(b as i128))).collect::<Box<[_]>>(),
            )),
        ));
        (var, ty)
    }
}
