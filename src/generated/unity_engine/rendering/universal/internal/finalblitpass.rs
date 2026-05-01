
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::rendering::universal::scriptablerenderpass::IScriptableRenderPass;
use crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/internal/finalblitpass/FinalBlitPass.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Internal",
    name = "FinalBlitPass"
)]
#[parent(crate::unity_engine::rendering::universal::scriptablerenderpass::ScriptableRenderPass)]
pub struct FinalBlitPass {
    #[rename(name = "m_Source")]
    pub m_source: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    #[rename(name = "m_BlitMaterial")]
    pub m_blit_material: crate::unity_engine::material::Material,
    #[rename(name = "m_CustomFinalMonoColor")]
    pub m_custom_final_mono_color: crate::unity_engine::vector4::Vector4,
    #[static_field]
    #[rename(name = "s_CustomFinalMonoColorProp")]
    pub s_custom_final_mono_color_prop: i32,
}

#[cfg(feature = "unity_engine-rendering-universal-internal-finalblitpass")]
#[::unity2::methods]
impl FinalBlitPass {
    #[method(name = "SetCustomFinalMonoColorToResult", args = 2)]
    pub fn set_custom_final_mono_color_to_result(
        self,
        color: crate::unity_engine::color::Color,
        ratio: f32,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        blit_material: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "Setup", args = 2)]
    pub fn setup(
        self,
        base_descriptor: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
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

#[cfg(feature = "unity_engine-rendering-universal-internal-finalblitpass")]
impl FinalBlitPass {
    pub fn new(
        evt: crate::unity_engine::rendering::universal::renderpassevent::RenderPassEvent,
        blit_material: crate::unity_engine::material::Material,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FinalBlitPass),
                ::core::stringify!(new),
            )
        });
        <Self as IFinalBlitPassMethods>::ctor(this, evt, blit_material);
        this
    }
}
