
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musicdata/MusicData.md")))]
#[::unity2::class(namespace = "App", name = "MusicData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: musicdata :: MusicData >)]
pub struct MusicData {}

#[cfg(feature = "app-musicdata")]
#[::unity2::methods]
impl MusicData {
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

    #[method(name = "get_Help", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Help", args = 1)]
    pub fn set_help(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Amiibo", args = 0)]
    pub fn get_amiibo(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Amiibo", args = 1)]
    pub fn set_amiibo(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ChangeEventName", args = 0)]
    pub fn get_change_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ChangeEventName", args = 1)]
    pub fn set_change_event_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsChange", args = 0)]
    pub fn get_is_change(self) -> bool;

    #[method(name = "set_IsChange", args = 1)]
    pub fn set_is_change(self, value: bool) -> ();

    #[method(name = "get_Gid", args = 0)]
    pub fn get_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Gid", args = 1)]
    pub fn set_gid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_GodData", args = 0)]
    pub fn get_god_data(self) -> crate::app::goddata::GodData;

    #[method(name = "set_GodData", args = 1)]
    pub fn set_god_data(self, value: crate::app::goddata::GodData) -> ();

    #[method(name = "OnCompleted", args = 0)]
    pub fn on_completed(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "FindData", args = 1)]
    pub fn find_data(change_event_name: ::unity2::Il2CppString)
        -> crate::app::musicdata::MusicData;

    #[method(name = "TryGetFromGodData", args = 1)]
    pub fn try_get_from_god_data(
        god_data: crate::app::goddata::GodData,
    ) -> crate::app::musicdata::MusicData;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-musicdata")]
impl MusicData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MusicData),
                ::core::stringify!(new),
            )
        });
        <Self as IMusicDataMethods>::ctor(this);
        this
    }
}
