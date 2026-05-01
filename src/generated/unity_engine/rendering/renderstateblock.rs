
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/renderstateblock/RenderStateBlock.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RenderStateBlock {
    pub m_blend_state: crate::unity_engine::rendering::blendstate::BlendState,
    pub m_raster_state: crate::unity_engine::rendering::rasterstate::RasterState,
    pub m_depth_state: crate::unity_engine::rendering::depthstate::DepthState,
    pub m_stencil_state: crate::unity_engine::rendering::stencilstate::StencilState,
    pub m_stencil_reference: i32,
    pub m_mask: crate::unity_engine::rendering::renderstatemask::RenderStateMask,
}

impl ::unity2::ClassIdentity for RenderStateBlock {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "RenderStateBlock";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RenderStateBlock {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-renderstateblock")]
#[::unity2::methods(value)]
impl RenderStateBlock {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, mask: crate::unity_engine::rendering::renderstatemask::RenderStateMask)
        -> ();

    #[method(name = "set_depthState", args = 1)]
    pub fn set_depth_state(
        self,
        value: crate::unity_engine::rendering::depthstate::DepthState,
    ) -> ();

    #[method(name = "get_stencilState", args = 0)]
    pub fn get_stencil_state(self) -> crate::unity_engine::rendering::stencilstate::StencilState;

    #[method(name = "set_stencilState", args = 1)]
    pub fn set_stencil_state(
        self,
        value: crate::unity_engine::rendering::stencilstate::StencilState,
    ) -> ();

    #[method(name = "get_stencilReference", args = 0)]
    pub fn get_stencil_reference(self) -> i32;

    #[method(name = "set_stencilReference", args = 1)]
    pub fn set_stencil_reference(self, value: i32) -> ();

    #[method(name = "get_mask", args = 0)]
    pub fn get_mask(self) -> crate::unity_engine::rendering::renderstatemask::RenderStateMask;

    #[method(name = "set_mask", args = 1)]
    pub fn set_mask(
        self,
        value: crate::unity_engine::rendering::renderstatemask::RenderStateMask,
    ) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::renderstateblock::RenderStateBlock,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
