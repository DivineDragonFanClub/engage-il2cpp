
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/managedstreamhelpers/ManagedStreamHelpers.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ManagedStreamHelpers")]
#[parent(crate::system::object::Object)]
pub struct ManagedStreamHelpers {}

#[cfg(feature = "unity_engine-managedstreamhelpers")]
#[::unity2::methods]
impl ManagedStreamHelpers {
    #[method(name = "ValidateLoadFromStream", args = 1)]
    pub fn validate_load_from_stream(stream: crate::system::io::stream::Stream) -> ();

    #[method(name = "ManagedStreamRead", args = 5)]
    pub fn managed_stream_read(
        buffer: ::unity2::Array<u8>,
        offset: i32,
        count: i32,
        stream: crate::system::io::stream::Stream,
        return_value_address: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "ManagedStreamSeek", args = 4)]
    pub fn managed_stream_seek(
        offset: i64,
        origin: u32,
        stream: crate::system::io::stream::Stream,
        return_value_address: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "ManagedStreamLength", args = 2)]
    pub fn managed_stream_length(
        stream: crate::system::io::stream::Stream,
        return_value_address: ::unity2::IntPtr,
    ) -> ();
}
