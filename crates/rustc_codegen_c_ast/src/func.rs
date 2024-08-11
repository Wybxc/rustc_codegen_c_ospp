use std::cell::{Cell, RefCell};

use rustc_data_structures::intern::Interned;

use crate::expr::CValue;
use crate::pretty::{Printer, INDENT};
use crate::r#type::{CFnPtr, CTy};
use crate::stmt::CStmt;
use crate::ModuleCtxt;

pub type CFunc<'mx> = Interned<'mx, CFuncKind<'mx>>;

#[derive(Debug, Clone)]
pub struct CFuncKind<'mx> {
    pub name: &'mx str,
    pub ty: CTy<'mx>,
    pub params: Box<[CValue<'mx>]>,
    pub body: RefCell<Vec<&'mx CBasicBlock<'mx>>>,
    local_var_counter: Cell<usize>,
}

impl<'mx> CFuncKind<'mx> {
    pub fn new(name: &'mx str, ty: CTy<'mx>) -> Self {
        let fn_ptr = ty.fn_ptr().expect("expected a function pointer type");
        let params = fn_ptr.args.iter().enumerate().map(|(i, _)| CValue::Local(i)).collect();
        let local_var_counter = Cell::new(fn_ptr.args.len());
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

    pub fn fn_ptr(&self) -> &'mx CFnPtr<'mx> {
        self.ty.fn_ptr().unwrap()
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
        let fn_ptr = func.fn_ptr();
        self.print_signature(fn_ptr.ret, func.0.name, &fn_ptr.args, Some(&func.0.params));
        self.word(";");
    }

    pub fn print_func(&mut self, func: CFunc) {
        self.ibox(0, |this| {
            let fn_ptr = func.fn_ptr();
            this.print_signature(fn_ptr.ret, func.0.name, &fn_ptr.args, Some(&func.0.params));
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

    pub fn print_signature(
        &mut self,
        ret_ty: CTy,
        name: &str,
        params: &[CTy],
        param_names: Option<&[CValue]>,
    ) {
        self.ibox(0, |this| {
            this.print_ty_decl(ret_ty, None);
            this.softbreak();
            this.word(name.to_string());

            this.valign_delim(("(", ")"), |this| {
                if let Some(param_names) = param_names {
                    this.seperated(",", params.iter().zip(param_names), |this, (&ty, &name)| {
                        this.print_ty_decl(ty, Some(name));
                    })
                } else {
                    this.seperated(",", params, |this, &ty| {
                        this.print_ty_decl(ty, None);
                    })
                }
            });
        });
    }

    fn print_bb(&mut self, bb: &CBasicBlock) {
        self.word(bb.label.to_string());
        self.word(":;");
        for stmt in bb.stmts.borrow().iter() {
            self.hardbreak();
            self.print_stmt(stmt, true);
        }
    }
}
