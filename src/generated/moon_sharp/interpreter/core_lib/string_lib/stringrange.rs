
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/string_lib/stringrange/StringRange.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.CoreLib.StringLib",
    name = "StringRange"
)]
#[parent(crate::system::object::Object)]
pub struct StringRange {}

#[cfg(feature = "moon_sharp-interpreter-core_lib-string_lib-stringrange")]
#[::unity2::methods]
impl StringRange {
    #[method(name = "get_Start", args = 0)]
    pub fn get_start(self) -> i32;

    #[method(name = "set_Start", args = 1)]
    pub fn set_start(self, value: i32) -> ();

    #[method(name = "get_End", args = 0)]
    pub fn get_end(self) -> i32;

    #[method(name = "set_End", args = 1)]
    pub fn set_end(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, start: i32, end: i32) -> ();

    #[method(name = "ApplyToString", args = 1)]
    pub fn apply_to_string(self, value: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Length", args = 0)]
    pub fn length(self) -> i32;
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-string_lib-stringrange")]
impl StringRange {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StringRange),
                ::core::stringify!(new),
            )
        });
        <Self as IStringRangeMethods>::ctor(this);
        this
    }

    pub fn new_2(start: i32, end: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StringRange),
                ::core::stringify!(new_2),
            )
        });
        <Self as IStringRangeMethods>::ctor_2(this, start, end);
        this
    }
}
