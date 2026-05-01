
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraph/RenderGraph_CompiledResourceInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderGraph_CompiledResourceInfo {
    pub producers: crate::system::collections::generic::list_1::List_1<i32>,
    pub consumers: crate::system::collections::generic::list_1::List_1<i32>,
    pub resource_created: bool,
    pub ref_count: i32,
}

impl ::unity2::ClassIdentity for RenderGraph_CompiledResourceInfo {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.RenderGraphModule";

    const NAME: &'static str = "RenderGraph.CompiledResourceInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderGraph_CompiledResourceInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraph")]
#[::unity2::methods(value)]
impl RenderGraph_CompiledResourceInfo {
    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraph/RenderGraph_CompiledPassInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderGraph_CompiledPassInfo {
    pub pass: crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphpass :: RenderGraphPass,
    pub resource_create_list: :: unity2 :: Array < crate :: system :: collections :: generic :: list_1 :: List_1 < i32 > >,
    pub resource_release_list: :: unity2 :: Array < crate :: system :: collections :: generic :: list_1 :: List_1 < i32 > >,
    pub ref_count: i32,
    pub culled: bool,
    pub has_side_effect: bool,
    pub sync_to_pass_index: i32,
    pub sync_from_pass_index: i32,
    pub need_graphics_fence: bool,
    pub fence: crate :: unity_engine :: rendering :: graphicsfence :: GraphicsFence,
    pub enable_async_compute: bool,
}

impl ::unity2::ClassIdentity for RenderGraph_CompiledPassInfo {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.RenderGraphModule";

    const NAME: &'static str = "RenderGraph.CompiledPassInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderGraph_CompiledPassInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraph")]
#[::unity2::methods(value)]
impl RenderGraph_CompiledPassInfo {
    #[method(name = "get_allowPassCulling", args = 0)]
    pub fn get_allow_pass_culling(self) -> bool;

    #[method(name = "Reset", args = 1)]
    pub fn reset(
        self,
        pass : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphpass :: RenderGraphPass,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraph/RenderGraph_OnGraphRegisteredDelegate.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraph.OnGraphRegisteredDelegate"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RenderGraph_OnGraphRegisteredDelegate {}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraph")]
#[::unity2::methods]
impl RenderGraph_OnGraphRegisteredDelegate {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        graph : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph,
    ) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraph")]
impl RenderGraph_OnGraphRegisteredDelegate {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraph_OnGraphRegisteredDelegate),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraph_OnGraphRegisteredDelegateMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraph/RenderGraph.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraph"
)]
#[parent(crate::system::object::Object)]
pub struct RenderGraph {
# [static_field] # [rename (name = "kMaxMRTCount")] pub k_max_mrt_count : i32 ,
# [rename (name = "m_Resources")] pub m_resources : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphresourceregistry :: RenderGraphResourceRegistry ,
# [rename (name = "m_RenderGraphPool")] pub m_render_graph_pool : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphobjectpool :: RenderGraphObjectPool ,
# [rename (name = "m_RenderPasses")] pub m_render_passes : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphpass :: RenderGraphPass > ,
# [rename (name = "m_RendererLists")] pub m_renderer_lists : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle > ,
# [rename (name = "m_DebugParameters")] pub m_debug_parameters : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphdebugparams :: RenderGraphDebugParams ,
# [rename (name = "m_Logger")] pub m_logger : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphlogger :: RenderGraphLogger ,
# [rename (name = "m_DefaultResources")] pub m_default_resources : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphdefaultresources :: RenderGraphDefaultResources ,
# [rename (name = "m_DefaultProfilingSamplers")] pub m_default_profiling_samplers : crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < i32 , crate :: unity_engine :: rendering :: profilingsampler :: ProfilingSampler > ,
# [rename (name = "m_ExecutionExceptionWasRaised")] pub m_execution_exception_was_raised : bool ,
# [rename (name = "m_RenderGraphContext")] pub m_render_graph_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext ,
# [rename (name = "m_PreviousCommandBuffer")] pub m_previous_command_buffer : crate :: unity_engine :: rendering :: commandbuffer :: CommandBuffer ,
# [rename (name = "m_CurrentImmediatePassIndex")] pub m_current_immediate_pass_index : i32 ,
# [rename (name = "m_ImmediateModeResourceList")] pub m_immediate_mode_resource_list : :: unity2 :: Array < crate :: system :: collections :: generic :: list_1 :: List_1 < i32 > > ,
# [rename (name = "m_CompiledResourcesInfos")] pub m_compiled_resources_infos : :: unity2 :: Array < crate :: unity_engine :: rendering :: dynamicarray_1 :: DynamicArray_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledResourceInfo > > ,
# [rename (name = "m_CompiledPassInfos")] pub m_compiled_pass_infos : crate :: unity_engine :: rendering :: dynamicarray_1 :: DynamicArray_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo > ,
# [rename (name = "m_CullingStack")] pub m_culling_stack : crate :: system :: collections :: generic :: stack_1 :: Stack_1 < i32 > ,
# [rename (name = "m_ExecutionCount")] pub m_execution_count : i32 ,
# [rename (name = "m_RenderGraphDebugData")] pub m_render_graph_debug_data : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphdebugdata :: RenderGraphDebugData ,
# [static_field] # [rename (name = "s_RegisteredGraphs")] pub s_registered_graphs : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph > ,
# [static_field] # [rename (name = "onGraphRegistered")] pub on_graph_registered : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_OnGraphRegisteredDelegate ,
# [static_field] # [rename (name = "onGraphUnregistered")] pub on_graph_unregistered : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_OnGraphRegisteredDelegate ,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraph")]
#[::unity2::methods]
impl RenderGraph {
    #[method(name = "get_name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_requireDebugData", args = 0)]
    pub fn get_require_debug_data() -> bool;

    #[method(name = "set_requireDebugData", args = 1)]
    pub fn set_require_debug_data(value: bool) -> ();

    #[method(name = "get_defaultResources", args = 0)]
    pub fn get_default_resources (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphdefaultresources :: RenderGraphDefaultResources ;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "GetDebugData", args = 0)]
    pub fn get_debug_data (self ,) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphdebugdata :: RenderGraphDebugData ;

    #[method(name = "EndFrame", args = 0)]
    pub fn end_frame(self) -> ();

    #[method(name = "ImportTexture", args = 1)]
    pub fn import_texture (self , rt : crate :: unity_engine :: rendering :: rthandle :: RTHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "ImportBackbuffer", args = 1)]
    pub fn import_backbuffer (self , rt : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "CreateTexture", args = 1)]
    pub fn create_texture (self , desc : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturedesc :: TextureDesc) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "CreateTexture", args = 1)]
    pub fn create_texture_2 (self , texture : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle ;

    #[method(name = "CreateTextureIfInvalid", args = 2)]
    pub fn create_texture_if_invalid(
        self,
        desc : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturedesc :: TextureDesc,
        texture : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> ();

    #[method(name = "GetTextureDesc", args = 1)]
    pub fn get_texture_desc(
        self,
        texture : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: texturehandle :: TextureHandle,
    ) -> crate::unity_engine::experimental::rendering::render_graph_module::texturedesc::TextureDesc;

    #[method(name = "CreateRendererList", args = 1)]
    pub fn create_renderer_list (self , desc : crate :: unity_engine :: experimental :: rendering :: rendererlistdesc :: RendererListDesc) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendererlisthandle :: RendererListHandle ;

    #[method(name = "ImportComputeBuffer", args = 1)]
    pub fn import_compute_buffer (self , compute_buffer : crate :: unity_engine :: computebuffer :: ComputeBuffer) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle ;

    #[method(name = "CreateComputeBuffer", args = 1)]
    pub fn create_compute_buffer (self , desc : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferdesc :: ComputeBufferDesc) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle ;

    #[method(name = "CreateComputeBuffer", args = 1)]
    pub fn create_compute_buffer_2 (self , compute_buffer : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle ;

    #[method(name = "GetComputeBufferDesc", args = 1)]
    pub fn get_compute_buffer_desc (self , compute_buffer : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferhandle :: ComputeBufferHandle) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: computebufferdesc :: ComputeBufferDesc ;

    #[method(name = "Begin", args = 1)]
    pub fn begin(
        self,
        parameters : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphparameters :: RenderGraphParameters,
    ) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "BeginProfilingSampler", args = 1)]
    pub fn begin_profiling_sampler(
        self,
        sampler: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    ) -> ();

    #[method(name = "EndProfilingSampler", args = 1)]
    pub fn end_profiling_sampler(
        self,
        sampler: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    ) -> ();

    #[method(name = "GetRegisteredRenderGraphs", args = 0)]
    pub fn get_registered_render_graphs() -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::experimental::rendering::render_graph_module::rendergraph::RenderGraph,
    >;

    #[method(name = "GetCompiledPassInfos", args = 0)]
    pub fn get_compiled_pass_infos (self ,) -> crate :: unity_engine :: rendering :: dynamicarray_1 :: DynamicArray_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo > ;

    #[method(name = "ClearCompiledGraph", args = 0)]
    pub fn clear_compiled_graph(self) -> ();

    #[method(name = "InvalidateContext", args = 0)]
    pub fn invalidate_context(self) -> ();

    #[method(name = "OnPassAdded", args = 1)]
    pub fn on_pass_added(
        self,
        pass : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphpass :: RenderGraphPass,
    ) -> ();

    #[method(name = "add_onGraphRegistered", args = 1)]
    pub fn add_on_graph_registered(
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_OnGraphRegisteredDelegate,
    ) -> ();

    #[method(name = "remove_onGraphRegistered", args = 1)]
    pub fn remove_on_graph_registered(
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_OnGraphRegisteredDelegate,
    ) -> ();

    #[method(name = "add_onGraphUnregistered", args = 1)]
    pub fn add_on_graph_unregistered(
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_OnGraphRegisteredDelegate,
    ) -> ();

    #[method(name = "remove_onGraphUnregistered", args = 1)]
    pub fn remove_on_graph_unregistered(
        value : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_OnGraphRegisteredDelegate,
    ) -> ();

    #[method(name = "InitResourceInfosData", args = 2)]
    pub fn init_resource_infos_data(
        self,
        resource_infos : crate :: unity_engine :: rendering :: dynamicarray_1 :: DynamicArray_1 < crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledResourceInfo >,
        count: i32,
    ) -> ();

    #[method(name = "InitializeCompilationData", args = 0)]
    pub fn initialize_compilation_data(self) -> ();

    #[method(name = "CountReferences", args = 0)]
    pub fn count_references(self) -> ();

    #[method(name = "CullOutputlessPasses", args = 0)]
    pub fn cull_outputless_passes(self) -> ();

    #[method(name = "CullUnusedPasses", args = 0)]
    pub fn cull_unused_passes(self) -> ();

    #[method(name = "UpdatePassSynchronization", args = 5)]
    pub fn update_pass_synchronization(
        self,
        current_pass_info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo,
        producer_pass_info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo,
        current_pass_index: i32,
        last_producer: i32,
        int_last_sync_index: i32,
    ) -> ();

    #[method(name = "UpdateResourceSynchronization", args = 4)]
    pub fn update_resource_synchronization(
        self,
        last_graphics_pipe_sync: i32,
        last_compute_pipe_sync: i32,
        current_pass_index: i32,
        resource : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledResourceInfo,
    ) -> ();

    #[method(name = "GetLatestProducerIndex", args = 2)]
    pub fn get_latest_producer_index(
        self,
        pass_index: i32,
        info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledResourceInfo,
    ) -> i32;

    #[method(name = "GetLatestValidReadIndex", args = 1)]
    pub fn get_latest_valid_read_index(
        self,
        info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledResourceInfo,
    ) -> i32;

    #[method(name = "GetFirstValidWriteIndex", args = 1)]
    pub fn get_first_valid_write_index(
        self,
        info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledResourceInfo,
    ) -> i32;

    #[method(name = "GetLatestValidWriteIndex", args = 1)]
    pub fn get_latest_valid_write_index(
        self,
        info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledResourceInfo,
    ) -> i32;

    #[method(name = "UpdateResourceAllocationAndSynchronization", args = 0)]
    pub fn update_resource_allocation_and_synchronization(self) -> ();

    #[method(name = "CompileRenderGraph", args = 0)]
    pub fn compile_render_graph(self) -> ();

    #[method(name = "CompilePassImmediatly", args = 1)]
    pub fn compile_pass_immediatly (self , pass : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphpass :: RenderGraphPass) -> crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo ;

    #[method(name = "ExecutePassImmediatly", args = 1)]
    pub fn execute_pass_immediatly(
        self,
        pass : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphpass :: RenderGraphPass,
    ) -> ();

    #[method(name = "ExecuteCompiledPass", args = 2)]
    pub fn execute_compiled_pass(
        self,
        pass_info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo,
        pass_index: i32,
    ) -> ();

    #[method(name = "ExecuteRenderGraph", args = 0)]
    pub fn execute_render_graph(self) -> ();

    #[method(name = "PreRenderPassSetRenderTargets", args = 2)]
    pub fn pre_render_pass_set_render_targets(
        self,
        pass_info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo,
        rg_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
    ) -> ();

    #[method(name = "PreRenderPassExecute", args = 2)]
    pub fn pre_render_pass_execute(
        self,
        pass_info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo,
        rg_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
    ) -> ();

    #[method(name = "PostRenderPassExecute", args = 2)]
    pub fn post_render_pass_execute(
        self,
        pass_info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo,
        rg_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
    ) -> ();

    #[method(name = "ClearRenderPasses", args = 0)]
    pub fn clear_render_passes(self) -> ();

    #[method(name = "ReleaseImmediateModeResources", args = 0)]
    pub fn release_immediate_mode_resources(self) -> ();

    #[method(name = "LogFrameInformation", args = 0)]
    pub fn log_frame_information(self) -> ();

    #[method(name = "LogRendererListsCreation", args = 0)]
    pub fn log_renderer_lists_creation(self) -> ();

    #[method(name = "LogRenderPassBegin", args = 1)]
    pub fn log_render_pass_begin(
        self,
        pass_info : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraph :: RenderGraph_CompiledPassInfo,
    ) -> ();

    #[method(name = "LogCulledPasses", args = 0)]
    pub fn log_culled_passes(self) -> ();

    #[method(name = "GetDefaultProfilingSampler", args = 1)]
    pub fn get_default_profiling_sampler(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::rendering::profilingsampler::ProfilingSampler;

    #[method(name = "UpdateImportedResourceLifeTime", args = 2)]
    pub fn update_imported_resource_life_time(
        self,
        data : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphdebugdata :: RenderGraphDebugData_ResourceDebugData,
        pass_list: crate::system::collections::generic::list_1::List_1<i32>,
    ) -> ();

    #[method(name = "GenerateDebugData", args = 0)]
    pub fn generate_debug_data(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraph")]
impl RenderGraph {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraph),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphMethods>::ctor(this, name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraph/RenderGraph_ProfilingScopePassData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraph.ProfilingScopePassData"
)]
#[parent(crate::system::object::Object)]
pub struct RenderGraph_ProfilingScopePassData {
    #[rename(name = "sampler")]
    pub sampler: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraph")]
#[::unity2::methods]
impl RenderGraph_ProfilingScopePassData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraph")]
impl RenderGraph_ProfilingScopePassData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraph_ProfilingScopePassData),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraph_ProfilingScopePassDataMethods>::ctor(this);
        this
    }
}
