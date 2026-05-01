
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/grassdata/GrassData_SpriteInfo.md")))]
#[::unity2::class(namespace = "App", name = "GrassData.SpriteInfo")]
#[parent(crate::system::object::Object)]
pub struct GrassData_SpriteInfo {
    #[rename(name = "m_sprite")]
    pub m_sprite: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_adjustScale")]
    pub m_adjust_scale: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_spriteScale")]
    pub m_sprite_scale: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_spriteScaleRandom")]
    pub m_sprite_scale_random: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_spriteAspect")]
    pub m_sprite_aspect: f32,
    #[rename(name = "m_spriteST")]
    pub m_sprite_st: crate::unity_engine::vector4::Vector4,
    #[rename(name = "m_topColor")]
    pub m_top_color: crate::unity_engine::color::Color,
    #[rename(name = "m_bottomColor")]
    pub m_bottom_color: crate::unity_engine::color::Color,
    #[rename(name = "m_blendCenter")]
    pub m_blend_center: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_windColorInfluence")]
    pub m_wind_color_influence: f32,
    #[rename(name = "m_lodPriority")]
    pub m_lod_priority: f32,
    #[rename(name = "m_useAutoAdjustScale")]
    pub m_use_auto_adjust_scale: bool,
}

#[cfg(feature = "app-grassdata")]
#[::unity2::methods]
impl GrassData_SpriteInfo {
    #[method(name = "Copy", args = 1)]
    pub fn copy(self, src: crate::app::grassdata::GrassData_SpriteInfo) -> ();

    #[method(name = "SetDefault", args = 0)]
    pub fn set_default(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-grassdata")]
impl GrassData_SpriteInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GrassData_SpriteInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IGrassData_SpriteInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/grassdata/GrassData.md")))]
#[::unity2::class(namespace = "App", name = "GrassData")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct GrassData {
    #[static_field]
    #[rename(name = "GrassInfoCount")]
    pub grass_info_count: i32,
    #[static_field]
    #[rename(name = "GrassInfoIndexUvChannel")]
    pub grass_info_index_uv_channel: i32,
    #[static_field]
    #[rename(name = "GrassOrgMeshUvChannel")]
    pub grass_org_mesh_uv_channel: i32,
    #[rename(name = "m_bilinear")]
    pub m_bilinear: bool,
    #[rename(name = "m_spriteAtlas")]
    pub m_sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_spriteInfos")]
    pub m_sprite_infos: crate::system::collections::generic::list_1::List_1<
        crate::app::grassdata::GrassData_SpriteInfo,
    >,
}

#[cfg(feature = "app-grassdata")]
#[::unity2::methods]
impl GrassData {
    #[method(name = "Copy", args = 1)]
    pub fn copy(self, src: crate::app::grassdata::GrassData) -> ();

    #[method(name = "VerifyData", args = 0)]
    pub fn verify_data(self) -> bool;

    #[method(name = "ParseSpriteInfos", args = 7)]
    pub fn parse_sprite_infos(
        src: crate::system::collections::generic::list_1::List_1<
            crate::app::grassdata::GrassData_SpriteInfo,
        >,
        out_scale: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        out_aspect: crate::system::collections::generic::list_1::List_1<f32>,
        out_st: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        out_top_color: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
        out_bottom_color: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
        out_wind_color_influence: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-grassdata")]
impl GrassData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GrassData),
                ::core::stringify!(new),
            )
        });
        <Self as IGrassDataMethods>::ctor(this);
        this
    }
}
