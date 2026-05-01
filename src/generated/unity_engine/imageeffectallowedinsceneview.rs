
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/imageeffectallowedinsceneview/ImageEffectAllowedInSceneView.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ImageEffectAllowedInSceneView")]
pub struct ImageEffectAllowedInSceneView {}

#[cfg(feature = "unity_engine-imageeffectallowedinsceneview")]
#[::unity2::methods]
impl ImageEffectAllowedInSceneView {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-imageeffectallowedinsceneview")]
impl ImageEffectAllowedInSceneView {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ImageEffectAllowedInSceneView),
                ::core::stringify!(new),
            )
        });
        <Self as IImageEffectAllowedInSceneViewMethods>::ctor(this);
        this
    }
}
