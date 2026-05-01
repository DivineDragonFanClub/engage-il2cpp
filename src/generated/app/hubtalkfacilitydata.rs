
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubtalkfacilitydata/HubTalkFacilityData.md")))]
#[::unity2::class(namespace = "App", name = "HubTalkFacilityData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubtalkfacilitydata :: HubTalkFacilityData >)]
pub struct HubTalkFacilityData {}

#[cfg(feature = "app-hubtalkfacilitydata")]
#[::unity2::methods]
impl HubTalkFacilityData {
    #[method(name = "get_Pattern", args = 0)]
    pub fn get_pattern(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pattern", args = 1)]
    pub fn set_pattern(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PID", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PID", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubtalkfacilitydata")]
impl HubTalkFacilityData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubTalkFacilityData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubTalkFacilityDataMethods>::ctor(this);
        this
    }
}
