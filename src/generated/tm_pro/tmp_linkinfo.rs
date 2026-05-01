
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_linkinfo/TMP_LinkInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TMP_LinkInfo {
    pub text_component: crate::tm_pro::tmp_text::TMP_Text,
    pub hash_code: i32,
    pub link_id_first_character_index: i32,
    pub link_id_length: i32,
    pub link_textfirst_character_index: i32,
    pub link_text_length: i32,
    pub link_id: ::unity2::Array<u16>,
}

impl ::unity2::ClassIdentity for TMP_LinkInfo {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "TMP_LinkInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TMP_LinkInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "tm_pro-tmp_linkinfo")]
#[::unity2::methods(value)]
impl TMP_LinkInfo {
    #[method(name = "SetLinkID", args = 3)]
    pub fn set_link_id(self, text: ::unity2::Array<u16>, start_index: i32, length: i32) -> ();

    #[method(name = "GetLinkText", args = 0)]
    pub fn get_link_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetLinkID", args = 0)]
    pub fn get_link_id(self) -> ::unity2::Il2CppString;
}
