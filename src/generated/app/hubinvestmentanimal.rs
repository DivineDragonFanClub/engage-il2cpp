
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubinvestmentanimal/HubInvestmentAnimal.md")))]
#[::unity2::class(namespace = "App", name = "HubInvestmentAnimal")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: hubinvestmentanimal :: HubInvestmentAnimal >)]
pub struct HubInvestmentAnimal {}

#[cfg(feature = "app-hubinvestmentanimal")]
#[::unity2::methods]
impl HubInvestmentAnimal {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_ANID", args = 0)]
    pub fn get_anid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ANID", args = 1)]
    pub fn set_anid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AppearRateLv1", args = 0)]
    pub fn get_appear_rate_lv1(self) -> u8;

    #[method(name = "set_AppearRateLv1", args = 1)]
    pub fn set_appear_rate_lv1(self, value: u8) -> ();

    #[method(name = "get_AppearRateLv2", args = 0)]
    pub fn get_appear_rate_lv2(self) -> u8;

    #[method(name = "set_AppearRateLv2", args = 1)]
    pub fn set_appear_rate_lv2(self, value: u8) -> ();

    #[method(name = "get_AppearRateLv3", args = 0)]
    pub fn get_appear_rate_lv3(self) -> u8;

    #[method(name = "set_AppearRateLv3", args = 1)]
    pub fn set_appear_rate_lv3(self, value: u8) -> ();

    #[method(name = "get_AppearRateLv4", args = 0)]
    pub fn get_appear_rate_lv4(self) -> u8;

    #[method(name = "set_AppearRateLv4", args = 1)]
    pub fn set_appear_rate_lv4(self, value: u8) -> ();

    #[method(name = "get_AppearRateLv5", args = 0)]
    pub fn get_appear_rate_lv5(self) -> u8;

    #[method(name = "set_AppearRateLv5", args = 1)]
    pub fn set_appear_rate_lv5(self, value: u8) -> ();

    #[method(name = "get_CaptureLevel", args = 0)]
    pub fn get_capture_level(self) -> u8;

    #[method(name = "set_CaptureLevel", args = 1)]
    pub fn set_capture_level(self, value: u8) -> ();

    #[method(name = "GetAppearRate", args = 1)]
    pub fn get_appear_rate(self, level: i32) -> u8;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubinvestmentanimal")]
impl HubInvestmentAnimal {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubInvestmentAnimal),
                ::core::stringify!(new),
            )
        });
        <Self as IHubInvestmentAnimalMethods>::ctor(this);
        this
    }
}
