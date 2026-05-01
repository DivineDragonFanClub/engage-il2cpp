
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/wellknownsymbols/WellKnownSymbols.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "WellKnownSymbols")]
#[parent(crate::system::object::Object)]
pub struct WellKnownSymbols {
    #[static_field]
    #[rename(name = "VARARGS")]
    pub varargs: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ENV")]
    pub env: ::unity2::Il2CppString,
}
