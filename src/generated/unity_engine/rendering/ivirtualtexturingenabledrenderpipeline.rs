
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/ivirtualtexturingenabledrenderpipeline/IVirtualTexturingEnabledRenderPipeline.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering",
    name = "IVirtualTexturingEnabledRenderPipeline"
)]
pub struct IVirtualTexturingEnabledRenderPipeline {}

#[cfg(feature = "unity_engine-rendering-ivirtualtexturingenabledrenderpipeline")]
#[::unity2::methods]
impl IVirtualTexturingEnabledRenderPipeline {
    #[method(name = "get_virtualTexturingEnabled", args = 0)]
    pub fn get_virtual_texturing_enabled(self) -> bool;
}
