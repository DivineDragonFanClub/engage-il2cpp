
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/rendertargetidentifier/RenderTargetIdentifier.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderTargetIdentifier {
    pub m_type: crate::unity_engine::rendering::builtinrendertexturetype::BuiltinRenderTextureType,
    pub m_name_id: i32,
    pub m_instance_id: i32,
    pub m_buffer_pointer: ::unity2::IntPtr,
    pub m_mip_level: i32,
    pub m_cube_face: crate::unity_engine::cubemapface::CubemapFace,
    pub m_depth_slice: i32,
}

impl ::unity2::ClassIdentity for RenderTargetIdentifier {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "RenderTargetIdentifier";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderTargetIdentifier {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-rendertargetidentifier")]
#[::unity2::methods(value)]
impl RenderTargetIdentifier {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        r#type: crate::unity_engine::rendering::builtinrendertexturetype::BuiltinRenderTextureType,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, name_id: i32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_3(
        self,
        name_id: i32,
        mip_level: i32,
        cube_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_4(
        self,
        render_target_identifier : crate :: unity_engine :: rendering :: rendertargetidentifier :: RenderTargetIdentifier,
        mip_level: i32,
        cube_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_5(self, tex: crate::unity_engine::texture::Texture) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_6(
        self,
        tex: crate::unity_engine::texture::Texture,
        mip_level: i32,
        cube_face: crate::unity_engine::cubemapface::CubemapFace,
        depth_slice: i32,
    ) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        r#type: crate::unity_engine::rendering::builtinrendertexturetype::BuiltinRenderTextureType,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit_2(
        name_id: i32,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit_3(
        tex: crate::unity_engine::texture::Texture,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        rhs: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        rhs: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        rhs: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> bool;
}
