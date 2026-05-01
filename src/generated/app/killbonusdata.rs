
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/killbonusdata/KillBonusData.md")))]
#[::unity2::class(namespace = "App", name = "KillBonusData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: killbonusdata :: KillBonusData >)]
pub struct KillBonusData {}

#[cfg(feature = "app-killbonusdata")]
#[::unity2::methods]
impl KillBonusData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Rate", args = 0)]
    pub fn get_rate(self) -> u8;

    #[method(name = "set_Rate", args = 1)]
    pub fn set_rate(self, value: u8) -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-killbonusdata")]
impl KillBonusData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KillBonusData),
                ::core::stringify!(new),
            )
        });
        <Self as IKillBonusDataMethods>::ctor(this);
        this
    }
}
