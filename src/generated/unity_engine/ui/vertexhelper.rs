
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/vertexhelper/VertexHelper.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "VertexHelper")]
#[parent(crate::system::object::Object)]
pub struct VertexHelper {
    #[rename(name = "m_Positions")]
    pub m_positions:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_Colors")]
    pub m_colors:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::color32::Color32>,
    #[rename(name = "m_Uv0S")]
    pub m_uv0_s:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector4::Vector4>,
    #[rename(name = "m_Uv1S")]
    pub m_uv1_s:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector4::Vector4>,
    #[rename(name = "m_Uv2S")]
    pub m_uv2_s:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector4::Vector4>,
    #[rename(name = "m_Uv3S")]
    pub m_uv3_s:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector4::Vector4>,
    #[rename(name = "m_Normals")]
    pub m_normals:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_Tangents")]
    pub m_tangents:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector4::Vector4>,
    #[rename(name = "m_Indices")]
    pub m_indices: crate::system::collections::generic::list_1::List_1<i32>,
    #[static_field]
    #[rename(name = "s_DefaultTangent")]
    pub s_default_tangent: crate::unity_engine::vector4::Vector4,
    #[static_field]
    #[rename(name = "s_DefaultNormal")]
    pub s_default_normal: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_ListsInitalized")]
    pub m_lists_initalized: bool,
}

#[cfg(feature = "unity_engine-ui-vertexhelper")]
#[::unity2::methods]
impl VertexHelper {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, m: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = "InitializeListIfRequired", args = 0)]
    pub fn initialize_list_if_required(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_currentVertCount", args = 0)]
    pub fn get_current_vert_count(self) -> i32;

    #[method(name = "get_currentIndexCount", args = 0)]
    pub fn get_current_index_count(self) -> i32;

    #[method(name = "PopulateUIVertex", args = 2)]
    pub fn populate_ui_vertex(self, vertex: crate::unity_engine::uivertex::UIVertex, i: i32) -> ();

    #[method(name = "SetUIVertex", args = 2)]
    pub fn set_ui_vertex(self, vertex: crate::unity_engine::uivertex::UIVertex, i: i32) -> ();

    #[method(name = "FillMesh", args = 1)]
    pub fn fill_mesh(self, mesh: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = "AddVert", args = 8)]
    pub fn add_vert(
        self,
        position: crate::unity_engine::vector3::Vector3,
        color: crate::unity_engine::color32::Color32,
        uv0: crate::unity_engine::vector4::Vector4,
        uv1: crate::unity_engine::vector4::Vector4,
        uv2: crate::unity_engine::vector4::Vector4,
        uv3: crate::unity_engine::vector4::Vector4,
        normal: crate::unity_engine::vector3::Vector3,
        tangent: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "AddVert", args = 6)]
    pub fn add_vert_2(
        self,
        position: crate::unity_engine::vector3::Vector3,
        color: crate::unity_engine::color32::Color32,
        uv0: crate::unity_engine::vector4::Vector4,
        uv1: crate::unity_engine::vector4::Vector4,
        normal: crate::unity_engine::vector3::Vector3,
        tangent: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "AddVert", args = 3)]
    pub fn add_vert_3(
        self,
        position: crate::unity_engine::vector3::Vector3,
        color: crate::unity_engine::color32::Color32,
        uv0: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "AddVert", args = 1)]
    pub fn add_vert_4(self, v: crate::unity_engine::uivertex::UIVertex) -> ();

    #[method(name = "AddTriangle", args = 3)]
    pub fn add_triangle(self, idx0: i32, idx1: i32, idx2: i32) -> ();

    #[method(name = "AddUIVertexQuad", args = 1)]
    pub fn add_ui_vertex_quad(
        self,
        verts: ::unity2::Array<crate::unity_engine::uivertex::UIVertex>,
    ) -> ();

    #[method(name = "AddUIVertexStream", args = 2)]
    pub fn add_ui_vertex_stream(
        self,
        verts: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
        indices: crate::system::collections::generic::list_1::List_1<i32>,
    ) -> ();

    #[method(name = "AddUIVertexTriangleStream", args = 1)]
    pub fn add_ui_vertex_triangle_stream(
        self,
        verts: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
    ) -> ();

    #[method(name = "GetUIVertexStream", args = 1)]
    pub fn get_ui_vertex_stream(
        self,
        stream: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-ui-vertexhelper")]
impl VertexHelper {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VertexHelper),
                ::core::stringify!(new),
            )
        });
        <Self as IVertexHelperMethods>::ctor(this);
        this
    }

    pub fn new_2(m: crate::unity_engine::mesh::Mesh) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VertexHelper),
                ::core::stringify!(new_2),
            )
        });
        <Self as IVertexHelperMethods>::ctor_2(this, m);
        this
    }
}
