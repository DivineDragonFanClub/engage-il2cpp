
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/database/Database.md")))]
#[::unity2::class(namespace = "App", name = "Database")]
#[parent(crate::system::object::Object)]
pub struct Database {
    #[static_field]
    #[rename(name = "s_Types")]
    pub s_types: ::unity2::Array<::unity2::SystemType>,
    #[static_field]
    #[rename(name = "s_Completed")]
    pub s_completed: bool,
}

#[cfg(feature = "app-database")]
#[::unity2::methods]
impl Database {
    #[method(name = "StaticFunc", args = 2)]
    pub fn static_func(r#type: ::unity2::SystemType, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "StaticFunc", args = 2)]
    pub fn static_func_2(
        types: ::unity2::Array<::unity2::SystemType>,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "Completed", args = 0)]
    pub fn completed() -> ();

    #[method(name = "IsCompleted", args = 0)]
    pub fn is_completed() -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "GetTypes", args = 0)]
    pub fn get_types() -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "Reload", args = 1)]
    pub fn reload(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Release", args = 0)]
    pub fn release() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-database")]
impl Database {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Database),
                ::core::stringify!(new),
            )
        });
        <Self as IDatabaseMethods>::ctor(this);
        this
    }
}
