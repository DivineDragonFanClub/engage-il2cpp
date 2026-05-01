
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/render_graph_module/rendergraphdebugparams/RenderGraphDebugParams.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.RenderGraphModule",
    name = "RenderGraphDebugParams"
)]
#[parent(crate::system::object::Object)]
pub struct RenderGraphDebugParams {
    #[rename(name = "clearRenderTargetsAtCreation")]
    pub clear_render_targets_at_creation: bool,
    #[rename(name = "clearRenderTargetsAtRelease")]
    pub clear_render_targets_at_release: bool,
    #[rename(name = "disablePassCulling")]
    pub disable_pass_culling: bool,
    #[rename(name = "immediateMode")]
    pub immediate_mode: bool,
    #[rename(name = "logFrameInformation")]
    pub log_frame_information: bool,
    #[rename(name = "logResources")]
    pub log_resources: bool,
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphdebugparams")]
#[::unity2::methods]
impl RenderGraphDebugParams {
    #[method(name = "RegisterDebug", args = 1)]
    pub fn register_debug(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "UnRegisterDebug", args = 1)]
    pub fn un_register_debug(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-render_graph_module-rendergraphdebugparams")]
impl RenderGraphDebugParams {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderGraphDebugParams),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderGraphDebugParamsMethods>::ctor(this);
        this
    }
}
