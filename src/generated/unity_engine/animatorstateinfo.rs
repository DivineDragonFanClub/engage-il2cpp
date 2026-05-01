
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animatorstateinfo/AnimatorStateInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AnimatorStateInfo {
    pub m_name: i32,
    pub m_path: i32,
    pub m_full_path: i32,
    pub m_normalized_time: f32,
    pub m_length: f32,
    pub m_speed: f32,
    pub m_speed_multiplier: f32,
    pub m_tag: i32,
    pub m_loop: i32,
}

impl ::unity2::ClassIdentity for AnimatorStateInfo {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "AnimatorStateInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimatorStateInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-animatorstateinfo")]
#[::unity2::methods(value)]
impl AnimatorStateInfo {
    #[method(name = "IsName", args = 1)]
    pub fn is_name(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "get_fullPathHash", args = 0)]
    pub fn get_full_path_hash(self) -> i32;

    #[method(name = "get_shortNameHash", args = 0)]
    pub fn get_short_name_hash(self) -> i32;

    #[method(name = "get_normalizedTime", args = 0)]
    pub fn get_normalized_time(self) -> f32;

    #[method(name = "get_length", args = 0)]
    pub fn get_length(self) -> f32;

    #[method(name = "get_speed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = "get_speedMultiplier", args = 0)]
    pub fn get_speed_multiplier(self) -> f32;

    #[method(name = "get_loop", args = 0)]
    pub fn get_loop(self) -> bool;
}
