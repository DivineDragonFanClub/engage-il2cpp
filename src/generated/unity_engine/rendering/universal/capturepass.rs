
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/capturepass/CapturePass.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "CapturePass")]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct CapturePass {
    #[rename(name = "m_CameraColorHandle")]
    pub m_camera_color_handle:
        crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    #[static_field]
    #[rename(name = "m_ProfilerTag")]
    pub m_profiler_tag: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "m_ProfilingSampler")]
    pub m_profiling_sampler: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
}

#[cfg(feature = "unity_engine-rendering-universal-capturepass")]
#[::unity2::methods]
impl CapturePass {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
    ) -> ();

    #[method(name = "Setup", args = 1)]
    pub fn setup(
        self,
        color_handle : crate :: unity_engine :: rendering :: universal :: rendertargethandle :: RenderTargetHandle,
    ) -> ();

    #[method(name = "Execute", args = 2)]
    pub fn execute(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-capturepass")]
impl CapturePass {
    pub fn new(
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CapturePass),
                ::core::stringify!(new),
            )
        });
        <Self as ICapturePassMethods>::ctor(this, evt);
        this
    }
}
