
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_locations/iresourcelocation/IResourceLocation.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceLocations",
    name = "IResourceLocation"
)]
pub struct IResourceLocation {}

#[cfg(feature = "unity_engine-resource_management-resource_locations-iresourcelocation")]
#[::unity2::methods]
impl IResourceLocation {
    #[method(name = "get_InternalId", args = 0)]
    pub fn get_internal_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ProviderId", args = 0)]
    pub fn get_provider_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Dependencies", args = 0)]
    pub fn get_dependencies (self ,) -> crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > ;

    #[method(name = "Hash", args = 1)]
    pub fn hash(self, result_type: ::unity2::SystemType) -> i32;

    #[method(name = "get_DependencyHashCode", args = 0)]
    pub fn get_dependency_hash_code(self) -> i32;

    #[method(name = "get_HasDependencies", args = 0)]
    pub fn get_has_dependencies(self) -> bool;

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::system::object::Object;

    #[method(name = "get_PrimaryKey", args = 0)]
    pub fn get_primary_key(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ResourceType", args = 0)]
    pub fn get_resource_type(self) -> ::unity2::SystemType;
}
