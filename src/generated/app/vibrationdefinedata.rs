
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/vibrationdefinedata/VibrationDefineData.md")))]
#[::unity2::class(namespace = "App", name = "VibrationDefineData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: vibrationdefinedata :: VibrationDefineData >)]
pub struct VibrationDefineData {}

#[cfg(feature = "app-vibrationdefinedata")]
#[::unity2::methods]
impl VibrationDefineData {
    #[method(name = "get_EventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EventName", args = 1)]
    pub fn set_event_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_VibrationFileName", args = 0)]
    pub fn get_vibration_file_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_VibrationFileName", args = 1)]
    pub fn set_vibration_file_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AmplitudeMagnitude", args = 0)]
    pub fn get_amplitude_magnitude(self) -> f32;

    #[method(name = "set_AmplitudeMagnitude", args = 1)]
    pub fn set_amplitude_magnitude(self, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();
}

#[cfg(feature = "app-vibrationdefinedata")]
impl VibrationDefineData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VibrationDefineData),
                ::core::stringify!(new),
            )
        });
        <Self as IVibrationDefineDataMethods>::ctor(this);
        this
    }
}
