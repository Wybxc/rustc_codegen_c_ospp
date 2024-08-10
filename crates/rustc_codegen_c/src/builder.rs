#![allow(unused_variables)] // TODO

use std::ops::Deref;

use rustc_abi::{HasDataLayout, TargetDataLayout};
use rustc_codegen_c_ast::expr::CValue;
use rustc_codegen_c_ast::func::{CBasicBlock, CFunc};
use rustc_codegen_c_ast::r#type::{CTy, CTyBase};
use rustc_codegen_ssa::common::{AtomicOrdering, IntPredicate, RealPredicate};
use rustc_codegen_ssa::mir::operand::OperandRef;
use rustc_codegen_ssa::mir::place::PlaceRef;
use rustc_codegen_ssa::traits::{
    BackendTypes, BuilderMethods, HasCodegen, IntrinsicCallMethods, LayoutTypeMethods, OverflowOp,
};
use rustc_codegen_ssa::MemFlags;
use rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs;
use rustc_middle::ty::layout::{
    FnAbiError, FnAbiOfHelpers, FnAbiRequest, HasParamEnv, HasTyCtxt, LayoutError, LayoutOfHelpers,
    TyAndLayout,
};
use rustc_middle::ty::{Instance, ParamEnv, Ty, TyCtxt, TyKind};
use rustc_target::abi::call::{Conv, FnAbi};
use rustc_target::spec::{HasTargetSpec, Target};
use rustc_type_ir::{IntTy, UintTy};

use crate::context::CodegenCx;

mod abi;
mod asm;
mod coverage_info;
mod debug_info;
mod expr;
mod intrinsic_call;
mod r#static;

pub struct Builder<'a, 'tcx, 'mx> {
    pub cx: &'a CodegenCx<'tcx, 'mx>,
    func: CFunc<'mx>,
    bb: &'mx CBasicBlock<'mx>,
}

impl<'a, 'tcx, 'mx> Deref for Builder<'a, 'tcx, 'mx> {
    type Target = CodegenCx<'tcx, 'mx>;

    fn deref<'b>(&'b self) -> &'a Self::Target {
        self.cx
    }
}

impl<'tcx, 'mx> HasCodegen<'tcx> for Builder<'_, 'tcx, 'mx> {
    type CodegenCx = CodegenCx<'tcx, 'mx>;
}

impl<'tcx, 'mx> HasDataLayout for Builder<'_, 'tcx, 'mx> {
    fn data_layout(&self) -> &TargetDataLayout {
        self.cx.data_layout()
    }
}

impl<'tcx, 'mx> HasTyCtxt<'tcx> for Builder<'_, 'tcx, 'mx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.cx.tcx()
    }
}

impl<'tcx, 'mx> HasParamEnv<'tcx> for Builder<'_, 'tcx, 'mx> {
    fn param_env(&self) -> ParamEnv<'tcx> {
        self.cx.param_env()
    }
}

impl<'tcx, 'mx> BackendTypes for Builder<'_, 'tcx, 'mx> {
    type Value = <CodegenCx<'tcx, 'mx> as BackendTypes>::Value;
    type Function = <CodegenCx<'tcx, 'mx> as BackendTypes>::Function;
    type BasicBlock = <CodegenCx<'tcx, 'mx> as BackendTypes>::BasicBlock;
    type Type = <CodegenCx<'tcx, 'mx> as BackendTypes>::Type;
    type Funclet = <CodegenCx<'tcx, 'mx> as BackendTypes>::Funclet;

    type DIScope = <CodegenCx<'tcx, 'mx> as BackendTypes>::DIScope;
    type DILocation = <CodegenCx<'tcx, 'mx> as BackendTypes>::DILocation;
    type DIVariable = <CodegenCx<'tcx, 'mx> as BackendTypes>::DIVariable;
}

impl<'tcx, 'mx> HasTargetSpec for Builder<'_, 'tcx, 'mx> {
    fn target_spec(&self) -> &Target {
        todo!()
    }
}

impl<'tcx, 'mx> LayoutOfHelpers<'tcx> for Builder<'_, 'tcx, 'mx> {
    type LayoutOfResult = TyAndLayout<'tcx>;

    fn handle_layout_err(&self, err: LayoutError<'tcx>, span: rustc_span::Span, ty: Ty<'tcx>) -> ! {
        todo!()
    }
}

impl<'tcx, 'mx> FnAbiOfHelpers<'tcx> for Builder<'_, 'tcx, 'mx> {
    type FnAbiOfResult = &'tcx FnAbi<'tcx, Ty<'tcx>>;

    fn handle_fn_abi_err(
        &self,
        err: FnAbiError<'tcx>,
        span: rustc_span::Span,
        fn_abi_request: FnAbiRequest<'tcx>,
    ) -> ! {
        todo!()
    }
}

impl<'a, 'tcx, 'mx> BuilderMethods<'a, 'tcx> for Builder<'a, 'tcx, 'mx> {
    fn build(cx: &'a Self::CodegenCx, llbb: Self::BasicBlock) -> Self {
        Self { cx, func: llbb.0, bb: llbb.1 }
    }

    fn cx(&self) -> &Self::CodegenCx {
        self.cx
    }

    fn llbb(&self) -> Self::BasicBlock {
        todo!()
    }

    fn set_span(&mut self, _span: rustc_span::Span) {}

    fn append_block(cx: &'a Self::CodegenCx, llfn: Self::Function, name: &str) -> Self::BasicBlock {
        let bb = llfn.0.new_bb(name, &cx.mcx);
        (llfn, bb)
    }

    fn append_sibling_block(&mut self, name: &str) -> Self::BasicBlock {
        let bb = self.func.0.new_bb(name, &self.cx.mcx);
        (self.func, bb)
    }

    fn switch_to_block(&mut self, llbb: Self::BasicBlock) {
        assert_eq!(
            self.func, llbb.0,
            "switch_to_block called on a block from a different function"
        );
        self.bb = llbb.1;
    }

    fn ret_void(&mut self) {
        self.bb.push_stmt(self.cx.mcx.ret(None));
    }

    fn ret(&mut self, mut v: Self::Value) {
        v = self.pointercast(v, self.func.fn_ptr().ret);
        self.bb.push_stmt(self.cx.mcx.ret(Some(self.cx.mcx.value(v.0))))
    }

    fn br(&mut self, dest: Self::BasicBlock) {
        assert_eq!(self.func, dest.0, "br called on a block from a different function");
        self.bb.push_stmt(self.cx.mcx.goto(dest.1.label));
    }

    fn cond_br(
        &mut self,
        cond: Self::Value,
        then_llbb: Self::BasicBlock,
        else_llbb: Self::BasicBlock,
    ) {
        assert_eq!(self.func, then_llbb.0, "br called on a block from a different function");
        assert_eq!(self.func, else_llbb.0, "br called on a block from a different function");
        let mcx = &self.cx.mcx;
        self.bb.push_stmt(self.cx.mcx.if_stmt(
            mcx.value(cond.0),
            mcx.goto(then_llbb.1.label),
            Some(mcx.goto(else_llbb.1.label)),
        ));
    }

    fn switch(
        &mut self,
        v: Self::Value,
        else_llbb: Self::BasicBlock,
        cases: impl ExactSizeIterator<Item = (u128, Self::BasicBlock)>,
    ) {
        todo!()
    }

    fn invoke(
        &mut self,
        llty: Self::Type,
        fn_attrs: Option<&CodegenFnAttrs>,
        fn_abi: Option<&FnAbi<'tcx, Ty<'tcx>>>,
        llfn: Self::Value,
        args: &[Self::Value],
        then: Self::BasicBlock,
        catch: Self::BasicBlock,
        funclet: Option<&Self::Funclet>,
        instance: Option<Instance<'tcx>>,
    ) -> Self::Value {
        todo!()
    }

    fn unreachable(&mut self) {
        self.abort();
    }

    fn add(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        self.binary_arith("+", lhs, rhs)
    }

    fn fadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fadd_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fadd_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn sub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        self.binary_arith("-", lhs, rhs)
    }

    fn fsub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fsub_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fsub_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn mul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        self.binary_arith("*", lhs, rhs)
    }

    fn fmul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fmul_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fmul_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn udiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn exactudiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn sdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        self.binary_arith("/", lhs, rhs)
    }

    fn exactsdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fdiv_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fdiv_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn urem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn srem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn frem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn frem_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn frem_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn shl(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn lshr(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn ashr(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn unchecked_sadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn unchecked_uadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn unchecked_ssub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn unchecked_usub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn unchecked_smul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn unchecked_umul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn and(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn or(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn xor(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn neg(&mut self, v: Self::Value) -> Self::Value {
        todo!()
    }

    fn fneg(&mut self, v: Self::Value) -> Self::Value {
        todo!()
    }

    fn not(&mut self, v: Self::Value) -> Self::Value {
        todo!()
    }

    // returns: (value: ty, overflowed: bool)
    fn checked_binop(
        &mut self,
        oop: OverflowOp,
        ty: Ty<'_>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> (Self::Value, Self::Value) {
        assert!(lhs.1 == rhs.1, "Checked binop on different types");

        let mcx = self.cx.mcx;
        let ret = self.func.0.next_local_var();
        let overflow = self.func.0.next_local_var();

        let op = match oop {
            OverflowOp::Add => match ty.kind() {
                TyKind::Int(IntTy::I8) => "__rust_ckd_add_i8",
                TyKind::Int(IntTy::I16) => "__rust_ckd_add_i16",
                TyKind::Int(IntTy::I32) => "__rust_ckd_add_i32",
                TyKind::Int(IntTy::I64) => "__rust_ckd_add_i64",
                TyKind::Int(IntTy::Isize) => "__rust_ckd_add_intptr",
                TyKind::Uint(UintTy::U8) => "__rust_ckd_add_u8",
                TyKind::Uint(UintTy::U16) => "__rust_ckd_add_u16",
                TyKind::Uint(UintTy::U32) => "__rust_ckd_add_u32",
                TyKind::Uint(UintTy::U64) => "__rust_ckd_add_u64",
                TyKind::Uint(UintTy::Usize) => "__rust_ckd_add_uintptr",
                _ => todo!(),
            },
            OverflowOp::Sub => match ty.kind() {
                TyKind::Int(IntTy::I8) => "__rust_ckd_sub_i8",
                TyKind::Int(IntTy::I16) => "__rust_ckd_sub_i16",
                TyKind::Int(IntTy::I32) => "__rust_ckd_sub_i32",
                TyKind::Int(IntTy::I64) => "__rust_ckd_sub_i64",
                TyKind::Int(IntTy::Isize) => "__rust_ckd_sub_intptr",
                TyKind::Uint(UintTy::U8) => "__rust_ckd_sub_u8",
                TyKind::Uint(UintTy::U16) => "__rust_ckd_sub_u16",
                TyKind::Uint(UintTy::U32) => "__rust_ckd_sub_u32",
                TyKind::Uint(UintTy::U64) => "__rust_ckd_sub_u64",
                TyKind::Uint(UintTy::Usize) => "__rust_ckd_sub_uintptr",
                _ => todo!(),
            },
            OverflowOp::Mul => match ty.kind() {
                TyKind::Int(IntTy::I8) => "__rust_ckd_mul_i8",
                TyKind::Int(IntTy::I16) => "__rust_ckd_mul_i16",
                TyKind::Int(IntTy::I32) => "__rust_ckd_mul_i32",
                TyKind::Int(IntTy::I64) => "__rust_ckd_mul_i64",
                TyKind::Int(IntTy::Isize) => "__rust_ckd_mul_intptr",
                TyKind::Uint(UintTy::U8) => "__rust_ckd_mul_u8",
                TyKind::Uint(UintTy::U16) => "__rust_ckd_mul_u16",
                TyKind::Uint(UintTy::U32) => "__rust_ckd_mul_u32",
                TyKind::Uint(UintTy::U64) => "__rust_ckd_mul_u64",
                TyKind::Uint(UintTy::Usize) => "__rust_ckd_mul_uintptr",
                _ => todo!(),
            },
        };

        self.bb.push_stmt(mcx.decl(mcx.var(ret, lhs.1, None)));
        self.bb.push_stmt(mcx.decl(mcx.var(
            overflow,
            mcx.bool(),
            Some(mcx.call(
                mcx.raw(op),
                [mcx.value(lhs.0), mcx.value(rhs.0), mcx.unary("&", mcx.value(ret))],
            )),
        )));

        ((ret, lhs.1), (overflow, mcx.bool()))
    }

    fn from_immediate(&mut self, val: Self::Value) -> Self::Value {
        val // TODO: other circumstances?
    }

    fn to_immediate_scalar(&mut self, val: Self::Value, scalar: rustc_abi::Scalar) -> Self::Value {
        val // TODO: other circumstances?
    }

    fn alloca(&mut self, size: rustc_abi::Size, align: rustc_abi::Align) -> Self::Value {
        let mcx = self.cx.mcx;

        let buf = self.func.0.next_local_var();
        self.bb.push_stmt(mcx.decl(mcx.var(
            buf,
            mcx.arr(mcx.char(), Some(size.bytes_usize().try_into().unwrap())),
            Some(mcx.init_list([mcx.value(mcx.scalar(0))])),
        )));

        let ret = self.func.0.next_local_var();
        let ty = mcx.ptr(mcx.char());
        self.bb.push_stmt(mcx.decl(mcx.var(ret, ty, Some(mcx.cast(ty, mcx.value(buf))))));

        (ret, ty)
    }

    fn dynamic_alloca(&mut self, size: Self::Value, align: rustc_abi::Align) -> Self::Value {
        todo!()
    }

    fn load(&mut self, ty: Self::Type, ptr: Self::Value, align: rustc_abi::Align) -> Self::Value {
        let mcx = self.cx.mcx;
        let ret = self.func.0.next_local_var();
        self.bb.push_stmt(mcx.decl(mcx.var(
            ret,
            ty,
            Some(mcx.unary("*", mcx.cast(mcx.ptr(ty), mcx.value(ptr.0)))),
        )));

        (ret, ty)
    }

    fn volatile_load(&mut self, ty: Self::Type, ptr: Self::Value) -> Self::Value {
        todo!()
    }

    fn atomic_load(
        &mut self,
        ty: Self::Type,
        ptr: Self::Value,
        order: rustc_codegen_ssa::common::AtomicOrdering,
        size: rustc_abi::Size,
    ) -> Self::Value {
        todo!()
    }

    fn load_operand(
        &mut self,
        place: PlaceRef<'tcx, Self::Value>,
    ) -> OperandRef<'tcx, Self::Value> {
        if place.val.llextra.is_none() {
            let ty = self.cx.backend_type(place.layout);
            let val = self.load(ty, place.val.llval, place.layout.align.abi);
            OperandRef::from_immediate_or_packed_pair(self, val, place.layout)
        } else {
            todo!()
        }
    }

    fn write_operand_repeatedly(
        &mut self,
        elem: OperandRef<'tcx, Self::Value>,
        count: u64,
        dest: PlaceRef<'tcx, Self::Value>,
    ) {
        todo!()
    }

    fn range_metadata(&mut self, load: Self::Value, range: rustc_abi::WrappingRange) {
        todo!()
    }

    fn nonnull_metadata(&mut self, load: Self::Value) {
        todo!()
    }

    fn store(
        &mut self,
        val: Self::Value,
        ptr: Self::Value,
        align: rustc_abi::Align,
    ) -> Self::Value {
        todo!()
    }

    fn store_with_flags(
        &mut self,
        val: Self::Value,
        ptr: Self::Value,
        align: rustc_abi::Align,
        flags: MemFlags, // TODO: align & flags
    ) -> Self::Value {
        let mcx = self.cx.mcx;
        self.bb.push_stmt(mcx.expr(mcx.assign(mcx.unary("*", mcx.value(ptr.0)), mcx.value(val.0))));
        (mcx.scalar(0), mcx.int(IntTy::I32))
    }

    fn atomic_store(
        &mut self,
        val: Self::Value,
        ptr: Self::Value,
        order: AtomicOrdering,
        size: rustc_abi::Size,
    ) {
        todo!()
    }

    fn gep(&mut self, ty: Self::Type, ptr: Self::Value, indices: &[Self::Value]) -> Self::Value {
        todo!()
    }

    fn inbounds_gep(
        &mut self,
        ty: Self::Type,
        ptr: Self::Value,
        indices: &[Self::Value],
    ) -> Self::Value {
        assert!(indices.len() == 1, "todo: inbounds_gep only supports one index");
        let mcx = self.cx.mcx;
        let ptr_ty = mcx.ptr(ty);
        let arr = self.func.0.next_local_var();
        self.bb.push_stmt(mcx.decl(mcx.var(arr, ptr_ty, Some(mcx.cast(ptr_ty, mcx.value(ptr.0))))));
        let ret = self.func.0.next_local_var();
        self.bb.push_stmt(mcx.decl(mcx.var(
            ret,
            ptr_ty,
            Some(mcx.unary("&", mcx.index(mcx.value(arr), mcx.value(indices[0].0)))),
        )));
        (ret, ptr_ty)
    }

    fn trunc(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn sext(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptoui_sat(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptosi_sat(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptoui(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptosi(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn uitofp(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn sitofp(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptrunc(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fpext(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn ptrtoint(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn inttoptr(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn bitcast(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    /// Performs cast between integers, x as ty in Rust.
    ///
    /// If the bit width is different, a truncation or extension is required.
    /// The type of extension—sign-extension or zero-extension—depends on the
    /// signedness of the source type.
    ///
    /// According to the C17 standard, section "6.3.1.3 Signed and unsigned
    /// integers", casting to an unsigned integer behaves the same as in Rust.
    /// However, casting to a signed integer is implementation-defined.
    ///
    /// Therefore, a two-step cast is necessary. First, cast to an unsigned
    /// integer via explicit conversion. Then, use a helper function to cast the
    /// result to a signed integer.
    fn intcast(&mut self, val: Self::Value, dest_ty: Self::Type, is_signed: bool) -> Self::Value {
        let mcx = self.cx.mcx;
        let ret = self.func.0.next_local_var();

        let dest = if let CTyBase::Primitive(ty) = dest_ty.base { ty } else { unreachable!() };

        let cast = if dest.is_signed() {
            let cast = mcx.cast(CTy::primitive(dest.to_unsigned()), mcx.value(val.0));
            mcx.call(
                mcx.raw("__rust_utos"),
                [
                    mcx.raw(dest.to_unsigned().to_str()),
                    mcx.raw(dest.to_str()),
                    cast,
                    mcx.raw(dest.max_value()),
                ],
            )
        } else {
            mcx.cast(CTy::primitive(dest), mcx.value(val.0))
        };
        self.bb.push_stmt(mcx.decl(mcx.var(ret, dest_ty, Some(cast))));
        (ret, dest_ty)
    }

    fn pointercast(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        if val.1 == dest_ty {
            return val;
        }

        let mcx = self.cx.mcx;
        let ret = self.func.0.next_local_var();

        self.bb.push_stmt(mcx.decl(mcx.var(
            ret,
            dest_ty,
            Some(mcx.cast(dest_ty, mcx.value(val.0))),
        )));
        (ret, dest_ty)
    }

    fn icmp(&mut self, op: IntPredicate, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        let op = match op {
            IntPredicate::IntEQ => "==",
            IntPredicate::IntNE => "!=",
            IntPredicate::IntUGT => ">",
            IntPredicate::IntUGE => ">=",
            IntPredicate::IntULT => "<",
            IntPredicate::IntULE => "<=",
            IntPredicate::IntSGT => ">",
            IntPredicate::IntSGE => ">=",
            IntPredicate::IntSLT => "<",
            IntPredicate::IntSLE => "<=",
        };

        self.binary_cmp(op, lhs, rhs)
    }

    fn fcmp(&mut self, op: RealPredicate, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn memcpy(
        &mut self,
        dst: Self::Value,
        dst_align: rustc_abi::Align,
        src: Self::Value,
        src_align: rustc_abi::Align,
        size: Self::Value,
        flags: rustc_codegen_ssa::MemFlags,
    ) {
        todo!()
    }

    fn memmove(
        &mut self,
        dst: Self::Value,
        dst_align: rustc_abi::Align,
        src: Self::Value,
        src_align: rustc_abi::Align,
        size: Self::Value,
        flags: rustc_codegen_ssa::MemFlags,
    ) {
        todo!()
    }

    fn memset(
        &mut self,
        ptr: Self::Value,
        fill_byte: Self::Value,
        size: Self::Value,
        align: rustc_abi::Align,
        flags: rustc_codegen_ssa::MemFlags,
    ) {
        todo!()
    }

    fn select(
        &mut self,
        cond: Self::Value,
        then_val: Self::Value,
        else_val: Self::Value,
    ) -> Self::Value {
        todo!()
    }

    fn va_arg(&mut self, list: Self::Value, ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn extract_element(&mut self, vec: Self::Value, idx: Self::Value) -> Self::Value {
        todo!()
    }

    fn vector_splat(&mut self, num_elts: usize, elt: Self::Value) -> Self::Value {
        todo!()
    }

    fn extract_value(&mut self, agg_val: Self::Value, idx: u64) -> Self::Value {
        // TODO: fat pointer?
        agg_val
    }

    fn insert_value(&mut self, agg_val: Self::Value, elt: Self::Value, idx: u64) -> Self::Value {
        todo!()
    }

    fn set_personality_fn(&mut self, personality: Self::Value) {
        todo!()
    }

    fn cleanup_landing_pad(&mut self, pers_fn: Self::Value) -> (Self::Value, Self::Value) {
        todo!()
    }

    fn filter_landing_pad(&mut self, pers_fn: Self::Value) -> (Self::Value, Self::Value) {
        todo!()
    }

    fn resume(&mut self, exn0: Self::Value, exn1: Self::Value) {
        todo!()
    }

    fn cleanup_pad(&mut self, parent: Option<Self::Value>, args: &[Self::Value]) -> Self::Funclet {
        todo!()
    }

    fn cleanup_ret(&mut self, funclet: &Self::Funclet, unwind: Option<Self::BasicBlock>) {
        todo!()
    }

    fn catch_pad(&mut self, parent: Self::Value, args: &[Self::Value]) -> Self::Funclet {
        todo!()
    }

    fn catch_switch(
        &mut self,
        parent: Option<Self::Value>,
        unwind: Option<Self::BasicBlock>,
        handlers: &[Self::BasicBlock],
    ) -> Self::Value {
        todo!()
    }

    fn atomic_cmpxchg(
        &mut self,
        dst: Self::Value,
        cmp: Self::Value,
        src: Self::Value,
        order: rustc_codegen_ssa::common::AtomicOrdering,
        failure_order: rustc_codegen_ssa::common::AtomicOrdering,
        weak: bool,
    ) -> (Self::Value, Self::Value) {
        todo!()
    }

    fn atomic_rmw(
        &mut self,
        op: rustc_codegen_ssa::common::AtomicRmwBinOp,
        dst: Self::Value,
        src: Self::Value,
        order: rustc_codegen_ssa::common::AtomicOrdering,
    ) -> Self::Value {
        todo!()
    }

    fn atomic_fence(
        &mut self,
        order: rustc_codegen_ssa::common::AtomicOrdering,
        scope: rustc_codegen_ssa::common::SynchronizationScope,
    ) {
        todo!()
    }

    fn set_invariant_load(&mut self, load: Self::Value) {
        todo!()
    }

    fn lifetime_start(&mut self, ptr: Self::Value, size: rustc_abi::Size) {
        todo!()
    }

    fn lifetime_end(&mut self, ptr: Self::Value, size: rustc_abi::Size) {
        todo!()
    }

    fn instrprof_increment(
        &mut self,
        fn_name: Self::Value,
        hash: Self::Value,
        num_counters: Self::Value,
        index: Self::Value,
    ) {
        todo!()
    }

    fn call(
        &mut self,
        llty: Self::Type,
        fn_attrs: Option<&CodegenFnAttrs>,
        fn_abi: Option<&FnAbi<'tcx, Ty<'tcx>>>,
        llfn: Self::Value,
        args: &[Self::Value],
        funclet: Option<&Self::Funclet>,
        instance: Option<Instance<'tcx>>,
    ) -> Self::Value {
        assert!(llfn.0.is_func(), "calling a non-function: {:?}", llfn);

        let mcx = self.cx.mcx;
        let fn_ptr = llty.fn_ptr().expect("not a function type");

        /* ABI-conversion:
           In case of undefined behavior, we use intptr_t as the Rust pointer type,
           but when calling into C libraries, the pointer type is actual C pointers.

           Therefore, when calling a `extern "C"` function, we need to convert
           the intptr_t pointers to the C pointers.
        */
        let call = match fn_ptr.abi {
            Conv::Rust => {
                let args = args.iter().map(|&(x, _)| mcx.value(x)).collect::<Box<[_]>>();
                mcx.call(mcx.value(llfn.0), args)
            }
            Conv::C => {
                let args = args
                    .iter()
                    .zip(fn_ptr.args.iter())
                    .map(|(&v, &ty)| self.pointercast(v, ty))
                    .map(|(x, _)| mcx.value(x))
                    .collect::<Box<[_]>>();
                mcx.call(mcx.value(llfn.0), args)
            }
            _ => todo!(),
        };

        let ret = if fn_ptr.ret.is_void() {
            self.bb.push_stmt(mcx.expr(call));
            CValue::Null
        } else {
            let ret = self.func.0.next_local_var();
            self.bb.push_stmt(mcx.decl(mcx.var(ret, fn_ptr.ret, Some(call))));
            ret
        };

        (ret, fn_ptr.ret)
    }

    fn zext(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn apply_attrs_to_cleanup_callsite(&mut self, llret: Self::Value) {
        todo!()
    }
}
