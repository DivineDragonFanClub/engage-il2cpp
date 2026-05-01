
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/util/locationcachekey/LocationCacheKey.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Util",
    name = "LocationCacheKey"
)]
#[parent(crate::system::object::Object)]
pub struct LocationCacheKey {
# [rename (name = "m_Location")] pub m_location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ,
# [rename (name = "m_DesiredType")] pub m_desired_type : :: unity2 :: SystemType ,
}

#[cfg(feature = "unity_engine-resource_management-util-locationcachekey")]
#[::unity2::methods]
impl LocationCacheKey {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        desired_type: ::unity2::SystemType,
    ) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(
        self,
        other : crate :: unity_engine :: resource_management :: util :: ioperationcachekey :: IOperationCacheKey,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_3(
        self,
        other: crate::unity_engine::resource_management::util::locationcachekey::LocationCacheKey,
    ) -> bool;
}

#[cfg(feature = "unity_engine-resource_management-util-locationcachekey")]
impl LocationCacheKey {
    pub fn new(
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        desired_type: ::unity2::SystemType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LocationCacheKey),
                ::core::stringify!(new),
            )
        });
        <Self as ILocationCacheKeyMethods>::ctor(this, location, desired_type);
        this
    }
}
