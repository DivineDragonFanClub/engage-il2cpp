
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/shadowsplitdata/ShadowSplitData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ShadowSplitData {}

impl ::unity2::ClassIdentity for ShadowSplitData {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "ShadowSplitData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ShadowSplitData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-shadowsplitdata")]
#[::unity2::methods(value)]
impl ShadowSplitData {
    #[method(name = "get_cullingPlaneCount", args = 0)]
    pub fn get_culling_plane_count(self) -> i32;

    #[method(name = "get_cullingSphere", args = 0)]
    pub fn get_culling_sphere(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "set_cullingSphere", args = 1)]
    pub fn set_culling_sphere(self, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "set_shadowCascadeBlendCullingFactor", args = 1)]
    pub fn set_shadow_cascade_blend_culling_factor(self, value: f32) -> ();

    #[method(name = "GetCullingPlane", args = 1)]
    pub fn get_culling_plane(self, index: i32) -> crate::unity_engine::plane::Plane;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other: crate::unity_engine::rendering::shadowsplitdata::ShadowSplitData,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
