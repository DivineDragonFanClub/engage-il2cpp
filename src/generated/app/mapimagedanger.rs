
use crate::app::mapimagecorebit::IMapImageCoreBit;
use crate::app::mapimagecorebit::MapImageCoreBit;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagedanger/MapImageDanger.md")))]
#[::unity2::class(namespace = "App", name = "MapImageDanger")]
#[parent(crate::app::mapimagecorebit::MapImageCoreBit)]
pub struct MapImageDanger {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Rod")]
    pub m_rod: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_AttackAll")]
    pub m_attack_all: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_RodAll")]
    pub m_rod_all: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_GunnerAll")]
    pub m_gunner_all: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_IsUpdate")]
    pub m_is_update: bool,
    #[rename(name = "m_UpdateIndex")]
    pub m_update_index: i32,
    #[rename(name = "m_UpdateTargets")]
    pub m_update_targets:
        crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_MapDnagerDeploy")]
    pub m_map_dnager_deploy: crate::app::mapdnagerdeploy::MapDnagerDeploy,
}

#[cfg(feature = "app-mapimagedanger")]
#[::unity2::methods]
impl MapImageDanger {
    #[method(name = "ClearImage", args = 0)]
    pub fn clear_image(self) -> ();

    #[method(name = "ClearUpdate", args = 0)]
    pub fn clear_update(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "GetRod", args = 2)]
    pub fn get_rod(self, x: i32, z: i32) -> bool;

    #[method(name = "GetAttackAll", args = 2)]
    pub fn get_attack_all(self, x: i32, z: i32) -> bool;

    #[method(name = "GetRodAll", args = 2)]
    pub fn get_rod_all(self, x: i32, z: i32) -> bool;

    #[method(name = "GetGunnerAll", args = 2)]
    pub fn get_gunner_all(self, x: i32, z: i32) -> bool;

    #[method(name = "ExistsRod", args = 0)]
    pub fn exists_rod(self) -> bool;

    #[method(name = "ExistsAttackAll", args = 0)]
    pub fn exists_attack_all(self) -> bool;

    #[method(name = "ExistsRodAll", args = 0)]
    pub fn exists_rod_all(self) -> bool;

    #[method(name = "ExistsGunnerAll", args = 0)]
    pub fn exists_gunner_all(self) -> bool;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "IsUpdateTarget", args = 1)]
    pub fn is_update_target(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "Coroutine", args = 0)]
    pub fn coroutine(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapimagedanger")]
impl MapImageDanger {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageDanger),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageDangerMethods>::ctor(this);
        this
    }
}
