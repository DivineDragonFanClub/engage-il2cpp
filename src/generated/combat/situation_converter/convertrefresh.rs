
use crate::combat::situation_converter::baseconverter::BaseConverter;
use crate::combat::situation_converter::baseconverter::IBaseConverter;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/convertrefresh/ConvertRefresh.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "ConvertRefresh")]
#[parent(crate::combat::situation_converter::baseconverter::BaseConverter)]
pub struct ConvertRefresh {
    #[rename(name = "m_IsFinished")]
    pub m_is_finished: bool,
}

#[cfg(feature = "combat-situation_converter-convertrefresh")]
#[::unity2::methods]
impl ConvertRefresh {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        situation: crate::combat::camerasituation::CameraSituation,
        arg: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "IsFramingPlayer", args = 0)]
    pub fn is_framing_player(self) -> bool;

    #[method(name = "IsFramingBoth", args = 0)]
    pub fn is_framing_both(self) -> bool;
}

#[cfg(feature = "combat-situation_converter-convertrefresh")]
impl ConvertRefresh {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConvertRefresh),
                ::core::stringify!(new),
            )
        });
        <Self as IConvertRefreshMethods>::ctor(this, data);
        this
    }
}
