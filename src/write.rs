use rustc_codegen_ssa::back::write::{CodegenContext, ModuleConfig};
use rustc_codegen_ssa::{CompiledModule, ModuleCodegen};
use rustc_errors::{DiagCtxtHandle, FatalError};
use rustc_session::config::OutputType;

use crate::module::Module;

//           mini_core.b9ae8e2840b8e4ad-cgu.0
// mini_core.mini_core.b9ae8e2840b8e4ad-cgu.0.rcgu.o

// output to `[crate-name].[cgu-name].rcgu.o`

pub(crate) unsafe fn codegen(
    cgcx: &CodegenContext<crate::CCodegen>,
    _dcx: DiagCtxtHandle<'_>,
    module: ModuleCodegen<Module>,
    _config: &ModuleConfig,
) -> Result<CompiledModule, FatalError> {
    let module_name = module.name.clone();
    let module_name = Some(&module_name[..]);
    let obj_out = cgcx.output_filenames.temp_path(OutputType::Object, module_name);
    let c_out = obj_out.with_extension("c");

    std::fs::write(&c_out, format!("{}", module.module_llvm)).map_err(|_| FatalError)?;

    Ok(module.into_compiled_module(true, false, false, false, false, &cgcx.output_filenames))
}

pub(crate) fn link(
    _cgcx: &CodegenContext<crate::CCodegen>,
    _dcx: DiagCtxtHandle<'_>,
    mut _modules: Vec<ModuleCodegen<Module>>,
) -> Result<ModuleCodegen<Module>, FatalError> {
    unimplemented!();
}
