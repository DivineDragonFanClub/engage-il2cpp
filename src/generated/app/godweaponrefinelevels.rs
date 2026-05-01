
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godweaponrefinelevels/GodWeaponRefineLevels.md")))]
#[::unity2::class(namespace = "App", name = "GodWeaponRefineLevels")]
#[parent(crate::system::object::Object)]
pub struct GodWeaponRefineLevels {
    #[rename(name = "m_Capacity")]
    pub m_capacity: u8,
    #[rename(name = "m_Power")]
    pub m_power: u8,
    #[rename(name = "m_Hit")]
    pub m_hit: u8,
    #[rename(name = "m_Critical")]
    pub m_critical: u8,
    #[rename(name = "m_Avoid")]
    pub m_avoid: u8,
    #[rename(name = "m_Secure")]
    pub m_secure: u8,
    #[rename(name = "m_Tech")]
    pub m_tech: u8,
    #[rename(name = "m_Quick")]
    pub m_quick: u8,
    #[rename(name = "m_Def")]
    pub m_def: u8,
    #[rename(name = "m_Mdef")]
    pub m_mdef: u8,
    #[rename(name = "m_Sid")]
    pub m_sid: ::unity2::Il2CppString,
}

#[cfg(feature = "app-godweaponrefinelevels")]
#[::unity2::methods]
impl GodWeaponRefineLevels {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, from: crate::app::godweaponrefinelevels::GodWeaponRefineLevels) -> ();
}

#[cfg(feature = "app-godweaponrefinelevels")]
impl GodWeaponRefineLevels {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodWeaponRefineLevels),
                ::core::stringify!(new),
            )
        });
        <Self as IGodWeaponRefineLevelsMethods>::ctor(this);
        this
    }
}
