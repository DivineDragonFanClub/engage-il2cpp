
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/nintendo/message_studio/lib/rubytaginfo/RubyTagInfo.md")))]
#[::unity2::class(namespace = "Nintendo.MessageStudio.Lib", name = "RubyTagInfo")]
#[parent(crate::system::object::Object)]
pub struct RubyTagInfo {}

#[cfg(feature = "nintendo-message_studio-lib-rubytaginfo")]
#[::unity2::methods]
impl RubyTagInfo {
    #[method(name = "get_Tag", args = 0)]
    pub fn get_tag(self) -> u16;

    #[method(name = "get_TagGroup", args = 0)]
    pub fn get_tag_group(self) -> u16;

    #[method(name = "get_ParentLength", args = 0)]
    pub fn get_parent_length(self) -> u16;

    #[method(name = "set_ParentLength", args = 1)]
    pub fn set_parent_length(self, value: u16) -> ();

    #[method(name = "get_Ruby", args = 0)]
    pub fn get_ruby(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Ruby", args = 1)]
    pub fn set_ruby(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, param: ::unity2::Array<u8>) -> ();
}

#[cfg(feature = "nintendo-message_studio-lib-rubytaginfo")]
impl RubyTagInfo {
    pub fn new(param: ::unity2::Array<u8>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RubyTagInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IRubyTagInfoMethods>::ctor(this, param);
        this
    }
}
