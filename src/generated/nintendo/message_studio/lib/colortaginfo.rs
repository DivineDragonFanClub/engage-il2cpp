
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/colortaginfo/ColorTagInfo.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "ColorTagInfo")]
#[parent(crate::system::object::Object)]
pub struct ColorTagInfo {}

#[cfg(feature = "nintendo-message_studio-lib-colortaginfo")]
#[::unity2::methods]
impl ColorTagInfo {
    #[method(name = "get_Tag", args = 0)]
    pub fn get_tag(self) -> u16;

    #[method(name = "get_TagGroup", args = 0)]
    pub fn get_tag_group(self) -> u16;

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::nintendo::message_studio::lib::lmscolor::LMSColor;

    #[method(name = "set_Color", args = 1)]
    pub fn set_color(self, value: crate::nintendo::message_studio::lib::lmscolor::LMSColor) -> ();

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> u16;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: u16) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, param: ::unity2::Array<u8>) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-colortaginfo")]
impl ColorTagInfo {
    pub fn new(param: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ColorTagInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IColorTagInfoMethods>::ctor(this, param);
        this
    }
}
