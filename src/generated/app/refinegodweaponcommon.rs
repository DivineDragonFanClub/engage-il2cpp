
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refinegodweaponcommon/RefineGodWeaponCommon.md")))]
#[::unity2::class(namespace = "App", name = "RefineGodWeaponCommon")]
#[parent(crate::system::object::Object)]
pub struct RefineGodWeaponCommon {}

#[cfg(feature = "app-refinegodweaponcommon")]
#[::unity2::methods]
impl RefineGodWeaponCommon {
    #[method(name = "GetCapacity", args = 1)]
    pub fn get_capacity(god_unit: crate::app::godunit::GodUnit) -> i32;

    #[method(name = "HasEfficacySkills", args = 1)]
    pub fn has_efficacy_skills(item_data: crate::app::itemdata::ItemData) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refinegodweaponcommon")]
impl RefineGodWeaponCommon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineGodWeaponCommon),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineGodWeaponCommonMethods>::ctor(this);
        this
    }
}
