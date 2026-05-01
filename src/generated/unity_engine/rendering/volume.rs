
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/volume/Volume.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "Volume")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct Volume {
    #[rename(name = "isGlobal")]
    pub is_global: bool,
    #[rename(name = "priority")]
    pub priority: f32,
    #[rename(name = "blendDistance")]
    pub blend_distance: f32,
    #[rename(name = "weight")]
    pub weight: f32,
    #[rename(name = "sharedProfile")]
    pub shared_profile: crate::unity_engine::rendering::volumeprofile::VolumeProfile,
    #[rename(name = "m_PreviousLayer")]
    pub m_previous_layer: i32,
    #[rename(name = "m_PreviousPriority")]
    pub m_previous_priority: f32,
    #[rename(name = "m_InternalProfile")]
    pub m_internal_profile: crate::unity_engine::rendering::volumeprofile::VolumeProfile,
}

#[cfg(feature = "unity_engine-rendering-volume")]
#[::unity2::methods]
impl Volume {
    #[method(name = "get_profile", args = 0)]
    pub fn get_profile(self) -> crate::unity_engine::rendering::volumeprofile::VolumeProfile;

    #[method(name = "set_profile", args = 1)]
    pub fn set_profile(
        self,
        value: crate::unity_engine::rendering::volumeprofile::VolumeProfile,
    ) -> ();

    #[method(name = "get_profileRef", args = 0)]
    pub fn get_profile_ref(self) -> crate::unity_engine::rendering::volumeprofile::VolumeProfile;

    #[method(name = "HasInstantiatedProfile", args = 0)]
    pub fn has_instantiated_profile(self) -> bool;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "UpdateLayer", args = 0)]
    pub fn update_layer(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-volume")]
impl Volume {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Volume),
                ::core::stringify!(new),
            )
        });
        <Self as IVolumeMethods>::ctor(this);
        this
    }
}
