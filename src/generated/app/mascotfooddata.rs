
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotfooddata/MascotFoodData.md")))]
#[::unity2::class(namespace = "App", name = "MascotFoodData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: mascotfooddata :: MascotFoodData >)]
pub struct MascotFoodData {}

#[cfg(feature = "app-mascotfooddata")]
#[::unity2::methods]
impl MascotFoodData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: i32) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mascotfooddata")]
impl MascotFoodData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotFoodData),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotFoodDataMethods>::ctor(this);
        this
    }
}
