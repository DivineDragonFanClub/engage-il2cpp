
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/filteringsettings/FilteringSettings.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct FilteringSettings {
    pub m_render_queue_range: crate::unity_engine::rendering::renderqueuerange::RenderQueueRange,
    pub m_layer_mask: i32,
    pub m_rendering_layer_mask: u32,
    pub m_exclude_motion_vector_objects: i32,
    pub m_sorting_layer_range: crate::unity_engine::rendering::sortinglayerrange::SortingLayerRange,
}

impl ::unity2::ClassIdentity for FilteringSettings {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "FilteringSettings";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FilteringSettings {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-filteringsettings")]
#[::unity2::methods(value)]
impl FilteringSettings {
    #[method(name = "set_renderQueueRange", args = 1)]
    pub fn set_render_queue_range(
        self,
        value: crate::unity_engine::rendering::renderqueuerange::RenderQueueRange,
    ) -> ();

    #[method(name = "set_layerMask", args = 1)]
    pub fn set_layer_mask(self, value: i32) -> ();

    #[method(name = "set_renderingLayerMask", args = 1)]
    pub fn set_rendering_layer_mask(self, value: u32) -> ();

    #[method(name = "set_excludeMotionVectorObjects", args = 1)]
    pub fn set_exclude_motion_vector_objects(self, value: bool) -> ();

    #[method(name = "set_sortingLayerRange", args = 1)]
    pub fn set_sorting_layer_range(
        self,
        value: crate::unity_engine::rendering::sortinglayerrange::SortingLayerRange,
    ) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::filteringsettings::FilteringSettings,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
