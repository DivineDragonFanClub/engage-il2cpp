
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubmaterialbonus/HubMaterialBonus.md")))]
#[::unity2::class(namespace = "App", name = "HubMaterialBonus")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubmaterialbonus :: HubMaterialBonus >)]
pub struct HubMaterialBonus {}

#[cfg(feature = "app-hubmaterialbonus")]
#[::unity2::methods]
impl HubMaterialBonus {
    #[method(name = "get_CID", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CID", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "set_Count", args = 1)]
    pub fn set_count(self, value: i32) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubmaterialbonus")]
impl HubMaterialBonus {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubMaterialBonus),
                ::core::stringify!(new),
            )
        });
        <Self as IHubMaterialBonusMethods>::ctor(this);
        this
    }
}
