
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/scriptablecullingparameters/ScriptableCullingParameters.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ScriptableCullingParameters {}

impl ::unity2::ClassIdentity for ScriptableCullingParameters {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "ScriptableCullingParameters";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ScriptableCullingParameters {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-scriptablecullingparameters")]
#[::unity2::methods(value)]
impl ScriptableCullingParameters {
    #[method(name = "set_maximumVisibleLights", args = 1)]
    pub fn set_maximum_visible_lights(self, value: i32) -> ();

    #[method(name = "get_cullingPlaneCount", args = 0)]
    pub fn get_culling_plane_count(self) -> i32;

    #[method(name = "set_isOrthographic", args = 1)]
    pub fn set_is_orthographic(self, value: bool) -> ();

    #[method(name = "set_shadowDistance", args = 1)]
    pub fn set_shadow_distance(self, value: f32) -> ();

    #[method(name = "get_cullingOptions", args = 0)]
    pub fn get_culling_options(
        self,
    ) -> crate::unity_engine::rendering::cullingoptions::CullingOptions;

    #[method(name = "set_cullingOptions", args = 1)]
    pub fn set_culling_options(
        self,
        value: crate::unity_engine::rendering::cullingoptions::CullingOptions,
    ) -> ();

    #[method(name = "set_stereoViewMatrix", args = 1)]
    pub fn set_stereo_view_matrix(self, value: crate::unity_engine::matrix4x4::Matrix4x4) -> ();

    #[method(name = "get_stereoProjectionMatrix", args = 0)]
    pub fn get_stereo_projection_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "set_stereoProjectionMatrix", args = 1)]
    pub fn set_stereo_projection_matrix(
        self,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "GetLayerCullingDistance", args = 1)]
    pub fn get_layer_culling_distance(self, layer_index: i32) -> f32;

    #[method(name = "GetCullingPlane", args = 1)]
    pub fn get_culling_plane(self, index: i32) -> crate::unity_engine::plane::Plane;

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
