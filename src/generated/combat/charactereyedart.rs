
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/charactereyedart/CharacterEyeDart.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterEyeDart")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterEyeDart {
    #[rename(name = "m_AmplitudeMin")]
    pub m_amplitude_min: f32,
    #[rename(name = "m_AmplitudeMax")]
    pub m_amplitude_max: f32,
    #[rename(name = "m_IntervalMin")]
    pub m_interval_min: f32,
    #[rename(name = "m_IntervalMax")]
    pub m_interval_max: f32,
    #[rename(name = "m_TransitionTime")]
    pub m_transition_time: f32,
    #[rename(name = "m_State")]
    pub m_state: crate::combat::charactereyedart::CharacterEyeDart_State,
    #[rename(name = "m_StayTime")]
    pub m_stay_time: f32,
    #[rename(name = "m_NowTime")]
    pub m_now_time: f32,
    #[rename(name = "m_PrevWeight")]
    pub m_prev_weight: f32,
    #[rename(name = "m_NextWeight")]
    pub m_next_weight: f32,
}

#[cfg(feature = "combat-charactereyedart")]
#[::unity2::methods]
impl CharacterEyeDart {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "ToStay", args = 0)]
    pub fn to_stay(self) -> ();

    #[method(name = "ToTransition", args = 0)]
    pub fn to_transition(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-charactereyedart")]
impl CharacterEyeDart {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterEyeDart),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterEyeDartMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/charactereyedart/CharacterEyeDart_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CharacterEyeDart_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CharacterEyeDart_State {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "CharacterEyeDart.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CharacterEyeDart_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CharacterEyeDart_State {
    pub fn stay() -> Self {
        Self { value: 0 }
    }

    pub fn transition() -> Self {
        Self { value: 1 }
    }
}
