
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingconfig_throwin/FishingConfig_ThrowIn.md")))]
#[::unity2::class(namespace = "App", name = "FishingConfig_ThrowIn")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FishingConfig_ThrowIn {
    #[rename(name = "m_ThrowInTime")]
    pub m_throw_in_time: f32,
    #[rename(name = "m_ThrowSEPlaySec")]
    pub m_throw_se_play_sec: f32,
    #[rename(name = "m_ThrowCameraChangeSec")]
    pub m_throw_camera_change_sec: f32,
    #[rename(name = "m_ThrowInSinkSec")]
    pub m_throw_in_sink_sec: f32,
}

#[cfg(feature = "app-fishingconfig_throwin")]
#[::unity2::methods]
impl FishingConfig_ThrowIn {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-fishingconfig_throwin")]
impl FishingConfig_ThrowIn {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingConfig_ThrowIn),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingConfig_ThrowInMethods>::ctor(this);
        this
    }
}
