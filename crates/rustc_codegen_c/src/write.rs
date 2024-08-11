use std::fs;
use std::io::Write;
use std::process::Stdio;

use rustc_codegen_ssa::back::command::Command;
use rustc_codegen_ssa::back::write::{CodegenContext, ModuleConfig};
use rustc_codegen_ssa::{CompiledModule, ModuleCodegen};
use rustc_errors::{DiagCtxtHandle, FatalError};
use rustc_session::config::{OptLevel, OutputType};
use tracing::error;

use crate::CodegenModule;

pub(crate) unsafe fn codegen(
    cgcx: &CodegenContext<crate::CCodegen>,
    _dcx: DiagCtxtHandle<'_>,
    module: ModuleCodegen<CodegenModule>,
    module_config: &ModuleConfig,
) -> Result<CompiledModule, FatalError> {
    let module_name = module.name.clone();
    let module_name = Some(&module_name[..]);
    let obj_out = cgcx.output_filenames.temp_path(OutputType::Object, module_name);
    let c_out = obj_out.with_extension("c");

    let backend_config = module.module_llvm.config.read();

    // output c source code
    let c_out_file = fs::File::create(&c_out).map_err(|_| FatalError)?;
    writeln!(&c_out_file, "// file: {}.c", module.name).map_err(|_| FatalError)?;
    write!(&c_out_file, "{}", module.module_llvm.module_source).map_err(|_| FatalError)?;

    // invoke cc to compile
    // TODO: handle long command line (windows)
    // TODO: flush_linked_file (windows)
    let mut cmd = Command::new(backend_config.cc.clone());
    cmd.arg(&c_out).arg("-o").arg(&obj_out).arg("-c").args(&backend_config.cflags);
    if let Some(opt_level) = module_config.opt_level {
        cmd.arg(match opt_level {
            OptLevel::No => "-O0",
            OptLevel::Less => "-O1",
            OptLevel::Default => "-O2",
            OptLevel::Aggressive => "-O3",
            OptLevel::Size => "-Os",
            OptLevel::SizeMin => "-Oz",
        });
    }

    let mut cmd = cmd.command();

    let output = match cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|child| child.wait_with_output())
    {
        Ok(output) => {
            output
            // flush_linked_file(&output, out_filename)?;
        }
        Err(e) => {
            error!("failed to spawn C compiler: {}", e);
            return Err(FatalError);
        }
    };

    if !output.status.success() {
        error!("compiler stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        error!("compiler stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        return Err(FatalError);
    }

    drop(backend_config);

    Ok(module.into_compiled_module(true, false, false, false, false, &cgcx.output_filenames))
}

pub(crate) fn link(
    _cgcx: &CodegenContext<crate::CCodegen>,
    _dcx: DiagCtxtHandle<'_>,
    mut _modules: Vec<ModuleCodegen<CodegenModule>>,
) -> Result<ModuleCodegen<CodegenModule>, FatalError> {
    unimplemented!();
}
