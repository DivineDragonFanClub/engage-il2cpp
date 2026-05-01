
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/jukeboxdata/JukeboxData.md")))]
#[::unity2::class(namespace = "App", name = "JukeboxData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: jukeboxdata :: JukeboxData >)]
pub struct JukeboxData {}

#[cfg(feature = "app-jukeboxdata")]
#[::unity2::methods]
impl JukeboxData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_EventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EventName", args = 1)]
    pub fn set_event_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-jukeboxdata")]
impl JukeboxData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JukeboxData),
                ::core::stringify!(new),
            )
        });
        <Self as IJukeboxDataMethods>::ctor(this);
        this
    }
}
