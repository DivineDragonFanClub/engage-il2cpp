
use crate::combat::autocamerabase::AutoCameraBase;
use crate::combat::autocamerabase::IAutoCameraBase;
use crate::combat::basecameracontroller::BaseCameraController;
use crate::combat::basecameracontroller::IBaseCameraController;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/autocamera/AutoCamera.md")))]
#[::unity2::class(namespace = "Combat", name = "AutoCamera")]
#[parent(crate::combat::autocamerabase::AutoCameraBase)]
pub struct AutoCamera {
    #[rename(name = "BirdsViewRatio")]
    pub birds_view_ratio: f32,
    #[rename(name = "DeltaLongitudeScale")]
    pub delta_longitude_scale: f32,
    #[rename(name = "DeltaLatitudeScale")]
    pub delta_latitude_scale: f32,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_DeltaLongitude")]
    pub m_delta_longitude: f32,
    #[rename(name = "m_DeltaLatitude")]
    pub m_delta_latitude: f32,
    #[rename(name = "m_IsBirdsView")]
    pub m_is_birds_view: bool,
}

#[cfg(feature = "combat-autocamera")]
#[::unity2::methods]
impl AutoCamera {
    #[method(name = "rr", args = 1)]
    pub fn rr(r: f32) -> f32;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "ChangeCut", args = 0)]
    pub fn change_cut(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "UpdateMove", args = 0)]
    pub fn update_move(self) -> ();

    #[method(name = "Clamp", args = 0)]
    pub fn clamp(self) -> bool;

    #[method(name = "IsOut", args = 2)]
    pub fn is_out(longitude: f32, latitude: f32) -> bool;

    #[method(name = "InRedZone", args = 2)]
    pub fn in_red_zone(longitude: f32, latitude: f32) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-autocamera")]
impl AutoCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AutoCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IAutoCameraMethods>::ctor(this);
        this
    }
}
