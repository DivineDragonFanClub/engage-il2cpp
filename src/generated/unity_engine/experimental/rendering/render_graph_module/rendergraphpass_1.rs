
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::experimental::rendering::render_graph_module::rendergraphpass::IRenderGraphPass;
use crate::unity_engine::experimental::rendering::render_graph_module::rendergraphpass::RenderGraphPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphpass_1/RenderGraphPass_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphPass`1"
)]
pub struct RenderGraphPass_1 < T0 : :: unity2 :: ClassIdentity > {
# [rename (name = "data")] pub data : T0 ,
# [rename (name = "renderFunc")] pub render_func : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: renderfunc_1 :: RenderFunc_1 < T0 > ,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphpass_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> RenderGraphPass_1<T0> {
    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        render_graph_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
    ) -> ();

    #[method(name = "Initialize", args = 4)]
    pub fn initialize(
        self,
        pass_index: i32,
        pass_data: T0,
        pass_name: ::unity2::Il2CppString,
        sampler: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    ) -> ();

    #[method(name = "Release", args = 1)]
    pub fn release(
        self,
        pool : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphobjectpool :: RenderGraphObjectPool,
    ) -> ();

    #[method(name = "HasRenderFunc", args = 0)]
    pub fn has_render_func(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphpass_1")]
impl<T0: ::unity2::ClassIdentity> RenderGraphPass_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphPass_1),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphPass_1Methods<T0>>::ctor(this);
        this
    }
}
