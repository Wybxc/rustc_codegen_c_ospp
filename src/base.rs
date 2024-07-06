use rustc_codegen_ssa::{ModuleCodegen, ModuleKind};
use rustc_middle::ty::TyCtxt;

use crate::module::{CDecl, CExpr, CStmt, CType, Module};

// note: parallel
// it seems this function will be invoked parallelly (if parallel codegen is enabled)

pub fn compile_codegen_unit(
    _tcx: TyCtxt<'_>,
    cgu_name: rustc_span::Symbol,
) -> (ModuleCodegen<Module>, u64) {
    let cost = 1;
    let name = cgu_name.as_str().to_owned();

    let module = Module {
        includes: vec!["stdio.h".to_owned()],
        decls: vec![CDecl::Function {
            name: "main".to_owned(),
            ty: CType::Builtin("int".to_owned()),
            params: vec![],
            body: CStmt::Compound(vec![
                CStmt::Expr(CExpr::Call {
                    callee: Box::new(CExpr::DeclRef { name: "printf".to_owned() }),
                    args: vec![CExpr::Literal(
                        "\"Hello, World!\\n\"".to_string(),
                    )],
                }),
                CStmt::Return(Some(Box::new(CExpr::Literal("0".to_string())))),
            ]),
        }],
    };

    let codegen = ModuleCodegen { name, module_llvm: module, kind: ModuleKind::Regular };
    (codegen, cost)
}
