
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musclepushupareadata/MusclePushUpAreaData.md")))]
#[::unity2::class(namespace = "App", name = "MusclePushUpAreaData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: musclepushupareadata :: MusclePushUpAreaData >)]
pub struct MusclePushUpAreaData {}

#[cfg(feature = "app-musclepushupareadata")]
#[::unity2::methods]
impl MusclePushUpAreaData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i32;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i32) -> ();

    #[method(name = "get_AreaP_Center", args = 0)]
    pub fn get_area_p_center(self) -> f32;

    #[method(name = "set_AreaP_Center", args = 1)]
    pub fn set_area_p_center(self, value: f32) -> ();

    #[method(name = "get_AreaP_Radius", args = 0)]
    pub fn get_area_p_radius(self) -> f32;

    #[method(name = "set_AreaP_Radius", args = 1)]
    pub fn set_area_p_radius(self, value: f32) -> ();

    #[method(name = "get_AreaG_Center", args = 0)]
    pub fn get_area_g_center(self) -> f32;

    #[method(name = "set_AreaG_Center", args = 1)]
    pub fn set_area_g_center(self, value: f32) -> ();

    #[method(name = "get_AreaG_Radius", args = 0)]
    pub fn get_area_g_radius(self) -> f32;

    #[method(name = "set_AreaG_Radius", args = 1)]
    pub fn set_area_g_radius(self, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetParam", args = 1)]
    pub fn get_param(level: i32) -> crate::app::musclepushupareadata::MusclePushUpAreaData;
}

#[cfg(feature = "app-musclepushupareadata")]
impl MusclePushUpAreaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MusclePushUpAreaData),
                ::core::stringify!(new),
            )
        });
        <Self as IMusclePushUpAreaDataMethods>::ctor(this);
        this
    }
}
