
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/drawskyboxpass/DrawSkyboxPass.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "DrawSkyboxPass")]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct DrawSkyboxPass {}

#[cfg(feature = "unity_engine-rendering-universal-drawskyboxpass")]
#[::unity2::methods]
impl DrawSkyboxPass {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
    ) -> ();

    #[method(name = "Execute", args = 2)]
    pub fn execute(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-drawskyboxpass")]
impl DrawSkyboxPass {
    pub fn new(
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DrawSkyboxPass),
                ::core::stringify!(new),
            )
        });
        <Self as IDrawSkyboxPassMethods>::ctor(this, evt);
        this
    }
}
