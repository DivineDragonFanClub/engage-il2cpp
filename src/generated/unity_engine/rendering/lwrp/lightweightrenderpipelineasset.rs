
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/lwrp/lightweightrenderpipelineasset/LightweightRenderPipelineAsset.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.LWRP",
    name = "LightweightRenderPipelineAsset"
)]
#[parent(crate::system::object::Object)]
pub struct LightweightRenderPipelineAsset {}

#[cfg(feature = "unity_engine-rendering-lwrp-lightweightrenderpipelineasset")]
#[::unity2::methods]
impl LightweightRenderPipelineAsset {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-lwrp-lightweightrenderpipelineasset")]
impl LightweightRenderPipelineAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LightweightRenderPipelineAsset),
                ::core::stringify!(new),
            )
        });
        <Self as ILightweightRenderPipelineAssetMethods>::ctor(this);
        this
    }
}
