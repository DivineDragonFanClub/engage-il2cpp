
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/token/Token.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Tree", name = "Token")]
#[parent(crate::system::object::Object)]
pub struct Token {
    #[rename(name = "SourceId")]
    pub source_id: i32,
    #[rename(name = "FromCol")]
    pub from_col: i32,
    #[rename(name = "ToCol")]
    pub to_col: i32,
    #[rename(name = "FromLine")]
    pub from_line: i32,
    #[rename(name = "ToLine")]
    pub to_line: i32,
    #[rename(name = "PrevCol")]
    pub prev_col: i32,
    #[rename(name = "PrevLine")]
    pub prev_line: i32,
    #[rename(name = "Type")]
    pub r#type: crate::moon_sharp::interpreter::tree::tokentype::TokenType,
}

#[cfg(feature = "moon_sharp-interpreter-tree-token")]
#[::unity2::methods]
impl Token {
    #[method(name = "get_Text", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Text", args = 1)]
    pub fn set_text(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 8)]
    pub fn ctor(
        self,
        r#type: crate::moon_sharp::interpreter::tree::tokentype::TokenType,
        source_id: i32,
        from_line: i32,
        from_col: i32,
        to_line: i32,
        to_col: i32,
        prev_line: i32,
        prev_col: i32,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetNumberValue", args = 0)]
    pub fn get_number_value(self) -> f64;

    #[method(name = "IsEndOfBlock", args = 0)]
    pub fn is_end_of_block(self) -> bool;

    #[method(name = "IsUnaryOperator", args = 0)]
    pub fn is_unary_operator(self) -> bool;

    #[method(name = "IsBinaryOperator", args = 0)]
    pub fn is_binary_operator(self) -> bool;

    #[method(name = "GetSourceRef", args = 1)]
    pub fn get_source_ref(
        self,
        is_step_stop: bool,
    ) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = "GetSourceRef", args = 2)]
    pub fn get_source_ref_2(
        self,
        to: crate::moon_sharp::interpreter::tree::token::Token,
        is_step_stop: bool,
    ) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = "GetSourceRefUpTo", args = 2)]
    pub fn get_source_ref_up_to(
        self,
        to: crate::moon_sharp::interpreter::tree::token::Token,
        is_step_stop: bool,
    ) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;
}

#[cfg(feature = "moon_sharp-interpreter-tree-token")]
impl Token {
    pub fn new(
        r#type: crate::moon_sharp::interpreter::tree::tokentype::TokenType,
        source_id: i32,
        from_line: i32,
        from_col: i32,
        to_line: i32,
        to_col: i32,
        prev_line: i32,
        prev_col: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Token),
                ::core::stringify!(new),
            )
        });
        <Self as ITokenMethods>::ctor(
            this, r#type, source_id, from_line, from_col, to_line, to_col, prev_line, prev_col,
        );
        this
    }
}
