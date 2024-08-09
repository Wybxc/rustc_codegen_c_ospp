#![feature(rustc_private)]

use std::cell::Cell;
use std::fmt::{self, Display};

use crate::r#type::{CTy, CTyKind};

extern crate rustc_arena;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_hash;
extern crate rustc_type_ir;

pub mod arena;
pub mod decl;
pub mod expr;
pub mod func;
mod intern;
pub mod module;
pub mod pretty;
pub mod stmt;
pub mod r#type;

#[derive(Clone, Copy)]
pub struct ModuleCtxt<'mx>(pub &'mx ModuleArena<'mx>);

impl<'mx> ModuleCtxt<'mx> {
    pub fn arena(&self) -> &'mx arena::Arena<'mx> {
        &self.0.arena
    }

    pub fn module(&self) -> &'mx module::Module<'mx> {
        &self.0.module
    }

    pub fn alloc_str(&self, s: &str) -> &'mx str {
        self.arena().alloc_str(s)
    }

    pub fn intern_ty(&self, ty: CTyKind<'mx>) -> CTy<'mx> {
        self.0.interner.intern_ty(self.arena(), ty)
    }
}

impl<'mx> Display for ModuleCtxt<'mx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = pretty::Printer::new();
        printer.print_module(self.module());
        write!(f, "{}", printer.finish())
    }
}

pub struct ModuleArena<'mx> {
    pub arena: arena::Arena<'mx>,
    pub module: module::Module<'mx>,
    interner: intern::Interner<'mx>,
    global_var_counter: Cell<usize>,
}

impl<'mx> ModuleArena<'mx> {
    pub fn new(helper: &'static str) -> Self {
        Self {
            arena: arena::Arena::default(),
            module: module::Module::new(helper),
            interner: intern::Interner::default(),
            global_var_counter: Cell::new(0),
        }
    }
}
