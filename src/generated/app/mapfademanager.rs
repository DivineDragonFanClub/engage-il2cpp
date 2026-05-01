
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapfademanager/MapFadeManager.md")))]
#[::unity2::class(namespace = "App", name = "MapFadeManager")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: mapfademanager :: MapFadeManager >)]
pub struct MapFadeManager {
    #[static_field]
    #[rename(name = "Max")]
    pub max: i32,
    #[rename(name = "m_Transparents")]
    pub m_transparents: crate::system::collections::generic::list_1::List_1<
        crate::app::charactercollision::CharacterCollision,
    >,
    #[rename(name = "m_Destructions")]
    pub m_destructions: crate::system::collections::generic::list_1::List_1<
        crate::app::charactercollision::CharacterCollision,
    >,
    #[rename(name = "m_Transparented")]
    pub m_transparented:
        crate::system::collections::generic::list_1::List_1<crate::app::mapcollision::MapCollision>,
    #[rename(name = "m_TempCollisions")]
    pub m_temp_collisions:
        crate::system::collections::generic::list_1::List_1<crate::app::mapcollision::MapCollision>,
    #[rename(name = "m_AlphaCollisions")]
    pub m_alpha_collisions: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::mapcollision::MapCollision,
        f32,
    >,
    #[rename(name = "m_FadeObjects")]
    pub m_fade_objects:
        crate::system::collections::generic::list_1::List_1<crate::app::mapobject::MapObject>,
    #[rename(name = "m_Colliders")]
    pub m_colliders: ::unity2::Array<crate::unity_engine::collider::Collider>,
    #[rename(name = "m_LayerDestructionMask")]
    pub m_layer_destruction_mask: i32,
    #[rename(name = "m_LayerTransparentMask")]
    pub m_layer_transparent_mask: i32,
    #[rename(name = "m_CameraPosition")]
    pub m_camera_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_CameraRotation")]
    pub m_camera_rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_FadeSpeed")]
    pub m_fade_speed: f32,
}

#[cfg(feature = "app-mapfademanager")]
#[::unity2::methods]
impl MapFadeManager {
    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "SetupFadeObject", args = 0)]
    pub fn setup_fade_object(self) -> ();

    #[method(name = "ClearFadeObject", args = 0)]
    pub fn clear_fade_object(self) -> ();

    #[method(name = "GetRadiusScale", args = 0)]
    pub fn get_radius_scale(self) -> f32;

    #[method(name = "ClearTransparent", args = 0)]
    pub fn clear_transparent(self) -> ();

    #[method(name = "UpdateTransparentPoint", args = 4)]
    pub fn update_transparent_point(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        radius: f32,
        is_character: bool,
        alpha: f32,
    ) -> ();

    #[method(name = "UpdateTransparentBetween", args = 5)]
    pub fn update_transparent_between(
        self,
        pos1: crate::unity_engine::vector3::Vector3,
        pos2: crate::unity_engine::vector3::Vector3,
        radius: f32,
        is_character: bool,
        alpha: f32,
    ) -> ();

    #[method(name = "UpdateTransparentImpl", args = 3)]
    pub fn update_transparent_impl(self, is_character: bool, alpha: f32, count: i32) -> ();

    #[method(name = "IsCutChange", args = 3)]
    pub fn is_cut_change(
        camera: crate::unity_engine::camera::Camera,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
    ) -> bool;

    #[method(name = "UpdateFadeSpeed", args = 0)]
    pub fn update_fade_speed(self) -> ();

    #[method(name = "UpdateTransparent", args = 0)]
    pub fn update_transparent(self) -> ();

    #[method(name = "UpdateDestruction", args = 2)]
    pub fn update_destruction(self, pos: crate::unity_engine::vector3::Vector3, radius: f32) -> ();

    #[method(name = "UpdateDestruction", args = 0)]
    pub fn update_destruction_2(self) -> ();

    #[method(name = "ClearDestruction", args = 0)]
    pub fn clear_destruction(self) -> ();

    #[method(name = "UpdateFaderObject", args = 0)]
    pub fn update_fader_object(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "Entry", args = 1)]
    pub fn entry(collision: crate::app::charactercollision::CharacterCollision) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(collision: crate::app::charactercollision::CharacterCollision) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapfademanager")]
impl MapFadeManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapFadeManager),
                ::core::stringify!(new),
            )
        });
        <Self as IMapFadeManagerMethods>::ctor(this);
        this
    }
}
