
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/shadowdrawingsettings/ShadowDrawingSettings.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ShadowDrawingSettings {
    pub m_culling_results: crate::unity_engine::rendering::cullingresults::CullingResults,
    pub m_light_index: i32,
    pub m_use_rendering_layer_mask_test: i32,
    pub m_split_data: crate::unity_engine::rendering::shadowsplitdata::ShadowSplitData,
}

impl ::unity2::ClassIdentity for ShadowDrawingSettings {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "ShadowDrawingSettings";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ShadowDrawingSettings {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-shadowdrawingsettings")]
#[::unity2::methods(value)]
impl ShadowDrawingSettings {
    #[method(name = "get_splitData", args = 0)]
    pub fn get_split_data(self)
        -> crate::unity_engine::rendering::shadowsplitdata::ShadowSplitData;

    #[method(name = "set_splitData", args = 1)]
    pub fn set_split_data(
        self,
        value: crate::unity_engine::rendering::shadowsplitdata::ShadowSplitData,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        culling_results: crate::unity_engine::rendering::cullingresults::CullingResults,
        light_index: i32,
    ) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::shadowdrawingsettings::ShadowDrawingSettings,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
