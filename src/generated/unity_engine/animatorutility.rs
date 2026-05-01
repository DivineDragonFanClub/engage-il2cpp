
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animatorutility/AnimatorUtility.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AnimatorUtility")]
#[parent(crate::system::object::Object)]
pub struct AnimatorUtility {}

#[cfg(feature = "unity_engine-animatorutility")]
#[::unity2::methods]
impl AnimatorUtility {
    #[method(name = "OptimizeTransformHierarchy", args = 2)]
    pub fn optimize_transform_hierarchy(
        go: crate::unity_engine::gameobject::GameObject,
        exposed_transforms: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();
}
