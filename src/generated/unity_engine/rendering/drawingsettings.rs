
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/drawingsettings/DrawingSettings.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct DrawingSettings {}

impl ::unity2::ClassIdentity for DrawingSettings {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "DrawingSettings";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DrawingSettings {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-drawingsettings")]
#[::unity2::methods(value)]
impl DrawingSettings {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        shader_pass_name: crate::unity_engine::rendering::shadertagid::ShaderTagId,
        sorting_settings: crate::unity_engine::rendering::sortingsettings::SortingSettings,
    ) -> ();

    #[method(name = "get_sortingSettings", args = 0)]
    pub fn get_sorting_settings(
        self,
    ) -> crate::unity_engine::rendering::sortingsettings::SortingSettings;

    #[method(name = "set_sortingSettings", args = 1)]
    pub fn set_sorting_settings(
        self,
        value: crate::unity_engine::rendering::sortingsettings::SortingSettings,
    ) -> ();

    #[method(name = "set_perObjectData", args = 1)]
    pub fn set_per_object_data(
        self,
        value: crate::unity_engine::rendering::perobjectdata::PerObjectData,
    ) -> ();

    #[method(name = "set_enableDynamicBatching", args = 1)]
    pub fn set_enable_dynamic_batching(self, value: bool) -> ();

    #[method(name = "set_enableInstancing", args = 1)]
    pub fn set_enable_instancing(self, value: bool) -> ();

    #[method(name = "set_overrideMaterial", args = 1)]
    pub fn set_override_material(self, value: crate::unity_engine::material::Material) -> ();

    #[method(name = "set_overrideMaterialPassIndex", args = 1)]
    pub fn set_override_material_pass_index(self, value: i32) -> ();

    #[method(name = "set_mainLightIndex", args = 1)]
    pub fn set_main_light_index(self, value: i32) -> ();

    #[method(name = "GetShaderPassName", args = 1)]
    pub fn get_shader_pass_name(
        self,
        index: i32,
    ) -> crate::unity_engine::rendering::shadertagid::ShaderTagId;

    #[method(name = "SetShaderPassName", args = 2)]
    pub fn set_shader_pass_name(
        self,
        index: i32,
        shader_pass_name: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    ) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::drawingsettings::DrawingSettings,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
