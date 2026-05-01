
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/serialization/serializationextensions/SerializationExtensions.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Serialization",
    name = "SerializationExtensions"
)]
#[parent(crate::system::object::Object)]
pub struct SerializationExtensions {
    #[static_field]
    #[rename(name = "LUAKEYWORDS")]
    pub luakeywords:
        crate::system::collections::generic::hashset_1::HashSet_1<::unity2::Il2CppString>,
}

#[cfg(feature = "moon_sharp-interpreter-serialization-serializationextensions")]
#[::unity2::methods]
impl SerializationExtensions {
    #[method(name = "Serialize", args = 3)]
    pub fn serialize(
        table: crate::moon_sharp::interpreter::table::Table,
        prefix_return: bool,
        tabs: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsStringIdentifierValid", args = 1)]
    pub fn is_string_identifier_valid(
        dyn_value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> bool;

    #[method(name = "SerializeValue", args = 2)]
    pub fn serialize_value(
        dyn_value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        tabs: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "EscapeString", args = 1)]
    pub fn escape_string(s: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
