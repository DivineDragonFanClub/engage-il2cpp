
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphpass/RenderGraphPass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphPass"
)]
#[parent(crate::system::object::Object)]
pub struct RenderGraphPass {
# [rename (name = "resourceReadLists")] pub resource_read_lists : :: unity2 :: Array < crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle > > ,
# [rename (name = "resourceWriteLists")] pub resource_write_lists : :: unity2 :: Array < crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle > > ,
# [rename (name = "transientResourceList")] pub transient_resource_list : :: unity2 :: Array < crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle > > ,
# [rename (name = "usedRendererListList")] pub used_renderer_list_list : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle > ,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphpass")]
#[::unity2::methods]
impl RenderGraphPass {
    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        render_graph_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
    ) -> ();

    #[method(name = "Release", args = 1)]
    pub fn release(
        self,
        pool : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphobjectpool :: RenderGraphObjectPool,
    ) -> ();

    #[method(name = "HasRenderFunc", args = 0)]
    pub fn has_render_func(self) -> bool;

    #[method(name = "get_name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "set_index", args = 1)]
    pub fn set_index(self, value: i32) -> ();

    #[method(name = "get_customSampler", args = 0)]
    pub fn get_custom_sampler(
        self,
    ) -> crate::unity_engine::rendering::profilingsampler::ProfilingSampler;

    #[method(name = "set_customSampler", args = 1)]
    pub fn set_custom_sampler(
        self,
        value: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    ) -> ();

    #[method(name = "get_enableAsyncCompute", args = 0)]
    pub fn get_enable_async_compute(self) -> bool;

    #[method(name = "set_enableAsyncCompute", args = 1)]
    pub fn set_enable_async_compute(self, value: bool) -> ();

    #[method(name = "get_allowPassCulling", args = 0)]
    pub fn get_allow_pass_culling(self) -> bool;

    #[method(name = "set_allowPassCulling", args = 1)]
    pub fn set_allow_pass_culling(self, value: bool) -> ();

    #[method(name = "get_depthBuffer", args = 0)]
    pub fn get_depth_buffer (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "set_depthBuffer", args = 1)]
    pub fn set_depth_buffer(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "get_colorBuffers", args = 0)]
    pub fn get_color_buffers (self ,) -> :: unity2 :: Array < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle > ;

    #[method(name = "set_colorBuffers", args = 1)]
    pub fn set_color_buffers(
        self,
        value : :: unity2 :: Array < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle >,
    ) -> ();

    #[method(name = "get_colorBufferMaxIndex", args = 0)]
    pub fn get_color_buffer_max_index(self) -> i32;

    #[method(name = "set_colorBufferMaxIndex", args = 1)]
    pub fn set_color_buffer_max_index(self, value: i32) -> ();

    #[method(name = "get_refCount", args = 0)]
    pub fn get_ref_count(self) -> i32;

    #[method(name = "set_refCount", args = 1)]
    pub fn set_ref_count(self, value: i32) -> ();

    #[method(name = "get_generateDebugData", args = 0)]
    pub fn get_generate_debug_data(self) -> bool;

    #[method(name = "set_generateDebugData", args = 1)]
    pub fn set_generate_debug_data(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "AddResourceWrite", args = 1)]
    pub fn add_resource_write(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> ();

    #[method(name = "AddResourceRead", args = 1)]
    pub fn add_resource_read(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> ();

    #[method(name = "AddTransientResource", args = 1)]
    pub fn add_transient_resource(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> ();

    #[method(name = "UseRendererList", args = 1)]
    pub fn use_renderer_list(
        self,
        renderer_list : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle,
    ) -> ();

    #[method(name = "EnableAsyncCompute", args = 1)]
    pub fn enable_async_compute(self, value: bool) -> ();

    #[method(name = "AllowPassCulling", args = 1)]
    pub fn allow_pass_culling(self, value: bool) -> ();

    #[method(name = "GenerateDebugData", args = 1)]
    pub fn generate_debug_data(self, value: bool) -> ();

    #[method(name = "SetColorBuffer", args = 2)]
    pub fn set_color_buffer(
        self,
        resource : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
        index: i32,
    ) -> ();

    #[method(name = "SetDepthBuffer", args = 2)]
    pub fn set_depth_buffer_2(
        self,
        resource : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
        flags : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: depthaccess :: DepthAccess,
    ) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphpass")]
impl RenderGraphPass {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphPass),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphPassMethods>::ctor(this);
        this
    }
}
