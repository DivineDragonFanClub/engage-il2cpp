
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::app::singletonpool_2::ISingletonPool_2;
use crate::app::singletonpool_2::SingletonPool_2;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitringpool/UnitRingPool.md")))]
#[::unity2::class(namespace = "App", name = "UnitRingPool")]
# [parent (crate :: app :: singletonpool_2 :: SingletonPool_2 < crate :: app :: unitringpool :: UnitRingPool , crate :: app :: unitring :: UnitRing >)]
pub struct UnitRingPool {
    #[static_field]
    #[rename(name = "MaxRingCount")]
    pub max_ring_count: i32,
    #[static_field]
    #[rename(name = "MaxOwnerCount")]
    pub max_owner_count: i32,
    #[static_field]
    #[rename(name = "MaxTotalCount")]
    pub max_total_count: i32,
}

#[cfg(feature = "app-unitringpool")]
#[::unity2::methods]
impl UnitRingPool {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "TryGet", args = 2)]
    pub fn try_get(
        rnid: ::unity2::Il2CppString,
        owner: crate::app::unit::Unit,
    ) -> crate::app::unitring::UnitRing;

    #[method(name = "TryGet", args = 2)]
    pub fn try_get_2(
        data: crate::app::ringdata::RingData,
        owner: crate::app::unit::Unit,
    ) -> crate::app::unitring::UnitRing;

    #[method(name = "GetAllStockCount", args = 1)]
    pub fn get_all_stock_count(data: crate::app::ringdata::RingData) -> i32;

    #[method(name = "GetStockCount", args = 2)]
    pub fn get_stock_count(data: crate::app::ringdata::RingData, is_equipped: bool) -> i32;

    #[method(name = "CanAdd", args = 2)]
    pub fn can_add(rnid: ::unity2::Il2CppString, stock_count: i32) -> bool;

    #[method(name = "CanAdd", args = 2)]
    pub fn can_add_2(ring: crate::app::unitring::UnitRing, stock_count: i32) -> bool;

    #[method(name = "Add", args = 3)]
    pub fn add(
        rnid: ::unity2::Il2CppString,
        owner: crate::app::unit::Unit,
        stock_count: i32,
    ) -> crate::app::unitring::UnitRing;

    #[method(name = "Add", args = 2)]
    pub fn add_2(ring: crate::app::unitring::UnitRing, stock_count: i32) -> ();

    #[method(name = "Sub", args = 3)]
    pub fn sub(rnid: ::unity2::Il2CppString, unit: crate::app::unit::Unit, stock_count: i32) -> ();

    #[method(name = "Sub", args = 2)]
    pub fn sub_2(ring: crate::app::unitring::UnitRing, stock_count: i32) -> ();

    #[method(name = "SetOwner", args = 2)]
    pub fn set_owner(rnid: ::unity2::Il2CppString, owner: crate::app::unit::Unit) -> ();

    #[method(name = "SetOwner", args = 2)]
    pub fn set_owner_2(ring: crate::app::unitring::UnitRing, owner: crate::app::unit::Unit) -> ();

    #[method(name = "ClearOwner", args = 2)]
    pub fn clear_owner(rnid: ::unity2::Il2CppString, owner: crate::app::unit::Unit) -> ();

    #[method(name = "ClearOwner", args = 1)]
    pub fn clear_owner_2(ring: crate::app::unitring::UnitRing) -> ();

    #[method(name = "Composite", args = 2)]
    pub fn composite(
        base_ring: crate::app::unitring::UnitRing,
        rings: ::unity2::Array<crate::app::unitring::UnitRing>,
    ) -> crate::app::unitring::UnitRing;

    #[method(name = "Delete", args = 2)]
    pub fn delete(rnid: ::unity2::Il2CppString, owner: crate::app::unit::Unit) -> ();

    #[method(name = "Delete", args = 1)]
    pub fn delete_2(ring: crate::app::unitring::UnitRing) -> ();

    #[method(name = "DeleteFromPool", args = 1)]
    pub fn delete_from_pool(ring: crate::app::unitring::UnitRing) -> ();

    #[method(name = "DbgAddAllCommonRings", args = 1)]
    pub fn dbg_add_all_common_rings(count: i32) -> ();
}

#[cfg(feature = "app-unitringpool")]
impl UnitRingPool {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitRingPool),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitRingPoolMethods>::ctor(this);
        this
    }
}
