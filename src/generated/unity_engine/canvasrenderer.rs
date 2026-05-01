
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/canvasrenderer/CanvasRenderer.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "CanvasRenderer")]
#[parent(crate::unity_engine::component::Component)]
pub struct CanvasRenderer {}

#[cfg(feature = "unity_engine-canvasrenderer")]
#[::unity2::methods]
impl CanvasRenderer {
    #[method(name = "get_hasPopInstruction", args = 0)]
    pub fn get_has_pop_instruction(self) -> bool;

    #[method(name = "set_hasPopInstruction", args = 1)]
    pub fn set_has_pop_instruction(self, value: bool) -> ();

    #[method(name = "get_materialCount", args = 0)]
    pub fn get_material_count(self) -> i32;

    #[method(name = "set_materialCount", args = 1)]
    pub fn set_material_count(self, value: i32) -> ();

    #[method(name = "get_popMaterialCount", args = 0)]
    pub fn get_pop_material_count(self) -> i32;

    #[method(name = "set_popMaterialCount", args = 1)]
    pub fn set_pop_material_count(self, value: i32) -> ();

    #[method(name = "get_absoluteDepth", args = 0)]
    pub fn get_absolute_depth(self) -> i32;

    #[method(name = "get_hasMoved", args = 0)]
    pub fn get_has_moved(self) -> bool;

    #[method(name = "get_cullTransparentMesh", args = 0)]
    pub fn get_cull_transparent_mesh(self) -> bool;

    #[method(name = "set_cullTransparentMesh", args = 1)]
    pub fn set_cull_transparent_mesh(self, value: bool) -> ();

    #[method(name = "get_hasRectClipping", args = 0)]
    pub fn get_has_rect_clipping(self) -> bool;

    #[method(name = "get_relativeDepth", args = 0)]
    pub fn get_relative_depth(self) -> i32;

    #[method(name = "get_cull", args = 0)]
    pub fn get_cull(self) -> bool;

    #[method(name = "set_cull", args = 1)]
    pub fn set_cull(self, value: bool) -> ();

    #[method(name = "get_isMask", args = 0)]
    pub fn get_is_mask(self) -> bool;

    #[method(name = "set_isMask", args = 1)]
    pub fn set_is_mask(self, value: bool) -> ();

    #[method(name = "SetColor", args = 1)]
    pub fn set_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "GetColor", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "EnableRectClipping", args = 1)]
    pub fn enable_rect_clipping(self, rect: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "get_clippingSoftness", args = 0)]
    pub fn get_clipping_softness(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_clippingSoftness", args = 1)]
    pub fn set_clipping_softness(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "DisableRectClipping", args = 0)]
    pub fn disable_rect_clipping(self) -> ();

    #[method(name = "SetMaterial", args = 2)]
    pub fn set_material(self, material: crate::unity_engine::material::Material, index: i32) -> ();

    #[method(name = "GetMaterial", args = 1)]
    pub fn get_material(self, index: i32) -> crate::unity_engine::material::Material;

    #[method(name = "SetPopMaterial", args = 2)]
    pub fn set_pop_material(
        self,
        material: crate::unity_engine::material::Material,
        index: i32,
    ) -> ();

    #[method(name = "GetPopMaterial", args = 1)]
    pub fn get_pop_material(self, index: i32) -> crate::unity_engine::material::Material;

    #[method(name = "SetTexture", args = 1)]
    pub fn set_texture(self, texture: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "SetAlphaTexture", args = 1)]
    pub fn set_alpha_texture(self, texture: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "SetMesh", args = 1)]
    pub fn set_mesh(self, mesh: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "GetAlpha", args = 0)]
    pub fn get_alpha(self) -> f32;

    #[method(name = "SetAlpha", args = 1)]
    pub fn set_alpha(self, alpha: f32) -> ();

    #[method(name = "GetInheritedAlpha", args = 0)]
    pub fn get_inherited_alpha(self) -> f32;

    #[method(name = "SetMaterial", args = 2)]
    pub fn set_material_2(
        self,
        material: crate::unity_engine::material::Material,
        texture: crate::unity_engine::texture::Texture,
    ) -> ();

    #[method(name = "GetMaterial", args = 0)]
    pub fn get_material_2(self) -> crate::unity_engine::material::Material;

    #[method(name = "SplitUIVertexStreams", args = 8)]
    pub fn split_ui_vertex_streams(
        verts: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
        positions: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
        uv0_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv1_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        indices: crate::system::collections::generic::list_1::List_1<i32>,
    ) -> ();

    #[method(name = "SplitUIVertexStreams", args = 10)]
    pub fn split_ui_vertex_streams_2(
        verts: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
        positions: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
        uv0_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv1_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv2_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv3_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        indices: crate::system::collections::generic::list_1::List_1<i32>,
    ) -> ();

    #[method(name = "CreateUIVertexStream", args = 8)]
    pub fn create_ui_vertex_stream(
        verts: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
        positions: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
        uv0_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv1_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        indices: crate::system::collections::generic::list_1::List_1<i32>,
    ) -> ();

    #[method(name = "CreateUIVertexStream", args = 10)]
    pub fn create_ui_vertex_stream_2(
        verts: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
        positions: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
        uv0_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv1_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv2_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv3_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        indices: crate::system::collections::generic::list_1::List_1<i32>,
    ) -> ();

    #[method(name = "AddUIVertexStream", args = 7)]
    pub fn add_ui_vertex_stream(
        verts: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
        positions: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
        uv0_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv1_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "AddUIVertexStream", args = 9)]
    pub fn add_ui_vertex_stream_2(
        verts: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
        positions: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
        uv0_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv1_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv2_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        uv3_s: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "SetVertices", args = 1)]
    pub fn set_vertices(
        self,
        vertices: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
    ) -> ();

    #[method(name = "SetVertices", args = 2)]
    pub fn set_vertices_2(
        self,
        vertices: ::unity2::Array<crate::unity_engine::uivertex::UIVertex>,
        size: i32,
    ) -> ();

    #[method(name = "SplitIndicesStreamsInternal", args = 2)]
    pub fn split_indices_streams_internal(
        verts: crate::system::object::Object,
        indices: crate::system::object::Object,
    ) -> ();

    #[method(name = "SplitUIVertexStreamsInternal", args = 9)]
    pub fn split_ui_vertex_streams_internal(
        verts: crate::system::object::Object,
        positions: crate::system::object::Object,
        colors: crate::system::object::Object,
        uv0_s: crate::system::object::Object,
        uv1_s: crate::system::object::Object,
        uv2_s: crate::system::object::Object,
        uv3_s: crate::system::object::Object,
        normals: crate::system::object::Object,
        tangents: crate::system::object::Object,
    ) -> ();

    #[method(name = "CreateUIVertexStreamInternal", args = 10)]
    pub fn create_ui_vertex_stream_internal(
        verts: crate::system::object::Object,
        positions: crate::system::object::Object,
        colors: crate::system::object::Object,
        uv0_s: crate::system::object::Object,
        uv1_s: crate::system::object::Object,
        uv2_s: crate::system::object::Object,
        uv3_s: crate::system::object::Object,
        normals: crate::system::object::Object,
        tangents: crate::system::object::Object,
        indices: crate::system::object::Object,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "SetColor_Injected", args = 1)]
    pub fn set_color_injected(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "GetColor_Injected", args = 1)]
    pub fn get_color_injected(self, ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "EnableRectClipping_Injected", args = 1)]
    pub fn enable_rect_clipping_injected(self, rect: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "get_clippingSoftness_Injected", args = 1)]
    pub fn get_clipping_softness_injected(self, ret: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "set_clippingSoftness_Injected", args = 1)]
    pub fn set_clipping_softness_injected(self, value: crate::unity_engine::vector2::Vector2)
        -> ();
}

#[cfg(feature = "unity_engine-canvasrenderer")]
impl CanvasRenderer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CanvasRenderer),
                ::core::stringify!(new),
            )
        });
        <Self as ICanvasRendererMethods>::ctor(this);
        this
    }
}
