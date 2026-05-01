
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::combat::combatsubspace::CombatSubspace;
use crate::combat::combatsubspace::ICombatSubspace;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatsubspacego/CombatSubspaceGo.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatSubspaceGo")]
#[parent(crate::combat::combatsubspace::CombatSubspace)]
pub struct CombatSubspaceGo {
    #[rename(name = "m_SceneName")]
    pub m_scene_name: ::unity2::Il2CppString,
}

#[cfg(feature = "combat-combatsubspacego")]
#[::unity2::methods]
impl CombatSubspaceGo {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, scene_name: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadScene", args = 0)]
    pub fn load_scene(self) -> ();

    #[method(name = "SwapScene", args = 0)]
    pub fn swap_scene(self) -> ();

    #[method(name = "ChangeCamera", args = 0)]
    pub fn change_camera(self) -> ();
}

#[cfg(feature = "combat-combatsubspacego")]
impl CombatSubspaceGo {
    pub fn new(scene_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatSubspaceGo),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatSubspaceGoMethods>::ctor(this, scene_name);
        this
    }
}
