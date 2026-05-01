
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animations/animationremovescaleplayable/AnimationRemoveScalePlayable.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AnimationRemoveScalePlayable {
    pub m_handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
}

impl ::unity2::ClassIdentity for AnimationRemoveScalePlayable {
    const NAMESPACE: &'static str = "UnityEngine.Animations";

    const NAME: &'static str = "AnimationRemoveScalePlayable";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimationRemoveScalePlayable {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-animations-animationremovescaleplayable")]
#[::unity2::methods(value)]
impl AnimationRemoveScalePlayable {
    #[method(name = "Create", args = 2)]
    pub fn create(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        input_count: i32,
    ) -> crate::unity_engine::animations::animationremovescaleplayable::AnimationRemoveScalePlayable;

    #[method(name = "CreateHandle", args = 2)]
    pub fn create_handle(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        input_count: i32,
    ) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, handle: crate::unity_engine::playables::playablehandle::PlayableHandle)
        -> ();

    #[method(name = "GetHandle", args = 0)]
    pub fn get_handle(self) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        playable : crate :: unity_engine :: animations :: animationremovescaleplayable :: AnimationRemoveScalePlayable,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other : crate :: unity_engine :: animations :: animationremovescaleplayable :: AnimationRemoveScalePlayable,
    ) -> bool;

    #[method(name = "CreateHandleInternal", args = 2)]
    pub fn create_handle_internal(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "CreateHandleInternal_Injected", args = 2)]
    pub fn create_handle_internal_injected(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;
}
