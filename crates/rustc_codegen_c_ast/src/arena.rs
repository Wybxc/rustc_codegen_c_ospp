use crate::decl::CDeclKind;
use crate::expr::CExprKind;
use crate::func::{CBasicBlock, CFuncKind};
use crate::r#type::CTyKind;
use crate::stmt::CStmtKind;

rustc_arena::declare_arena!([
    [] decl: CDeclKind<'tcx>,
    [] expr: CExprKind<'tcx>,
    [] func: CFuncKind<'tcx>,
    [] bb: CBasicBlock<'tcx>,
    [] stmt: CStmtKind<'tcx>,
    [] ty: CTyKind<'tcx>,
]);
