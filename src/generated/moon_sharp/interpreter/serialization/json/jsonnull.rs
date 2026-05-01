
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/serialization/json/jsonnull/JsonNull.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Serialization.Json",
    name = "JsonNull"
)]
#[parent(crate::system::object::Object)]
pub struct JsonNull {}

#[cfg(feature = "moon_sharp-interpreter-serialization-json-jsonnull")]
#[::unity2::methods]
impl JsonNull {
    #[method(name = "isNull", args = 0)]
    pub fn is_null() -> bool;

    #[method(name = "IsJsonNull", args = 1)]
    pub fn is_json_null(v: crate::moon_sharp::interpreter::dynvalue::DynValue) -> bool;

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-serialization-json-jsonnull")]
impl JsonNull {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(JsonNull),
                ::core::stringify!(new),
            )
        });
        <Self as IJsonNullMethods>::ctor(this);
        this
    }
}
