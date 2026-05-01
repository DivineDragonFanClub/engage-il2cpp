
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphresourceregistry/RenderGraphResourceRegistry_TextureResource.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphResourceRegistry.TextureResource"
)]
# [parent (crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_RenderGraphResource_2 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturedesc :: TextureDesc , crate :: unity_engine :: rendering :: rthandle :: RTHandle >)]
pub struct RenderGraphResourceRegistry_TextureResource {}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
#[::unity2::methods]
impl RenderGraphResourceRegistry_TextureResource {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
impl RenderGraphResourceRegistry_TextureResource {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphResourceRegistry_TextureResource),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphResourceRegistry_TextureResourceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphresourceregistry/RenderGraphResourceRegistry_RendererListResource.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderGraphResourceRegistry_RendererListResource {
    pub desc: crate::unity_engine::experimental::rendering::rendererlistdesc::RendererListDesc,
    pub renderer_list: crate::unity_engine::experimental::rendering::rendererlist::RendererList,
}

impl ::unity2::ClassIdentity for RenderGraphResourceRegistry_RendererListResource {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.RenderGraphModule";

    const NAME: &'static str = "RenderGraphResourceRegistry.RendererListResource";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderGraphResourceRegistry_RendererListResource {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
#[::unity2::methods(value)]
impl RenderGraphResourceRegistry_RendererListResource {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        desc: crate::unity_engine::experimental::rendering::rendererlistdesc::RendererListDesc,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphresourceregistry/RenderGraphResourceRegistry_IRenderGraphResource.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphResourceRegistry.IRenderGraphResource"
)]
#[parent(crate::system::object::Object)]
pub struct RenderGraphResourceRegistry_IRenderGraphResource {
    #[rename(name = "imported")]
    pub imported: bool,
    #[rename(name = "cachedHash")]
    pub cached_hash: i32,
    #[rename(name = "transientPassIndex")]
    pub transient_pass_index: i32,
    #[rename(name = "wasReleased")]
    pub was_released: bool,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
#[::unity2::methods]
impl RenderGraphResourceRegistry_IRenderGraphResource {
    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsCreated", args = 0)]
    pub fn is_created(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
impl RenderGraphResourceRegistry_IRenderGraphResource {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphResourceRegistry_IRenderGraphResource),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphResourceRegistry_IRenderGraphResourceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphresourceregistry/RenderGraphResourceRegistry.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphResourceRegistry"
)]
#[parent(crate::system::object::Object)]
pub struct RenderGraphResourceRegistry {
# [static_field] # [rename (name = "s_EmptyName")] pub s_empty_name : crate :: unity_engine :: rendering :: shadertagid :: ShaderTagId ,
# [static_field] # [rename (name = "m_CurrentRegistry")] pub m_current_registry : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry ,
# [rename (name = "m_Resources")] pub m_resources : :: unity2 :: Array < crate :: unity_engine :: rendering :: dynamicarray_1 :: DynamicArray_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_IRenderGraphResource > > ,
# [rename (name = "m_TexturePool")] pub m_texture_pool : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturepool :: TexturePool ,
# [rename (name = "m_TextureCreationIndex")] pub m_texture_creation_index : i32 ,
# [rename (name = "m_ComputeBufferPool")] pub m_compute_buffer_pool : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferpool :: ComputeBufferPool ,
# [rename (name = "m_RendererListResources")] pub m_renderer_list_resources : crate :: unity_engine :: rendering :: dynamicarray_1 :: DynamicArray_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_RendererListResource > ,
# [rename (name = "m_RenderGraphDebug")] pub m_render_graph_debug : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphdebugparams :: RenderGraphDebugParams ,
# [rename (name = "m_Logger")] pub m_logger : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphlogger :: RenderGraphLogger ,
# [rename (name = "m_CurrentFrameIndex")] pub m_current_frame_index : i32 ,
# [rename (name = "m_CurrentBackbuffer")] pub m_current_backbuffer : crate :: unity_engine :: rendering :: rthandle :: RTHandle ,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
#[::unity2::methods]
impl RenderGraphResourceRegistry {
    #[method(name = "get_current", args = 0)]
    pub fn get_current () -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry ;

    #[method(name = "set_current", args = 1)]
    pub fn set_current(
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry,
    ) -> ();

    #[method(name = "GetTexture", args = 1)]
    pub fn get_texture(
        self,
        handle : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> crate::unity_engine::rendering::rthandle::RTHandle;

    #[method(name = "GetRendererList", args = 1)]
    pub fn get_renderer_list(
        self,
        handle : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle,
    ) -> crate::unity_engine::experimental::rendering::rendererlist::RendererList;

    #[method(name = "GetComputeBuffer", args = 1)]
    pub fn get_compute_buffer(
        self,
        handle : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle,
    ) -> crate::unity_engine::computebuffer::ComputeBuffer;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        render_graph_debug : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphdebugparams :: RenderGraphDebugParams,
        logger : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphlogger :: RenderGraphLogger,
    ) -> ();

    #[method(name = "BeginRender", args = 2)]
    pub fn begin_render(self, current_frame_index: i32, execution_count: i32) -> ();

    #[method(name = "EndRender", args = 0)]
    pub fn end_render(self) -> ();

    #[method(name = "CheckHandleValidity", args = 1)]
    pub fn check_handle_validity(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> ();

    #[method(name = "CheckHandleValidity", args = 2)]
    pub fn check_handle_validity_2(
        self,
        r#type : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourcetype :: RenderGraphResourceType,
        index: i32,
    ) -> ();

    #[method(name = "GetResourceName", args = 1)]
    pub fn get_resource_name(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetResourceName", args = 2)]
    pub fn get_resource_name_2(
        self,
        r#type : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourcetype :: RenderGraphResourceType,
        index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsResourceImported", args = 1)]
    pub fn is_resource_imported(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> bool;

    #[method(name = "IsResourceCreated", args = 1)]
    pub fn is_resource_created(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> bool;

    #[method(name = "IsRendererListCreated", args = 1)]
    pub fn is_renderer_list_created(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle,
    ) -> bool;

    #[method(name = "IsResourceImported", args = 2)]
    pub fn is_resource_imported_2(
        self,
        r#type : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourcetype :: RenderGraphResourceType,
        index: i32,
    ) -> bool;

    #[method(name = "GetResourceTransientIndex", args = 1)]
    pub fn get_resource_transient_index(
        self,
        res : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> i32;

    #[method(name = "ImportTexture", args = 1)]
    pub fn import_texture (self , rt : crate :: unity_engine :: rendering :: rthandle :: RTHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "ImportBackbuffer", args = 1)]
    pub fn import_backbuffer (self , rt : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "CreateTexture", args = 2)]
    pub fn create_texture (self , desc : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturedesc :: TextureDesc , transient_pass_index : i32) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "GetTextureResourceCount", args = 0)]
    pub fn get_texture_resource_count(self) -> i32;

    #[method(name = "GetTextureResource", args = 1)]
    pub fn get_texture_resource (self , handle : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_TextureResource ;

    #[method(name = "GetTextureResourceDesc", args = 1)]
    pub fn get_texture_resource_desc(
        self,
        handle : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle,
    ) -> crate::unity_engine::experimental::rendering::render_graph_module::texturedesc::TextureDesc;

    #[method(name = "CreateRendererList", args = 1)]
    pub fn create_renderer_list (self , desc : crate :: unity_engine :: experimental :: rendering :: rendererlistdesc :: RendererListDesc) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle ;

    #[method(name = "ImportComputeBuffer", args = 1)]
    pub fn import_compute_buffer (self , compute_buffer : crate :: unity_engine :: computebuffer :: ComputeBuffer) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle ;

    #[method(name = "CreateComputeBuffer", args = 2)]
    pub fn create_compute_buffer (self , desc : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferdesc :: ComputeBufferDesc , transient_pass_index : i32) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle ;

    #[method(name = "GetComputeBufferResourceDesc", args = 1)]
    pub fn get_compute_buffer_resource_desc (self , handle : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferdesc :: ComputeBufferDesc ;

    #[method(name = "GetComputeBufferResourceCount", args = 0)]
    pub fn get_compute_buffer_resource_count(self) -> i32;

    #[method(name = "GetComputeBufferResource", args = 1)]
    pub fn get_compute_buffer_resource (self , handle : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: resourcehandle :: ResourceHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_ComputeBufferResource ;

    #[method(name = "CreateAndClearTexture", args = 2)]
    pub fn create_and_clear_texture(
        self,
        rg_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
        index: i32,
    ) -> ();

    #[method(name = "CreateComputeBuffer", args = 2)]
    pub fn create_compute_buffer_2(
        self,
        rg_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
        index: i32,
    ) -> ();

    #[method(name = "ReleaseTexture", args = 2)]
    pub fn release_texture(
        self,
        rg_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
        index: i32,
    ) -> ();

    #[method(name = "ReleaseComputeBuffer", args = 2)]
    pub fn release_compute_buffer(
        self,
        rg_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
        index: i32,
    ) -> ();

    #[method(name = "ValidateTextureDesc", args = 1)]
    pub fn validate_texture_desc(
        self,
        desc : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturedesc :: TextureDesc,
    ) -> ();

    #[method(name = "ValidateRendererListDesc", args = 1)]
    pub fn validate_renderer_list_desc(
        self,
        desc: crate::unity_engine::experimental::rendering::rendererlistdesc::RendererListDesc,
    ) -> ();

    #[method(name = "ValidateComputeBufferDesc", args = 1)]
    pub fn validate_compute_buffer_desc(
        self,
        desc : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferdesc :: ComputeBufferDesc,
    ) -> ();

    #[method(name = "CreateRendererLists", args = 1)]
    pub fn create_renderer_lists(
        self,
        renderer_lists : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle >,
    ) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, on_exception: bool) -> ();

    #[method(name = "PurgeUnusedResources", args = 0)]
    pub fn purge_unused_resources(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "LogTextureCreation", args = 1)]
    pub fn log_texture_creation(
        self,
        rt : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_TextureResource,
    ) -> ();

    #[method(name = "LogTextureRelease", args = 1)]
    pub fn log_texture_release(
        self,
        rt : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_TextureResource,
    ) -> ();

    #[method(name = "LogComputeBufferCreation", args = 1)]
    pub fn log_compute_buffer_creation(
        self,
        buffer : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_ComputeBufferResource,
    ) -> ();

    #[method(name = "LogComputeBufferRelease", args = 1)]
    pub fn log_compute_buffer_release(
        self,
        buffer : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_ComputeBufferResource,
    ) -> ();

    #[method(name = "LogResources", args = 0)]
    pub fn log_resources(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
impl RenderGraphResourceRegistry {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphResourceRegistry),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphResourceRegistryMethods>::ctor(this);
        this
    }

    pub fn new_2(
        render_graph_debug : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphdebugparams :: RenderGraphDebugParams,
        logger : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphlogger :: RenderGraphLogger,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphResourceRegistry),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRenderGraphResourceRegistryMethods>::ctor_2(this, render_graph_debug, logger);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphresourceregistry/RenderGraphResourceRegistry_RenderGraphResource_2.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphResourceRegistry.RenderGraphResource`2"
)]
pub struct RenderGraphResourceRegistry_RenderGraphResource_2<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
> {
    #[rename(name = "desc")]
    pub desc: T0,
    #[rename(name = "resource")]
    pub resource: T1,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    RenderGraphResourceRegistry_RenderGraphResource_2<T0, T1>
{
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "IsCreated", args = 0)]
    pub fn is_created(self) -> bool;
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    RenderGraphResourceRegistry_RenderGraphResource_2<T0, T1>
{
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphResourceRegistry_RenderGraphResource_2),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphResourceRegistry_RenderGraphResource_2Methods<T0, T1>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphresourceregistry/RenderGraphResourceRegistry_ComputeBufferResource.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphResourceRegistry.ComputeBufferResource"
)]
# [parent (crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry_RenderGraphResource_2 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferdesc :: ComputeBufferDesc , crate :: unity_engine :: computebuffer :: ComputeBuffer >)]
pub struct RenderGraphResourceRegistry_ComputeBufferResource {}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
#[::unity2::methods]
impl RenderGraphResourceRegistry_ComputeBufferResource {
    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourceregistry")]
impl RenderGraphResourceRegistry_ComputeBufferResource {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphResourceRegistry_ComputeBufferResource),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphResourceRegistry_ComputeBufferResourceMethods>::ctor(this);
        this
    }
}
