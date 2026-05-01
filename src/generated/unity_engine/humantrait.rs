
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/humantrait/HumanTrait.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "HumanTrait")]
#[parent(crate::system::object::Object)]
pub struct HumanTrait {}

#[cfg(feature = "unity_engine-humantrait")]
#[::unity2::methods]
impl HumanTrait {
    #[method(name = "get_MuscleCount", args = 0)]
    pub fn get_muscle_count() -> i32;

    #[method(name = "GetBoneIndexFromMono", args = 1)]
    pub fn get_bone_index_from_mono(human_id: i32) -> i32;

    #[method(name = "get_MuscleName", args = 0)]
    pub fn get_muscle_name() -> ::unity2::Array<::unity2::Il2CppString>;
}
