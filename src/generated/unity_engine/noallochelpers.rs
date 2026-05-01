
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/noallochelpers/NoAllocHelpers.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "NoAllocHelpers")]
#[parent(crate::system::object::Object)]
pub struct NoAllocHelpers {}

#[cfg(feature = "unity_engine-noallochelpers")]
#[::unity2::methods]
impl NoAllocHelpers {
    #[method(name = "SafeLength", args = 1)]
    pub fn safe_length(values: ::unity2::IlInstance) -> i32;

    #[method(name = "Internal_ResizeList", args = 2)]
    pub fn internal_resize_list(list: crate::system::object::Object, size: i32) -> ();

    #[method(name = "ExtractArrayFromList", args = 1)]
    pub fn extract_array_from_list(list: crate::system::object::Object) -> ::unity2::IlInstance;
}
