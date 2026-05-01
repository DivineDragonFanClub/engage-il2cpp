
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/materialpropertyblock/MaterialPropertyBlock.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "MaterialPropertyBlock")]
#[parent(crate::system::object::Object)]
pub struct MaterialPropertyBlock {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-materialpropertyblock")]
#[::unity2::methods]
impl MaterialPropertyBlock {
    #[method(name = "SetFloatImpl", args = 2)]
    pub fn set_float_impl(self, name: i32, value: f32) -> ();

    #[method(name = "SetVectorImpl", args = 2)]
    pub fn set_vector_impl(self, name: i32, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "SetColorImpl", args = 2)]
    pub fn set_color_impl(self, name: i32, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetTextureImpl", args = 2)]
    pub fn set_texture_impl(self, name: i32, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "CreateImpl", args = 0)]
    pub fn create_impl() -> ::unity2::IntPtr;

    #[method(name = "DestroyImpl", args = 1)]
    pub fn destroy_impl(mpb: ::unity2::IntPtr) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, keep_memory: bool) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear_2(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "SetFloat", args = 2)]
    pub fn set_float(self, name: ::unity2::Il2CppString, value: f32) -> ();

    #[method(name = "SetInt", args = 2)]
    pub fn set_int(self, name_id: i32, value: i32) -> ();

    #[method(name = "SetVector", args = 2)]
    pub fn set_vector(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetVector", args = 2)]
    pub fn set_vector_2(self, name_id: i32, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "SetColor", args = 2)]
    pub fn set_color(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetTexture", args = 2)]
    pub fn set_texture(self, name_id: i32, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "SetVectorImpl_Injected", args = 2)]
    pub fn set_vector_impl_injected(
        self,
        name: i32,
        value: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetColorImpl_Injected", args = 2)]
    pub fn set_color_impl_injected(self, name: i32, value: crate::unity_engine::color::Color)
        -> ();
}

#[cfg(feature = "unity_engine-materialpropertyblock")]
impl MaterialPropertyBlock {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaterialPropertyBlock),
                ::core::stringify!(new),
            )
        });
        <Self as IMaterialPropertyBlockMethods>::ctor(this);
        this
    }
}
