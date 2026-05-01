
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubrandomset/HubRandomSet.md")))]
#[::unity2::class(namespace = "App", name = "HubRandomSet")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: hubrandomset :: HubRandomSet >)]
pub struct HubRandomSet {}

#[cfg(feature = "app-hubrandomset")]
#[::unity2::methods]
impl HubRandomSet {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Rate", args = 0)]
    pub fn get_rate(self) -> i32;

    #[method(name = "set_Rate", args = 1)]
    pub fn set_rate(self, value: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "set_Count", args = 1)]
    pub fn set_count(self, value: i32) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubrandomset")]
impl HubRandomSet {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubRandomSet),
                ::core::stringify!(new),
            )
        });
        <Self as IHubRandomSetMethods>::ctor(this);
        this
    }
}
