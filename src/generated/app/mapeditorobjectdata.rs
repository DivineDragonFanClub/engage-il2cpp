
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeditorobjectdata/MapEditorObjectData.md")))]
#[::unity2::class(namespace = "App", name = "MapEditorObjectData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: mapeditorobjectdata :: MapEditorObjectData >)]
pub struct MapEditorObjectData {}

#[cfg(feature = "app-mapeditorobjectdata")]
#[::unity2::methods]
impl MapEditorObjectData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_ObjectName", args = 0)]
    pub fn get_object_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ObjectName", args = 1)]
    pub fn set_object_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SoundEvent", args = 0)]
    pub fn get_sound_event(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SoundEvent", args = 1)]
    pub fn set_sound_event(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Category", args = 0)]
    pub fn get_category(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Category", args = 1)]
    pub fn set_category(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ObjectKey", args = 0)]
    pub fn get_object_key(self) -> i32;

    #[method(name = "set_ObjectKey", args = 1)]
    pub fn set_object_key(self, value: i32) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnCompleted", args = 0)]
    pub fn on_completed(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapeditorobjectdata")]
impl MapEditorObjectData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditorObjectData),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditorObjectDataMethods>::ctor(this);
        this
    }
}
