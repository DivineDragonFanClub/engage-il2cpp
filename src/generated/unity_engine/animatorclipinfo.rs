
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animatorclipinfo/AnimatorClipInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AnimatorClipInfo {
    pub m_clip_instance_id: i32,
    pub m_weight: f32,
}

impl ::unity2::ClassIdentity for AnimatorClipInfo {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "AnimatorClipInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimatorClipInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-animatorclipinfo")]
#[::unity2::methods(value)]
impl AnimatorClipInfo {
    #[method(name = "get_clip", args = 0)]
    pub fn get_clip(self) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "get_weight", args = 0)]
    pub fn get_weight(self) -> f32;

    #[method(name = "InstanceIDToAnimationClipPPtr", args = 1)]
    pub fn instance_id_to_animation_clip_p_ptr(
        instance_id: i32,
    ) -> crate::unity_engine::animationclip::AnimationClip;
}
