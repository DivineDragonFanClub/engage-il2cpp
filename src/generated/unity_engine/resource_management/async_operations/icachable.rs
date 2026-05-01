
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/async_operations/icachable/ICachable.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.AsyncOperations",
    name = "ICachable"
)]
pub struct ICachable {}

#[cfg(feature = "unity_engine-resource_management-async_operations-icachable")]
#[::unity2::methods]
impl ICachable {
    #[method(name = "get_Key", args = 0)]
    pub fn get_key(
        self,
    ) -> crate::unity_engine::resource_management::util::ioperationcachekey::IOperationCacheKey;

    #[method(name = "set_Key", args = 1)]
    pub fn set_key(
        self,
        value : crate :: unity_engine :: resource_management :: util :: ioperationcachekey :: IOperationCacheKey,
    ) -> ();
}
