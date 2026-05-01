
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingsizetabledata/FishingSizeTableData.md")))]
#[::unity2::class(namespace = "App", name = "FishingSizeTableData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: fishingsizetabledata :: FishingSizeTableData >)]
pub struct FishingSizeTableData {}

#[cfg(feature = "app-fishingsizetabledata")]
#[::unity2::methods]
impl FishingSizeTableData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SizeName", args = 0)]
    pub fn get_size_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SizeName", args = 1)]
    pub fn set_size_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SizeMinimum", args = 0)]
    pub fn get_size_minimum(self) -> f32;

    #[method(name = "set_SizeMinimum", args = 1)]
    pub fn set_size_minimum(self, value: f32) -> ();

    #[method(name = "get_SizeMaximum", args = 0)]
    pub fn get_size_maximum(self) -> f32;

    #[method(name = "set_SizeMaximum", args = 1)]
    pub fn set_size_maximum(self, value: f32) -> ();

    #[method(name = "get_BonusMinimum", args = 0)]
    pub fn get_bonus_minimum(self) -> f32;

    #[method(name = "set_BonusMinimum", args = 1)]
    pub fn set_bonus_minimum(self, value: f32) -> ();

    #[method(name = "get_BonusMaximum", args = 0)]
    pub fn get_bonus_maximum(self) -> f32;

    #[method(name = "set_BonusMaximum", args = 1)]
    pub fn set_bonus_maximum(self, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();
}

#[cfg(feature = "app-fishingsizetabledata")]
impl FishingSizeTableData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingSizeTableData),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingSizeTableDataMethods>::ctor(this);
        this
    }
}
