
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/fonttaginfo/FontTagInfo.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "FontTagInfo")]
#[parent(crate::system::object::Object)]
pub struct FontTagInfo {}

#[cfg(feature = "nintendo-message_studio-lib-fonttaginfo")]
#[::unity2::methods]
impl FontTagInfo {
    #[method(name = "get_Tag", args = 0)]
    pub fn get_tag(self) -> u16;

    #[method(name = "get_TagGroup", args = 0)]
    pub fn get_tag_group(self) -> u16;

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> u16;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: u16) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, param: ::unity2::Array<u8>) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-fonttaginfo")]
impl FontTagInfo {
    pub fn new(param: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FontTagInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IFontTagInfoMethods>::ctor(this, param);
        this
    }
}
