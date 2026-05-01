
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/resource_locators/resourcelocationmap/ResourceLocationMap.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets.ResourceLocators",
    name = "ResourceLocationMap"
)]
#[parent(crate::system::object::Object)]
pub struct ResourceLocationMap {}

#[cfg(feature = "unity_engine-addressable_assets-resource_locators-resourcelocationmap")]
#[::unity2::methods]
impl ResourceLocationMap {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, id: ::unity2::Il2CppString, capacity: i32) -> ();

    #[method(name = "get_LocatorId", args = 0)]
    pub fn get_locator_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LocatorId", args = 1)]
    pub fn set_locator_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        id: ::unity2::Il2CppString,
        locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: addressable_assets :: resource_locators :: resourcelocationdata :: ResourceLocationData >,
    ) -> ();

    #[method(name = "get_Locations", args = 0)]
    pub fn get_locations (self ,) -> crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < crate :: system :: object :: Object , crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > > ;

    #[method(name = "set_Locations", args = 1)]
    pub fn set_locations(
        self,
        value : crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < crate :: system :: object :: Object , crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > >,
    ) -> ();

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<
        crate::system::object::Object,
    >;

    #[method(name = "Locate", args = 3)]
    pub fn locate(
        self,
        key: crate::system::object::Object,
        r#type: ::unity2::SystemType,
        locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation >,
    ) -> bool;

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        key: crate::system::object::Object,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_2(
        self,
        key: crate::system::object::Object,
        locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation >,
    ) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-resource_locators-resourcelocationmap")]
impl ResourceLocationMap {
    pub fn new(id: ::unity2::Il2CppString, capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourceLocationMap),
                ::core::stringify!(new),
            )
        });
        <Self as IResourceLocationMapMethods>::ctor(this, id, capacity);
        this
    }

    pub fn new_2(
        id: ::unity2::Il2CppString,
        locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: addressable_assets :: resource_locators :: resourcelocationdata :: ResourceLocationData >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourceLocationMap),
                ::core::stringify!(new_2),
            )
        });
        <Self as IResourceLocationMapMethods>::ctor_2(this, id, locations);
        this
    }
}
