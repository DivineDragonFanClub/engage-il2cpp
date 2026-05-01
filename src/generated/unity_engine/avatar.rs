
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/avatar/Avatar.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Avatar")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct Avatar {}

#[cfg(feature = "unity_engine-avatar")]
#[::unity2::methods]
impl Avatar {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_isValid", args = 0)]
    pub fn get_is_valid(self) -> bool;

    #[method(name = "get_isHuman", args = 0)]
    pub fn get_is_human(self) -> bool;

    #[method(name = "get_humanDescription", args = 0)]
    pub fn get_human_description(self) -> crate::unity_engine::humandescription::HumanDescription;

    #[method(name = "SetMuscleMinMax", args = 3)]
    pub fn set_muscle_min_max(self, muscle_id: i32, min: f32, max: f32) -> ();

    #[method(name = "SetParameter", args = 2)]
    pub fn set_parameter(self, parameter_id: i32, value: f32) -> ();

    #[method(name = "GetAxisLength", args = 1)]
    pub fn get_axis_length(self, human_id: i32) -> f32;

    #[method(name = "GetPreRotation", args = 1)]
    pub fn get_pre_rotation(self, human_id: i32) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "GetPostRotation", args = 1)]
    pub fn get_post_rotation(self, human_id: i32) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "GetZYPostQ", args = 3)]
    pub fn get_zy_post_q(
        self,
        human_id: i32,
        parent_q: crate::unity_engine::quaternion::Quaternion,
        q: crate::unity_engine::quaternion::Quaternion,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "GetZYRoll", args = 2)]
    pub fn get_zy_roll(
        self,
        human_id: i32,
        uvw: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "GetLimitSign", args = 1)]
    pub fn get_limit_sign(self, human_id: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Internal_GetAxisLength", args = 1)]
    pub fn internal_get_axis_length(self, human_id: i32) -> f32;

    #[method(name = "Internal_GetPreRotation", args = 1)]
    pub fn internal_get_pre_rotation(
        self,
        human_id: i32,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Internal_GetPostRotation", args = 1)]
    pub fn internal_get_post_rotation(
        self,
        human_id: i32,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Internal_GetZYPostQ", args = 3)]
    pub fn internal_get_zy_post_q(
        self,
        human_id: i32,
        parent_q: crate::unity_engine::quaternion::Quaternion,
        q: crate::unity_engine::quaternion::Quaternion,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Internal_GetZYRoll", args = 2)]
    pub fn internal_get_zy_roll(
        self,
        human_id: i32,
        uvw: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "Internal_GetLimitSign", args = 1)]
    pub fn internal_get_limit_sign(self, human_id: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_humanDescription_Injected", args = 1)]
    pub fn get_human_description_injected(
        self,
        ret: crate::unity_engine::humandescription::HumanDescription,
    ) -> ();

    #[method(name = "Internal_GetPreRotation_Injected", args = 2)]
    pub fn internal_get_pre_rotation_injected(
        self,
        human_id: i32,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Internal_GetPostRotation_Injected", args = 2)]
    pub fn internal_get_post_rotation_injected(
        self,
        human_id: i32,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Internal_GetZYPostQ_Injected", args = 4)]
    pub fn internal_get_zy_post_q_injected(
        self,
        human_id: i32,
        parent_q: crate::unity_engine::quaternion::Quaternion,
        q: crate::unity_engine::quaternion::Quaternion,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Internal_GetZYRoll_Injected", args = 3)]
    pub fn internal_get_zy_roll_injected(
        self,
        human_id: i32,
        uvw: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "Internal_GetLimitSign_Injected", args = 2)]
    pub fn internal_get_limit_sign_injected(
        self,
        human_id: i32,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();
}

#[cfg(feature = "unity_engine-avatar")]
impl Avatar {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Avatar),
                ::core::stringify!(new),
            )
        });
        <Self as IAvatarMethods>::ctor(this);
        this
    }
}
