
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/gizmos/Gizmos.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Gizmos")]
#[parent(crate::system::object::Object)]
pub struct Gizmos {}

#[cfg(feature = "unity_engine-gizmos")]
#[::unity2::methods]
impl Gizmos {
    #[method(name = "DrawLine", args = 2)]
    pub fn draw_line(
        from: crate::unity_engine::vector3::Vector3,
        to: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "DrawWireSphere", args = 2)]
    pub fn draw_wire_sphere(center: crate::unity_engine::vector3::Vector3, radius: f32) -> ();

    #[method(name = "DrawSphere", args = 2)]
    pub fn draw_sphere(center: crate::unity_engine::vector3::Vector3, radius: f32) -> ();

    #[method(name = "DrawWireCube", args = 2)]
    pub fn draw_wire_cube(
        center: crate::unity_engine::vector3::Vector3,
        size: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "DrawCube", args = 2)]
    pub fn draw_cube(
        center: crate::unity_engine::vector3::Vector3,
        size: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "DrawIcon", args = 3)]
    pub fn draw_icon(
        center: crate::unity_engine::vector3::Vector3,
        name: ::unity2::Il2CppString,
        allow_scaling: bool,
    ) -> ();

    #[method(name = "DrawIcon", args = 4)]
    pub fn draw_icon_2(
        center: crate::unity_engine::vector3::Vector3,
        name: ::unity2::Il2CppString,
        allow_scaling: bool,
        tint: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "get_color", args = 0)]
    pub fn get_color() -> crate::unity_engine::color::Color;

    #[method(name = "set_color", args = 1)]
    pub fn set_color(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_matrix", args = 1)]
    pub fn set_matrix(value: crate::unity_engine::matrix4x4::Matrix4x4) -> ();

    #[method(name = "DrawLine_Injected", args = 2)]
    pub fn draw_line_injected(
        from: crate::unity_engine::vector3::Vector3,
        to: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "DrawWireSphere_Injected", args = 2)]
    pub fn draw_wire_sphere_injected(
        center: crate::unity_engine::vector3::Vector3,
        radius: f32,
    ) -> ();

    #[method(name = "DrawSphere_Injected", args = 2)]
    pub fn draw_sphere_injected(center: crate::unity_engine::vector3::Vector3, radius: f32) -> ();

    #[method(name = "DrawWireCube_Injected", args = 2)]
    pub fn draw_wire_cube_injected(
        center: crate::unity_engine::vector3::Vector3,
        size: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "DrawCube_Injected", args = 2)]
    pub fn draw_cube_injected(
        center: crate::unity_engine::vector3::Vector3,
        size: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "DrawIcon_Injected", args = 4)]
    pub fn draw_icon_injected(
        center: crate::unity_engine::vector3::Vector3,
        name: ::unity2::Il2CppString,
        allow_scaling: bool,
        tint: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "get_color_Injected", args = 1)]
    pub fn get_color_injected(ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_color_Injected", args = 1)]
    pub fn set_color_injected(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_matrix_Injected", args = 1)]
    pub fn set_matrix_injected(value: crate::unity_engine::matrix4x4::Matrix4x4) -> ();
}
