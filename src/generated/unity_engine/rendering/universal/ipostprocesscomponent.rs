
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/ipostprocesscomponent/IPostProcessComponent.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "IPostProcessComponent"
)]
pub struct IPostProcessComponent {}

#[cfg(feature = "unity_engine-rendering-universal-ipostprocesscomponent")]
#[::unity2::methods]
impl IPostProcessComponent {
    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "IsTileCompatible", args = 0)]
    pub fn is_tile_compatible(self) -> bool;
}
