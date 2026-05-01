
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/rendererlistdesc/RendererListDesc.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RendererListDesc {}

impl ::unity2::ClassIdentity for RendererListDesc {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";

    const NAME: &'static str = "RendererListDesc";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RendererListDesc {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-rendererlistdesc")]
#[::unity2::methods(value)]
impl RendererListDesc {
    #[method(name = "get_cullingResult", args = 0)]
    pub fn get_culling_result(
        self,
    ) -> crate::unity_engine::rendering::cullingresults::CullingResults;

    #[method(name = "set_cullingResult", args = 1)]
    pub fn set_culling_result(
        self,
        value: crate::unity_engine::rendering::cullingresults::CullingResults,
    ) -> ();

    #[method(name = "get_camera", args = 0)]
    pub fn get_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "set_camera", args = 1)]
    pub fn set_camera(self, value: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "get_passName", args = 0)]
    pub fn get_pass_name(self) -> crate::unity_engine::rendering::shadertagid::ShaderTagId;

    #[method(name = "set_passName", args = 1)]
    pub fn set_pass_name(
        self,
        value: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    ) -> ();

    #[method(name = "get_passNames", args = 0)]
    pub fn get_pass_names(
        self,
    ) -> ::unity2::Array<crate::unity_engine::rendering::shadertagid::ShaderTagId>;

    #[method(name = "set_passNames", args = 1)]
    pub fn set_pass_names(
        self,
        value: ::unity2::Array<crate::unity_engine::rendering::shadertagid::ShaderTagId>,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        pass_name: crate::unity_engine::rendering::shadertagid::ShaderTagId,
        culling_result: crate::unity_engine::rendering::cullingresults::CullingResults,
        camera: crate::unity_engine::camera::Camera,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        pass_names: ::unity2::Array<crate::unity_engine::rendering::shadertagid::ShaderTagId>,
        culling_result: crate::unity_engine::rendering::cullingresults::CullingResults,
        camera: crate::unity_engine::camera::Camera,
    ) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;
}
