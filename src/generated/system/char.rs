
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/char/Char.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Char {
    pub m_value: u16,
}

impl ::unity2::ClassIdentity for Char {
    const NAMESPACE: &'static str = "System";

    const NAME: &'static str = "Char";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Char {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-char")]
#[::unity2::methods(value)]
impl Char {
    #[method(name = "IsLatin1", args = 1)]
    pub fn is_latin1(ch: u16) -> bool;

    #[method(name = "IsAscii", args = 1)]
    pub fn is_ascii(ch: u16) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: u16) -> bool;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to(self, value: crate::system::object::Object) -> i32;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to_2(self, value: u16) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "ToString", args = 1)]
    pub fn to_string_2(c: u16) -> ::unity2::Il2CppString;

    #[method(name = "Parse", args = 1)]
    pub fn parse(s: ::unity2::Il2CppString) -> u16;

    #[method(name = "TryParse", args = 2)]
    pub fn try_parse(s: ::unity2::Il2CppString, result: u16) -> bool;

    #[method(name = "IsDigit", args = 1)]
    pub fn is_digit(c: u16) -> bool;

    #[method(name = "IsLetter", args = 1)]
    pub fn is_letter(c: u16) -> bool;

    #[method(name = "IsWhiteSpaceLatin1", args = 1)]
    pub fn is_white_space_latin1(c: u16) -> bool;

    #[method(name = "IsWhiteSpace", args = 1)]
    pub fn is_white_space(c: u16) -> bool;

    #[method(name = "IsUpper", args = 1)]
    pub fn is_upper(c: u16) -> bool;

    #[method(name = "IsLower", args = 1)]
    pub fn is_lower(c: u16) -> bool;

    #[method(name = "IsPunctuation", args = 1)]
    pub fn is_punctuation(c: u16) -> bool;

    #[method(name = "IsLetterOrDigit", args = 1)]
    pub fn is_letter_or_digit(c: u16) -> bool;

    #[method(name = "ToUpper", args = 1)]
    pub fn to_upper(c: u16) -> u16;

    #[method(name = "ToUpperInvariant", args = 1)]
    pub fn to_upper_invariant(c: u16) -> u16;

    #[method(name = "ToLower", args = 1)]
    pub fn to_lower(c: u16) -> u16;

    #[method(name = "ToLowerInvariant", args = 1)]
    pub fn to_lower_invariant(c: u16) -> u16;

    #[method(name = "IsControl", args = 1)]
    pub fn is_control(c: u16) -> bool;

    #[method(name = "IsNumber", args = 1)]
    pub fn is_number(c: u16) -> bool;

    #[method(name = "IsNumber", args = 2)]
    pub fn is_number_2(s: ::unity2::Il2CppString, index: i32) -> bool;

    #[method(name = "IsSeparatorLatin1", args = 1)]
    pub fn is_separator_latin1(c: u16) -> bool;

    #[method(name = "IsSeparator", args = 1)]
    pub fn is_separator(c: u16) -> bool;

    #[method(name = "IsSurrogate", args = 1)]
    pub fn is_surrogate(c: u16) -> bool;

    #[method(name = "IsSurrogate", args = 2)]
    pub fn is_surrogate_2(s: ::unity2::Il2CppString, index: i32) -> bool;

    #[method(name = "IsSymbol", args = 1)]
    pub fn is_symbol(c: u16) -> bool;

    #[method(name = "IsHighSurrogate", args = 1)]
    pub fn is_high_surrogate(c: u16) -> bool;

    #[method(name = "IsHighSurrogate", args = 2)]
    pub fn is_high_surrogate_2(s: ::unity2::Il2CppString, index: i32) -> bool;

    #[method(name = "IsLowSurrogate", args = 1)]
    pub fn is_low_surrogate(c: u16) -> bool;

    #[method(name = "IsSurrogatePair", args = 2)]
    pub fn is_surrogate_pair(high_surrogate: u16, low_surrogate: u16) -> bool;

    #[method(name = "ConvertFromUtf32", args = 1)]
    pub fn convert_from_utf32(utf32: i32) -> ::unity2::Il2CppString;

    #[method(name = "ConvertToUtf32", args = 2)]
    pub fn convert_to_utf32(high_surrogate: u16, low_surrogate: u16) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
