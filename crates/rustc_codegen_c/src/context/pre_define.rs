use rustc_codegen_c_ast::func::CFuncKind;
use rustc_codegen_ssa::traits::{LayoutTypeMethods, PreDefineMethods};
use rustc_data_structures::intern::Interned;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::mono::{Linkage, Visibility};
use rustc_middle::ty::layout::FnAbiOf;
use rustc_middle::ty::{self, Instance};

use crate::context::CodegenCx;

impl<'tcx, 'mx> PreDefineMethods<'tcx> for CodegenCx<'tcx, 'mx> {
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
        let attrs = self.tcx.codegen_fn_attrs(instance.def_id());
        let is_main = attrs.export_name.is_some_and(|name| name.as_str() == "main");

        let fn_abi = self.fn_abi_of_instance(instance, ty::List::empty());
        let fn_ptr = self.fn_decl_backend_type(fn_abi);

        let symbol_name = symbol_name.replace('.', "_");
        let func = CFuncKind::new(self.mcx.alloc_str(&symbol_name), fn_ptr, is_main);
        let func = Interned::new_unchecked(self.mcx.create_func(func));
        self.mcx.module().push_func(func);
        self.function_instances.borrow_mut().insert(instance, func);
    }
}
