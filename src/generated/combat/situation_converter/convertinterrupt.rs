
use crate::combat::situation_converter::baseconverter::BaseConverter;
use crate::combat::situation_converter::baseconverter::IBaseConverter;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/convertinterrupt/ConvertInterrupt.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "ConvertInterrupt")]
#[parent(crate::combat::situation_converter::baseconverter::BaseConverter)]
pub struct ConvertInterrupt {}

#[cfg(feature = "combat-situation_converter-convertinterrupt")]
#[::unity2::methods]
impl ConvertInterrupt {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "get_MainCamera", args = 0)]
    pub fn get_main_camera(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "set_MainCamera", args = 1)]
    pub fn set_main_camera(self, value: crate::combat::cameraposition::CameraPosition) -> ();

    #[method(name = "get_InterruptCamera", args = 0)]
    pub fn get_interrupt_camera(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "set_InterruptCamera", args = 1)]
    pub fn set_interrupt_camera(self, value: crate::combat::cameraposition::CameraPosition) -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        situation: crate::combat::camerasituation::CameraSituation,
        arg: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;
}

#[cfg(feature = "combat-situation_converter-convertinterrupt")]
impl ConvertInterrupt {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConvertInterrupt),
                ::core::stringify!(new),
            )
        });
        <Self as IConvertInterruptMethods>::ctor(this, data);
        this
    }
}
