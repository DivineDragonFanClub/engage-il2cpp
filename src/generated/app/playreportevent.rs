
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/playreportevent/PlayReportEvent.md")))]
#[::unity2::class(namespace = "App", name = "PlayReportEvent")]
#[parent(crate::system::object::Object)]
pub struct PlayReportEvent {
    #[static_field]
    #[rename(name = "s_EventId")]
    pub s_event_id: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_KeyValueCount")]
    pub s_key_value_count: i32,
}

#[cfg(feature = "app-playreportevent")]
#[::unity2::methods]
impl PlayReportEvent {
    #[method(name = "End", args = 0)]
    pub fn end() -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(key: ::unity2::Il2CppString, val: u64) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_2(key: ::unity2::Il2CppString, val: f32) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_3(key: ::unity2::Il2CppString, val: f64) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_4(key: ::unity2::Il2CppString, val: ::unity2::Il2CppString) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_5(key: ::unity2::Il2CppString, val: bool) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_6(key: ::unity2::Il2CppString, val: ::unity2::Array<u8>) -> ();

    #[method(name = "AddStruct", args = 1)]
    pub fn add_struct(key: ::unity2::Il2CppString) -> ();

    #[method(name = "PrepareAddReport", args = 0)]
    pub fn prepare_add_report() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-playreportevent")]
impl PlayReportEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayReportEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayReportEventMethods>::ctor(this);
        this
    }
}
