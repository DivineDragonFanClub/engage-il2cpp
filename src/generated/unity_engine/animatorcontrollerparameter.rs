
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animatorcontrollerparameter/AnimatorControllerParameter.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AnimatorControllerParameter")]
#[parent(crate::system::object::Object)]
pub struct AnimatorControllerParameter {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_Type")]
    pub m_type:
        crate::unity_engine::animatorcontrollerparametertype::AnimatorControllerParameterType,
    #[rename(name = "m_DefaultFloat")]
    pub m_default_float: f32,
    #[rename(name = "m_DefaultInt")]
    pub m_default_int: i32,
    #[rename(name = "m_DefaultBool")]
    pub m_default_bool: bool,
}

#[cfg(feature = "unity_engine-animatorcontrollerparameter")]
#[::unity2::methods]
impl AnimatorControllerParameter {
    #[method(name = "get_name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, o: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-animatorcontrollerparameter")]
impl AnimatorControllerParameter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimatorControllerParameter),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimatorControllerParameterMethods>::ctor(this);
        this
    }
}
