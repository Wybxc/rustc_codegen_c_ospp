use std::sync::Arc;
use std::time::Instant;

use parking_lot::RwLock;
use rustc_codegen_c_ast::{ModuleArena, ModuleCtxt};
use rustc_codegen_ssa::mono_item::MonoItemExt;
use rustc_codegen_ssa::{ModuleCodegen, ModuleKind};
use rustc_middle::dep_graph;
use rustc_middle::ty::TyCtxt;

use crate::builder::Builder;
use crate::context::CodegenCx;
use crate::{BackendConfig, CodegenModule};

// note: parallel
// it seems this function will be invoked parallelly (if parallel codegen is enabled)

pub fn compile_codegen_unit(
    tcx: TyCtxt<'_>,
    cgu_name: rustc_span::Symbol,
    config: Arc<RwLock<BackendConfig>>,
) -> (ModuleCodegen<CodegenModule>, u64) {
    let start_time = Instant::now();

    let dep_node = tcx.codegen_unit(cgu_name).codegen_dep_node(tcx);
    let (module, _) = tcx.dep_graph.with_task(
        dep_node,
        tcx,
        cgu_name,
        module_codegen,
        Some(dep_graph::hash_result),
    );

    let time_to_codegen = start_time.elapsed();
    let cost = time_to_codegen
        .as_secs()
        .saturating_mul(1_000_000_000)
        .saturating_add(time_to_codegen.subsec_nanos() as u64);

    let module = ModuleCodegen {
        name: module.name,
        module_llvm: CodegenModule { module_source: module.module_llvm, config },
        kind: ModuleKind::Regular,
    };
    (module, cost)
}

fn module_codegen(tcx: TyCtxt<'_>, cgu_name: rustc_span::Symbol) -> ModuleCodegen<String> {
    let cgu = tcx.codegen_unit(cgu_name);

    let mcx = ModuleArena::new();
    let mcx = ModuleCtxt(&mcx);

    mcx.module().push_includes(&[
        "stdlib.h",
        "stdint.h",
        "stddef.h",
        "stdbool.h",
        "rust_runtime.h",
    ]);

    let cx = CodegenCx::new(tcx, mcx);

    let mono_items = cgu.items_in_deterministic_order(tcx);
    for &(mono_item, data) in &mono_items {
        mono_item.predefine::<Builder<'_, '_, '_>>(&cx, data.linkage, data.visibility);
    }

    // ... and now that we have everything pre-defined, fill out those definitions.
    for &(mono_item, _) in &mono_items {
        mono_item.define::<Builder<'_, '_, '_>>(&cx);
    }

    let module = mcx.to_string();
    ModuleCodegen { name: cgu_name.to_string(), module_llvm: module, kind: ModuleKind::Regular }
}
