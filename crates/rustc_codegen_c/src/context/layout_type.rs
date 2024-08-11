use rustc_abi::{Abi, Integer, Primitive};
use rustc_codegen_c_ast::r#type::CTy;
use rustc_codegen_ssa::traits::LayoutTypeMethods;
use rustc_middle::ty::layout::{HasParamEnv, TyAndLayout};
use rustc_middle::ty::Ty;
use rustc_target::abi::call::{Conv, FnAbi, PassMode};
use rustc_type_ir::{IntTy, TyKind, UintTy};

use crate::context::CodegenCx;

impl<'tcx, 'mx> CodegenCx<'tcx, 'mx> {
    pub fn layout_of(&self, ty: Ty<'tcx>) -> TyAndLayout<'tcx> {
        self.tcx.layout_of(self.param_env().and(ty)).unwrap()
    }

    fn get_cty(&self, layout: TyAndLayout<'tcx>, abi: Conv) -> CTy<'mx> {
        match layout.abi {
            Abi::Uninhabited => self.mcx.void(),
            Abi::Scalar(scalar) => self.get_cty_scalar(layout.ty, abi),
            Abi::ScalarPair(_, _) => todo!(),
            Abi::Vector { element, count } => todo!(),
            Abi::Aggregate { sized } => self.get_cty_agg(layout, abi),
        }
    }

    fn get_cty_scalar(&self, ty: Ty<'tcx>, abi: Conv) -> CTy<'mx> {
        let mcx = self.mcx;
        match ty.kind() {
            TyKind::Bool => mcx.bool(),
            TyKind::Char => mcx.int(IntTy::I32),
            TyKind::Int(int) => mcx.int(*int),
            TyKind::Uint(uint) => mcx.uint(*uint),
            TyKind::Float(_) => todo!(),
            TyKind::Ref(_, ty, m) => {
                mcx.ptr(self.get_cty(self.layout_of(*ty), abi)).to_const_if(m.is_not())
            }
            TyKind::RawPtr(ty, m) => match abi {
                Conv::C => mcx.ptr(self.get_cty(self.layout_of(*ty), abi)).to_const_if(m.is_not()),
                Conv::Rust => mcx.int(IntTy::Isize),
                _ => todo!(),
            },
            TyKind::Adt(def, args) => {
                if self.tcx.lang_items().c_void().is_some_and(|void| def.did() == void) {
                    self.mcx.void()
                } else {
                    todo!()
                }
            }
            _ => todo!(),
        }
    }

    fn get_cty_agg(&self, layout: TyAndLayout<'tcx>, abi: Conv) -> CTy<'mx> {
        let mcx = self.mcx;
        let ty = self.tcx.erase_regions(layout.ty);
        match ty.kind() {
            TyKind::Array(ty, _) => mcx.arr(
                self.get_cty(self.layout_of(*ty), abi),
                Some(layout.fields.count().try_into().unwrap()), // TODO: [_; 0]
            ),
            _ => mcx.void(), // TODO
        }
    }
}

impl<'tcx, 'mx> LayoutTypeMethods<'tcx> for CodegenCx<'tcx, 'mx> {
    fn backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        self.get_cty(layout, Conv::Rust)
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
                PassMode::Direct(_) => args.push(self.get_cty(arg.layout, fn_abi.conv)),
                PassMode::Pair(_, _) => {
                    args.push(self.scalar_pair_element_backend_type(arg.layout, 0, false));
                    args.push(self.scalar_pair_element_backend_type(arg.layout, 1, false));
                }
                PassMode::Cast { .. } => todo!(),
                PassMode::Indirect { .. } => todo!(),
            }
        }
        let ret = self.get_cty(fn_abi.ret.layout, fn_abi.conv);
        self.mcx.fn_ptr(ret, args.into(), fn_abi.conv)
    }

    fn fn_ptr_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        todo!()
    }

    fn reg_backend_type(&self, ty: &rustc_target::abi::call::Reg) -> Self::Type {
        todo!()
    }

    fn immediate_backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        self.get_cty(layout, Conv::Rust)
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
