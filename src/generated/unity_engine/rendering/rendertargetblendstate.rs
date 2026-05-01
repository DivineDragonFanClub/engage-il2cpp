
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/rendertargetblendstate/RenderTargetBlendState.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderTargetBlendState {
    pub m_write_mask: u8,
    pub m_source_color_blend_mode: u8,
    pub m_destination_color_blend_mode: u8,
    pub m_source_alpha_blend_mode: u8,
    pub m_destination_alpha_blend_mode: u8,
    pub m_color_blend_operation: u8,
    pub m_alpha_blend_operation: u8,
    pub m_padding: u8,
}

impl ::unity2::ClassIdentity for RenderTargetBlendState {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "RenderTargetBlendState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderTargetBlendState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-rendertargetblendstate")]
#[::unity2::methods(value)]
impl RenderTargetBlendState {
    #[method(name = "get_defaultValue", args = 0)]
    pub fn get_default_value(
    ) -> crate::unity_engine::rendering::rendertargetblendstate::RenderTargetBlendState;

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        write_mask: crate::unity_engine::rendering::colorwritemask::ColorWriteMask,
        source_color_blend_mode: crate::unity_engine::rendering::blendmode::BlendMode,
        destination_color_blend_mode: crate::unity_engine::rendering::blendmode::BlendMode,
        source_alpha_blend_mode: crate::unity_engine::rendering::blendmode::BlendMode,
        destination_alpha_blend_mode: crate::unity_engine::rendering::blendmode::BlendMode,
        color_blend_operation: crate::unity_engine::rendering::blendop::BlendOp,
        alpha_blend_operation: crate::unity_engine::rendering::blendop::BlendOp,
    ) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::rendertargetblendstate::RenderTargetBlendState,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
