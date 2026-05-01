
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/util/serializedtyperestrictionattribute/SerializedTypeRestrictionAttribute.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Util",
    name = "SerializedTypeRestrictionAttribute"
)]
pub struct SerializedTypeRestrictionAttribute {
    #[rename(name = "type")]
    pub r#type: ::unity2::SystemType,
}

#[cfg(feature = "unity_engine-resource_management-util-serializedtyperestrictionattribute")]
#[::unity2::methods]
impl SerializedTypeRestrictionAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-util-serializedtyperestrictionattribute")]
impl SerializedTypeRestrictionAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SerializedTypeRestrictionAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as ISerializedTypeRestrictionAttributeMethods>::ctor(this);
        this
    }
}
