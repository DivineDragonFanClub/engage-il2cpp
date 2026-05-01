
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingconfig_movecircle/FishingConfig_MoveCircle.md")))]
#[::unity2::class(namespace = "App", name = "FishingConfig_MoveCircle")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FishingConfig_MoveCircle {
    #[rename(name = "m_CameraRotateSpeed")]
    pub m_camera_rotate_speed: f32,
    #[rename(name = "m_DistanceMoveSpeed")]
    pub m_distance_move_speed: f32,
    #[rename(name = "m_RotateMinimum")]
    pub m_rotate_minimum: f32,
    #[rename(name = "m_RotateMax")]
    pub m_rotate_max: f32,
    #[rename(name = "m_DistanceMinimum")]
    pub m_distance_minimum: f32,
    #[rename(name = "m_DistanceMax")]
    pub m_distance_max: f32,
    #[rename(name = "m_RipplesInterval")]
    pub m_ripples_interval: f32,
    #[rename(name = "m_RipplesPosList")]
    pub m_ripples_pos_list: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_RipplesRandomRange")]
    pub m_ripples_random_range: f32,
}

#[cfg(feature = "app-fishingconfig_movecircle")]
#[::unity2::methods]
impl FishingConfig_MoveCircle {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-fishingconfig_movecircle")]
impl FishingConfig_MoveCircle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingConfig_MoveCircle),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingConfig_MoveCircleMethods>::ctor(this);
        this
    }
}
