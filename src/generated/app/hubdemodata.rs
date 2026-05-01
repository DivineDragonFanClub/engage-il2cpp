
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubdemodata/HubDemoData.md")))]
#[::unity2::class(namespace = "App", name = "HubDemoData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubdemodata :: HubDemoData >)]
pub struct HubDemoData {}

#[cfg(feature = "app-hubdemodata")]
#[::unity2::methods]
impl HubDemoData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Locator", args = 0)]
    pub fn get_locator(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Locator", args = 1)]
    pub fn set_locator(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MID", args = 0)]
    pub fn get_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MID", args = 1)]
    pub fn set_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CameraName", args = 0)]
    pub fn get_camera_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CameraName", args = 1)]
    pub fn set_camera_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Tutorial", args = 0)]
    pub fn get_tutorial(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Tutorial", args = 1)]
    pub fn set_tutorial(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Timezone", args = 0)]
    pub fn get_timezone(self) -> i32;

    #[method(name = "set_Timezone", args = 1)]
    pub fn set_timezone(self, value: i32) -> ();

    #[method(name = "get_FlagName", args = 0)]
    pub fn get_flag_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FlagName", args = 1)]
    pub fn set_flag_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ManualCullingName", args = 0)]
    pub fn get_manual_culling_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ManualCullingName", args = 1)]
    pub fn set_manual_culling_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LodBias", args = 0)]
    pub fn get_lod_bias(self) -> f32;

    #[method(name = "set_LodBias", args = 1)]
    pub fn set_lod_bias(self, value: f32) -> ();

    #[method(name = "get_IsDisabledLodCrossfadeAnime", args = 0)]
    pub fn get_is_disabled_lod_crossfade_anime(self) -> bool;

    #[method(name = "set_IsDisabledLodCrossfadeAnime", args = 1)]
    pub fn set_is_disabled_lod_crossfade_anime(self, value: bool) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubdemodata")]
impl HubDemoData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubDemoData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubDemoDataMethods>::ctor(this);
        this
    }
}
