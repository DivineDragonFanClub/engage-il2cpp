
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/assetbundlelocalresource/AssetBundleLocalResource.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "AssetBundleLocalResource"
)]
#[parent(crate::system::object::Object)]
pub struct AssetBundleLocalResource {
    #[rename(name = "m_AssetBundle")]
    pub m_asset_bundle: crate::unity_engine::assetbundle::AssetBundle,
    #[rename(name = "m_RequestOperation")]
    pub m_request_operation: crate::unity_engine::asyncoperation::AsyncOperation,
    #[rename(name = "m_ProgressHandler")]
    pub m_progress_handler: crate::system::func_1::Func_1<f32>,
    #[rename(name = "m_WaitForCompletionHandler")]
    pub m_wait_for_completion_handler: crate::system::func_1::Func_1<bool>,
    #[rename(name = "m_CompleteHandler")]
    pub m_complete_handler:
        crate::system::action_1::Action_1<crate::unity_engine::asyncoperation::AsyncOperation>,
    #[rename(name = "m_ProvideHandle")]
    pub m_provide_handle:
        crate::unity_engine::resource_management::resource_providers::providehandle::ProvideHandle,
    #[rename(name = "m_Key")]
    pub m_key: ::unity2::Il2CppString,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_Completed")]
    pub m_completed: bool,
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundlelocalresource")]
#[::unity2::methods]
impl AssetBundleLocalResource {
    #[method(name = "GetAssetBundle", args = 0)]
    pub fn get_asset_bundle(self) -> crate::unity_engine::assetbundle::AssetBundle;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "PercentComplete", args = 0)]
    pub fn percent_complete(self) -> f32;

    #[method(name = "LocalRequestOperationCompleted", args = 1)]
    pub fn local_request_operation_completed(
        self,
        op: crate::unity_engine::asyncoperation::AsyncOperation,
    ) -> ();

    #[method(name = "WaitForCompletionHandler", args = 0)]
    pub fn wait_for_completion_handler(self) -> bool;

    #[method(name = "Load", args = 2)]
    pub fn load(
        self,
        key: ::unity2::Il2CppString,
        provide_handle : crate :: unity_engine :: resource_management :: resource_providers :: providehandle :: ProvideHandle,
    ) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-assetbundlelocalresource")]
impl AssetBundleLocalResource {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetBundleLocalResource),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetBundleLocalResourceMethods>::ctor(this);
        this
    }
}
