
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_locations/ilocationsizedata/ILocationSizeData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceLocations",
    name = "ILocationSizeData"
)]
pub struct ILocationSizeData {}

#[cfg(feature = "unity_engine-resource_management-resource_locations-ilocationsizedata")]
#[::unity2::methods]
impl ILocationSizeData {
    #[method(name = "ComputeSize", args = 2)]
    pub fn compute_size(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        resource_manager : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager,
    ) -> i64;
}
