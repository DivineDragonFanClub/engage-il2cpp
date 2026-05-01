
use crate::app::linknode_1::ILinkNode_1;
use crate::app::linknode_1::LinkNode_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitring/UnitRing.md")))]
#[::unity2::class(namespace = "App", name = "UnitRing")]
# [parent (crate :: app :: linknode_1 :: LinkNode_1 < crate :: app :: unitring :: UnitRing >)]
pub struct UnitRing {
    #[static_field]
    #[rename(name = "MaxStockCount")]
    pub max_stock_count: i32,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::ringdata::RingData,
    #[rename(name = "m_Owner")]
    pub m_owner: crate::app::unit::Unit,
    #[rename(name = "m_StockCount")]
    pub m_stock_count: u8,
}

#[cfg(feature = "app-unitring")]
#[::unity2::methods]
impl UnitRing {
    #[method(name = "Build", args = 3)]
    pub fn build(
        self,
        rnid: ::unity2::Il2CppString,
        owner: crate::app::unit::Unit,
        stock_count: i32,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetFullName", args = 0)]
    pub fn get_full_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ChangeOwner", args = 1)]
    pub fn change_owner(self, owner: crate::app::unit::Unit) -> ();

    #[method(name = "SetStockCount", args = 1)]
    pub fn set_stock_count(self, stock_count: i32) -> ();

    #[method(name = "AddStockCount", args = 1)]
    pub fn add_stock_count(self, stock_count: i32) -> ();

    #[method(name = "get_SortKey", args = 0)]
    pub fn get_sort_key(self) -> i32;

    #[method(name = "IsSingleStockOnly", args = 0)]
    pub fn is_single_stock_only(self) -> bool;

    #[method(name = "IsSingleStockOnly", args = 2)]
    pub fn is_single_stock_only_2(
        data: crate::app::ringdata::RingData,
        owner: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::app::ringdata::RingData;

    #[method(name = "get_Owner", args = 0)]
    pub fn get_owner(self) -> crate::app::unit::Unit;

    #[method(name = "get_StockCount", args = 0)]
    pub fn get_stock_count(self) -> i32;

    #[method(name = "get_Rnid", args = 0)]
    pub fn get_rnid(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitring")]
impl UnitRing {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitRing),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitRingMethods>::ctor(this);
        this
    }
}
