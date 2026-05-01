
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/lexerutils/LexerUtils.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Tree", name = "LexerUtils")]
#[parent(crate::system::object::Object)]
pub struct LexerUtils {}

#[cfg(feature = "moon_sharp-interpreter-tree-lexerutils")]
#[::unity2::methods]
impl LexerUtils {
    #[method(name = "ParseNumber", args = 1)]
    pub fn parse_number(t: crate::moon_sharp::interpreter::tree::token::Token) -> f64;

    #[method(name = "ParseHexInteger", args = 1)]
    pub fn parse_hex_integer(t: crate::moon_sharp::interpreter::tree::token::Token) -> f64;

    #[method(name = "ReadHexProgressive", args = 3)]
    pub fn read_hex_progressive(
        s: ::unity2::Il2CppString,
        d: f64,
        digits: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "ParseHexFloat", args = 1)]
    pub fn parse_hex_float(t: crate::moon_sharp::interpreter::tree::token::Token) -> f64;

    #[method(name = "HexDigit2Value", args = 1)]
    pub fn hex_digit2_value(c: u16) -> i32;

    #[method(name = "CharIsDigit", args = 1)]
    pub fn char_is_digit(c: u16) -> bool;

    #[method(name = "CharIsHexDigit", args = 1)]
    pub fn char_is_hex_digit(c: u16) -> bool;

    #[method(name = "AdjustLuaLongString", args = 1)]
    pub fn adjust_lua_long_string(str: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "UnescapeLuaString", args = 2)]
    pub fn unescape_lua_string(
        token: crate::moon_sharp::interpreter::tree::token::Token,
        str: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "ConvertUtf32ToChar", args = 1)]
    pub fn convert_utf32_to_char(i: i32) -> ::unity2::Il2CppString;
}
