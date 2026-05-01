
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_stylesheet/TMP_StyleSheet.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_StyleSheet")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct TMP_StyleSheet {
    #[rename(name = "m_StyleList")]
    pub m_style_list:
        crate::system::collections::generic::list_1::List_1<crate::tm_pro::tmp_style::TMP_Style>,
    #[rename(name = "m_StyleLookupDictionary")]
    pub m_style_lookup_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::tm_pro::tmp_style::TMP_Style,
    >,
}

#[cfg(feature = "tm_pro-tmp_stylesheet")]
#[::unity2::methods]
impl TMP_StyleSheet {
    #[method(name = "get_styles", args = 0)]
    pub fn get_styles(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::tm_pro::tmp_style::TMP_Style>;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "GetStyle", args = 1)]
    pub fn get_style(self, hash_code: i32) -> crate::tm_pro::tmp_style::TMP_Style;

    #[method(name = "GetStyle", args = 1)]
    pub fn get_style_2(self, name: ::unity2::Il2CppString) -> crate::tm_pro::tmp_style::TMP_Style;

    #[method(name = "RefreshStyles", args = 0)]
    pub fn refresh_styles(self) -> ();

    #[method(name = "LoadStyleDictionaryInternal", args = 0)]
    pub fn load_style_dictionary_internal(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_stylesheet")]
impl TMP_StyleSheet {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_StyleSheet),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_StyleSheetMethods>::ctor(this);
        this
    }
}
