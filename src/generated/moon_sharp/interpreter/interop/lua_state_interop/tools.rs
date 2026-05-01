
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/lua_state_interop/tools/Tools.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.LuaStateInterop",
    name = "Tools"
)]
#[parent(crate::system::object::Object)]
pub struct Tools {}

#[cfg(feature = "moon_sharp-interpreter-interop-lua_state_interop-tools")]
#[::unity2::methods]
impl Tools {
    #[method(name = "IsNumericType", args = 1)]
    pub fn is_numeric_type(o: crate::system::object::Object) -> bool;

    #[method(name = "IsPositive", args = 2)]
    pub fn is_positive(value: crate::system::object::Object, zero_is_positive: bool) -> bool;

    #[method(name = "ToUnsigned", args = 1)]
    pub fn to_unsigned(value: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "ToInteger", args = 2)]
    pub fn to_integer(
        value: crate::system::object::Object,
        round: bool,
    ) -> crate::system::object::Object;

    #[method(name = "UnboxToLong", args = 2)]
    pub fn unbox_to_long(value: crate::system::object::Object, round: bool) -> i64;

    #[method(name = "ReplaceMetaChars", args = 1)]
    pub fn replace_meta_chars(input: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "sprintf", args = 2)]
    pub fn sprintf(
        format: ::unity2::Il2CppString,
        parameters: ::unity2::Array<crate::system::object::Object>,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FormatOct", args = 7)]
    pub fn format_oct(
        native_format: ::unity2::Il2CppString,
        alternate: bool,
        field_length: i32,
        field_precision: i32,
        left2_right: bool,
        padding: u16,
        value: crate::system::object::Object,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FormatHex", args = 7)]
    pub fn format_hex(
        native_format: ::unity2::Il2CppString,
        alternate: bool,
        field_length: i32,
        field_precision: i32,
        left2_right: bool,
        padding: u16,
        value: crate::system::object::Object,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FormatNumber", args = 9)]
    pub fn format_number(
        native_format: ::unity2::Il2CppString,
        alternate: bool,
        field_length: i32,
        field_precision: i32,
        left2_right: bool,
        positive_sign: bool,
        positive_space: bool,
        padding: u16,
        value: crate::system::object::Object,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
