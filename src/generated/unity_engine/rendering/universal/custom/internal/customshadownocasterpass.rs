
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/custom/internal/customshadownocasterpass/CustomShadowNoCasterPass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Custom.Internal",
    name = "CustomShadowNoCasterPass"
)]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct CustomShadowNoCasterPass {
    #[static_field]
    #[rename(name = "k_MaxCascades")]
    pub k_max_cascades: i32,
    #[static_field]
    #[rename(name = "k_ShadowmapBufferBits")]
    pub k_shadowmap_buffer_bits: i32,
    #[static_field]
    #[rename(name = "k_ShadowmapWidth")]
    pub k_shadowmap_width: i32,
    #[static_field]
    #[rename(name = "k_ShadowmapHeight")]
    pub k_shadowmap_height: i32,
    #[rename(name = "m_CustomShadowmap")]
    pub m_custom_shadowmap:
        crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    #[rename(name = "m_CustomShadowmapTexture")]
    pub m_custom_shadowmap_texture: crate::unity_engine::rendertexture::RenderTexture,
    #[rename(name = "m_CustomShadowMatrices")]
    pub m_custom_shadow_matrices: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    #[static_field]
    #[rename(name = "m_ProfilerTag")]
    pub m_profiler_tag: ::unity2::Il2CppString,
    #[rename(name = "m_ProfilingSampler")]
    pub m_profiling_sampler: crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
}

#[cfg(feature = "unity_engine-rendering-universal-custom-internal-customshadownocasterpass")]
#[::unity2::methods]
impl CustomShadowNoCasterPass {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
    ) -> ();

    #[method(name = "Configure", args = 2)]
    pub fn configure(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        camera_texture_descriptor : crate :: unity_engine :: rendertexturedescriptor :: RenderTextureDescriptor,
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

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-custom-internal-customshadownocasterpass")]
impl CustomShadowNoCasterPass {
    pub fn new(
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomShadowNoCasterPass),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomShadowNoCasterPassMethods>::ctor(this, evt);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/custom/internal/customshadownocasterpass/CustomShadowNoCasterPass_CustomShadowConstantBuffer.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Custom.Internal",
    name = "CustomShadowNoCasterPass.CustomShadowConstantBuffer"
)]
#[parent(crate::system::object::Object)]
pub struct CustomShadowNoCasterPass_CustomShadowConstantBuffer {
    #[static_field]
    #[rename(name = "_WorldToShadow")]
    pub world_to_shadow: i32,
    #[static_field]
    #[rename(name = "_ShadowParams")]
    pub shadow_params: i32,
    #[static_field]
    #[rename(name = "_ShadowOffset0")]
    pub shadow_offset0: i32,
    #[static_field]
    #[rename(name = "_ShadowOffset1")]
    pub shadow_offset1: i32,
    #[static_field]
    #[rename(name = "_ShadowOffset2")]
    pub shadow_offset2: i32,
    #[static_field]
    #[rename(name = "_ShadowOffset3")]
    pub shadow_offset3: i32,
    #[static_field]
    #[rename(name = "_ShadowmapSize")]
    pub shadowmap_size: i32,
}
