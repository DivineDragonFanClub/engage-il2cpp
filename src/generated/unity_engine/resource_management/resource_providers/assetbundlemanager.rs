
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/assetbundlemanager/AssetBundleManager.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "AssetBundleManager"
)]
#[parent(crate::system::object::Object)]
pub struct AssetBundleManager {
# [static_field] # [rename (name = "s_ResourceStack")] pub s_resource_stack : crate :: system :: collections :: generic :: stack_1 :: Stack_1 < crate :: unity_engine :: resource_management :: resource_providers :: assetbundlelocalresource :: AssetBundleLocalResource > ,
# [static_field] # [rename (name = "s_ResourceEntry")] pub s_resource_entry : crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < :: unity2 :: Il2CppString , crate :: unity_engine :: resource_management :: resource_providers :: assetbundlelocalresource :: AssetBundleLocalResource > ,
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundlemanager")]
#[::unity2::methods]
impl AssetBundleManager {
    #[method(name = "Initialize", args = 1)]
    pub fn initialize(capacity: i32) -> ();

    #[method(name = "PopAssetBundleLocalResource", args = 1)]
    pub fn pop_asset_bundle_local_resource(
        provide_handle : crate :: unity_engine :: resource_management :: resource_providers :: providehandle :: ProvideHandle,
    ) -> ();

    #[method(name = "PushAssetBundleLocalResource", args = 1)]
    pub fn push_asset_bundle_local_resource(
        resource : crate :: unity_engine :: resource_management :: resource_providers :: assetbundlelocalresource :: AssetBundleLocalResource,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundlemanager")]
impl AssetBundleManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetBundleManager),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetBundleManagerMethods>::ctor(this);
        this
    }
}
