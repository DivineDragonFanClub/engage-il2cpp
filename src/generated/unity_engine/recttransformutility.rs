
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/recttransformutility/RectTransformUtility.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "RectTransformUtility")]
#[parent(crate::system::object::Object)]
pub struct RectTransformUtility {
    #[static_field]
    #[rename(name = "s_Corners")]
    pub s_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
}

#[cfg(feature = "unity_engine-recttransformutility")]
#[::unity2::methods]
impl RectTransformUtility {
    #[method(name = "PixelAdjustPoint", args = 3)]
    pub fn pixel_adjust_point(
        point: crate::unity_engine::vector2::Vector2,
        element_transform: crate::unity_engine::transform::Transform,
        canvas: crate::unity_engine::canvas::Canvas,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "PixelAdjustRect", args = 2)]
    pub fn pixel_adjust_rect(
        rect_transform: crate::unity_engine::recttransform::RectTransform,
        canvas: crate::unity_engine::canvas::Canvas,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = "PointInRectangle", args = 4)]
    pub fn point_in_rectangle(
        screen_point: crate::unity_engine::vector2::Vector2,
        rect: crate::unity_engine::recttransform::RectTransform,
        cam: crate::unity_engine::camera::Camera,
        offset: crate::unity_engine::vector4::Vector4,
    ) -> bool;

    #[method(name = "RectangleContainsScreenPoint", args = 3)]
    pub fn rectangle_contains_screen_point(
        rect: crate::unity_engine::recttransform::RectTransform,
        screen_point: crate::unity_engine::vector2::Vector2,
        cam: crate::unity_engine::camera::Camera,
    ) -> bool;

    #[method(name = "RectangleContainsScreenPoint", args = 4)]
    pub fn rectangle_contains_screen_point_2(
        rect: crate::unity_engine::recttransform::RectTransform,
        screen_point: crate::unity_engine::vector2::Vector2,
        cam: crate::unity_engine::camera::Camera,
        offset: crate::unity_engine::vector4::Vector4,
    ) -> bool;

    #[method(name = "ScreenPointToWorldPointInRectangle", args = 4)]
    pub fn screen_point_to_world_point_in_rectangle(
        rect: crate::unity_engine::recttransform::RectTransform,
        screen_point: crate::unity_engine::vector2::Vector2,
        cam: crate::unity_engine::camera::Camera,
        world_point: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "ScreenPointToLocalPointInRectangle", args = 4)]
    pub fn screen_point_to_local_point_in_rectangle(
        rect: crate::unity_engine::recttransform::RectTransform,
        screen_point: crate::unity_engine::vector2::Vector2,
        cam: crate::unity_engine::camera::Camera,
        local_point: crate::unity_engine::vector2::Vector2,
    ) -> bool;

    #[method(name = "ScreenPointToRay", args = 2)]
    pub fn screen_point_to_ray(
        cam: crate::unity_engine::camera::Camera,
        screen_pos: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::ray::Ray;

    #[method(name = "WorldToScreenPoint", args = 2)]
    pub fn world_to_screen_point(
        cam: crate::unity_engine::camera::Camera,
        world_point: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "CalculateRelativeRectTransformBounds", args = 2)]
    pub fn calculate_relative_rect_transform_bounds(
        root: crate::unity_engine::transform::Transform,
        child: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "CalculateRelativeRectTransformBounds", args = 1)]
    pub fn calculate_relative_rect_transform_bounds_2(
        trans: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "FlipLayoutOnAxis", args = 4)]
    pub fn flip_layout_on_axis(
        rect: crate::unity_engine::recttransform::RectTransform,
        axis: i32,
        keep_positioning: bool,
        recursive: bool,
    ) -> ();

    #[method(name = "FlipLayoutAxes", args = 3)]
    pub fn flip_layout_axes(
        rect: crate::unity_engine::recttransform::RectTransform,
        keep_positioning: bool,
        recursive: bool,
    ) -> ();

    #[method(name = "GetTransposed", args = 1)]
    pub fn get_transposed(
        input: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "PixelAdjustPoint_Injected", args = 4)]
    pub fn pixel_adjust_point_injected(
        point: crate::unity_engine::vector2::Vector2,
        element_transform: crate::unity_engine::transform::Transform,
        canvas: crate::unity_engine::canvas::Canvas,
        ret: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "PixelAdjustRect_Injected", args = 3)]
    pub fn pixel_adjust_rect_injected(
        rect_transform: crate::unity_engine::recttransform::RectTransform,
        canvas: crate::unity_engine::canvas::Canvas,
        ret: crate::unity_engine::rect::Rect,
    ) -> ();

    #[method(name = "PointInRectangle_Injected", args = 4)]
    pub fn point_in_rectangle_injected(
        screen_point: crate::unity_engine::vector2::Vector2,
        rect: crate::unity_engine::recttransform::RectTransform,
        cam: crate::unity_engine::camera::Camera,
        offset: crate::unity_engine::vector4::Vector4,
    ) -> bool;
}
