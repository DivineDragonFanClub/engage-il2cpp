
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcamera/MapCamera_InterpolatorShake.md")))]
#[::unity2::class(namespace = "App", name = "MapCamera.InterpolatorShake")]
#[parent(crate::system::object::Object)]
pub struct MapCamera_InterpolatorShake {
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_Magnitude")]
    pub m_magnitude: f32,
    #[rename(name = "m_Offset")]
    pub m_offset: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "app-mapcamera")]
#[::unity2::methods]
impl MapCamera_InterpolatorShake {
    #[method(name = "Run", args = 2)]
    pub fn run(self, time: f32, magnitude: f32) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "get_Offset", args = 0)]
    pub fn get_offset(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapcamera")]
impl MapCamera_InterpolatorShake {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapCamera_InterpolatorShake),
                ::core::stringify!(new),
            )
        });
        <Self as IMapCamera_InterpolatorShakeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcamera/MapCamera.md")))]
#[::unity2::class(namespace = "App", name = "MapCamera")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapcamera :: MapCamera >)]
pub struct MapCamera {
    #[rename(name = "m_Position")]
    pub m_position: crate::app::interpolatorvector3::InterpolatorVector3,
    #[rename(name = "m_Rotation")]
    pub m_rotation: crate::app::interpolatorvector3::InterpolatorVector3,
    #[rename(name = "m_Distance")]
    pub m_distance: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_EffectShake")]
    pub m_effect_shake: crate::app::mapcamera::MapCamera_InterpolatorShake,
    #[rename(name = "m_ActionShake")]
    pub m_action_shake: crate::app::mapcamera::MapCamera_InterpolatorShake,
    #[rename(name = "m_Fov")]
    pub m_fov: f32,
}

#[cfg(feature = "app-mapcamera")]
#[::unity2::methods]
impl MapCamera {
    #[method(name = "GetCamera", args = 0)]
    pub fn get_camera() -> crate::unity_engine::camera::Camera;

    #[method(name = "SetPosition", args = 2)]
    pub fn set_position(pos: crate::unity_engine::vector3::Vector3, speed: f32) -> ();

    #[method(name = "SetPosition", args = 2)]
    pub fn set_position_2(x: i32, z: i32) -> ();

    #[method(name = "SetRotation", args = 1)]
    pub fn set_rotation(rot: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "SetDistance", args = 1)]
    pub fn set_distance(distance: f32) -> ();

    #[method(name = "GetRotation", args = 0)]
    pub fn get_rotation() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Instant", args = 1)]
    pub fn instant(camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "EffectShake", args = 2)]
    pub fn effect_shake(time: f32, magnitude: f32) -> ();

    #[method(name = "ActionShake", args = 2)]
    pub fn action_shake(time: f32, magnitude: f32) -> ();

    #[method(name = "CheckScrollStrictly", args = 0)]
    pub fn check_scroll_strictly() -> bool;

    #[method(name = "CheckScrollLoosely", args = 0)]
    pub fn check_scroll_loosely() -> bool;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "Commit", args = 1)]
    pub fn commit(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "GetParamTime", args = 1)]
    pub fn get_param_time(self, name: ::unity2::Il2CppString) -> f32;

    #[method(name = "SetPositionImpl", args = 2)]
    pub fn set_position_impl(self, pos: crate::unity_engine::vector3::Vector3, speed: f32) -> ();

    #[method(name = "SetRotationImpl", args = 1)]
    pub fn set_rotation_impl(self, rot: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "SetDistanceImpl", args = 1)]
    pub fn set_distance_impl(self, distance: f32) -> ();

    #[method(name = "InstantImpl", args = 1)]
    pub fn instant_impl(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapcamera")]
impl MapCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IMapCameraMethods>::ctor(this);
        this
    }
}
