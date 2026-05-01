
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventcharactermouthcontroller/EventCharacterMouthController.md")))]
#[::unity2::class(namespace = "App", name = "EventCharacterMouthController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct EventCharacterMouthController {
    #[rename(name = "LayerNameArray")]
    pub layer_name_array: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_animLayerIndexArray")]
    pub m_anim_layer_index_array: ::unity2::Array<i32>,
    #[rename(name = "m_weight")]
    pub m_weight: ::unity2::Array<crate::app::weightfader::WeightFader>,
    #[rename(name = "m_voiceEventName")]
    pub m_voice_event_name: ::unity2::Il2CppString,
    #[rename(name = "m_weight_a")]
    pub m_weight_a: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_weight_i")]
    pub m_weight_i: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_weight_u")]
    pub m_weight_u: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_weight_e")]
    pub m_weight_e: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_weight_o")]
    pub m_weight_o: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_weightScale_vol")]
    pub m_weight_scale_vol: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_weightOffset_vol")]
    pub m_weight_offset_vol: crate::unity_engine::animationcurve::AnimationCurve,
}

#[cfg(feature = "app-eventcharactermouthcontroller")]
#[::unity2::methods]
impl EventCharacterMouthController {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "GetWeight", args = 1)]
    pub fn get_weight(self, anim_layer_index: i32) -> f32;

    #[method(name = "SetVoiceEventName", args = 1)]
    pub fn set_voice_event_name(self, voice_event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetWeight", args = 3)]
    pub fn set_weight(self, anim_layer_index: i32, weight: f32, msec: f32) -> ();
}

#[cfg(feature = "app-eventcharactermouthcontroller")]
impl EventCharacterMouthController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventCharacterMouthController),
                ::core::stringify!(new),
            )
        });
        <Self as IEventCharacterMouthControllerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventcharactermouthcontroller/EventCharacterMouthController_AnimLayer.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct EventCharacterMouthController_AnimLayer {
    pub value: i32,
}

impl ::unity2::ClassIdentity for EventCharacterMouthController_AnimLayer {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "EventCharacterMouthController.AnimLayer";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for EventCharacterMouthController_AnimLayer {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl EventCharacterMouthController_AnimLayer {
    pub fn layer_a() -> Self {
        Self { value: 0 }
    }

    pub fn layer_i() -> Self {
        Self { value: 1 }
    }

    pub fn layer_u() -> Self {
        Self { value: 2 }
    }

    pub fn layer_e() -> Self {
        Self { value: 3 }
    }

    pub fn layer_o() -> Self {
        Self { value: 4 }
    }

    pub fn max() -> Self {
        Self { value: 5 }
    }
}
