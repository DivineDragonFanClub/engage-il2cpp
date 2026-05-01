
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeditchecker/MapEditChecker_CheckRange.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapEditChecker_CheckRange {
    pub min_x: i32,
    pub max_x: i32,
    pub min_z: i32,
    pub max_z: i32,
}

impl ::unity2::ClassIdentity for MapEditChecker_CheckRange {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapEditChecker.CheckRange";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapEditChecker_CheckRange {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapeditchecker")]
#[::unity2::methods(value)]
impl MapEditChecker_CheckRange {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(self, x1: i32, x2: i32, z1: i32, z2: i32) -> ();

    #[method(name = "IsRange", args = 1)]
    pub fn is_range(self, pos: crate::app::mappos::MapPos) -> bool;

    #[method(name = "IsRange", args = 2)]
    pub fn is_range_2(self, x: i32, z: i32) -> bool;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeditchecker/MapEditChecker.md")))]
#[::unity2::class(namespace = "App", name = "MapEditChecker")]
#[parent(crate::system::object::Object)]
pub struct MapEditChecker {}

#[cfg(feature = "app-mapeditchecker")]
#[::unity2::methods]
impl MapEditChecker {
    #[method(name = "Check", args = 1)]
    pub fn check(is_opponent: bool) -> bool;

    #[method(name = "get_SizeX", args = 0)]
    pub fn get_size_x() -> i32;

    #[method(name = "get_SizeZ", args = 0)]
    pub fn get_size_z() -> i32;

    #[method(name = "GetUnitCount", args = 0)]
    pub fn get_unit_count() -> i32;

    #[method(name = "GetOpponentCount", args = 0)]
    pub fn get_opponent_count() -> i32;

    #[method(name = "CanMove", args = 3)]
    pub fn can_move(x: i32, z: i32, breakable_is_true: bool) -> bool;

    #[method(name = "CanMoveBreakable", args = 2)]
    pub fn can_move_breakable(x: i32, z: i32) -> bool;

    #[method(name = "GetTerrain", args = 2)]
    pub fn get_terrain(x: i32, z: i32) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "ExistsUnit", args = 2)]
    pub fn exists_unit(x: i32, z: i32) -> bool;

    #[method(name = "CheckMyMap", args = 0)]
    pub fn check_my_map() -> bool;

    #[method(name = "CheckOpponentMap", args = 0)]
    pub fn check_opponent_map() -> bool;

    #[method(name = "CheckRecursive", args = 4)]
    pub fn check_recursive(
        pos: crate::app::mappos::MapPos,
        range: crate::app::mapeditchecker::MapEditChecker_CheckRange,
        reach_set: crate::system::collections::generic::hashset_1::HashSet_1<
            crate::app::mappos::MapPos,
        >,
        total_count: i32,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapeditchecker")]
impl MapEditChecker {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditChecker),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditCheckerMethods>::ctor(this);
        this
    }
}
