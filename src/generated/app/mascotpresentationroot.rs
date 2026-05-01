
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotpresentationroot/MascotPresentationRoot.md")))]
#[::unity2::class(namespace = "App", name = "MascotPresentationRoot")]
#[parent(crate::system::object::Object)]
pub struct MascotPresentationRoot {
    #[rename(name = "Root")]
    pub root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "MascotRoot")]
    pub mascot_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "PlayerRoot")]
    pub player_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "CameraRoot")]
    pub camera_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "PlayerReserveRoot")]
    pub player_reserve_root: crate::unity_engine::transform::Transform,
    #[rename(name = "PlayerReservePosition")]
    pub player_reserve_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "PlayerReserveRotationY")]
    pub player_reserve_rotation_y: f32,
    #[rename(name = "MascotReserveRoot")]
    pub mascot_reserve_root: crate::unity_engine::transform::Transform,
    #[rename(name = "MascotReservePosition")]
    pub mascot_reserve_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "MascotReserveRotation")]
    pub mascot_reserve_rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "CameraReserveRoot")]
    pub camera_reserve_root: crate::unity_engine::transform::Transform,
    #[rename(name = "m_Apply")]
    pub m_apply: bool,
}

#[cfg(feature = "app-mascotpresentationroot")]
#[::unity2::methods]
impl MascotPresentationRoot {
    #[method(name = "Setup", args = 1)]
    pub fn setup(self, root: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Apply", args = 0)]
    pub fn apply(self) -> ();

    #[method(name = "Revert", args = 0)]
    pub fn revert(self) -> ();

    #[method(name = "get_PlayerController", args = 0)]
    pub fn get_player_controller(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "get_MascotController", args = 0)]
    pub fn get_mascot_controller(self) -> crate::app::hubmascotcontroller::HubMascotController;

    #[method(name = "GetLocation", args = 0)]
    pub fn get_location() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetRotation", args = 0)]
    pub fn get_rotation() -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mascotpresentationroot")]
impl MascotPresentationRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotPresentationRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotPresentationRootMethods>::ctor(this);
        this
    }
}
