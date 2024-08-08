use crate::pretty::{Printer, INDENT};
use crate::r#type::CTy;
use crate::ModuleCtxt;

/// Values of C variable, parameters and scalars
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum CValue<'mx> {
    Null,
    Scalar(&'mx i128),
    Local(usize),
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
    Call { callee: CExpr<'mx>, args: Vec<CExpr<'mx>> },
    Member { expr: CExpr<'mx>, arrow: bool, field: &'mx str },
    InitList { exprs: Vec<CExpr<'mx>> },
}

impl<'mx> ModuleCtxt<'mx> {
    fn create_expr(&self, expr: CExprKind<'mx>) -> CExpr<'mx> {
        self.arena().alloc(expr)
    }

    pub fn raw(&self, raw: &'static str) -> CExpr<'mx> {
        self.create_expr(CExprKind::Raw(raw))
    }

    pub fn scalar(&self, scalar: i128) -> CValue<'mx> {
        CValue::Scalar(self.arena().alloc(scalar))
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

    pub fn call(&self, callee: CExpr<'mx>, args: Vec<CExpr<'mx>>) -> CExpr<'mx> {
        self.create_expr(CExprKind::Call { callee, args })
    }

    pub fn member(&self, expr: CExpr<'mx>, field: &'mx str) -> CExpr<'mx> {
        self.create_expr(CExprKind::Member { expr, field, arrow: false })
    }

    pub fn init_list(&self, exprs: Vec<CExpr<'mx>>) -> CExpr<'mx> {
        self.create_expr(CExprKind::InitList { exprs })
    }
}

impl Printer {
    pub fn print_value(&mut self, value: CValue) {
        match value {
            CValue::Null => self.word("NULL"),
            CValue::Scalar(i) => self.word(i.to_string()),
            CValue::Local(i) => self.word(format!("_{}", i)),
        }
    }

    pub fn print_expr(&mut self, expr: CExpr) {
        match expr {
            CExprKind::Raw(raw) => self.word(*raw),
            CExprKind::Value(value) => self.print_value(*value),
            CExprKind::Unary { op, expr } => self.ibox_delim(INDENT, ("(", ")"), 0, |this| {
                this.word(*op);
                this.print_expr(expr);
            }),
            CExprKind::Binary { lhs, rhs, op } => self.ibox_delim(INDENT, ("(", ")"), 0, |this| {
                this.ibox(-INDENT, |this| this.print_expr(lhs));

                this.softbreak();
                this.word(*op);
                this.nbsp();

                this.print_expr(rhs);
            }),
            CExprKind::Index { expr, index } => {
                self.print_expr(expr);
                self.ibox_delim(INDENT, ("[", "]"), 0, |this| this.print_expr(index));
            }
            CExprKind::Cast { ty, expr } => self.ibox(INDENT, |this| {
                this.word("(");
                this.print_ty_decl(*ty, None);
                this.word(")");

                this.nbsp();
                this.print_expr(expr);
            }),
            CExprKind::Call { callee, args } => self.ibox(INDENT, |this| {
                this.print_expr(callee);
                this.cbox_delim(INDENT, ("(", ")"), 0, |this| {
                    this.seperated(",", args, |this, arg| this.print_expr(arg))
                });
            }),
            CExprKind::Member { expr, arrow, field } => self.cbox(INDENT, |this| {
                this.print_expr(expr);
                this.zerobreak();
                if *arrow {
                    this.word("->");
                } else {
                    this.word(".");
                }
                this.word(field.to_string());
            }),
            CExprKind::InitList { exprs } => self.ibox(INDENT, |this| {
                this.ibox_delim(INDENT, ("{", "}"), 0, |this| {
                    this.seperated(",", exprs, |this, expr| this.print_expr(expr));
                })
            }),
        }
    }
}
