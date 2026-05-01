
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusserverdata/VersusServerData.md")))]
#[::unity2::class(namespace = "App", name = "VersusServerData")]
#[parent(crate::system::object::Object)]
pub struct VersusServerData {
    #[static_field]
    #[rename(name = "BufferSize")]
    pub buffer_size: i32,
    #[rename(name = "m_Buffer")]
    pub m_buffer: ::unity2::Array<u8>,
    #[rename(name = "m_Stream")]
    pub m_stream: crate::app::stream_2::Stream_2,
    #[rename(name = "m_IsEmpty")]
    pub m_is_empty: bool,
}

#[cfg(feature = "app-versusserverdata")]
#[::unity2::methods]
impl VersusServerData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetBinary", args = 1)]
    pub fn set_binary(self, bin: ::unity2::Array<u8>) -> ();

    #[method(name = "Serialize", args = 0)]
    pub fn serialize(self) -> ();

    #[method(name = "Deserialize", args = 0)]
    pub fn deserialize(self) -> bool;

    #[method(name = "get_Buffer", args = 0)]
    pub fn get_buffer(self) -> ::unity2::Array<u8>;

    #[method(name = "get_Size", args = 0)]
    pub fn get_size(self) -> i32;

    #[method(name = "get_IsEmpty", args = 0)]
    pub fn get_is_empty(self) -> bool;
}

#[cfg(feature = "app-versusserverdata")]
impl VersusServerData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusServerData),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusServerDataMethods>::ctor(this);
        this
    }
}
