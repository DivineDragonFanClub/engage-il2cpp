
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animations/animationoffsetplayable/AnimationOffsetPlayable.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AnimationOffsetPlayable {
    pub m_handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
}

impl ::unity2::ClassIdentity for AnimationOffsetPlayable {
    const NAMESPACE: &'static str = "UnityEngine.Animations";

    const NAME: &'static str = "AnimationOffsetPlayable";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimationOffsetPlayable {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-animations-animationoffsetplayable")]
#[::unity2::methods(value)]
impl AnimationOffsetPlayable {
    #[method(name = "Create", args = 4)]
    pub fn create(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
        input_count: i32,
    ) -> crate::unity_engine::animations::animationoffsetplayable::AnimationOffsetPlayable;

    #[method(name = "CreateHandle", args = 4)]
    pub fn create_handle(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
        input_count: i32,
    ) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, handle: crate::unity_engine::playables::playablehandle::PlayableHandle)
        -> ();

    #[method(name = "GetHandle", args = 0)]
    pub fn get_handle(self) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        playable: crate::unity_engine::animations::animationoffsetplayable::AnimationOffsetPlayable,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::animations::animationoffsetplayable::AnimationOffsetPlayable,
    ) -> bool;

    #[method(name = "CreateHandleInternal", args = 4)]
    pub fn create_handle_internal(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "CreateHandleInternal_Injected", args = 4)]
    pub fn create_handle_internal_injected(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
        handle: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> bool;
}
