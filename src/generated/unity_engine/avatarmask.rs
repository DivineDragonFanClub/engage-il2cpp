
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/avatarmask/AvatarMask.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AvatarMask")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct AvatarMask {}

#[cfg(feature = "unity_engine-avatarmask")]
#[::unity2::methods]
impl AvatarMask {
    #[method(name = "GetHumanoidBodyPartActive", args = 1)]
    pub fn get_humanoid_body_part_active(
        self,
        index: crate::unity_engine::avatarmaskbodypart::AvatarMaskBodyPart,
    ) -> bool;

    #[method(name = "get_transformCount", args = 0)]
    pub fn get_transform_count(self) -> i32;

    #[method(name = "GetTransformPath", args = 1)]
    pub fn get_transform_path(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetTransformWeight", args = 1)]
    pub fn get_transform_weight(self, index: i32) -> f32;

    #[method(name = "GetTransformActive", args = 1)]
    pub fn get_transform_active(self, index: i32) -> bool;
}
