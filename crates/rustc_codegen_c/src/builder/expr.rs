use rustc_codegen_c_ast::expr::CValue;
use rustc_codegen_c_ast::r#type::CTy;

use crate::builder::Builder;

type Value<'mx> = (CValue<'mx>, CTy<'mx>);

impl<'a, 'tcx, 'mx> Builder<'a, 'tcx, 'mx> {
    pub fn unary(&mut self, op: &'static str, expr: Value<'mx>) -> Value<'mx> {
        let mcx = self.mcx;
        let ty = expr.1;
        let ret = self.func.0.next_local_var();

        self.bb.push_stmt(mcx.decl(mcx.var(ret, ty, Some(mcx.unary(op, mcx.value(expr.0))))));

        (ret, ty)
    }

    pub fn binary_arith(
        &mut self,
        op: &'static str,
        lhs: Value<'mx>,
        rhs: Value<'mx>,
    ) -> Value<'mx> {
        assert!(lhs.1 == rhs.1, "cannot perform binary operation on different types");

        let mcx = self.mcx;
        let ty = lhs.1;
        let ret = self.func.0.next_local_var();

        self.bb.push_stmt(mcx.decl(mcx.var(
            ret,
            ty,
            Some(mcx.binary(mcx.value(lhs.0), mcx.value(rhs.0), op)),
        )));

        (ret, ty)
    }

    pub fn binary_cmp(&mut self, op: &'static str, lhs: Value<'mx>, rhs: Value<'mx>) -> Value<'mx> {
        assert!(lhs.1 == rhs.1, "cannot perform binary operation on different types");

        let mcx = self.mcx;
        let ty = mcx.bool();
        let ret = self.func.0.next_local_var();

        self.bb.push_stmt(mcx.decl(mcx.var(
            ret,
            ty,
            Some(mcx.binary(mcx.value(lhs.0), mcx.value(rhs.0), op)),
        )));

        (ret, ty)
    }
}
