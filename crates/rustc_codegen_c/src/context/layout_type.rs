use rustc_abi::{Abi, Integer, Primitive};
use rustc_codegen_c_ast::r#type::CTy;
use rustc_codegen_ssa::traits::LayoutTypeMethods;
use rustc_middle::ty::layout::TyAndLayout;
use rustc_middle::ty::Ty;
use rustc_target::abi::call::{Conv, FnAbi, PassMode};
use rustc_type_ir::{IntTy, TyKind, UintTy};

use crate::context::CodegenCx;

impl<'tcx, 'mx> CodegenCx<'tcx, 'mx> {
    fn get_cty(&self, ty: Ty<'tcx>, abi: Conv) -> CTy<'mx> {
        match abi {
            Conv::C => self.get_cty_cabi(ty),
            Conv::Rust => self.get_cty_rabi(ty),
            _ => todo!(),
        }
    }

    fn get_cty_rabi(&self, ty: Ty<'tcx>) -> CTy<'mx> {
        match ty.kind() {
            TyKind::Bool => self.mcx.bool(),
            TyKind::Char => self.mcx.int(IntTy::I32),
            TyKind::Int(int) => self.mcx.int(*int),
            TyKind::Uint(uint) => self.mcx.uint(*uint),
            TyKind::Float(_) => todo!(),
            TyKind::Adt(_, _) => self.mcx.void(), // TODO: struct
            TyKind::Foreign(_) => todo!(),
            TyKind::Str => self.mcx.char(),
            TyKind::Array(_, _) => todo!(),
            TyKind::Pat(_, _) => todo!(),
            TyKind::Slice(_) => todo!(),
            TyKind::RawPtr(_, _) => self.mcx.int(IntTy::Isize),
            TyKind::Ref(_, ty, m) => {
                let mut ty = self.mcx.ptr(self.get_cty_rabi(*ty));
                if m.is_not() {
                    ty = ty.to_const();
                }
                ty
            }
            TyKind::FnDef(_, _) => todo!(),
            TyKind::FnPtr(_) => todo!(),
            TyKind::Dynamic(_, _, _) => todo!(),
            TyKind::Closure(_, _) => todo!(),
            TyKind::CoroutineClosure(_, _) => todo!(),
            TyKind::Coroutine(_, _) => todo!(),
            TyKind::CoroutineWitness(_, _) => todo!(),
            TyKind::Never => self.mcx.void(),
            TyKind::Tuple(t) => {
                if t.is_empty() {
                    self.mcx.void()
                } else {
                    todo!()
                }
            }
            TyKind::Alias(_, _) => todo!(),
            TyKind::Param(_) => todo!(),
            TyKind::Bound(_, _) => todo!(),
            TyKind::Placeholder(_) => todo!(),
            TyKind::Infer(_) => todo!(),
            TyKind::Error(_) => todo!(),
        }
    }

    /// Get the C type of the given type, used in `extern "C"` functions signatures
    fn get_cty_cabi(&self, ty: Ty<'tcx>) -> CTy<'mx> {
        match ty.kind() {
            TyKind::Bool => self.mcx.bool(),
            TyKind::Char => self.mcx.int(IntTy::I32),
            TyKind::Int(int) => self.mcx.int(*int),
            TyKind::Uint(uint) => self.mcx.uint(*uint),
            TyKind::RawPtr(ty, m) => {
                let mut ty = self.get_cty_cabi(*ty);
                if m.is_not() {
                    ty = ty.to_const();
                }
                self.mcx.ptr(ty)
            }

            _ => todo!(), // note: some cases are unreachable
        }
    }
}

impl<'tcx, 'mx> LayoutTypeMethods<'tcx> for CodegenCx<'tcx, 'mx> {
    fn backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        self.get_cty_rabi(layout.ty)
    }

    fn cast_backend_type(&self, ty: &rustc_target::abi::call::CastTarget) -> Self::Type {
        todo!()
    }

    fn fn_decl_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        assert!(!fn_abi.c_variadic, "TODO: variadic parameters");

        let mut args = Vec::with_capacity(fn_abi.args.len());
        for arg in &fn_abi.args {
            match arg.mode {
                PassMode::Ignore => continue,
                PassMode::Direct(_) => args.push(self.get_cty(arg.layout.ty, fn_abi.conv)),
                PassMode::Pair(_, _) => {
                    args.push(self.scalar_pair_element_backend_type(arg.layout, 0, false));
                    args.push(self.scalar_pair_element_backend_type(arg.layout, 1, false));
                }
                PassMode::Cast { .. } => todo!(),
                PassMode::Indirect { .. } => todo!(),
            }
        }
        let ret = self.get_cty(fn_abi.ret.layout.ty, fn_abi.conv);
        self.mcx.fn_ptr(ret, args.into(), fn_abi.conv)
    }

    fn fn_ptr_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        todo!()
    }

    fn reg_backend_type(&self, ty: &rustc_target::abi::call::Reg) -> Self::Type {
        todo!()
    }

    fn immediate_backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        self.get_cty_rabi(layout.ty)
    }

    fn is_backend_immediate(&self, layout: TyAndLayout<'tcx>) -> bool {
        match layout.abi {
            Abi::Scalar(_) | Abi::Vector { .. } => true,
            Abi::ScalarPair(..) | Abi::Uninhabited | Abi::Aggregate { .. } => false,
        }
    }

    fn is_backend_scalar_pair(&self, layout: TyAndLayout<'tcx>) -> bool {
        match layout.abi {
            Abi::ScalarPair(..) => true,
            Abi::Uninhabited | Abi::Scalar(_) | Abi::Vector { .. } | Abi::Aggregate { .. } => false,
        }
    }

    fn scalar_pair_element_backend_type(
        &self,
        layout: TyAndLayout<'tcx>,
        index: usize,
        immediate: bool,
    ) -> Self::Type {
        let (a, b) = match layout.abi {
            Abi::ScalarPair(ref a, ref b) => (a, b),
            _ => panic!("scalar_pair_element_backend_type({:?}): not applicable", layout),
        };
        let scalar = [a, b][index];

        match scalar.primitive() {
            Primitive::Int(int, true) => self.mcx.int(match int {
                Integer::I8 => IntTy::I8,
                Integer::I16 => IntTy::I16,
                Integer::I32 => IntTy::I32,
                Integer::I64 => IntTy::I64,
                Integer::I128 => IntTy::I128,
            }),
            Primitive::Int(int, false) => self.mcx.uint(match int {
                Integer::I8 => UintTy::U8,
                Integer::I16 => UintTy::U16,
                Integer::I32 => UintTy::U32,
                Integer::I64 => UintTy::U64,
                Integer::I128 => UintTy::U128,
            }),
            Primitive::Float(_) => todo!(),
            Primitive::Pointer(_) => self.mcx.ptr(self.mcx.void()),
        }
    }
}
