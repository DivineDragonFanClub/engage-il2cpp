
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/debugging/watchitem/WatchItem.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Debugging", name = "WatchItem")]
#[parent(crate::system::object::Object)]
pub struct WatchItem {}

#[cfg(feature = "moon_sharp-interpreter-debugging-watchitem")]
#[::unity2::methods]
impl WatchItem {
    #[method(name = "get_Address", args = 0)]
    pub fn get_address(self) -> i32;

    #[method(name = "set_Address", args = 1)]
    pub fn set_address(self, value: i32) -> ();

    #[method(name = "get_BasePtr", args = 0)]
    pub fn get_base_ptr(self) -> i32;

    #[method(name = "set_BasePtr", args = 1)]
    pub fn set_base_ptr(self, value: i32) -> ();

    #[method(name = "get_RetAddress", args = 0)]
    pub fn get_ret_address(self) -> i32;

    #[method(name = "set_RetAddress", args = 1)]
    pub fn set_ret_address(self, value: i32) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: crate::moon_sharp::interpreter::dynvalue::DynValue) -> ();

    #[method(name = "get_LValue", args = 0)]
    pub fn get_l_value(self) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "set_LValue", args = 1)]
    pub fn set_l_value(self, value: crate::moon_sharp::interpreter::symbolref::SymbolRef) -> ();

    #[method(name = "get_IsError", args = 0)]
    pub fn get_is_error(self) -> bool;

    #[method(name = "set_IsError", args = 1)]
    pub fn set_is_error(self, value: bool) -> ();

    #[method(name = "get_Location", args = 0)]
    pub fn get_location(self) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = "set_Location", args = 1)]
    pub fn set_location(
        self,
        value: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-debugging-watchitem")]
impl WatchItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WatchItem),
                ::core::stringify!(new),
            )
        });
        <Self as IWatchItemMethods>::ctor(this);
        this
    }
}
