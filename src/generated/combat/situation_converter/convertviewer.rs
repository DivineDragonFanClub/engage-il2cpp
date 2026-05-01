
use crate::combat::situation_converter::baseconverter::BaseConverter;
use crate::combat::situation_converter::baseconverter::IBaseConverter;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/convertviewer/ConvertViewer.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "ConvertViewer")]
#[parent(crate::combat::situation_converter::baseconverter::BaseConverter)]
pub struct ConvertViewer {}

#[cfg(feature = "combat-situation_converter-convertviewer")]
#[::unity2::methods]
impl ConvertViewer {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        situation: crate::combat::camerasituation::CameraSituation,
        arg: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;
}

#[cfg(feature = "combat-situation_converter-convertviewer")]
impl ConvertViewer {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConvertViewer),
                ::core::stringify!(new),
            )
        });
        <Self as IConvertViewerMethods>::ctor(this, data);
        this
    }
}
