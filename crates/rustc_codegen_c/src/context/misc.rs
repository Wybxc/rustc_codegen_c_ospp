use std::cell::RefCell;

use rustc_codegen_ssa::traits::{LayoutTypeMethods, MiscMethods};
use rustc_hash::FxHashMap;
use rustc_middle::mir::mono::CodegenUnit;
use rustc_middle::ty::layout::FnAbiOf;
use rustc_middle::ty::{self, Instance, PolyExistentialTraitRef, Ty};

use crate::context::CodegenCx;

impl<'tcx, 'mx> MiscMethods<'tcx> for CodegenCx<'tcx, 'mx> {
    fn vtables(
        &self,
    ) -> &RefCell<FxHashMap<(Ty<'tcx>, Option<PolyExistentialTraitRef<'tcx>>), Self::Value>> {
        todo!()
    }

    fn get_fn(&self, instance: Instance<'tcx>) -> Self::Function {
        *self.function_instances.borrow().get(&instance).unwrap()
    }

    fn get_fn_addr(&self, instance: Instance<'tcx>) -> Self::Value {
        if let Some(func) = self.function_instances.borrow().get(&instance) {
            let val = self.mcx.fn_ref(func.0.name);
            let ty = func.0.ty;
            return (val, ty).into();
        }

        if let Some(&val) = self.function_declarations.borrow().get(&instance) {
            return val;
        }

        let mcx = self.mcx;

        let val = mcx.fn_ref(mcx.alloc_str(self.tcx.symbol_name(instance).name));
        let ty = self.fn_decl_backend_type(self.fn_abi_of_instance(instance, ty::List::empty()));
        mcx.module().push_decl(mcx.func(val, ty.fn_ptr().unwrap()));

        self.function_declarations.borrow_mut().insert(instance, (val, ty).into());

        (val, ty).into()
    }

    fn eh_personality(&self) -> Self::Value {
        todo!()
    }

    fn sess(&self) -> &rustc_session::Session {
        self.tcx.sess
    }

    fn codegen_unit(&self) -> &'tcx CodegenUnit<'tcx> {
        todo!()
    }

    fn set_frame_pointer_type(&self, llfn: Self::Function) {
        todo!()
    }

    fn apply_target_cpu_attr(&self, llfn: Self::Function) {
        todo!()
    }

    fn declare_c_main(&self, fn_type: Self::Type) -> Option<Self::Function> {
        todo!()
    }
}
