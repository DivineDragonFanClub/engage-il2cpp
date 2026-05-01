
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapprojection/MapProjection.md")))]
#[::unity2::class(namespace = "App", name = "MapProjection")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: mapprojection :: MapProjection >)]
pub struct MapProjection {
    #[rename(name = "m_Texture")]
    pub m_texture: crate::unity_engine::texture::Texture,
    #[rename(name = "m_OffsetX")]
    pub m_offset_x: f32,
    #[rename(name = "m_OffsetY")]
    pub m_offset_y: f32,
    #[rename(name = "m_SpeedX")]
    pub m_speed_x: f32,
    #[rename(name = "m_SpeedY")]
    pub m_speed_y: f32,
    #[rename(name = "m_Scale")]
    pub m_scale: f32,
    #[rename(name = "m_Alpha")]
    pub m_alpha: f32,
    #[rename(name = "m_SightSideColor")]
    pub m_sight_side_color: crate::unity_engine::color32::Color32,
    #[rename(name = "m_SightDarkColor")]
    pub m_sight_dark_color: crate::unity_engine::color32::Color32,
    #[rename(name = "m_SightMaskColor")]
    pub m_sight_mask_color: crate::unity_engine::color32::Color32,
    #[rename(name = "m_SightTexture")]
    pub m_sight_texture: crate::unity_engine::texture2d::Texture2D,
    #[rename(name = "m_SightColors")]
    pub m_sight_colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
    #[rename(name = "m_SightImage")]
    pub m_sight_image: crate::app::mapimagesight::MapImageSight,
    #[rename(name = "m_SightWidth")]
    pub m_sight_width: i32,
    #[rename(name = "m_SightHeight")]
    pub m_sight_height: i32,
    #[rename(name = "m_MapProjectionTex")]
    pub m_map_projection_tex: i32,
    #[rename(name = "m_MapProjectionScale")]
    pub m_map_projection_scale: i32,
    #[rename(name = "m_MapProjectionAlpha")]
    pub m_map_projection_alpha: i32,
    #[rename(name = "m_MapProjectionOffset")]
    pub m_map_projection_offset: i32,
    #[rename(name = "m_MapProjectionSpeed")]
    pub m_map_projection_speed: i32,
}

#[cfg(feature = "app-mapprojection")]
#[::unity2::methods]
impl MapProjection {
    #[method(name = "IsUsable", args = 0)]
    pub fn is_usable(self) -> bool;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "GetSightFilter", args = 0)]
    pub fn get_sight_filter(self) -> crate::unity_engine::filtermode::FilterMode;

    #[method(name = "GetSightSize", args = 0)]
    pub fn get_sight_size(self) -> i32;

    #[method(name = "GetSightScale", args = 0)]
    pub fn get_sight_scale(self) -> i32;

    #[method(name = "GetSightShift", args = 0)]
    pub fn get_sight_shift(self) -> i32;

    #[method(name = "IsChangedSightTexture", args = 0)]
    pub fn is_changed_sight_texture(self) -> bool;

    #[method(name = "TryCreateSightTexture", args = 0)]
    pub fn try_create_sight_texture(self) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "ClearSightTexture", args = 0)]
    pub fn clear_sight_texture(self) -> ();

    #[method(name = "TryDeleteSightTexture", args = 0)]
    pub fn try_delete_sight_texture(self) -> ();

    #[method(name = "UpdateSightTexture", args = 3)]
    pub fn update_sight_texture(
        self,
        image: crate::app::mapimagesight::MapImageSight,
        w: i32,
        h: i32,
    ) -> ();

    #[method(name = "Commit", args = 7)]
    pub fn commit(
        self,
        texture: crate::unity_engine::texture::Texture,
        scale: f32,
        alpha: f32,
        offset_x: f32,
        offset_y: f32,
        speed_x: f32,
        speed_y: f32,
    ) -> ();

    #[method(name = "Commit", args = 1)]
    pub fn commit_2(self, mode: crate::app::viewmode::ViewMode_Mode) -> ();

    #[method(name = "Commit", args = 0)]
    pub fn commit_3(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapprojection")]
impl MapProjection {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapProjection),
                ::core::stringify!(new),
            )
        });
        <Self as IMapProjectionMethods>::ctor(this);
        this
    }
}
