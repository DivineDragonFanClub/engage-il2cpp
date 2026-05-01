
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubresourcedata/HubResourceData.md")))]
#[::unity2::class(namespace = "App", name = "HubResourceData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubresourcedata :: HubResourceData >)]
pub struct HubResourceData {}

#[cfg(feature = "app-hubresourcedata")]
#[::unity2::methods]
impl HubResourceData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubresourcedata")]
impl HubResourceData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubResourceData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubResourceDataMethods>::ctor(this);
        this
    }
}
