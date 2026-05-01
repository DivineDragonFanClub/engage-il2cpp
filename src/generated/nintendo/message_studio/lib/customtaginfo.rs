
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/customtaginfo/CustomTagInfo.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "CustomTagInfo")]
#[parent(crate::system::object::Object)]
pub struct CustomTagInfo {}

#[cfg(feature = "nintendo-message_studio-lib-customtaginfo")]
#[::unity2::methods]
impl CustomTagInfo {
    #[method(name = "get_Tag", args = 0)]
    pub fn get_tag(self) -> u16;

    #[method(name = "set_Tag", args = 1)]
    pub fn set_tag(self, value: u16) -> ();

    #[method(name = "get_TagGroup", args = 0)]
    pub fn get_tag_group(self) -> u16;

    #[method(name = "set_TagGroup", args = 1)]
    pub fn set_tag_group(self, value: u16) -> ();

    #[method(name = "get_Params", args = 0)]
    pub fn get_params(self) -> ::unity2::Array<u8>;

    #[method(name = "set_Params", args = 1)]
    pub fn set_params(self, value: ::unity2::Array<u8>) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, group: u16, tag: u16, param: ::unity2::Array<u8>) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-customtaginfo")]
impl CustomTagInfo {
    pub fn new(group: u16, tag: u16, param: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomTagInfo),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomTagInfoMethods>::ctor(this, group, tag, param);
        this
    }
}
