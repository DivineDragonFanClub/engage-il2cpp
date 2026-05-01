
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/humandescription/HumanDescription.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HumanDescription {
    pub human: ::unity2::Array<crate::unity_engine::humanbone::HumanBone>,
    pub skeleton: ::unity2::Array<crate::unity_engine::skeletonbone::SkeletonBone>,
    pub m_arm_twist: f32,
    pub m_fore_arm_twist: f32,
    pub m_upper_leg_twist: f32,
    pub m_leg_twist: f32,
    pub m_arm_stretch: f32,
    pub m_leg_stretch: f32,
    pub m_feet_spacing: f32,
    pub m_global_scale: f32,
    pub m_root_motion_bone_name: ::unity2::Il2CppString,
    pub m_has_translation_do_f: bool,
    pub m_has_extra_root: bool,
    pub m_skeleton_has_parents: bool,
}

impl ::unity2::ClassIdentity for HumanDescription {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "HumanDescription";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HumanDescription {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
