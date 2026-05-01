
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_characterinfo/TMP_CharacterInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TMP_CharacterInfo {
    pub character: u16,
    pub index: i32,
    pub string_length: i32,
    pub element_type: crate::tm_pro::tmp_textelementtype::TMP_TextElementType,
    pub text_element: crate::tm_pro::tmp_textelement::TMP_TextElement,
    pub font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    pub sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    pub sprite_index: i32,
    pub material: crate::unity_engine::material::Material,
    pub material_reference_index: i32,
    pub is_using_alternate_typeface: bool,
    pub point_size: f32,
    pub line_number: i32,
    pub page_number: i32,
    pub vertex_index: i32,
    pub vertex_bl: crate::tm_pro::tmp_vertex::TMP_Vertex,
    pub vertex_tl: crate::tm_pro::tmp_vertex::TMP_Vertex,
    pub vertex_tr: crate::tm_pro::tmp_vertex::TMP_Vertex,
    pub vertex_br: crate::tm_pro::tmp_vertex::TMP_Vertex,
    pub top_left: crate::unity_engine::vector3::Vector3,
    pub bottom_left: crate::unity_engine::vector3::Vector3,
    pub top_right: crate::unity_engine::vector3::Vector3,
    pub bottom_right: crate::unity_engine::vector3::Vector3,
    pub origin: f32,
    pub x_advance: f32,
    pub ascender: f32,
    pub base_line: f32,
    pub descender: f32,
    pub adjusted_ascender: f32,
    pub adjusted_descender: f32,
    pub aspect_ratio: f32,
    pub scale: f32,
    pub color: crate::unity_engine::color32::Color32,
    pub underline_color: crate::unity_engine::color32::Color32,
    pub underline_vertex_index: i32,
    pub strikethrough_color: crate::unity_engine::color32::Color32,
    pub strikethrough_vertex_index: i32,
    pub highlight_color: crate::unity_engine::color32::Color32,
    pub highlight_state: crate::tm_pro::highlightstate::HighlightState,
    pub style: crate::tm_pro::fontstyles::FontStyles,
    pub is_visible: bool,
}

impl ::unity2::ClassIdentity for TMP_CharacterInfo {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "TMP_CharacterInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TMP_CharacterInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
