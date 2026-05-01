
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/xr/hashcodehelper/HashCodeHelper.md")))]
#[::unity2::class(namespace = "UnityEngine.XR", name = "HashCodeHelper")]
#[parent(crate::system::object::Object)]
pub struct HashCodeHelper {}

#[cfg(feature = "unity_engine-xr-hashcodehelper")]
#[::unity2::methods]
impl HashCodeHelper {
    #[method(name = "Combine", args = 2)]
    pub fn combine(hash1: i32, hash2: i32) -> i32;
}
