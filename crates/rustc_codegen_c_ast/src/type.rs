use std::collections::VecDeque;
use std::num::NonZeroUsize;

use rustc_data_structures::intern::Interned;
use rustc_target::abi::call::Conv;
use rustc_type_ir::{IntTy, UintTy};

use crate::expr::CValue;
use crate::pretty::{Printer, INDENT};
use crate::ModuleCtxt;

/// C types with qualifiers.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CTy<'mx> {
    pub base: CTyBase<'mx>,
    pub quals: CTyQuals,
}

impl<'mx> CTy<'mx> {
    pub fn primitive(ty: CPTy) -> Self {
        Self { base: CTyBase::Primitive(ty), quals: CTyQuals::empty() }
    }

    /// Whether the type is void.
    pub fn is_void(self) -> bool {
        matches!(self.base, CTyBase::Primitive(CPTy::Void))
    }

    /// Whether the type is a signed integer.
    pub fn is_signed(self) -> bool {
        if let CTyBase::Primitive(ty) = self.base {
            ty.is_signed()
        } else {
            false
        }
    }

    /// Gets the function pointer type if this is a function pointer.
    pub fn fn_ptr(self) -> Option<&'mx CFnPtr<'mx>> {
        if let CTyBase::Ref(ty) = self.base {
            if let CTyKind::FnPtr(fn_ptr) = ty.0 {
                return Some(fn_ptr);
            }
        }
        None
    }

    pub fn to_const_if(self, cond: bool) -> Self {
        if cond {
            return self;
        }
        Self { base: self.base, quals: self.quals | CTyQuals::CONST }
    }

    pub fn to_volatile_if(self, cond: bool) -> Self {
        if cond {
            return self;
        }
        Self { base: self.base, quals: self.quals | CTyQuals::VOLATILE }
    }

    pub fn to_restrict_if(self, cond: bool) -> Self {
        if cond {
            return self;
        }
        Self { base: self.base, quals: self.quals | CTyQuals::RESTRICT }
    }
}

impl<'mx> From<CTyBase<'mx>> for CTy<'mx> {
    fn from(base: CTyBase<'mx>) -> Self {
        CTy { base, quals: CTyQuals::empty() }
    }
}

impl std::fmt::Debug for CTy<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pp = Printer::new();
        pp.print_ty_decl(*self, None);
        write!(f, "{}", pp.finish())
    }
}

/// C types.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum CTyBase<'mx> {
    Primitive(CPTy),
    Ref(Interned<'mx, CTyKind<'mx>>),
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct CTyQuals: u8 {
        const CONST = 1;
        const VOLATILE = 2;
        const RESTRICT = 4;
    }
}

/// C primitive types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CPTy {
    Void,
    Bool,
    Char,

    Isize,
    I8,
    I16,
    I32,
    I64,

    Usize,
    U8,
    U16,
    U32,
    U64,
}

impl CPTy {
    /// Whether the type is a signed integer.
    pub fn is_signed(self) -> bool {
        matches!(self, CPTy::Isize | CPTy::I8 | CPTy::I16 | CPTy::I32 | CPTy::I64)
    }

    /// The unsigned version of this type.
    ///
    /// ## Panic
    ///
    /// Panics if the type is not a signed integer.
    pub fn to_unsigned(self) -> CPTy {
        match self {
            CPTy::Isize => CPTy::Usize,
            CPTy::I8 => CPTy::U8,
            CPTy::I16 => CPTy::U16,
            CPTy::I32 => CPTy::U32,
            CPTy::I64 => CPTy::U64,
            _ => unreachable!(),
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            CPTy::Void => "void",
            CPTy::Bool => "bool",
            CPTy::Char => "char",

            CPTy::Isize => "intptr_t",
            CPTy::I8 => "int8_t",
            CPTy::I16 => "int16_t",
            CPTy::I32 => "int32_t",
            CPTy::I64 => "int64_t",

            CPTy::Usize => "uintptr_t",
            CPTy::U8 => "uint8_t",
            CPTy::U16 => "uint16_t",
            CPTy::U32 => "uint32_t",
            CPTy::U64 => "uint64_t",
        }
    }

    /// The maximum value of this type. From `<stdint.h>`.
    ///
    /// ## Panic
    ///
    /// Panics if the type is not an integer.
    pub fn max_value(self) -> &'static str {
        match self {
            CPTy::Isize => "SIZE_MAX",
            CPTy::I8 => "INT8_MAX",
            CPTy::I16 => "INT16_MAX",
            CPTy::I32 => "INT32_MAX",
            CPTy::I64 => "INT64_MAX",
            CPTy::Usize => "SIZE_MAX",
            CPTy::U8 => "UINT8_MAX",
            CPTy::U16 => "UINT16_MAX",
            CPTy::U32 => "UINT32_MAX",
            CPTy::U64 => "UINT64_MAX",
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CTyKind<'mx> {
    Pointer(CTy<'mx>),
    // Record(String),
    Array(CTy<'mx>, Option<NonZeroUsize>),
    FnPtr(CFnPtr<'mx>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CFnPtr<'mx> {
    pub ret: CTy<'mx>,
    pub args: Box<[CTy<'mx>]>,
    pub abi: Conv,
}

impl<'mx> ModuleCtxt<'mx> {
    /// Get the void type
    pub const fn void(&self) -> CTy<'mx> {
        CTy { base: CTyBase::Primitive(CPTy::Void), quals: CTyQuals::empty() }
    }

    /// Get the bool type
    pub const fn bool(&self) -> CTy<'mx> {
        CTy { base: CTyBase::Primitive(CPTy::U8), quals: CTyQuals::empty() }
    }

    /// Get the char type
    pub const fn char(&self) -> CTy<'mx> {
        CTy { base: CTyBase::Primitive(CPTy::Char), quals: CTyQuals::empty() }
    }

    /// Get the type of an signed integer
    pub fn int(&self, int: IntTy) -> CTy<'mx> {
        match int {
            IntTy::Isize => CTyBase::Primitive(CPTy::Isize),
            IntTy::I8 => CTyBase::Primitive(CPTy::I8),
            IntTy::I16 => CTyBase::Primitive(CPTy::I16),
            IntTy::I32 => CTyBase::Primitive(CPTy::I32),
            IntTy::I64 => CTyBase::Primitive(CPTy::I64),
            IntTy::I128 => unimplemented!("i128 not supported yet"),
        }
        .into()
    }

    /// Get the type of an unsigned integer
    pub fn uint(&self, uint: UintTy) -> CTy<'mx> {
        match uint {
            UintTy::Usize => CTyBase::Primitive(CPTy::Usize),
            UintTy::U8 => CTyBase::Primitive(CPTy::U8),
            UintTy::U16 => CTyBase::Primitive(CPTy::U16),
            UintTy::U32 => CTyBase::Primitive(CPTy::U32),
            UintTy::U64 => CTyBase::Primitive(CPTy::U64),
            UintTy::U128 => unimplemented!("u128 not supported yet"),
        }
        .into()
    }

    /// Get the pointer type
    pub fn ptr(&self, ty: CTy<'mx>) -> CTy<'mx> {
        self.intern_ty(CTyKind::Pointer(ty)).into()
    }

    /// Get the array type
    pub fn arr(&self, ty: CTy<'mx>, n: Option<NonZeroUsize>) -> CTy<'mx> {
        self.intern_ty(CTyKind::Array(ty, n)).into()
    }

    /// Get the function type
    pub fn fn_ptr(&self, ret: CTy<'mx>, args: Box<[CTy<'mx>]>, abi: Conv) -> CTy<'mx> {
        self.intern_ty(CTyKind::FnPtr(CFnPtr { ret, args, abi })).into()
    }
}

impl Printer {
    pub fn print_ty_decl(&mut self, mut ty: CTy, val: Option<CValue>) {
        enum TyDeclPart<'mx> {
            Ident(Option<CValue<'mx>>),
            Ptr(CTyQuals),
            Array(Option<NonZeroUsize>, CTyQuals),
            FnArgs(Box<[CTy<'mx>]>),
            LParen,
            RParen,
        }

        impl<'mx> TyDeclPart<'mx> {
            fn print(&self, printer: &mut Printer) {
                match self {
                    TyDeclPart::Ident(val) => {
                        if let &Some(val) = val {
                            printer.print_value(val);
                        }
                    }
                    TyDeclPart::Ptr(quals) => {
                        printer.word("*");
                        printer.print_ty_quals(*quals);
                    }
                    TyDeclPart::Array(n, quals) => {
                        printer.word("[");
                        printer.print_ty_quals(*quals);
                        if let Some(n) = n {
                            printer.word(format!("{}", n));
                        }
                        printer.word("]");
                    }
                    TyDeclPart::FnArgs(args) => printer.ibox_delim(INDENT, ("(", ")"), |p| {
                        p.seperated(",", args, |p, arg| p.print_ty_decl(*arg, None))
                    }),
                    TyDeclPart::LParen => printer.word("("),
                    TyDeclPart::RParen => printer.word(")"),
                }
            }
        }

        let mut decl_parts = VecDeque::new();
        decl_parts.push_front(TyDeclPart::Ident(val));
        while let CTyBase::Ref(kind) = ty.base {
            match kind.0 {
                CTyKind::Pointer(_) => decl_parts.push_front(TyDeclPart::Ptr(ty.quals)),
                CTyKind::Array(_, n) => decl_parts.push_back(TyDeclPart::Array(*n, ty.quals)),
                CTyKind::FnPtr(CFnPtr { args, .. }) => {
                    decl_parts.push_front(TyDeclPart::LParen);
                    decl_parts.push_front(TyDeclPart::RParen);
                    decl_parts.push_back(TyDeclPart::FnArgs(args.clone()));
                }
            }
            ty = match kind.0 {
                CTyKind::Pointer(ty) => *ty,
                CTyKind::Array(ty, _) => *ty,
                CTyKind::FnPtr(CFnPtr { ret, .. }) => *ret,
            };
        }

        let CTyBase::Primitive(base) = ty.base else { unreachable!() };
        self.print_ty_quals(ty.quals);
        self.word(base.to_str());
        if val.is_some() {
            self.nbsp();
        }
        for part in decl_parts {
            part.print(self);
        }
    }

    fn print_ty_quals(&mut self, quals: CTyQuals) {
        if quals.contains(CTyQuals::CONST) {
            self.word("const");
            self.softbreak();
        }
        if quals.contains(CTyQuals::VOLATILE) {
            self.word("volatile");
            self.softbreak();
        }
        if quals.contains(CTyQuals::RESTRICT) {
            self.word("restrict");
            self.softbreak();
        }
    }
}
