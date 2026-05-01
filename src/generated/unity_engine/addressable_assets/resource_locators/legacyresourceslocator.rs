
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/resource_locators/legacyresourceslocator/LegacyResourcesLocator.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets.ResourceLocators",
    name = "LegacyResourcesLocator"
)]
#[parent(crate::system::object::Object)]
pub struct LegacyResourcesLocator {}

#[cfg(feature = "unity_engine-addressable_assets-resource_locators-legacyresourceslocator")]
#[::unity2::methods]
impl LegacyResourcesLocator {
    #[method(name = "Locate", args = 3)]
    pub fn locate(
        self,
        key: crate::system::object::Object,
        r#type: ::unity2::SystemType,
        locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation >,
    ) -> bool;

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::system::object::Object,
    >;

    #[method(name = "get_LocatorId", args = 0)]
    pub fn get_locator_id(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-resource_locators-legacyresourceslocator")]
impl LegacyResourcesLocator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LegacyResourcesLocator),
                ::core::stringify!(new),
            )
        });
        <Self as ILegacyResourcesLocatorMethods>::ctor(this);
        this
    }
}
