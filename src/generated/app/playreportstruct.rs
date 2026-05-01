
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/playreportstruct/PlayReportStruct.md")))]
#[::unity2::class(namespace = "App", name = "PlayReportStruct")]
#[parent(crate::system::object::Object)]
pub struct PlayReportStruct {
    #[static_field]
    #[rename(name = "s_KeyValueCount")]
    pub s_key_value_count: i32,
}

#[cfg(feature = "app-playreportstruct")]
#[::unity2::methods]
impl PlayReportStruct {
    #[method(name = "SetBuffer", args = 1)]
    pub fn set_buffer(buffer_size: i64) -> ();

    #[method(name = "GetKeyValueCount", args = 0)]
    pub fn get_key_value_count() -> i32;

    #[method(name = "Add", args = 2)]
    pub fn add(key: ::unity2::Il2CppString, val: i64) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_2(key: ::unity2::Il2CppString, val: u64) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_3(key: ::unity2::Il2CppString, val: f32) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_4(key: ::unity2::Il2CppString, val: f64) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_5(key: ::unity2::Il2CppString, val: ::unity2::Il2CppString) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_6(key: ::unity2::Il2CppString, val: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-playreportstruct")]
impl PlayReportStruct {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayReportStruct),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayReportStructMethods>::ctor(this);
        this
    }
}
