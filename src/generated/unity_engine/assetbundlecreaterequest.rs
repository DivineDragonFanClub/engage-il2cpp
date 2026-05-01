
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::asyncoperation::AsyncOperation;
use crate::unity_engine::asyncoperation::IAsyncOperation;
use crate::unity_engine::yieldinstruction::IYieldInstruction;
use crate::unity_engine::yieldinstruction::YieldInstruction;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/assetbundlecreaterequest/AssetBundleCreateRequest.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AssetBundleCreateRequest")]
#[parent(crate::unity_engine::asyncoperation::AsyncOperation)]
pub struct AssetBundleCreateRequest {}

#[cfg(feature = "unity_engine-assetbundlecreaterequest")]
#[::unity2::methods]
impl AssetBundleCreateRequest {
    #[method(name = "get_assetBundle", args = 0)]
    pub fn get_asset_bundle(self) -> crate::unity_engine::assetbundle::AssetBundle;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-assetbundlecreaterequest")]
impl AssetBundleCreateRequest {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssetBundleCreateRequest),
                ::core::stringify!(new),
            )
        });
        <Self as IAssetBundleCreateRequestMethods>::ctor(this);
        this
    }
}
