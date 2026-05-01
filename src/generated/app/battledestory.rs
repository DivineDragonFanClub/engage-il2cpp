
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battledestory/BattleDestory.md")))]
#[::unity2::class(namespace = "App", name = "BattleDestory")]
#[parent(crate::system::object::Object)]
pub struct BattleDestory {
    #[rename(name = "Inspector")]
    pub inspector: crate::app::pokeinspector::PokeInspector,
    #[rename(name = "Overlap")]
    pub overlap: crate::app::mapoverlap::MapOverlap_Data,
}

#[cfg(feature = "app-battledestory")]
#[::unity2::methods]
impl BattleDestory {
    #[method(name = "Entry", args = 2)]
    pub fn entry(self, x: i32, z: i32) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_Exist", args = 0)]
    pub fn get_exist(self) -> bool;

    #[method(name = "get_Hp", args = 0)]
    pub fn get_hp(self) -> i32;

    #[method(name = "set_Hp", args = 1)]
    pub fn set_hp(self, value: i32) -> ();

    #[method(name = "get_MaxHp", args = 0)]
    pub fn get_max_hp(self) -> i32;

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "PlayDamage", args = 1)]
    pub fn play_damage(self, hp: i32) -> ();

    #[method(name = "Commit", args = 1)]
    pub fn commit(self, hp: i32) -> ();

    #[method(name = "Update", args = 1)]
    pub fn update(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-battledestory")]
impl BattleDestory {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleDestory),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleDestoryMethods>::ctor(this);
        this
    }
}
