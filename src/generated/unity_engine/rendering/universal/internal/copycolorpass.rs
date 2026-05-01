
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/copycolorpass/CopyColorPass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Internal",
    name = "CopyColorPass"
)]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct CopyColorPass {
    #[rename(name = "m_SampleOffsetShaderHandle")]
    pub m_sample_offset_shader_handle: i32,
    #[rename(name = "m_SamplingMaterial")]
    pub m_sampling_material: crate::unity_engine::material::Material,
    #[rename(name = "m_DownsamplingMethod")]
    pub m_downsampling_method:
        crate::unity_engine::rendering::universal::downsampling::Downsampling,
    #[rename(name = "m_CopyColorMaterial")]
    pub m_copy_color_material: crate::unity_engine::material::Material,
}

#[cfg(feature = "unity_engine-rendering-universal-internal-copycolorpass")]
#[::unity2::methods]
impl CopyColorPass {
    #[method(name = "get_source", args = 0)]
    pub fn get_source(
        self,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = "set_source", args = 1)]
    pub fn set_source(
        self,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = "get_destination", args = 0)]
    pub fn get_destination(
        self,
    ) -> crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle;

    #[method(name = "set_destination", args = 1)]
    pub fn set_destination(
        self,
        value: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        sampling_material: crate::unity_engine::material::Material,
        copy_color_material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "Setup", args = 3)]
    pub fn setup(
        self,
        source: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        destination : crate :: unity_engine :: rendering :: universal :: rendertargethandle :: RenderTargetHandle,
        downsampling: crate::unity_engine::rendering::universal::downsampling::Downsampling,
    ) -> ();

    #[method(name = "OnCameraSetup", args = 2)]
    pub fn on_camera_setup(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = "Execute", args = 2)]
    pub fn execute(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        rendering_data: crate::unity_engine::rendering::universal::renderingdata::RenderingData,
    ) -> ();

    #[method(name = "OnCameraCleanup", args = 1)]
    pub fn on_camera_cleanup(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-internal-copycolorpass")]
impl CopyColorPass {
    pub fn new(
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        sampling_material: crate::unity_engine::material::Material,
        copy_color_material: crate::unity_engine::material::Material,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CopyColorPass),
                ::core::stringify!(new),
            )
        });
        <Self as ICopyColorPassMethods>::ctor(this, evt, sampling_material, copy_color_material);
        this
    }
}
