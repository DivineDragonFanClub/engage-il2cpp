
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::audiobehaviour::AudioBehaviour;
use crate::unity_engine::audiobehaviour::IAudioBehaviour;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/audiosource/AudioSource.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AudioSource")]
#[parent(crate::unity_engine::audiobehaviour::AudioBehaviour)]
pub struct AudioSource {}

#[cfg(feature = "unity_engine-audiosource")]
#[::unity2::methods]
impl AudioSource {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-audiosource")]
impl AudioSource {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AudioSource),
                ::core::stringify!(new),
            )
        });
        <Self as IAudioSourceMethods>::ctor(this);
        this
    }
}
