use rustc_codegen_c_ast::r#type::CTy;
use rustc_codegen_ssa::mir::place::PlaceRef;
use rustc_codegen_ssa::traits::{BuilderMethods, LayoutTypeMethods};

use crate::builder::Builder;
use crate::context::Value;

impl<'a, 'tcx, 'mx> Builder<'a, 'tcx, 'mx> {
    pub fn alloc(&mut self, size: usize) -> Value<'mx> {
        let mcx = self.cx.mcx;

        /* Pending allocation
           Since we don't know the exact type of the allocation, we use `char[N]` as a
           placeholder. But if we can know the type later (e.g. if we get a `PlaceRef` of
           the allocation), we can use that type instead.
        */
        let alloc =
            self.func.0.new_pending_alloc(mcx.arr(mcx.char(), Some(size.try_into().unwrap())));

        Value::LValue { cval: alloc }
    }

    /// Realize an allocation.
    pub fn realize(&mut self, place: PlaceRef<'tcx, Value<'mx>>) -> (CTy<'mx>, Value<'mx>) {
        let ty = self.cx.backend_type(place.layout);
        let val = match place.val.llval {
            Value::LValue { cval } => {
                self.func.0.realize_alloc(cval, ty);
                place.val.llval
            }
            Value::RValue { cval, ty: alloc_ty } => {
                self.pointercast(place.val.llval, self.mcx.ptr(ty))
            }
        };
        (ty, val)
    }
}
