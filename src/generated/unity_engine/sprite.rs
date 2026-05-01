
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/sprite/Sprite.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Sprite")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct Sprite {}

#[cfg(feature = "unity_engine-sprite")]
#[::unity2::methods]
impl Sprite {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetPackingMode", args = 0)]
    pub fn get_packing_mode(self) -> i32;

    #[method(name = "GetPackingRotation", args = 0)]
    pub fn get_packing_rotation(self) -> i32;

    #[method(name = "GetPacked", args = 0)]
    pub fn get_packed(self) -> i32;

    #[method(name = "GetTextureRect", args = 0)]
    pub fn get_texture_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "GetTextureRectOffset", args = 0)]
    pub fn get_texture_rect_offset(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetInnerUVs", args = 0)]
    pub fn get_inner_u_vs(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetOuterUVs", args = 0)]
    pub fn get_outer_u_vs(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetPadding", args = 0)]
    pub fn get_padding(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "CreateSpriteWithoutTextureScripting", args = 4)]
    pub fn create_sprite_without_texture_scripting(
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_to_units: f32,
        texture: crate::unity_engine::texture2d::Texture2D,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "CreateSprite", args = 8)]
    pub fn create_sprite(
        texture: crate::unity_engine::texture2d::Texture2D,
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_per_unit: f32,
        extrude: u32,
        mesh_type: crate::unity_engine::spritemeshtype::SpriteMeshType,
        border: crate::unity_engine::vector4::Vector4,
        generate_fallback_physics_shape: bool,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "get_bounds", args = 0)]
    pub fn get_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "get_rect", args = 0)]
    pub fn get_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "get_border", args = 0)]
    pub fn get_border(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "get_texture", args = 0)]
    pub fn get_texture(self) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "GetSecondaryTexture", args = 1)]
    pub fn get_secondary_texture(self, index: i32) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "get_pixelsPerUnit", args = 0)]
    pub fn get_pixels_per_unit(self) -> f32;

    #[method(name = "get_spriteAtlasTextureScale", args = 0)]
    pub fn get_sprite_atlas_texture_scale(self) -> f32;

    #[method(name = "get_associatedAlphaSplitTexture", args = 0)]
    pub fn get_associated_alpha_split_texture(self) -> crate::unity_engine::texture2d::Texture2D;

    #[method(name = "get_pivot", args = 0)]
    pub fn get_pivot(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_vertices", args = 0)]
    pub fn get_vertices(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "get_triangles", args = 0)]
    pub fn get_triangles(self) -> ::unity2::Array<u16>;

    #[method(name = "get_uv", args = 0)]
    pub fn get_uv(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "GetPhysicsShapeCount", args = 0)]
    pub fn get_physics_shape_count(self) -> i32;

    #[method(name = "GetPhysicsShapePointCount", args = 1)]
    pub fn get_physics_shape_point_count(self, shape_idx: i32) -> i32;

    #[method(name = "Internal_GetPhysicsShapePointCount", args = 1)]
    pub fn internal_get_physics_shape_point_count(self, shape_idx: i32) -> i32;

    #[method(name = "GetPhysicsShape", args = 2)]
    pub fn get_physics_shape(
        self,
        shape_idx: i32,
        physics_shape: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector2::Vector2,
        >,
    ) -> i32;

    #[method(name = "GetPhysicsShapeImpl", args = 3)]
    pub fn get_physics_shape_impl(
        sprite: crate::unity_engine::sprite::Sprite,
        shape_idx: i32,
        physics_shape: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector2::Vector2,
        >,
    ) -> ();

    #[method(name = "OverridePhysicsShape", args = 1)]
    pub fn override_physics_shape(
        self,
        physics_shapes: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            ::unity2::Array<crate::unity_engine::vector2::Vector2>,
        >,
    ) -> ();

    #[method(name = "OverridePhysicsShapeCount", args = 2)]
    pub fn override_physics_shape_count(
        sprite: crate::unity_engine::sprite::Sprite,
        physics_shape_count: i32,
    ) -> ();

    #[method(name = "OverridePhysicsShape", args = 3)]
    pub fn override_physics_shape_2(
        sprite: crate::unity_engine::sprite::Sprite,
        physics_shape: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
        idx: i32,
    ) -> ();

    #[method(name = "OverrideGeometry", args = 2)]
    pub fn override_geometry(
        self,
        vertices: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
        triangles: ::unity2::Array<u16>,
    ) -> ();

    #[method(name = "Create", args = 4)]
    pub fn create(
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_to_units: f32,
        texture: crate::unity_engine::texture2d::Texture2D,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Create", args = 3)]
    pub fn create_2(
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_to_units: f32,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Create", args = 8)]
    pub fn create_3(
        texture: crate::unity_engine::texture2d::Texture2D,
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_per_unit: f32,
        extrude: u32,
        mesh_type: crate::unity_engine::spritemeshtype::SpriteMeshType,
        border: crate::unity_engine::vector4::Vector4,
        generate_fallback_physics_shape: bool,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Create", args = 7)]
    pub fn create_4(
        texture: crate::unity_engine::texture2d::Texture2D,
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_per_unit: f32,
        extrude: u32,
        mesh_type: crate::unity_engine::spritemeshtype::SpriteMeshType,
        border: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Create", args = 6)]
    pub fn create_5(
        texture: crate::unity_engine::texture2d::Texture2D,
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_per_unit: f32,
        extrude: u32,
        mesh_type: crate::unity_engine::spritemeshtype::SpriteMeshType,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Create", args = 5)]
    pub fn create_6(
        texture: crate::unity_engine::texture2d::Texture2D,
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_per_unit: f32,
        extrude: u32,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Create", args = 4)]
    pub fn create_7(
        texture: crate::unity_engine::texture2d::Texture2D,
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_per_unit: f32,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Create", args = 3)]
    pub fn create_8(
        texture: crate::unity_engine::texture2d::Texture2D,
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetTextureRect_Injected", args = 1)]
    pub fn get_texture_rect_injected(self, ret: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "GetTextureRectOffset_Injected", args = 1)]
    pub fn get_texture_rect_offset_injected(self, ret: crate::unity_engine::vector2::Vector2)
        -> ();

    #[method(name = "GetInnerUVs_Injected", args = 1)]
    pub fn get_inner_u_vs_injected(self, ret: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "GetOuterUVs_Injected", args = 1)]
    pub fn get_outer_u_vs_injected(self, ret: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "GetPadding_Injected", args = 1)]
    pub fn get_padding_injected(self, ret: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "CreateSpriteWithoutTextureScripting_Injected", args = 4)]
    pub fn create_sprite_without_texture_scripting_injected(
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_to_units: f32,
        texture: crate::unity_engine::texture2d::Texture2D,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "CreateSprite_Injected", args = 8)]
    pub fn create_sprite_injected(
        texture: crate::unity_engine::texture2d::Texture2D,
        rect: crate::unity_engine::rect::Rect,
        pivot: crate::unity_engine::vector2::Vector2,
        pixels_per_unit: f32,
        extrude: u32,
        mesh_type: crate::unity_engine::spritemeshtype::SpriteMeshType,
        border: crate::unity_engine::vector4::Vector4,
        generate_fallback_physics_shape: bool,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "get_bounds_Injected", args = 1)]
    pub fn get_bounds_injected(self, ret: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "get_rect_Injected", args = 1)]
    pub fn get_rect_injected(self, ret: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "get_border_Injected", args = 1)]
    pub fn get_border_injected(self, ret: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "get_pivot_Injected", args = 1)]
    pub fn get_pivot_injected(self, ret: crate::unity_engine::vector2::Vector2) -> ();
}

#[cfg(feature = "unity_engine-sprite")]
impl Sprite {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Sprite),
                ::core::stringify!(new),
            )
        });
        <Self as ISpriteMethods>::ctor(this);
        this
    }
}
