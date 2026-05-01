
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animations/animationplayableoutput/AnimationPlayableOutput.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AnimationPlayableOutput {
    pub m_handle: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
}

impl ::unity2::ClassIdentity for AnimationPlayableOutput {
    const NAMESPACE: &'static str = "UnityEngine.Animations";

    const NAME: &'static str = "AnimationPlayableOutput";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimationPlayableOutput {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-animations-animationplayableoutput")]
#[::unity2::methods(value)]
impl AnimationPlayableOutput {
    #[method(name = "Create", args = 3)]
    pub fn create(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        name: ::unity2::Il2CppString,
        target: crate::unity_engine::animator::Animator,
    ) -> crate::unity_engine::animations::animationplayableoutput::AnimationPlayableOutput;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        handle: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> ();

    #[method(name = "get_Null", args = 0)]
    pub fn get_null(
    ) -> crate::unity_engine::animations::animationplayableoutput::AnimationPlayableOutput;

    #[method(name = "GetHandle", args = 0)]
    pub fn get_handle(
        self,
    ) -> crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        output: crate::unity_engine::animations::animationplayableoutput::AnimationPlayableOutput,
    ) -> crate::unity_engine::playables::playableoutput::PlayableOutput;

    #[method(name = "op_Explicit", args = 1)]
    pub fn op_explicit(
        output: crate::unity_engine::playables::playableoutput::PlayableOutput,
    ) -> crate::unity_engine::animations::animationplayableoutput::AnimationPlayableOutput;

    #[method(name = "SetTarget", args = 1)]
    pub fn set_target(self, value: crate::unity_engine::animator::Animator) -> ();

    #[method(name = "InternalSetTarget", args = 2)]
    pub fn internal_set_target(
        handle: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        target: crate::unity_engine::animator::Animator,
    ) -> ();
}
