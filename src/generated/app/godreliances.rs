
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godreliances/GodRelianceS.md")))]
#[::unity2::class(namespace = "App", name = "GodRelianceS")]
#[parent(crate::system::object::Object)]
pub struct GodRelianceS {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_PartnerPid")]
    pub m_partner_pid: ::unity2::Il2CppString,
}

#[cfg(feature = "app-godreliances")]
#[::unity2::methods]
impl GodRelianceS {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_PartnerPid", args = 0)]
    pub fn get_partner_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PartnerPid", args = 1)]
    pub fn set_partner_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-godreliances")]
impl GodRelianceS {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRelianceS),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRelianceSMethods>::ctor(this);
        this
    }
}
