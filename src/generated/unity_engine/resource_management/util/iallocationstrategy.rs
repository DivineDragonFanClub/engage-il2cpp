
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/util/iallocationstrategy/IAllocationStrategy.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Util",
    name = "IAllocationStrategy"
)]
pub struct IAllocationStrategy {}

#[cfg(feature = "unity_engine-resource_management-util-iallocationstrategy")]
#[::unity2::methods]
impl IAllocationStrategy {
    #[method(name = "New", args = 2)]
    pub fn new(self, r#type: ::unity2::SystemType, type_hash: i32)
        -> crate::system::object::Object;

    #[method(name = "Release", args = 2)]
    pub fn release(self, type_hash: i32, obj: crate::system::object::Object) -> ();
}
