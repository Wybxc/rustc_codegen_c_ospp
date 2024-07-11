#![allow(unused_variables)] // TODO

use std::cell::RefCell;

use rustc_abi::{HasDataLayout, TargetDataLayout};
use rustc_codegen_ssa::traits::BackendTypes;
use rustc_hash::FxHashMap;
use rustc_middle::ty::layout::{
    FnAbiError, FnAbiOfHelpers, FnAbiRequest, HasParamEnv, HasTyCtxt, LayoutError, LayoutOfHelpers,
    TyAndLayout,
};
use rustc_middle::ty::{ParamEnv, Ty, TyCtxt};
use rustc_target::abi::call::FnAbi;
use rustc_target::spec::{HasTargetSpec, Target};

use crate::module::{CDecl, CExpr, CStmt, CType, Module};

mod asm;
mod base_type;
mod r#const;
mod debug_info;
mod layout_type;
mod misc;
mod pre_define;
mod r#static;
mod type_membership;

pub struct CodegenCx<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub functions: RefCell<FxHashMap<String, ()>>,
}

impl<'tcx> CodegenCx<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self { tcx, functions: RefCell::new(FxHashMap::default()) }
    }

    pub fn finish(self) -> Module {
        let mut decls = vec![];

        for (name, _) in self.functions.into_inner() {
            decls.push(CDecl::Function {
                name,
                ty: CType::Builtin("int".to_owned()),
                params: vec![],
                body: CStmt::Compound(vec![CStmt::Return(Some(Box::new(CExpr::Literal(
                    "0".to_owned(),
                ))))]),
            });
        }

        Module { includes: vec![], decls }
    }
}

impl<'tcx> BackendTypes for CodegenCx<'tcx> {
    type Value = ();
    type Function = &'tcx str;
    type BasicBlock = ();
    type Type = ();
    type Funclet = ();
    type DIScope = ();
    type DILocation = ();
    type DIVariable = ();
}

impl<'tcx> HasTargetSpec for CodegenCx<'tcx> {
    fn target_spec(&self) -> &Target {
        todo!()
    }
}

impl<'tcx> HasParamEnv<'tcx> for CodegenCx<'tcx> {
    fn param_env(&self) -> ParamEnv<'tcx> {
        ParamEnv::reveal_all()
    }
}

impl<'tcx> HasTyCtxt<'tcx> for CodegenCx<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

impl HasDataLayout for CodegenCx<'_> {
    fn data_layout(&self) -> &TargetDataLayout {
        todo!()
    }
}

impl<'tcx> LayoutOfHelpers<'tcx> for CodegenCx<'tcx> {
    type LayoutOfResult = TyAndLayout<'tcx>;

    fn handle_layout_err(&self, err: LayoutError<'tcx>, span: rustc_span::Span, ty: Ty<'tcx>) -> ! {
        todo!()
    }
}

impl<'tcx> FnAbiOfHelpers<'tcx> for CodegenCx<'tcx> {
    type FnAbiOfResult = &'tcx FnAbi<'tcx, Ty<'tcx>>;

    fn handle_fn_abi_err(
        &self,
        err: FnAbiError<'tcx>,
        span: rustc_span::Span,
        fn_abi_request: FnAbiRequest<'tcx>,
    ) -> ! {
        todo!()
    }
}
