use std::cell::{Cell, RefCell};

use rustc_data_structures::intern::Interned;

use crate::expr::CValue;
use crate::pretty::{Printer, INDENT};
use crate::r#type::CTy;
use crate::stmt::CStmt;
use crate::ModuleCtxt;

pub type CFunc<'mx> = Interned<'mx, CFuncKind<'mx>>;

#[derive(Debug, Clone)]
pub struct CFuncKind<'mx> {
    pub name: &'mx str,
    pub ty: CTy<'mx>,
    pub params: Vec<(CTy<'mx>, CValue<'mx>)>,
    pub body: RefCell<Vec<&'mx CBasicBlock<'mx>>>,
    local_var_counter: Cell<usize>,
}

impl<'mx> CFuncKind<'mx> {
    pub fn new(name: &'mx str, ty: CTy<'mx>, params: impl IntoIterator<Item = CTy<'mx>>) -> Self {
        let params = params
            .into_iter()
            .enumerate()
            .map(|(i, ty)| (ty, CValue::Local(i)))
            .collect::<Vec<_>>();
        let local_var_counter = Cell::new(params.len());

        Self { name, ty, params, body: RefCell::new(Vec::new()), local_var_counter }
    }

    pub fn next_local_var(&self) -> CValue {
        let val = CValue::Local(self.local_var_counter.get());
        self.local_var_counter.set(self.local_var_counter.get() + 1);
        val
    }

    pub fn new_bb(&self, label: &str, mcx: &ModuleCtxt<'mx>) -> &'mx CBasicBlock<'mx> {
        let label = mcx.alloc_str(label);
        let bb = mcx.create_bb(CBasicBlock::new(label));
        self.body.borrow_mut().push(bb);
        bb
    }
}

#[derive(Debug, Clone)]
pub struct CBasicBlock<'mx> {
    pub label: &'mx str,
    pub stmts: RefCell<Vec<CStmt<'mx>>>,
}

impl<'mx> CBasicBlock<'mx> {
    pub fn new(label: &'mx str) -> Self {
        Self { label, stmts: RefCell::new(Vec::new()) }
    }

    pub fn push_stmt(&self, stmt: CStmt<'mx>) {
        self.stmts.borrow_mut().push(stmt);
    }
}

impl<'mx> ModuleCtxt<'mx> {
    pub fn create_func(&self, func: CFuncKind<'mx>) -> &'mx CFuncKind<'mx> {
        self.arena().alloc(func)
    }

    pub fn create_bb(&self, bb: CBasicBlock<'mx>) -> &'mx CBasicBlock<'mx> {
        self.arena().alloc(bb)
    }
}

impl Printer {
    pub fn print_func_decl(&mut self, func: CFunc) {
        self.print_signature(func);
        self.word(";");
    }

    pub fn print_func(&mut self, func: CFunc) {
        self.ibox(0, |this| {
            this.print_signature(func);
            this.softbreak();
            this.word("{");
            this.break_offset(0, 0);
            this.cbox(INDENT, |this| {
                for &bb in func.0.body.borrow().iter() {
                    this.print_bb(bb);
                    this.break_offset(0, -INDENT);
                }
            });
            this.word("}");
        })
    }

    fn print_signature(&mut self, func: CFunc) {
        self.ibox(0, |this| {
            this.print_ty_decl(func.0.ty, None);
            this.softbreak();
            this.word(func.0.name.to_string());

            this.valign_delim(("(", ")"), |this| {
                this.seperated(",", &func.0.params, |this, &(ty, name)| {
                    this.print_ty_decl(ty, Some(name));
                })
            });
        });
    }

    fn print_bb(&mut self, bb: &CBasicBlock) {
        self.word(bb.label.to_string());
        self.word(":;");
        for stmt in bb.stmts.borrow().iter() {
            self.hardbreak();
            self.print_stmt(stmt);
        }
    }
}
