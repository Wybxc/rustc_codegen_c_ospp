use std::cell::RefCell;

use crate::decl::CDecl;
use crate::func::CFunc;
use crate::pretty::Printer;

#[derive(Debug, Clone)]
pub struct Module<'mx> {
    pub includes: RefCell<Vec<&'static str>>,
    pub decls: RefCell<Vec<CDecl<'mx>>>,
    pub funcs: RefCell<Vec<CFunc<'mx>>>,
}

impl<'mx> Module<'mx> {
    pub fn new() -> Self {
        Self {
            includes: RefCell::new(Vec::new()),
            decls: RefCell::new(Vec::new()),
            funcs: RefCell::new(Vec::new()),
        }
    }

    pub fn push_includes(&self, includes: &[&'static str]) {
        self.includes.borrow_mut().extend(includes);
    }

    pub fn push_decl(&self, decl: CDecl<'mx>) {
        self.decls.borrow_mut().push(decl);
    }

    pub fn push_func(&self, func: CFunc<'mx>) {
        self.funcs.borrow_mut().push(func);
    }
}

impl<'mx> Default for Module<'mx> {
    fn default() -> Self {
        Self::new()
    }
}

impl Printer {
    pub fn print_module(&mut self, module: &Module) {
        self.cbox(0, |this| {
            for &include in module.includes.borrow().iter() {
                this.word("#include <");
                this.word(include);
                this.word(">");
                this.hardbreak();
            }

            this.hardbreak();
            this.word("/* rustc_codegen_c: interface */");
            this.hardbreak();

            for &decl in module.decls.borrow().iter() {
                this.hardbreak();
                this.print_decl(decl, true);
            }

            for &func in module.funcs.borrow().iter() {
                this.hardbreak();
                this.print_func_decl(func);
            }

            this.hardbreak();
            this.hardbreak();
            this.word("/* rustc_codegen_c: implementation */");

            for &func in module.funcs.borrow().iter() {
                this.hardbreak();
                this.hardbreak();
                this.print_func(func);
            }

            this.hardbreak();
        });
    }
}
