use std::cell::RefCell;
use std::hash::Hash;

use rustc_data_structures::fingerprint::Fingerprint;
use rustc_data_structures::intern::Interned;
use rustc_data_structures::stable_hasher::StableHasher;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::arena::Arena;
use crate::r#type::{CTyBase, CTyKind};
use crate::ModuleCtxt;

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

/// A manager for unique names.
#[derive(Debug, Default, Clone)]
pub struct NameManager<'mx> {
    names: RefCell<FxHashSet<&'mx str>>,
}

impl<'mx> NameManager<'mx> {
    /// Create a new name manager.
    pub fn new() -> Self {
        Self { names: RefCell::new(FxHashSet::default()) }
    }

    /// Generate a new unique name.
    pub fn intern(&self, name: &str, mcx: &ModuleCtxt<'mx>) -> &'mx str {
        let mut names = self.names.borrow_mut();
        if !names.contains(name) {
            let name = mcx.alloc_str(name);
            names.insert(name);
            return name;
        }

        let mut alt = 1;
        loop {
            let name = mcx.alloc_str(&format!("{}_{}", name, alt));
            if !names.contains(name) {
                names.insert(name);
                return name;
            }
            alt += 1;
        }
    }
}
