
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmobunit/GmapMobUnit.md")))]
#[::unity2::class(namespace = "App", name = "GmapMobUnit")]
#[parent(crate::system::object::Object)]
pub struct GmapMobUnit {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-gmapmobunit")]
#[::unity2::methods]
impl GmapMobUnit {
    #[method(name = "get_Actor", args = 0)]
    pub fn get_actor(self) -> crate::app::unitactor::UnitActor;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        iid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetUnitFromUnitPool", args = 3)]
    pub fn set_unit_from_unit_pool(
        self,
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        iid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetPosition", args = 3)]
    pub fn set_position(self, x: f32, y: f32, z: f32) -> ();

    #[method(name = "LoadActor", args = 0)]
    pub fn load_actor(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "UnloadUnit", args = 0)]
    pub fn unload_unit(self) -> ();
}

#[cfg(feature = "app-gmapmobunit")]
impl GmapMobUnit {
    pub fn new(
        pid: ::unity2::Il2CppString,
        jid: ::unity2::Il2CppString,
        iid: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMobUnit),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapMobUnitMethods>::ctor(this, pid, jid, iid);
        this
    }
}
