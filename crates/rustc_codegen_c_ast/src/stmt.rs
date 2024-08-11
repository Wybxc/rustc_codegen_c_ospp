use crate::decl::CDecl;
use crate::expr::CExpr;
use crate::pretty::{Printer, INDENT};
use crate::ModuleCtxt;

pub type CStmt<'mx> = &'mx CStmtKind<'mx>;

#[derive(Debug, Clone)]
pub enum CStmtKind<'mx> {
    If { cond: CExpr<'mx>, then_br: CStmt<'mx>, else_br: Option<CStmt<'mx>> },
    For { init: CStmt<'mx>, cond: CExpr<'mx>, next: CStmt<'mx>, body: CStmt<'mx> },
    Return(Option<CExpr<'mx>>),
    Decl(CDecl<'mx>),
    Expr(CExpr<'mx>),
    Goto(&'mx str),
}

impl<'mx> ModuleCtxt<'mx> {
    fn create_stmt(self, stmt: CStmtKind<'mx>) -> CStmt<'mx> {
        self.arena().alloc(stmt)
    }

    pub fn if_stmt(
        self,
        cond: CExpr<'mx>,
        then_br: CStmt<'mx>,
        else_br: Option<CStmt<'mx>>,
    ) -> CStmt<'mx> {
        self.create_stmt(CStmtKind::If { cond, then_br, else_br })
    }

    pub fn for_stmt(
        self,
        init: CStmt<'mx>,
        cond: CExpr<'mx>,
        next: CStmt<'mx>,
        body: CStmt<'mx>,
    ) -> CStmt<'mx> {
        self.create_stmt(CStmtKind::For { init, cond, next, body })
    }

    pub fn ret(self, expr: Option<CExpr<'mx>>) -> CStmt<'mx> {
        self.create_stmt(CStmtKind::Return(expr))
    }

    pub fn decl(self, decl: CDecl<'mx>) -> CStmt<'mx> {
        self.create_stmt(CStmtKind::Decl(decl))
    }

    pub fn expr(self, expr: CExpr<'mx>) -> CStmt<'mx> {
        self.create_stmt(CStmtKind::Expr(expr))
    }

    pub fn goto(self, label: &'mx str) -> CStmt<'mx> {
        self.create_stmt(CStmtKind::Goto(label))
    }
}

impl Printer {
    pub fn print_stmt(&mut self, stmt: CStmt, trailing_semicolon: bool) {
        match stmt {
            CStmtKind::If { cond, then_br, else_br } => self.ibox(INDENT, |this| {
                this.word("if");
                this.softbreak();
                this.word("(");
                this.print_expr(cond, true);
                this.word(")");
                this.softbreak();
                this.print_stmt(then_br, true);
                if let Some(else_br) = else_br {
                    this.softbreak();
                    this.word("else");
                    this.softbreak();
                    this.print_stmt(else_br, true);
                }
            }),
            CStmtKind::For { init, cond, next, body } => self.ibox(INDENT, |this| {
                this.word("for");
                this.word("(");
                this.print_stmt(init, false);
                this.word(";");
                this.softbreak();
                this.print_expr(cond, true);
                this.word(";");
                this.softbreak();
                this.print_stmt(next, false);
                this.word(")");
                this.softbreak();
                this.print_stmt(body, true);
            }),
            CStmtKind::Return(ret) => {
                self.ibox(INDENT, |this| {
                    this.word("return");
                    if let Some(ret) = ret {
                        this.softbreak();
                        this.print_expr(ret, false);
                    }
                    if trailing_semicolon {
                        this.word(";");
                    }
                });
            }
            CStmtKind::Decl(decl) => self.print_decl(decl, trailing_semicolon),
            CStmtKind::Expr(expr) => {
                self.print_expr(expr, true);
                if trailing_semicolon {
                    self.word(";");
                }
            }
            CStmtKind::Goto(label) => {
                self.word(format!("goto {}", label));
                if trailing_semicolon {
                    self.word(";");
                }
            }
        }
    }
}
