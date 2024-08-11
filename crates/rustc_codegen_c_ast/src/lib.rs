#![feature(rustc_private)]

use std::cell::{Cell, RefCell};
use std::fmt::{self, Display};

use rustc_hash::FxHashSet;

use crate::r#type::{CTyBase, CTyKind};

extern crate bitflags;
extern crate rustc_arena;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_hash;
extern crate rustc_target;
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

    pub fn intern_ty(&self, ty: CTyKind<'mx>) -> CTyBase<'mx> {
        self.0.interner.intern_ty(self.arena(), ty)
    }

    /// Generate a new unique name for a typedef.
    pub fn new_typename(&self, name: &str) -> &'mx str {
        let mut typenames = self.0.typenames.borrow_mut();
        if !typenames.contains(name) {
            let name = self.arena().alloc_str(name);
            typenames.insert(name);
            return name;
        }

        let mut alt = 1;
        loop {
            let name = self.arena().alloc_str(&format!("{}_{}", name, alt));
            if !typenames.contains(name) {
                typenames.insert(name);
                return name;
            }
            alt += 1;
        }
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
    typenames: RefCell<FxHashSet<&'mx str>>,
}

impl<'mx> ModuleArena<'mx> {
    pub fn new() -> Self {
        Self {
            arena: arena::Arena::default(),
            module: module::Module::new(),
            interner: intern::Interner::default(),
            global_var_counter: Cell::new(0),
            typenames: RefCell::new(FxHashSet::default()),
        }
    }
}

impl<'mx> Default for ModuleArena<'mx> {
    fn default() -> Self {
        Self::new()
    }
}
