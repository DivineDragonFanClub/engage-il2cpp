
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapeditorcategorydata/MapEditorCategoryData.md")))]
#[::unity2::class(namespace = "App", name = "MapEditorCategoryData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: mapeditorcategorydata :: MapEditorCategoryData >)]
pub struct MapEditorCategoryData {}

#[cfg(feature = "app-mapeditorcategorydata")]
#[::unity2::methods]
impl MapEditorCategoryData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CountMax", args = 0)]
    pub fn get_count_max(self) -> i32;

    #[method(name = "set_CountMax", args = 1)]
    pub fn set_count_max(self, value: i32) -> ();

    #[method(name = "get_IconName", args = 0)]
    pub fn get_icon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_IconName", args = 1)]
    pub fn set_icon_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapeditorcategorydata")]
impl MapEditorCategoryData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEditorCategoryData),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEditorCategoryDataMethods>::ctor(this);
        this
    }
}
