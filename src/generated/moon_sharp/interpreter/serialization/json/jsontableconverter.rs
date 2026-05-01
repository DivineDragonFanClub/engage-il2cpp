
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/serialization/json/jsontableconverter/JsonTableConverter.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Serialization.Json",
    name = "JsonTableConverter"
)]
#[parent(crate::system::object::Object)]
pub struct JsonTableConverter {}

#[cfg(feature = "moon_sharp-interpreter-serialization-json-jsontableconverter")]
#[::unity2::methods]
impl JsonTableConverter {
    #[method(name = "TableToJson", args = 1)]
    pub fn table_to_json(
        table: crate::moon_sharp::interpreter::table::Table,
    ) -> ::unity2::Il2CppString;

    #[method(name = "ObjectToJson", args = 1)]
    pub fn object_to_json(obj: crate::system::object::Object) -> ::unity2::Il2CppString;

    #[method(name = "EscapeString", args = 1)]
    pub fn escape_string(s: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "IsValueJsonCompatible", args = 1)]
    pub fn is_value_json_compatible(
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> bool;

    #[method(name = "JsonToTable", args = 2)]
    pub fn json_to_table(
        json: ::unity2::Il2CppString,
        script: crate::moon_sharp::interpreter::script::Script,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "AssertToken", args = 2)]
    pub fn assert_token(
        l: crate::moon_sharp::interpreter::tree::lexer::Lexer,
        r#type: crate::moon_sharp::interpreter::tree::tokentype::TokenType,
    ) -> ();

    #[method(name = "ParseJsonArray", args = 2)]
    pub fn parse_json_array(
        l: crate::moon_sharp::interpreter::tree::lexer::Lexer,
        script: crate::moon_sharp::interpreter::script::Script,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "ParseJsonObject", args = 2)]
    pub fn parse_json_object(
        l: crate::moon_sharp::interpreter::tree::lexer::Lexer,
        script: crate::moon_sharp::interpreter::script::Script,
    ) -> crate::moon_sharp::interpreter::table::Table;

    #[method(name = "ParseJsonValue", args = 2)]
    pub fn parse_json_value(
        l: crate::moon_sharp::interpreter::tree::lexer::Lexer,
        script: crate::moon_sharp::interpreter::script::Script,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;
}
