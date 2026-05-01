
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_meshinfo/TMP_MeshInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TMP_MeshInfo {
    pub mesh: crate::unity_engine::mesh::Mesh,
    pub vertex_count: i32,
    pub vertices: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    pub normals: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    pub tangents: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    pub uvs0: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
    pub uvs2: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
    pub colors32: ::unity2::Array<crate::unity_engine::color32::Color32>,
    pub triangles: ::unity2::Array<i32>,
    pub material: crate::unity_engine::material::Material,
}

impl ::unity2::ClassIdentity for TMP_MeshInfo {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "TMP_MeshInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TMP_MeshInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "tm_pro-tmp_meshinfo")]
#[::unity2::methods(value)]
impl TMP_MeshInfo {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, mesh: crate::unity_engine::mesh::Mesh, size: i32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        size: i32,
        is_volumetric: bool,
    ) -> ();

    #[method(name = "ResizeMeshInfo", args = 1)]
    pub fn resize_mesh_info(self, size: i32) -> ();

    #[method(name = "ResizeMeshInfo", args = 2)]
    pub fn resize_mesh_info_2(self, size: i32, is_volumetric: bool) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear_2(self, upload_changes: bool) -> ();

    #[method(name = "ClearUnusedVertices", args = 0)]
    pub fn clear_unused_vertices(self) -> ();

    #[method(name = "ClearUnusedVertices", args = 1)]
    pub fn clear_unused_vertices_2(self, start_index: i32) -> ();

    #[method(name = "ClearUnusedVertices", args = 2)]
    pub fn clear_unused_vertices_3(self, start_index: i32, update_mesh: bool) -> ();

    #[method(name = "SortGeometry", args = 1)]
    pub fn sort_geometry(self, order: crate::tm_pro::vertexsortingorder::VertexSortingOrder) -> ();

    #[method(name = "SortGeometry", args = 1)]
    pub fn sort_geometry_2(
        self,
        sorting_order: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            i32,
        >,
    ) -> ();

    #[method(name = "SwapVertexData", args = 2)]
    pub fn swap_vertex_data(self, src: i32, dst: i32) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
