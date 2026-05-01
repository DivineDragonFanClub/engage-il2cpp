
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/framedata/FrameData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FrameData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FrameData_Flags {
    const NAMESPACE: &'static str = "UnityEngine.Playables";

    const NAME: &'static str = "FrameData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FrameData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FrameData_Flags {
    pub fn evaluate() -> Self {
        Self { value: 1 }
    }

    pub fn seek_occured() -> Self {
        Self { value: 2 }
    }

    pub fn r#loop() -> Self {
        Self { value: 4 }
    }

    pub fn hold() -> Self {
        Self { value: 8 }
    }

    pub fn effective_play_state_delayed() -> Self {
        Self { value: 16 }
    }

    pub fn effective_play_state_playing() -> Self {
        Self { value: 32 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/framedata/FrameData_EvaluationType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FrameData_EvaluationType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FrameData_EvaluationType {
    const NAMESPACE: &'static str = "UnityEngine.Playables";

    const NAME: &'static str = "FrameData.EvaluationType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FrameData_EvaluationType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FrameData_EvaluationType {
    pub fn evaluate() -> Self {
        Self { value: 0 }
    }

    pub fn playback() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/framedata/FrameData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct FrameData {
    pub m_frame_id: u64,
    pub m_delta_time: f64,
    pub m_weight: f32,
    pub m_effective_weight: f32,
    pub m_effective_parent_delay: f64,
    pub m_effective_parent_speed: f32,
    pub m_effective_speed: f32,
    pub m_flags: crate::unity_engine::playables::framedata::FrameData_Flags,
    pub m_output: crate::unity_engine::playables::playableoutput::PlayableOutput,
}

impl ::unity2::ClassIdentity for FrameData {
    const NAMESPACE: &'static str = "UnityEngine.Playables";

    const NAME: &'static str = "FrameData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FrameData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-playables-framedata")]
#[::unity2::methods(value)]
impl FrameData {
    #[method(name = "HasFlags", args = 1)]
    pub fn has_flags(
        self,
        flag: crate::unity_engine::playables::framedata::FrameData_Flags,
    ) -> bool;

    #[method(name = "get_deltaTime", args = 0)]
    pub fn get_delta_time(self) -> f32;

    #[method(name = "get_effectiveParentSpeed", args = 0)]
    pub fn get_effective_parent_speed(self) -> f32;

    #[method(name = "get_effectiveSpeed", args = 0)]
    pub fn get_effective_speed(self) -> f32;

    #[method(name = "get_evaluationType", args = 0)]
    pub fn get_evaluation_type(
        self,
    ) -> crate::unity_engine::playables::framedata::FrameData_EvaluationType;

    #[method(name = "get_timeLooped", args = 0)]
    pub fn get_time_looped(self) -> bool;

    #[method(name = "get_timeHeld", args = 0)]
    pub fn get_time_held(self) -> bool;

    #[method(name = "get_output", args = 0)]
    pub fn get_output(self) -> crate::unity_engine::playables::playableoutput::PlayableOutput;

    #[method(name = "get_effectivePlayState", args = 0)]
    pub fn get_effective_play_state(self) -> crate::unity_engine::playables::playstate::PlayState;
}
