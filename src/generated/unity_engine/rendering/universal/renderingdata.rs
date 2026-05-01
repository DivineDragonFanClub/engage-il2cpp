
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/renderingdata/RenderingData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderingData {
    pub cull_results: crate::unity_engine::rendering::cullingresults::CullingResults,
    pub camera_data: crate::unity_engine::rendering::universal::cameradata::CameraData,
    pub light_data: crate::unity_engine::rendering::universal::lightdata::LightData,
    pub shadow_data: crate::unity_engine::rendering::universal::shadowdata::ShadowData,
    pub post_processing_data:
        crate::unity_engine::rendering::universal::postprocessingdata::PostProcessingData,
    pub supports_dynamic_batching: bool,
    pub per_object_data: crate::unity_engine::rendering::perobjectdata::PerObjectData,
    pub post_processing_enabled: bool,
    pub gpu_save_mode: bool,
}

impl ::unity2::ClassIdentity for RenderingData {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";

    const NAME: &'static str = "RenderingData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderingData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
