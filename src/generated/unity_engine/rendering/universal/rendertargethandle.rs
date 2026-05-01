
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/rendertargethandle/RenderTargetHandle.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderTargetHandle {}

impl ::unity2::ClassIdentity for RenderTargetHandle {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";

    const NAME: &'static str = "RenderTargetHandle";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderTargetHandle {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-universal-rendertargethandle")]
#[::unity2::methods(value)]
impl RenderTargetHandle {
    #[method(name = "set_id", args = 1)]
    pub fn set_id(self, value: i32) -> ();

    #[method(name = "get_id", args = 0)]
    pub fn get_id(self) -> i32;

    #[method(name = "set_rtid", args = 1)]
    pub fn set_rtid(
        self,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = "get_rtid", args = 0)]
    pub fn get_rtid(
        self,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        render_target_identifier : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
    ) -> ();

    #[method(name = "GetCameraTarget", args = 1)]
    pub fn get_camera_target(
        xr: crate::unity_engine::rendering::universal::xrpass::XRPass,
    ) -> crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle;

    #[method(name = "Init", args = 1)]
    pub fn init(self, shader_property: ::unity2::Il2CppString) -> ();

    #[method(name = "Init", args = 1)]
    pub fn init_2(
        self,
        render_target_identifier : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
    ) -> ();

    #[method(name = "Identifier", args = 0)]
    pub fn identifier(
        self,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = "HasInternalRenderTargetId", args = 0)]
    pub fn has_internal_render_target_id(self) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        c1: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
        c2: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        c1: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
        c2: crate::unity_engine::rendering::universal::rendertargethandle::RenderTargetHandle,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
