
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/graphicsfence/GraphicsFence.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct GraphicsFence {
    pub m_ptr: ::unity2::IntPtr,
    pub m_version: i32,
    pub m_fence_type: crate::unity_engine::rendering::graphicsfencetype::GraphicsFenceType,
}

impl ::unity2::ClassIdentity for GraphicsFence {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "GraphicsFence";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GraphicsFence {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-graphicsfence")]
#[::unity2::methods(value)]
impl GraphicsFence {
    #[method(name = "TranslateSynchronizationStageToFlags", args = 1)]
    pub fn translate_synchronization_stage_to_flags(
        s: crate::unity_engine::rendering::synchronisationstage::SynchronisationStage,
    ) -> crate::unity_engine::rendering::synchronisationstageflags::SynchronisationStageFlags;

    #[method(name = "InitPostAllocation", args = 0)]
    pub fn init_post_allocation(self) -> ();

    #[method(name = "IsFencePending", args = 0)]
    pub fn is_fence_pending(self) -> bool;

    #[method(name = "Validate", args = 0)]
    pub fn validate(self) -> ();

    #[method(name = "GetPlatformNotSupportedVersion", args = 0)]
    pub fn get_platform_not_supported_version(self) -> i32;

    #[method(name = "GetVersionNumber", args = 1)]
    pub fn get_version_number(fence_ptr: ::unity2::IntPtr) -> i32;
}
