
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_style/TMP_Style.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_Style")]
#[parent(crate::system::object::Object)]
pub struct TMP_Style {
    #[static_field]
    #[rename(name = "k_NormalStyle")]
    pub k_normal_style: crate::tm_pro::tmp_style::TMP_Style,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_HashCode")]
    pub m_hash_code: i32,
    #[rename(name = "m_OpeningDefinition")]
    pub m_opening_definition: ::unity2::Il2CppString,
    #[rename(name = "m_ClosingDefinition")]
    pub m_closing_definition: ::unity2::Il2CppString,
    #[rename(name = "m_OpeningTagArray")]
    pub m_opening_tag_array: ::unity2::Array<i32>,
    #[rename(name = "m_ClosingTagArray")]
    pub m_closing_tag_array: ::unity2::Array<i32>,
    #[rename(name = "m_OpeningTagUnicodeArray")]
    pub m_opening_tag_unicode_array: ::unity2::Array<u32>,
    #[rename(name = "m_ClosingTagUnicodeArray")]
    pub m_closing_tag_unicode_array: ::unity2::Array<u32>,
}

#[cfg(feature = "tm_pro-tmp_style")]
#[::unity2::methods]
impl TMP_Style {
    #[method(name = "get_NormalStyle", args = 0)]
    pub fn get_normal_style() -> crate::tm_pro::tmp_style::TMP_Style;

    #[method(name = "get_name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_hashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "set_hashCode", args = 1)]
    pub fn set_hash_code(self, value: i32) -> ();

    #[method(name = "get_styleOpeningDefinition", args = 0)]
    pub fn get_style_opening_definition(self) -> ::unity2::Il2CppString;

    #[method(name = "get_styleClosingDefinition", args = 0)]
    pub fn get_style_closing_definition(self) -> ::unity2::Il2CppString;

    #[method(name = "get_styleOpeningTagArray", args = 0)]
    pub fn get_style_opening_tag_array(self) -> ::unity2::Array<i32>;

    #[method(name = "get_styleClosingTagArray", args = 0)]
    pub fn get_style_closing_tag_array(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        style_name: ::unity2::Il2CppString,
        style_opening_definition: ::unity2::Il2CppString,
        style_closing_definition: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "RefreshStyle", args = 0)]
    pub fn refresh_style(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_style")]
impl TMP_Style {
    pub fn new(
        style_name: ::unity2::Il2CppString,
        style_opening_definition: ::unity2::Il2CppString,
        style_closing_definition: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_Style),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_StyleMethods>::ctor(
            this,
            style_name,
            style_opening_definition,
            style_closing_definition,
        );
        this
    }
}
