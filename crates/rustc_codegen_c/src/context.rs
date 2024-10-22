#![allow(unused_variables)] // TODO

use std::cell::RefCell;

use rustc_abi::{HasDataLayout, TargetDataLayout};
use rustc_codegen_c_ast::expr::CValue;
use rustc_codegen_c_ast::func::{CBasicBlock, CFunc};
use rustc_codegen_c_ast::r#type::CTy;
use rustc_codegen_c_ast::ModuleCtxt;
use rustc_codegen_ssa::traits::BackendTypes;
use rustc_hash::FxHashMap;
use rustc_middle::ty::layout::{
    FnAbiError, FnAbiOfHelpers, FnAbiRequest, HasParamEnv, HasTyCtxt, LayoutError, LayoutOfHelpers,
    TyAndLayout,
};
use rustc_middle::ty::{Instance, ParamEnv, Ty, TyCtxt};
use rustc_target::abi::call::FnAbi;
use rustc_target::spec::{HasTargetSpec, Target};

mod asm;
mod base_type;
mod r#const;
mod debug_info;
mod layout_type;
mod misc;
mod pre_define;
mod r#static;
mod type_membership;

pub struct CodegenCx<'tcx, 'mx> {
    pub tcx: TyCtxt<'tcx>,
    pub mcx: ModuleCtxt<'mx>,

    // function declarations (in another crate or extern)
    function_declarations: RefCell<FxHashMap<Instance<'tcx>, Value<'mx>>>,
    // function instances (in this crate)
    function_instances: RefCell<FxHashMap<Instance<'tcx>, CFunc<'mx>>>,
}

impl<'tcx, 'mx> CodegenCx<'tcx, 'mx> {
    pub fn new(tcx: TyCtxt<'tcx>, mcx: ModuleCtxt<'mx>) -> Self {
        Self {
            tcx,
            mcx,
            function_declarations: RefCell::new(FxHashMap::default()),
            function_instances: RefCell::new(FxHashMap::default()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Value<'mx> {
    RValue { cval: CValue<'mx>, ty: CTy<'mx> },
    LValue { cval: CValue<'mx> },
}

impl<'mx> From<(CValue<'mx>, CTy<'mx>)> for Value<'mx> {
    fn from((cval, ty): (CValue<'mx>, CTy<'mx>)) -> Self {
        Self::RValue { cval, ty }
    }
}

impl<'mx> Value<'mx> {
    pub fn cval(self) -> CValue<'mx> {
        match self {
            Self::RValue { cval, .. } => cval,
            Self::LValue { cval } => cval,
        }
    }

    pub fn ty(self) -> CTy<'mx> {
        match self {
            Self::RValue { ty, .. } => ty,
            Self::LValue { .. } => panic!("loading lvalue as rvalue"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BasicBlock<'mx> {
    pub cbb: &'mx CBasicBlock<'mx>,
    pub func: CFunc<'mx>,
}

impl<'tcx, 'mx> BackendTypes for CodegenCx<'tcx, 'mx> {
    type Value = Value<'mx>;
    type Function = CFunc<'mx>;
    type BasicBlock = BasicBlock<'mx>;
    type Type = CTy<'mx>;
    type Funclet = ();
    type DIScope = ();
    type DILocation = ();
    type DIVariable = ();
}

impl<'tcx, 'mx> HasTargetSpec for CodegenCx<'tcx, 'mx> {
    fn target_spec(&self) -> &Target {
        todo!()
    }
}

impl<'tcx, 'mx> HasParamEnv<'tcx> for CodegenCx<'tcx, 'mx> {
    fn param_env(&self) -> ParamEnv<'tcx> {
        ParamEnv::reveal_all()
    }
}

impl<'tcx, 'mx> HasTyCtxt<'tcx> for CodegenCx<'tcx, 'mx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

impl<'tcx, 'mx> HasDataLayout for CodegenCx<'tcx, 'mx> {
    fn data_layout(&self) -> &TargetDataLayout {
        self.tcx.data_layout()
    }
}

impl<'tcx, 'mx> LayoutOfHelpers<'tcx> for CodegenCx<'tcx, 'mx> {
    type LayoutOfResult = TyAndLayout<'tcx>;

    fn handle_layout_err(&self, err: LayoutError<'tcx>, span: rustc_span::Span, ty: Ty<'tcx>) -> ! {
        todo!()
    }
}

impl<'tcx, 'mx> FnAbiOfHelpers<'tcx> for CodegenCx<'tcx, 'mx> {
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
