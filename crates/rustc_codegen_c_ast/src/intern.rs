use std::cell::RefCell;
use std::hash::Hash;

use rustc_data_structures::fingerprint::Fingerprint;
use rustc_data_structures::intern::Interned;
use rustc_data_structures::stable_hasher::StableHasher;
use rustc_hash::FxHashMap;

use crate::arena::Arena;
use crate::r#type::{CTyBase, CTyKind};

#[derive(Default)]
pub struct Interner<'mx> {
    ty: RefCell<FxHashMap<Fingerprint, CTyBase<'mx>>>,
}

impl<'mx> Interner<'mx> {
    pub fn intern_ty(&self, arena: &'mx Arena<'mx>, ty: CTyKind<'mx>) -> CTyBase<'mx> {
        let fingerprint = fingerprint(&ty);
        *self
            .ty
            .borrow_mut()
            .entry(fingerprint)
            .or_insert_with(|| CTyBase::Ref(Interned::new_unchecked(arena.alloc(ty))))
    }
}

fn fingerprint(x: impl Hash) -> Fingerprint {
    let mut hasher = StableHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}
