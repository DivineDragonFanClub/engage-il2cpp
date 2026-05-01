
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphresourcepool_1/RenderGraphResourcePool_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphResourcePool`1"
)]
pub struct RenderGraphResourcePool_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_CurrentFrameIndex")]
    pub s_current_frame_index: i32,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourcepool_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> RenderGraphResourcePool_1<T0> {
    #[method(name = "ReleaseInternalResource", args = 1)]
    pub fn release_internal_resource(self, res: T0) -> ();

    #[method(name = "GetResourceName", args = 1)]
    pub fn get_resource_name(self, res: T0) -> ::unity2::Il2CppString;

    #[method(name = "GetResourceSize", args = 1)]
    pub fn get_resource_size(self, res: T0) -> i64;

    #[method(name = "GetResourceTypeName", args = 0)]
    pub fn get_resource_type_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ReleaseResource", args = 3)]
    pub fn release_resource(self, hash: i32, resource: T0, current_frame_index: i32) -> ();

    #[method(name = "TryGetResource", args = 2)]
    pub fn try_get_resource(self, hash_code: i32, resource: T0) -> bool;

    #[method(name = "PurgeUnusedResources", args = 1)]
    pub fn purge_unused_resources(self, current_frame_index: i32) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "RegisterFrameAllocation", args = 2)]
    pub fn register_frame_allocation(self, hash: i32, value: T0) -> ();

    #[method(name = "UnregisterFrameAllocation", args = 2)]
    pub fn unregister_frame_allocation(self, hash: i32, value: T0) -> ();

    #[method(name = "CheckFrameAllocation", args = 2)]
    pub fn check_frame_allocation(self, on_exception: bool, frame_index: i32) -> ();

    #[method(name = "LogResources", args = 1)]
    pub fn log_resources(
        self,
        logger : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphlogger :: RenderGraphLogger,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphresourcepool_1")]
impl<T0: ::unity2::ClassIdentity> RenderGraphResourcePool_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphResourcePool_1),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphResourcePool_1Methods<T0>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphresourcepool_1/RenderGraphResourcePool_1_ResourceLogInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderGraphResourcePool_1_ResourceLogInfo<T0> {
    pub _phantom: ::core::marker::PhantomData<(T0,)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity
    for RenderGraphResourcePool_1_ResourceLogInfo<T0>
{
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.RenderGraphModule";

    const NAME: &'static str = "RenderGraphResourcePool`1.ResourceLogInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[<T0 as ::unity2::ClassIdentity>::class()])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType
    for RenderGraphResourcePool_1_ResourceLogInfo<T0>
{
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
