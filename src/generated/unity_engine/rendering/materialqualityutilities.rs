
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/materialqualityutilities/MaterialQualityUtilities.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "MaterialQualityUtilities")]
#[parent(crate::system::object::Object)]
pub struct MaterialQualityUtilities {
    #[static_field]
    #[rename(name = "KeywordNames")]
    pub keyword_names: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "EnumNames")]
    pub enum_names: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "Keywords")]
    pub keywords: ::unity2::Array<crate::unity_engine::rendering::shaderkeyword::ShaderKeyword>,
}

#[cfg(feature = "unity_engine-rendering-materialqualityutilities")]
#[::unity2::methods]
impl MaterialQualityUtilities {
    #[method(name = "GetHighestQuality", args = 1)]
    pub fn get_highest_quality(
        levels: crate::unity_engine::rendering::materialquality::MaterialQuality,
    ) -> crate::unity_engine::rendering::materialquality::MaterialQuality;

    #[method(name = "GetClosestQuality", args = 2)]
    pub fn get_closest_quality(
        available_levels: crate::unity_engine::rendering::materialquality::MaterialQuality,
        requested_level: crate::unity_engine::rendering::materialquality::MaterialQuality,
    ) -> crate::unity_engine::rendering::materialquality::MaterialQuality;

    #[method(name = "SetGlobalShaderKeywords", args = 1)]
    pub fn set_global_shader_keywords(
        level: crate::unity_engine::rendering::materialquality::MaterialQuality,
    ) -> ();

    #[method(name = "SetGlobalShaderKeywords", args = 2)]
    pub fn set_global_shader_keywords_2(
        level: crate::unity_engine::rendering::materialquality::MaterialQuality,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "ToFirstIndex", args = 1)]
    pub fn to_first_index(
        level: crate::unity_engine::rendering::materialquality::MaterialQuality,
    ) -> i32;

    #[method(name = "FromIndex", args = 1)]
    pub fn from_index(
        index: i32,
    ) -> crate::unity_engine::rendering::materialquality::MaterialQuality;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
