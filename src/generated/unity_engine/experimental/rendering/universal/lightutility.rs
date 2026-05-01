
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lightutility/LightUtility.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "LightUtility"
)]
#[parent(crate::system::object::Object)]
pub struct LightUtility {}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lightutility")]
#[::unity2::methods]
impl LightUtility {
    #[method(name = "CheckForChange", args = 2)]
    pub fn check_for_change(a: i32, b: i32) -> bool;

    #[method(name = "CheckForChange", args = 2)]
    pub fn check_for_change_2(a: f32, b: f32) -> bool;

    #[method(name = "CheckForChange", args = 2)]
    pub fn check_for_change_3(a: bool, b: bool) -> bool;

    #[method(name = "GenerateParametricMesh", args = 5)]
    pub fn generate_parametric_mesh(
        mesh: crate::unity_engine::mesh::Mesh,
        radius: f32,
        falloff_distance: f32,
        angle: f32,
        sides: i32,
    ) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "GenerateSpriteMesh", args = 2)]
    pub fn generate_sprite_mesh(
        mesh: crate::unity_engine::mesh::Mesh,
        sprite: crate::unity_engine::sprite::Sprite,
    ) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "GetFalloffShape", args = 1)]
    pub fn get_falloff_shape(
        shape_path: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector2::Vector2>;

    #[method(name = "GenerateShapeMesh", args = 3)]
    pub fn generate_shape_mesh(
        mesh: crate::unity_engine::mesh::Mesh,
        shape_path: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        falloff_distance: f32,
    ) -> crate::unity_engine::bounds::Bounds;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lightutility/LightUtility_SpriteLightMeshVertex.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct LightUtility_SpriteLightMeshVertex {
    pub position: crate::unity_engine::vector3::Vector3,
    pub color: crate::unity_engine::color::Color,
    pub uv: crate::unity_engine::vector2::Vector2,
}

impl ::unity2::ClassIdentity for LightUtility_SpriteLightMeshVertex {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.Universal";

    const NAME: &'static str = "LightUtility.SpriteLightMeshVertex";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for LightUtility_SpriteLightMeshVertex {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lightutility")]
#[::unity2::methods(value)]
impl LightUtility_SpriteLightMeshVertex {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lightutility/LightUtility_ParametricLightMeshVertex.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct LightUtility_ParametricLightMeshVertex {}

impl ::unity2::ClassIdentity for LightUtility_ParametricLightMeshVertex {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.Universal";

    const NAME: &'static str = "LightUtility.ParametricLightMeshVertex";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for LightUtility_ParametricLightMeshVertex {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lightutility")]
#[::unity2::methods(value)]
impl LightUtility_ParametricLightMeshVertex {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
