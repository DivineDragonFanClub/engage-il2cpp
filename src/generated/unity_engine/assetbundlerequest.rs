
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::asyncoperation::AsyncOperation;
use crate::unity_engine::asyncoperation::IAsyncOperation;
use crate::unity_engine::resourcerequest::IResourceRequest;
use crate::unity_engine::resourcerequest::ResourceRequest;
use crate::unity_engine::yieldinstruction::IYieldInstruction;
use crate::unity_engine::yieldinstruction::YieldInstruction;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/assetbundlerequest/AssetBundleRequest.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AssetBundleRequest")]
#[parent(crate::unity_engine::resourcerequest::ResourceRequest)]
pub struct AssetBundleRequest {}

#[cfg(feature = "unity_engine-assetbundlerequest")]
#[::unity2::methods]
impl AssetBundleRequest {
    #[method(name = "GetResult", args = 0)]
    pub fn get_result(self) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "get_asset", args = 0)]
    pub fn get_asset(self) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "get_allAssets", args = 0)]
    pub fn get_all_assets(self) -> ::unity2::Array<crate::unity_engine::object_2::Object_2>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-assetbundlerequest")]
impl AssetBundleRequest {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetBundleRequest),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetBundleRequestMethods>::ctor(this);
        this
    }
}
