
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphlogger/RenderGraphLogger.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphLogger"
)]
#[parent(crate::system::object::Object)]
pub struct RenderGraphLogger {
    #[rename(name = "m_CurrentIndentation")]
    pub m_current_indentation: i32,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphlogger")]
#[::unity2::methods]
impl RenderGraphLogger {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "IncrementIndentation", args = 1)]
    pub fn increment_indentation(self, value: i32) -> ();

    #[method(name = "DecrementIndentation", args = 1)]
    pub fn decrement_indentation(self, value: i32) -> ();

    #[method(name = "LogLine", args = 2)]
    pub fn log_line(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "GetLog", args = 0)]
    pub fn get_log(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphlogger")]
impl RenderGraphLogger {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphLogger),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphLoggerMethods>::ctor(this);
        this
    }
}
