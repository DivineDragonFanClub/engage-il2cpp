
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::experimental::rendering::render_graph_module::rendergraphresourcepool_1::IRenderGraphResourcePool_1;
use crate::unity_engine::experimental::rendering::render_graph_module::rendergraphresourcepool_1::RenderGraphResourcePool_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/texturepool/TexturePool.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "TexturePool"
)]
# [parent (crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourcepool_1 :: RenderGraphResourcePool_1 < crate :: unity_engine :: rendering :: rthandle :: RTHandle >)]
pub struct TexturePool {}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-texturepool")]
#[::unity2::methods]
impl TexturePool {
    #[method(name = "ReleaseInternalResource", args = 1)]
    pub fn release_internal_resource(
        self,
        res: crate::unity_engine::rendering::rthandle::RTHandle,
    ) -> ();

    #[method(name = "GetResourceName", args = 1)]
    pub fn get_resource_name(
        self,
        res: crate::unity_engine::rendering::rthandle::RTHandle,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetResourceSize", args = 1)]
    pub fn get_resource_size(self, res: crate::unity_engine::rendering::rthandle::RTHandle) -> i64;

    #[method(name = "GetResourceTypeName", args = 0)]
    pub fn get_resource_type_name(self) -> ::unity2::Il2CppString;

    #[method(name = "PurgeUnusedResources", args = 1)]
    pub fn purge_unused_resources(self, current_frame_index: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-texturepool")]
impl TexturePool {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TexturePool),
                ::core::stringify!(new),
            )
        });
        <Self as ITexturePoolMethods>::ctor(this);
        this
    }
}
