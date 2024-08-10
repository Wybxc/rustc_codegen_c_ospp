use crate::pretty::{Printer, INDENT};
use crate::r#type::CTy;
use crate::ModuleCtxt;

/// Values of C variable, parameters and scalars
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum CValue<'mx> {
    Null,
    Scalar(i128),
    Local(usize),
    Global(usize),
    Func(&'mx str),
}

impl<'mx> CValue<'mx> {
    pub fn is_func(&self) -> bool {
        matches!(self, CValue::Func(_))
    }
}

pub type CExpr<'mx> = &'mx CExprKind<'mx>;

#[derive(Debug, Clone)]
pub enum CExprKind<'mx> {
    Raw(&'static str),
    Value(CValue<'mx>),
    Unary { op: &'static str, expr: CExpr<'mx> },
    Binary { lhs: CExpr<'mx>, rhs: CExpr<'mx>, op: &'static str },
    Index { expr: CExpr<'mx>, index: CExpr<'mx> },
    Cast { ty: CTy<'mx>, expr: CExpr<'mx> },
    Call { callee: CExpr<'mx>, args: Box<[CExpr<'mx>]> },
    Member { expr: CExpr<'mx>, arrow: bool, field: &'mx str },
    InitList { exprs: Box<[CExpr<'mx>]> },
}

impl<'mx> ModuleCtxt<'mx> {
    pub fn next_global_var(&self) -> CValue<'mx> {
        let var = CValue::Global(self.0.global_var_counter.get());
        self.0.global_var_counter.set(self.0.global_var_counter.get() + 1);
        var
    }

    pub fn fn_ref(&self, name: &'mx str) -> CValue<'mx> {
        CValue::Func(name)
    }

    fn create_expr(&self, expr: CExprKind<'mx>) -> CExpr<'mx> {
        self.arena().alloc(expr)
    }

    pub fn raw(&self, raw: &'static str) -> CExpr<'mx> {
        self.create_expr(CExprKind::Raw(raw))
    }

    pub fn scalar(&self, scalar: i128) -> CValue<'mx> {
        CValue::Scalar(scalar)
    }

    pub fn value(&self, value: CValue<'mx>) -> CExpr<'mx> {
        self.create_expr(CExprKind::Value(value))
    }

    pub fn unary(&self, op: &'static str, expr: CExpr<'mx>) -> CExpr<'mx> {
        self.create_expr(CExprKind::Unary { op, expr })
    }

    pub fn binary(&self, lhs: CExpr<'mx>, rhs: CExpr<'mx>, op: &'static str) -> CExpr<'mx> {
        self.create_expr(CExprKind::Binary { lhs, rhs, op })
    }

    pub fn index(&self, expr: CExpr<'mx>, index: CExpr<'mx>) -> CExpr<'mx> {
        self.create_expr(CExprKind::Index { expr, index })
    }

    pub fn assign(&self, lhs: CExpr<'mx>, rhs: CExpr<'mx>) -> CExpr<'mx> {
        self.binary(lhs, rhs, "=")
    }

    pub fn cast(&self, ty: CTy<'mx>, expr: CExpr<'mx>) -> CExpr<'mx> {
        self.create_expr(CExprKind::Cast { ty, expr })
    }

    pub fn call(&self, callee: CExpr<'mx>, args: impl Into<Box<[CExpr<'mx>]>>) -> CExpr<'mx> {
        self.create_expr(CExprKind::Call { callee, args: args.into() })
    }

    pub fn member(&self, expr: CExpr<'mx>, field: &'mx str) -> CExpr<'mx> {
        self.create_expr(CExprKind::Member { expr, field, arrow: false })
    }

    pub fn init_list(&self, exprs: impl Into<Box<[CExpr<'mx>]>>) -> CExpr<'mx> {
        self.create_expr(CExprKind::InitList { exprs: exprs.into() })
    }
}

impl Printer {
    pub fn print_value(&mut self, value: CValue) {
        match value {
            CValue::Null => self.word("NULL"),
            CValue::Scalar(i) => self.word(i.to_string()),
            CValue::Local(i) => self.word(format!("_{}", i)),
            CValue::Global(i) => self.word(format!("_g{}", i)), // TODO: add module-specific prefix
            CValue::Func(raw) => self.word(raw.to_string()),
        }
    }

    pub fn print_expr(&mut self, expr: CExpr, outer: bool) {
        let delim = if outer { ("", "") } else { ("(", ")") };
        match expr {
            CExprKind::Raw(raw) => self.word(*raw),
            CExprKind::Value(value) => self.print_value(*value),
            CExprKind::Unary { op, expr } => self.ibox_delim(INDENT, delim, |this| {
                this.word(*op);
                this.print_expr(expr, false);
            }),
            CExprKind::Binary { lhs, rhs, op } => self.ibox_delim(INDENT, delim, |this| {
                this.ibox(-INDENT, |this| this.print_expr(lhs, false));

                this.softbreak();
                this.word(*op);
                this.nbsp();

                this.print_expr(rhs, false);
            }),
            CExprKind::Index { expr, index } => {
                self.print_expr(expr, false);
                self.ibox_delim(INDENT, ("[", "]"), |this| this.print_expr(index, false));
            }
            CExprKind::Cast { ty, expr } => self.ibox(INDENT, |this| {
                this.word("(");
                this.print_ty_decl(*ty, None);
                this.word(")");

                this.nbsp();
                this.print_expr(expr, false);
            }),
            CExprKind::Call { callee, args } => self.ibox(INDENT, |this| {
                this.print_expr(callee, false);
                this.cbox_delim(INDENT, ("(", ")"), 0, |this| {
                    this.seperated(",", args, |this, arg| this.print_expr(arg, false))
                });
            }),
            CExprKind::Member { expr, arrow, field } => self.cbox(INDENT, |this| {
                this.print_expr(expr, false);
                this.zerobreak();
                if *arrow {
                    this.word("->");
                } else {
                    this.word(".");
                }
                this.word(field.to_string());
            }),
            CExprKind::InitList { exprs } => self.ibox(INDENT, |this| {
                this.ibox_delim(INDENT, ("{", "}"), |this| {
                    this.seperated(",", exprs, |this, expr| this.print_expr(expr, false));
                })
            }),
        }
    }
}
