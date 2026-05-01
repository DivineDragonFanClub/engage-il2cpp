
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/jobs/ijobparallelfortransform/IJobParallelForTransform.md")))]
#[::unity2::class(namespace = "UnityEngine.Jobs", name = "IJobParallelForTransform")]
pub struct IJobParallelForTransform {}

#[cfg(feature = "unity_engine-jobs-ijobparallelfortransform")]
#[::unity2::methods]
impl IJobParallelForTransform {
    #[method(name = "Execute", args = 2)]
    pub fn execute(
        self,
        index: i32,
        transform: crate::unity_engine::jobs::transformaccess::TransformAccess,
    ) -> ();
}
