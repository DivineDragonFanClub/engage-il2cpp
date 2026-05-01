
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameuserhubdata/GameUserHubData.md")))]
#[::unity2::class(namespace = "App", name = "GameUserHubData")]
#[parent(crate::system::object::Object)]
pub struct GameUserHubData {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
}

#[cfg(feature = "app-gameuserhubdata")]
#[::unity2::methods]
impl GameUserHubData {
    #[method(name = "get_CameraSpeedMin", args = 0)]
    pub fn get_camera_speed_min() -> i32;

    #[method(name = "get_CameraSpeedMax", args = 0)]
    pub fn get_camera_speed_max() -> i32;

    #[method(name = "get_LastPosition", args = 0)]
    pub fn get_last_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_LastPosition", args = 1)]
    pub fn set_last_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_LastDirY", args = 0)]
    pub fn get_last_dir_y(self) -> f32;

    #[method(name = "set_LastDirY", args = 1)]
    pub fn set_last_dir_y(self, value: f32) -> ();

    #[method(name = "get_CameraSpeed", args = 0)]
    pub fn get_camera_speed(self) -> i32;

    #[method(name = "set_CameraSpeed", args = 1)]
    pub fn set_camera_speed(self, value: i32) -> ();

    #[method(name = "get_IsMinimapRotate", args = 0)]
    pub fn get_is_minimap_rotate(self) -> bool;

    #[method(name = "set_IsMinimapRotate", args = 1)]
    pub fn set_is_minimap_rotate(self, value: bool) -> ();

    #[method(name = "get_CameraZoomParam", args = 0)]
    pub fn get_camera_zoom_param(self) -> f32;

    #[method(name = "set_CameraZoomParam", args = 1)]
    pub fn set_camera_zoom_param(self, value: f32) -> ();

    #[method(name = "get_MapMode", args = 0)]
    pub fn get_map_mode(self) -> i32;

    #[method(name = "set_MapMode", args = 1)]
    pub fn set_map_mode(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-gameuserhubdata")]
impl GameUserHubData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameUserHubData),
                ::core::stringify!(new),
            )
        });
        <Self as IGameUserHubDataMethods>::ctor(this);
        this
    }
}
