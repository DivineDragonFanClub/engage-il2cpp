
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animations/animationclipplayable/AnimationClipPlayable.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AnimationClipPlayable {
    pub m_handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
}

impl ::unity2::ClassIdentity for AnimationClipPlayable {
    const NAMESPACE: &'static str = "UnityEngine.Animations";

    const NAME: &'static str = "AnimationClipPlayable";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimationClipPlayable {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-animations-animationclipplayable")]
#[::unity2::methods(value)]
impl AnimationClipPlayable {
    #[method(name = "Create", args = 2)]
    pub fn create(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> crate::unity_engine::animations::animationclipplayable::AnimationClipPlayable;

    #[method(name = "CreateHandle", args = 2)]
    pub fn create_handle(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        clip: crate::unity_engine::animationclip::AnimationClip,
    ) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, handle: crate::unity_engine::playables::playablehandle::PlayableHandle)
        -> ();

    #[method(name = "GetHandle", args = 0)]
    pub fn get_handle(self) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        playable: crate::unity_engine::animations::animationclipplayable::AnimationClipPlayable,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::animations::animationclipplayable::AnimationClipPlayable,
    ) -> bool;

    #[method(name = "SetApplyFootIK", args = 1)]
    pub fn set_apply_foot_ik(self, value: bool) -> ();

    #[method(name = "SetApplyPlayableIK", args = 1)]
    pub fn set_apply_playable_ik(self, value: bool) -> ();

    #[method(name = "SetRemoveStartOffset", args = 1)]
    pub fn set_remove_start_offset(self, value: bool) -> ();

    #[method(name = "SetOverrideLoopTime", args = 1)]
    pub fn set_override_loop_time(self, value: bool) -> ();

    #[method(name = "SetLoopTime", args = 1)]
    pub fn set_loop_time(self, value: bool) -> ();

    #[method(name = "CreateHandleInternal", args = 3)]
    pub fn create_handle_internal(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        clip: crate::unity_engine::animationclip::AnimationClip,
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;

    #[method(name = "SetApplyFootIKInternal", args = 2)]
    pub fn set_apply_foot_ik_internal(
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: bool,
    ) -> ();

    #[method(name = "SetApplyPlayableIKInternal", args = 2)]
    pub fn set_apply_playable_ik_internal(
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: bool,
    ) -> ();

    #[method(name = "SetRemoveStartOffsetInternal", args = 2)]
    pub fn set_remove_start_offset_internal(
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: bool,
    ) -> ();

    #[method(name = "SetOverrideLoopTimeInternal", args = 2)]
    pub fn set_override_loop_time_internal(
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: bool,
    ) -> ();

    #[method(name = "SetLoopTimeInternal", args = 2)]
    pub fn set_loop_time_internal(
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
        value: bool,
    ) -> ();

    #[method(name = "CreateHandleInternal_Injected", args = 3)]
    pub fn create_handle_internal_injected(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        clip: crate::unity_engine::animationclip::AnimationClip,
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;
}
