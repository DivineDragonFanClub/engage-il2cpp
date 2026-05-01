
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/random/Random.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Random")]
#[parent(crate::system::object::Object)]
pub struct Random {}

#[cfg(feature = "unity_engine-random")]
#[::unity2::methods]
impl Random {
    #[method(name = "InitState", args = 1)]
    pub fn init_state(seed: i32) -> ();

    #[method(name = "Range", args = 2)]
    pub fn range(min_inclusive: f32, max_inclusive: f32) -> f32;

    #[method(name = "Range", args = 2)]
    pub fn range_2(min_inclusive: i32, max_exclusive: i32) -> i32;

    #[method(name = "RandomRangeInt", args = 2)]
    pub fn random_range_int(min_inclusive: i32, max_exclusive: i32) -> i32;

    #[method(name = "get_value", args = 0)]
    pub fn get_value() -> f32;

    #[method(name = "get_insideUnitSphere", args = 0)]
    pub fn get_inside_unit_sphere() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_insideUnitSphere_Injected", args = 1)]
    pub fn get_inside_unit_sphere_injected(ret: crate::unity_engine::vector3::Vector3) -> ();
}
