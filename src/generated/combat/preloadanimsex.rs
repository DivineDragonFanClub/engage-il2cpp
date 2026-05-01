
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/preloadanimsex/PreloadAnimsEx.md")))]
#[::unity2::class(namespace = "Combat", name = "PreloadAnimsEx")]
#[parent(crate::system::object::Object)]
pub struct PreloadAnimsEx {}

#[cfg(feature = "combat-preloadanimsex")]
#[::unity2::methods]
impl PreloadAnimsEx {
    #[method(name = "not", args = 2)]
    pub fn not(
        lhs: crate::combat::preloadanims::PreloadAnims,
        rhs: crate::combat::preloadanims::PreloadAnims,
    ) -> bool;

    #[method(name = "has", args = 2)]
    pub fn has(
        lhs: crate::combat::preloadanims::PreloadAnims,
        rhs: crate::combat::preloadanims::PreloadAnims,
    ) -> bool;

    #[method(name = "IsFullLoadable", args = 1)]
    pub fn is_full_loadable(f: crate::combat::preloadanims::PreloadAnims) -> bool;

    #[method(name = "HashToBit", args = 1)]
    pub fn hash_to_bit(hash: i32) -> crate::combat::preloadanims::PreloadAnims;

    #[method(name = "GetAllPreloadNames", args = 0)]
    pub fn get_all_preload_names(
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;
}
