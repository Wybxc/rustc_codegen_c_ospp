use crate::builder::Builder;
use crate::context::Value;

impl<'a, 'tcx, 'mx> Builder<'a, 'tcx, 'mx> {
    pub fn unary(&mut self, op: &'static str, expr: Value<'mx>) -> Value<'mx> {
        let mcx = self.mcx;
        let ty = expr.ty();
        let ret = self.func.0.next_local_var();

        self.bb.push_stmt(mcx.decl(mcx.var(ret, ty, Some(mcx.unary(op, mcx.value(expr.cval()))))));

        (ret, ty).into()
    }

    pub fn binary_arith(
        &mut self,
        op: &'static str,
        lhs: Value<'mx>,
        rhs: Value<'mx>,
    ) -> Value<'mx> {
        assert!(lhs.ty() == rhs.ty(), "cannot perform binary operation on different types");

        let mcx = self.mcx;
        let ty = lhs.ty();
        let ret = self.func.0.next_local_var();

        self.bb.push_stmt(mcx.decl(mcx.var(
            ret,
            ty,
            Some(mcx.binary(mcx.value(lhs.cval()), mcx.value(rhs.cval()), op)),
        )));

        (ret, ty).into()
    }

    pub fn binary_cmp(&mut self, op: &'static str, lhs: Value<'mx>, rhs: Value<'mx>) -> Value<'mx> {
        assert!(lhs.ty() == rhs.ty(), "cannot perform binary operation on different types");

        let mcx = self.mcx;
        let ty = mcx.bool();
        let ret = self.func.0.next_local_var();

        self.bb.push_stmt(mcx.decl(mcx.var(
            ret,
            ty,
            Some(mcx.binary(mcx.value(lhs.cval()), mcx.value(rhs.cval()), op)),
        )));

        (ret, ty).into()
    }
}
