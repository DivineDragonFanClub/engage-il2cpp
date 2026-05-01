
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/renderfunc_1/RenderFunc_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderFunc`1"
)]
pub struct RenderFunc_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-renderfunc_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> RenderFunc_1<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        data: T0,
        render_graph_context : crate :: unity_engine :: experimental :: rendering :: render_graph_module :: rendergraphcontext :: RenderGraphContext,
    ) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-renderfunc_1")]
impl<T0: ::unity2::ClassIdentity> RenderFunc_1<T0> {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderFunc_1),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderFunc_1Methods<T0>>::ctor(this, object, method);
        this
    }
}
