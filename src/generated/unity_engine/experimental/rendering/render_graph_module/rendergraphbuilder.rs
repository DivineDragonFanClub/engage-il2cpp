
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphbuilder/RenderGraphBuilder.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderGraphBuilder {
    pub m_render_pass: crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphpass :: RenderGraphPass,
    pub m_resources: crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry,
    pub m_render_graph: crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph,
    pub m_disposed: bool,
}

impl ::unity2::ClassIdentity for RenderGraphBuilder {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.RenderGraphModule";

    const NAME: &'static str = "RenderGraphBuilder";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderGraphBuilder {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphbuilder")]
#[::unity2::methods(value)]
impl RenderGraphBuilder {
    #[method(name = "UseColorBuffer", args = 2)]
    pub fn use_color_buffer (self , input : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle , index : i32) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "UseDepthBuffer", args = 2)]
    pub fn use_depth_buffer (self , input : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle , flags : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: depthaccess :: DepthAccess) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "ReadTexture", args = 1)]
    pub fn read_texture (self , input : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "WriteTexture", args = 1)]
    pub fn write_texture (self , input : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "CreateTransientTexture", args = 1)]
    pub fn create_transient_texture (self , desc : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturedesc :: TextureDesc) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "CreateTransientTexture", args = 1)]
    pub fn create_transient_texture_2 (self , texture : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "UseRendererList", args = 1)]
    pub fn use_renderer_list (self , input : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle ;

    #[method(name = "ReadComputeBuffer", args = 1)]
    pub fn read_compute_buffer (self , input : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle ;

    #[method(name = "WriteComputeBuffer", args = 1)]
    pub fn write_compute_buffer (self , input : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle ;

    #[method(name = "CreateTransientComputeBuffer", args = 1)]
    pub fn create_transient_compute_buffer (self , desc : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferdesc :: ComputeBufferDesc) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle ;

    #[method(name = "CreateTransientComputeBuffer", args = 1)]
    pub fn create_transient_compute_buffer_2 (self , computebuffer : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle ;

    #[method(name = "EnableAsyncCompute", args = 1)]
    pub fn enable_async_compute(self, value: bool) -> ();

    #[method(name = "AllowPassCulling", args = 1)]
    pub fn allow_pass_culling(self, value: bool) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        render_pass : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphpass :: RenderGraphPass,
        resources : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry,
        render_graph : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph,
    ) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose_2(self, disposing: bool) -> ();

    #[method(name = "CheckResource", args = 1)]
    pub fn check_resource(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> ();

    #[method(name = "GenerateDebugData", args = 1)]
    pub fn generate_debug_data(self, value: bool) -> ();
}
