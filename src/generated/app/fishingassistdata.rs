
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingassistdata/FishingAssistData.md")))]
#[::unity2::class(namespace = "App", name = "FishingAssistData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: fishingassistdata :: FishingAssistData >)]
pub struct FishingAssistData {
    #[rename(name = "Param")]
    pub param: ::unity2::Array<f32>,
}

#[cfg(feature = "app-fishingassistdata")]
#[::unity2::methods]
impl FishingAssistData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Level_00", args = 0)]
    pub fn get_level_00(self) -> f32;

    #[method(name = "set_Level_00", args = 1)]
    pub fn set_level_00(self, value: f32) -> ();

    #[method(name = "get_Level_01", args = 0)]
    pub fn get_level_01(self) -> f32;

    #[method(name = "set_Level_01", args = 1)]
    pub fn set_level_01(self, value: f32) -> ();

    #[method(name = "get_Level_02", args = 0)]
    pub fn get_level_02(self) -> f32;

    #[method(name = "set_Level_02", args = 1)]
    pub fn set_level_02(self, value: f32) -> ();

    #[method(name = "get_Level_03", args = 0)]
    pub fn get_level_03(self) -> f32;

    #[method(name = "set_Level_03", args = 1)]
    pub fn set_level_03(self, value: f32) -> ();

    #[method(name = "get_Level_04", args = 0)]
    pub fn get_level_04(self) -> f32;

    #[method(name = "set_Level_04", args = 1)]
    pub fn set_level_04(self, value: f32) -> ();

    #[method(name = "get_Level_05", args = 0)]
    pub fn get_level_05(self) -> f32;

    #[method(name = "set_Level_05", args = 1)]
    pub fn set_level_05(self, value: f32) -> ();

    #[method(name = "get_Level_06", args = 0)]
    pub fn get_level_06(self) -> f32;

    #[method(name = "set_Level_06", args = 1)]
    pub fn set_level_06(self, value: f32) -> ();

    #[method(name = "get_Level_07", args = 0)]
    pub fn get_level_07(self) -> f32;

    #[method(name = "set_Level_07", args = 1)]
    pub fn set_level_07(self, value: f32) -> ();

    #[method(name = "get_Level_08", args = 0)]
    pub fn get_level_08(self) -> f32;

    #[method(name = "set_Level_08", args = 1)]
    pub fn set_level_08(self, value: f32) -> ();

    #[method(name = "get_Level_09", args = 0)]
    pub fn get_level_09(self) -> f32;

    #[method(name = "set_Level_09", args = 1)]
    pub fn set_level_09(self, value: f32) -> ();

    #[method(name = "get_Level_10", args = 0)]
    pub fn get_level_10(self) -> f32;

    #[method(name = "set_Level_10", args = 1)]
    pub fn set_level_10(self, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();
}

#[cfg(feature = "app-fishingassistdata")]
impl FishingAssistData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingAssistData),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingAssistDataMethods>::ctor(this);
        this
    }
}
