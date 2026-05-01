
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitpool/UnitPool.md")))]
#[::unity2::class(namespace = "App", name = "UnitPool")]
#[parent(crate::system::object::Object)]
pub struct UnitPool {
    #[static_field]
    #[rename(name = "UnitMax")]
    pub unit_max: i32,
    #[static_field]
    #[rename(name = "MapForceUnitMax")]
    pub map_force_unit_max: i32,
    #[static_field]
    #[rename(name = "GuestMax")]
    pub guest_max: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "s_Units")]
    pub s_units: ::unity2::Array<crate::app::unit::Unit>,
    #[static_field]
    #[rename(name = "s_Forces")]
    pub s_forces: ::unity2::Array<crate::app::force::Force>,
}

#[cfg(feature = "app-unitpool")]
#[::unity2::methods]
impl UnitPool {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset() -> ();

    #[method(name = "GetCount", args = 1)]
    pub fn get_count(force_mask: u32) -> i32;

    #[method(name = "Get", args = 1)]
    pub fn get(index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetDirect", args = 1)]
    pub fn get_direct(index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetFirst", args = 2)]
    pub fn get_first(force_mask: u32, start_force_index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetLast", args = 2)]
    pub fn get_last(force_mask: u32, start_force_index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetHero", args = 1)]
    pub fn get_hero(consider_relay: bool) -> crate::app::unit::Unit;

    #[method(name = "GetFromPerson", args = 2)]
    pub fn get_from_person(
        person: crate::app::persondata::PersonData,
        consider_relay: bool,
    ) -> crate::app::unit::Unit;

    #[method(name = "TryGetFromPerson", args = 2)]
    pub fn try_get_from_person(
        pid: ::unity2::Il2CppString,
        consider_relay: bool,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetFromPerson", args = 2)]
    pub fn get_from_person_2(
        pid: ::unity2::Il2CppString,
        consider_relay: bool,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetFromPerson", args = 2)]
    pub fn get_from_person_3(
        person: crate::app::persondata::PersonData,
        force_mask: u32,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetFromPerson", args = 2)]
    pub fn get_from_person_4(
        pid: ::unity2::Il2CppString,
        force_mask: u32,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetFromFace", args = 2)]
    pub fn get_from_face(
        person: crate::app::persondata::PersonData,
        consider_relay: bool,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetFromFace", args = 2)]
    pub fn get_from_face_2(
        pid: ::unity2::Il2CppString,
        consider_relay: bool,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetForce", args = 1)]
    pub fn get_force(index: i32) -> crate::app::force::Force;

    #[method(name = "Serialize", args = 2)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2, force_mask: u32) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeForOnline", args = 1)]
    pub fn deserialize_for_online(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeImpl", args = 2)]
    pub fn deserialize_impl(stream: crate::app::stream_2::Stream_2, is_online: bool) -> ();

    #[method(name = "CalcIdent", args = 1)]
    pub fn calc_ident(unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetFromIdent", args = 2)]
    pub fn get_from_ident(
        force: crate::app::force::Force_Type,
        ident: i32,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetEnemyIdent", args = 2)]
    pub fn get_enemy_ident(
        force: crate::app::force::Force_Type,
        ident: i32,
    ) -> crate::app::unit::Unit;

    #[method(name = "Dump", args = 0)]
    pub fn dump() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitpool")]
impl UnitPool {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitPool),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitPoolMethods>::ctor(this);
        this
    }
}
