use std::cell::{Cell, RefCell};

use rustc_data_structures::fx::FxIndexMap;
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
    is_main: bool,
    body: RefCell<Vec<&'mx CBasicBlock<'mx>>>,
    alloc: RefCell<FxIndexMap<CValue<'mx>, PendingAlloc<'mx>>>,
    local_var_counter: Cell<usize>,
}

impl<'mx> CFuncKind<'mx> {
    pub fn new(name: &'mx str, ty: CTy<'mx>, is_main: bool) -> Self {
        let fn_ptr = ty.fn_ptr().expect("expected a function pointer type");
        let params = fn_ptr.args.iter().enumerate().map(|(i, _)| CValue::Local(i)).collect();
        let local_var_counter = Cell::new(fn_ptr.args.len());
        let body = RefCell::new(Vec::new());
        let alloc = RefCell::new(FxIndexMap::default());

        // TODO: diagnosis output instead of panic
        if is_main {
            let fn_ptr = ty.fn_ptr().unwrap();
            if !fn_ptr.ret.is_signed() {
                panic!("main function must return signed integer");
            }
            if fn_ptr.args.len() != 0 && fn_ptr.args.len() != 2 {
                panic!("main function must take 0 or 2 arguments, but takes {}", fn_ptr.args.len());
            }
            if fn_ptr.args.len() == 2 {
                if !fn_ptr.args[0].is_signed() {
                    panic!("argc must be signed integer");
                }
                if !fn_ptr.args[1].is_ptr() {
                    panic!("argv must be pointer");
                }
            }
        }

        Self { name, ty, params, is_main, body, alloc, local_var_counter }
    }

    pub fn next_local_var(&self) -> CValue<'mx> {
        let val = CValue::Local(self.local_var_counter.get());
        self.local_var_counter.set(self.local_var_counter.get() + 1);
        val
    }

    pub fn new_pending_alloc(&self, fallback: CTy<'mx>) -> CValue<'mx> {
        let val = self.next_local_var();
        self.alloc.borrow_mut().insert(val, PendingAlloc { ty: None, fallback });
        val
    }

    pub fn realize_alloc(&self, val: CValue<'mx>, ty: CTy<'mx>) {
        let mut alloc = self.alloc.borrow_mut();
        match alloc.get_mut(&val) {
            Some(PendingAlloc { ty: Some(alloc_ty), .. }) => {
                assert_eq!(ty, *alloc_ty, "alloc mismatch: {:?} vs {:?}", ty, alloc_ty)
            }
            Some(PendingAlloc { ty: alloc_ty @ None, .. }) => {
                alloc_ty.replace(ty);
            }
            None => {
                panic!("alloc not found: {:?}", val)
            }
        }
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
pub struct PendingAlloc<'mx> {
    pub ty: Option<CTy<'mx>>,
    pub fallback: CTy<'mx>, // fallback type char[N] if ty is None
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
        if func.is_main {
            self.print_signature_main(&func.0.params)
        } else {
            self.print_signature(fn_ptr.ret, func.0.name, &fn_ptr.args, Some(&func.0.params));
        }
        self.word(";");
    }

    pub fn print_func(&mut self, func: CFunc) {
        self.ibox(0, |this| {
            let fn_ptr = func.fn_ptr();
            if func.is_main {
                this.print_signature_main(&func.0.params)
            } else {
                this.print_signature(fn_ptr.ret, func.0.name, &fn_ptr.args, Some(&func.0.params));
            }
            this.softbreak();
            this.word("{");
            if func.0.alloc.borrow().is_empty() {
                this.break_offset(0, 0);
            } else {
                this.break_offset(0, INDENT);
            }
            this.cbox(INDENT, |this| {
                this.seperated("", func.0.alloc.borrow().iter(), |this, (var, alloc)| {
                    this.print_pending_alloc(*var, alloc);
                });
                if !func.0.alloc.borrow().is_empty() {
                    this.break_offset(0, -INDENT);
                }
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
                        this.print_ty_decl(ty, Some(name.to_string()));
                    })
                } else {
                    this.seperated(",", params, |this, &ty| {
                        this.print_ty_decl(ty, None);
                    })
                }
            });
        });
    }

    fn print_signature_main(&mut self, param_names: &[CValue]) {
        self.ibox(0, |this| {
            this.word("int");
            this.softbreak();
            this.word("main");

            this.valign_delim(("(", ")"), |this| {
                if param_names.is_empty() {
                    return;
                }
                this.word("int");
                this.softbreak();
                this.print_value(param_names[0]);
                this.word(",");
                this.softbreak();
                this.word("char**");
                this.softbreak();
                this.print_value(param_names[1]);
            });
        });
    }

    fn print_pending_alloc(&mut self, val: CValue, alloc: &PendingAlloc) {
        if let Some(ty) = alloc.ty {
            self.print_ty_decl(ty, Some(val.to_string()));
        } else {
            self.print_ty_decl(alloc.fallback, Some(val.to_string()));
        }
        self.word(";");
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
