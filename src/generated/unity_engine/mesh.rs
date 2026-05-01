
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/mesh/Mesh_MeshData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Mesh_MeshData {
    pub m_ptr: ::unity2::IntPtr,
}

impl ::unity2::ClassIdentity for Mesh_MeshData {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Mesh.MeshData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mesh_MeshData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/mesh/Mesh_MeshDataArray.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Mesh_MeshDataArray {}

impl ::unity2::ClassIdentity for Mesh_MeshDataArray {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Mesh.MeshDataArray";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mesh_MeshDataArray {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-mesh")]
#[::unity2::methods(value)]
impl Mesh_MeshDataArray {
    #[method(name = "ApplyToMeshImpl", args = 3)]
    pub fn apply_to_mesh_impl(
        mesh: crate::unity_engine::mesh::Mesh,
        data: ::unity2::IntPtr,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "get_Length", args = 0)]
    pub fn get_length(self) -> i32;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "ApplyToMeshAndDispose", args = 2)]
    pub fn apply_to_mesh_and_dispose(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "ApplyToMeshesAndDispose", args = 2)]
    pub fn apply_to_meshes_and_dispose(
        self,
        meshes: ::unity2::Array<crate::unity_engine::mesh::Mesh>,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, mesh: crate::unity_engine::mesh::Mesh, check_read_write: bool) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        meshes: ::unity2::Array<crate::unity_engine::mesh::Mesh>,
        meshes_count: i32,
        check_read_write: bool,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, meshes_count: i32) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/mesh/Mesh.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Mesh")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct Mesh {}

#[cfg(feature = "unity_engine-mesh")]
#[::unity2::methods]
impl Mesh {
    #[method(name = "Internal_Create", args = 1)]
    pub fn internal_create(mono: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "FromInstanceID", args = 1)]
    pub fn from_instance_id(id: i32) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "get_indexFormat", args = 0)]
    pub fn get_index_format(self) -> crate::unity_engine::rendering::indexformat::IndexFormat;

    #[method(name = "set_indexFormat", args = 1)]
    pub fn set_index_format(
        self,
        value: crate::unity_engine::rendering::indexformat::IndexFormat,
    ) -> ();

    #[method(name = "GetTotalIndexCount", args = 0)]
    pub fn get_total_index_count(self) -> u32;

    #[method(name = "SetIndexBufferParams", args = 2)]
    pub fn set_index_buffer_params(
        self,
        index_count: i32,
        format: crate::unity_engine::rendering::indexformat::IndexFormat,
    ) -> ();

    #[method(name = "InternalSetIndexBufferData", args = 6)]
    pub fn internal_set_index_buffer_data(
        self,
        data: ::unity2::IntPtr,
        data_start: i32,
        mesh_buffer_start: i32,
        count: i32,
        elem_size: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "InternalSetIndexBufferDataFromArray", args = 6)]
    pub fn internal_set_index_buffer_data_from_array(
        self,
        data: ::unity2::IlInstance,
        data_start: i32,
        mesh_buffer_start: i32,
        count: i32,
        elem_size: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetVertexBufferParamsFromPtr", args = 3)]
    pub fn set_vertex_buffer_params_from_ptr(
        self,
        vertex_count: i32,
        attributes_ptr: ::unity2::IntPtr,
        attributes_count: i32,
    ) -> ();

    #[method(name = "SetVertexBufferParamsFromArray", args = 2)]
    pub fn set_vertex_buffer_params_from_array(
        self,
        vertex_count: i32,
        attributes: ::unity2::Array<
            crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor,
        >,
    ) -> ();

    #[method(name = "InternalSetVertexBufferData", args = 7)]
    pub fn internal_set_vertex_buffer_data(
        self,
        stream: i32,
        data: ::unity2::IntPtr,
        data_start: i32,
        mesh_buffer_start: i32,
        count: i32,
        elem_size: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "InternalSetVertexBufferDataFromArray", args = 7)]
    pub fn internal_set_vertex_buffer_data_from_array(
        self,
        stream: i32,
        data: ::unity2::IlInstance,
        data_start: i32,
        mesh_buffer_start: i32,
        count: i32,
        elem_size: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetVertexAttributesAlloc", args = 0)]
    pub fn get_vertex_attributes_alloc(self) -> ::unity2::IlInstance;

    #[method(name = "GetVertexAttributesArray", args = 1)]
    pub fn get_vertex_attributes_array(
        self,
        attributes: ::unity2::Array<
            crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor,
        >,
    ) -> i32;

    #[method(name = "GetVertexAttributesList", args = 1)]
    pub fn get_vertex_attributes_list(
        self,
        attributes: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor,
        >,
    ) -> i32;

    #[method(name = "GetVertexAttributeCountImpl", args = 0)]
    pub fn get_vertex_attribute_count_impl(self) -> i32;

    #[method(name = "GetVertexAttribute", args = 1)]
    pub fn get_vertex_attribute(
        self,
        index: i32,
    ) -> crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor;

    #[method(name = "GetIndexStartImpl", args = 1)]
    pub fn get_index_start_impl(self, submesh: i32) -> u32;

    #[method(name = "GetIndexCountImpl", args = 1)]
    pub fn get_index_count_impl(self, submesh: i32) -> u32;

    #[method(name = "GetTrianglesCountImpl", args = 1)]
    pub fn get_triangles_count_impl(self, submesh: i32) -> u32;

    #[method(name = "GetBaseVertexImpl", args = 1)]
    pub fn get_base_vertex_impl(self, submesh: i32) -> u32;

    #[method(name = "GetTrianglesImpl", args = 2)]
    pub fn get_triangles_impl(self, submesh: i32, apply_base_vertex: bool) -> ::unity2::Array<i32>;

    #[method(name = "GetIndicesImpl", args = 2)]
    pub fn get_indices_impl(self, submesh: i32, apply_base_vertex: bool) -> ::unity2::Array<i32>;

    #[method(name = "SetIndicesImpl", args = 8)]
    pub fn set_indices_impl(
        self,
        submesh: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        indices_format: crate::unity_engine::rendering::indexformat::IndexFormat,
        indices: ::unity2::IlInstance,
        array_start: i32,
        array_size: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetIndicesNativeArrayImpl", args = 8)]
    pub fn set_indices_native_array_impl(
        self,
        submesh: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        indices_format: crate::unity_engine::rendering::indexformat::IndexFormat,
        indices: ::unity2::IntPtr,
        array_start: i32,
        array_size: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "GetTrianglesNonAllocImpl", args = 3)]
    pub fn get_triangles_non_alloc_impl(
        self,
        values: ::unity2::Array<i32>,
        submesh: i32,
        apply_base_vertex: bool,
    ) -> ();

    #[method(name = "GetTrianglesNonAllocImpl16", args = 3)]
    pub fn get_triangles_non_alloc_impl16(
        self,
        values: ::unity2::Array<u16>,
        submesh: i32,
        apply_base_vertex: bool,
    ) -> ();

    #[method(name = "GetIndicesNonAllocImpl", args = 3)]
    pub fn get_indices_non_alloc_impl(
        self,
        values: ::unity2::Array<i32>,
        submesh: i32,
        apply_base_vertex: bool,
    ) -> ();

    #[method(name = "GetIndicesNonAllocImpl16", args = 3)]
    pub fn get_indices_non_alloc_impl16(
        self,
        values: ::unity2::Array<u16>,
        submesh: i32,
        apply_base_vertex: bool,
    ) -> ();

    #[method(name = "PrintErrorCantAccessChannel", args = 1)]
    pub fn print_error_cant_access_channel(
        self,
        ch: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
    ) -> ();

    #[method(name = "HasVertexAttribute", args = 1)]
    pub fn has_vertex_attribute(
        self,
        attr: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
    ) -> bool;

    #[method(name = "GetVertexAttributeDimension", args = 1)]
    pub fn get_vertex_attribute_dimension(
        self,
        attr: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
    ) -> i32;

    #[method(name = "GetVertexAttributeFormat", args = 1)]
    pub fn get_vertex_attribute_format(
        self,
        attr: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
    ) -> crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat;

    #[method(name = "SetArrayForChannelImpl", args = 8)]
    pub fn set_array_for_channel_impl(
        self,
        channel: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
        format: crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat,
        dim: i32,
        values: ::unity2::IlInstance,
        array_size: i32,
        values_start: i32,
        values_count: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetNativeArrayForChannelImpl", args = 8)]
    pub fn set_native_array_for_channel_impl(
        self,
        channel: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
        format: crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat,
        dim: i32,
        values: ::unity2::IntPtr,
        array_size: i32,
        values_start: i32,
        values_count: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetAllocArrayFromChannelImpl", args = 3)]
    pub fn get_alloc_array_from_channel_impl(
        self,
        channel: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
        format: crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat,
        dim: i32,
    ) -> ::unity2::IlInstance;

    #[method(name = "GetArrayFromChannelImpl", args = 4)]
    pub fn get_array_from_channel_impl(
        self,
        channel: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
        format: crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat,
        dim: i32,
        values: ::unity2::IlInstance,
    ) -> ();

    #[method(name = "get_vertexBufferCount", args = 0)]
    pub fn get_vertex_buffer_count(self) -> i32;

    #[method(name = "GetNativeVertexBufferPtr", args = 1)]
    pub fn get_native_vertex_buffer_ptr(self, index: i32) -> ::unity2::IntPtr;

    #[method(name = "GetNativeIndexBufferPtr", args = 0)]
    pub fn get_native_index_buffer_ptr(self) -> ::unity2::IntPtr;

    #[method(name = "get_blendShapeCount", args = 0)]
    pub fn get_blend_shape_count(self) -> i32;

    #[method(name = "ClearBlendShapes", args = 0)]
    pub fn clear_blend_shapes(self) -> ();

    #[method(name = "GetBlendShapeName", args = 1)]
    pub fn get_blend_shape_name(self, shape_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetBlendShapeIndex", args = 1)]
    pub fn get_blend_shape_index(self, blend_shape_name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetBlendShapeFrameCount", args = 1)]
    pub fn get_blend_shape_frame_count(self, shape_index: i32) -> i32;

    #[method(name = "GetBlendShapeFrameWeight", args = 2)]
    pub fn get_blend_shape_frame_weight(self, shape_index: i32, frame_index: i32) -> f32;

    #[method(name = "GetBlendShapeFrameVertices", args = 5)]
    pub fn get_blend_shape_frame_vertices(
        self,
        shape_index: i32,
        frame_index: i32,
        delta_vertices: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        delta_normals: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        delta_tangents: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> ();

    #[method(name = "AddBlendShapeFrame", args = 5)]
    pub fn add_blend_shape_frame(
        self,
        shape_name: ::unity2::Il2CppString,
        frame_weight: f32,
        delta_vertices: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        delta_normals: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        delta_tangents: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> ();

    #[method(name = "HasBoneWeights", args = 0)]
    pub fn has_bone_weights(self) -> bool;

    #[method(name = "GetBoneWeightsImpl", args = 0)]
    pub fn get_bone_weights_impl(
        self,
    ) -> ::unity2::Array<crate::unity_engine::boneweight::BoneWeight>;

    #[method(name = "SetBoneWeightsImpl", args = 1)]
    pub fn set_bone_weights_impl(
        self,
        weights: ::unity2::Array<crate::unity_engine::boneweight::BoneWeight>,
    ) -> ();

    #[method(name = "InternalSetBoneWeights", args = 4)]
    pub fn internal_set_bone_weights(
        self,
        bones_per_vertex: ::unity2::IntPtr,
        bones_per_vertex_size: i32,
        weights: ::unity2::IntPtr,
        weights_size: i32,
    ) -> ();

    #[method(name = "GetAllBoneWeightsArraySize", args = 0)]
    pub fn get_all_bone_weights_array_size(self) -> i32;

    #[method(name = "GetAllBoneWeightsArray", args = 0)]
    pub fn get_all_bone_weights_array(self) -> ::unity2::IntPtr;

    #[method(name = "GetBonesPerVertexArray", args = 0)]
    pub fn get_bones_per_vertex_array(self) -> ::unity2::IntPtr;

    #[method(name = "GetBindposeCount", args = 0)]
    pub fn get_bindpose_count(self) -> i32;

    #[method(name = "get_bindposes", args = 0)]
    pub fn get_bindposes(self) -> ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>;

    #[method(name = "set_bindposes", args = 1)]
    pub fn set_bindposes(
        self,
        value: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    ) -> ();

    #[method(name = "GetBoneWeightsNonAllocImpl", args = 1)]
    pub fn get_bone_weights_non_alloc_impl(
        self,
        values: ::unity2::Array<crate::unity_engine::boneweight::BoneWeight>,
    ) -> ();

    #[method(name = "GetBindposesNonAllocImpl", args = 1)]
    pub fn get_bindposes_non_alloc_impl(
        self,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    ) -> ();

    #[method(name = "get_isReadable", args = 0)]
    pub fn get_is_readable(self) -> bool;

    #[method(name = "get_canAccess", args = 0)]
    pub fn get_can_access(self) -> bool;

    #[method(name = "get_vertexCount", args = 0)]
    pub fn get_vertex_count(self) -> i32;

    #[method(name = "get_subMeshCount", args = 0)]
    pub fn get_sub_mesh_count(self) -> i32;

    #[method(name = "set_subMeshCount", args = 1)]
    pub fn set_sub_mesh_count(self, value: i32) -> ();

    #[method(name = "SetSubMesh", args = 3)]
    pub fn set_sub_mesh(
        self,
        index: i32,
        desc: crate::unity_engine::rendering::submeshdescriptor::SubMeshDescriptor,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetSubMesh", args = 1)]
    pub fn get_sub_mesh(
        self,
        index: i32,
    ) -> crate::unity_engine::rendering::submeshdescriptor::SubMeshDescriptor;

    #[method(name = "SetAllSubMeshesAtOnceFromArray", args = 4)]
    pub fn set_all_sub_meshes_at_once_from_array(
        self,
        desc: ::unity2::Array<crate::unity_engine::rendering::submeshdescriptor::SubMeshDescriptor>,
        start: i32,
        count: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetAllSubMeshesAtOnceFromNativeArray", args = 4)]
    pub fn set_all_sub_meshes_at_once_from_native_array(
        self,
        desc: ::unity2::IntPtr,
        start: i32,
        count: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "get_bounds", args = 0)]
    pub fn get_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "set_bounds", args = 1)]
    pub fn set_bounds(self, value: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "ClearImpl", args = 1)]
    pub fn clear_impl(self, keep_vertex_layout: bool) -> ();

    #[method(name = "RecalculateBoundsImpl", args = 1)]
    pub fn recalculate_bounds_impl(
        self,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "RecalculateNormalsImpl", args = 1)]
    pub fn recalculate_normals_impl(
        self,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "RecalculateTangentsImpl", args = 1)]
    pub fn recalculate_tangents_impl(
        self,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "MarkDynamicImpl", args = 0)]
    pub fn mark_dynamic_impl(self) -> ();

    #[method(name = "MarkModified", args = 0)]
    pub fn mark_modified(self) -> ();

    #[method(name = "UploadMeshDataImpl", args = 1)]
    pub fn upload_mesh_data_impl(self, mark_no_longer_readable: bool) -> ();

    #[method(name = "GetTopologyImpl", args = 1)]
    pub fn get_topology_impl(self, submesh: i32)
        -> crate::unity_engine::meshtopology::MeshTopology;

    #[method(name = "RecalculateUVDistributionMetricImpl", args = 2)]
    pub fn recalculate_uv_distribution_metric_impl(
        self,
        uv_set_index: i32,
        uv_area_threshold: f32,
    ) -> ();

    #[method(name = "RecalculateUVDistributionMetricsImpl", args = 1)]
    pub fn recalculate_uv_distribution_metrics_impl(self, uv_area_threshold: f32) -> ();

    #[method(name = "GetUVDistributionMetric", args = 1)]
    pub fn get_uv_distribution_metric(self, uv_set_index: i32) -> f32;

    #[method(name = "CombineMeshesImpl", args = 4)]
    pub fn combine_meshes_impl(
        self,
        combine: ::unity2::Array<crate::unity_engine::combineinstance::CombineInstance>,
        merge_sub_meshes: bool,
        use_matrices: bool,
        has_lightmap_data: bool,
    ) -> ();

    #[method(name = "OptimizeImpl", args = 0)]
    pub fn optimize_impl(self) -> ();

    #[method(name = "OptimizeIndexBuffersImpl", args = 0)]
    pub fn optimize_index_buffers_impl(self) -> ();

    #[method(name = "OptimizeReorderVertexBufferImpl", args = 0)]
    pub fn optimize_reorder_vertex_buffer_impl(self) -> ();

    #[method(name = "GetUVChannel", args = 1)]
    pub fn get_uv_channel(
        uv_index: i32,
    ) -> crate::unity_engine::rendering::vertexattribute::VertexAttribute;

    #[method(name = "DefaultDimensionForChannel", args = 1)]
    pub fn default_dimension_for_channel(
        channel: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
    ) -> i32;

    #[method(name = "SetSizedArrayForChannel", args = 8)]
    pub fn set_sized_array_for_channel(
        self,
        channel: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
        format: crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat,
        dim: i32,
        values: ::unity2::IlInstance,
        values_array_length: i32,
        values_start: i32,
        values_count: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetSizedNativeArrayForChannel", args = 8)]
    pub fn set_sized_native_array_for_channel(
        self,
        channel: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
        format: crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat,
        dim: i32,
        values: ::unity2::IntPtr,
        values_array_length: i32,
        values_start: i32,
        values_count: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "get_vertices", args = 0)]
    pub fn get_vertices(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "set_vertices", args = 1)]
    pub fn set_vertices(self, value: ::unity2::Array<crate::unity_engine::vector3::Vector3>) -> ();

    #[method(name = "get_normals", args = 0)]
    pub fn get_normals(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "set_normals", args = 1)]
    pub fn set_normals(self, value: ::unity2::Array<crate::unity_engine::vector3::Vector3>) -> ();

    #[method(name = "get_tangents", args = 0)]
    pub fn get_tangents(self) -> ::unity2::Array<crate::unity_engine::vector4::Vector4>;

    #[method(name = "set_tangents", args = 1)]
    pub fn set_tangents(self, value: ::unity2::Array<crate::unity_engine::vector4::Vector4>) -> ();

    #[method(name = "get_uv", args = 0)]
    pub fn get_uv(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "set_uv", args = 1)]
    pub fn set_uv(self, value: ::unity2::Array<crate::unity_engine::vector2::Vector2>) -> ();

    #[method(name = "get_uv2", args = 0)]
    pub fn get_uv2(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "set_uv2", args = 1)]
    pub fn set_uv2(self, value: ::unity2::Array<crate::unity_engine::vector2::Vector2>) -> ();

    #[method(name = "get_uv3", args = 0)]
    pub fn get_uv3(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "set_uv3", args = 1)]
    pub fn set_uv3(self, value: ::unity2::Array<crate::unity_engine::vector2::Vector2>) -> ();

    #[method(name = "get_uv4", args = 0)]
    pub fn get_uv4(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "set_uv4", args = 1)]
    pub fn set_uv4(self, value: ::unity2::Array<crate::unity_engine::vector2::Vector2>) -> ();

    #[method(name = "get_uv5", args = 0)]
    pub fn get_uv5(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "set_uv5", args = 1)]
    pub fn set_uv5(self, value: ::unity2::Array<crate::unity_engine::vector2::Vector2>) -> ();

    #[method(name = "get_uv6", args = 0)]
    pub fn get_uv6(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "set_uv6", args = 1)]
    pub fn set_uv6(self, value: ::unity2::Array<crate::unity_engine::vector2::Vector2>) -> ();

    #[method(name = "get_uv7", args = 0)]
    pub fn get_uv7(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "set_uv7", args = 1)]
    pub fn set_uv7(self, value: ::unity2::Array<crate::unity_engine::vector2::Vector2>) -> ();

    #[method(name = "get_uv8", args = 0)]
    pub fn get_uv8(self) -> ::unity2::Array<crate::unity_engine::vector2::Vector2>;

    #[method(name = "set_uv8", args = 1)]
    pub fn set_uv8(self, value: ::unity2::Array<crate::unity_engine::vector2::Vector2>) -> ();

    #[method(name = "get_colors", args = 0)]
    pub fn get_colors(self) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "set_colors", args = 1)]
    pub fn set_colors(self, value: ::unity2::Array<crate::unity_engine::color::Color>) -> ();

    #[method(name = "get_colors32", args = 0)]
    pub fn get_colors32(self) -> ::unity2::Array<crate::unity_engine::color32::Color32>;

    #[method(name = "set_colors32", args = 1)]
    pub fn set_colors32(self, value: ::unity2::Array<crate::unity_engine::color32::Color32>) -> ();

    #[method(name = "GetVertices", args = 1)]
    pub fn get_vertices_2(
        self,
        vertices: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
    ) -> ();

    #[method(name = "SetVertices", args = 1)]
    pub fn set_vertices_2(
        self,
        in_vertices: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
    ) -> ();

    #[method(name = "SetVertices", args = 3)]
    pub fn set_vertices_3(
        self,
        in_vertices: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetVertices", args = 4)]
    pub fn set_vertices_4(
        self,
        in_vertices: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetVertices", args = 3)]
    pub fn set_vertices_5(
        self,
        in_vertices: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetVertices", args = 4)]
    pub fn set_vertices_6(
        self,
        in_vertices: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetNormals", args = 1)]
    pub fn get_normals_2(
        self,
        normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
    ) -> ();

    #[method(name = "SetNormals", args = 1)]
    pub fn set_normals_2(
        self,
        in_normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
    ) -> ();

    #[method(name = "SetNormals", args = 3)]
    pub fn set_normals_3(
        self,
        in_normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetNormals", args = 4)]
    pub fn set_normals_4(
        self,
        in_normals: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetNormals", args = 3)]
    pub fn set_normals_5(
        self,
        in_normals: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetNormals", args = 4)]
    pub fn set_normals_6(
        self,
        in_normals: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetTangents", args = 1)]
    pub fn get_tangents_2(
        self,
        tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "SetTangents", args = 1)]
    pub fn set_tangents_2(
        self,
        in_tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "SetTangents", args = 3)]
    pub fn set_tangents_3(
        self,
        in_tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetTangents", args = 4)]
    pub fn set_tangents_4(
        self,
        in_tangents: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetTangents", args = 3)]
    pub fn set_tangents_5(
        self,
        in_tangents: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetTangents", args = 4)]
    pub fn set_tangents_6(
        self,
        in_tangents: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetColors", args = 1)]
    pub fn get_colors_2(
        self,
        colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
    ) -> ();

    #[method(name = "SetColors", args = 1)]
    pub fn set_colors_2(
        self,
        in_colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
    ) -> ();

    #[method(name = "SetColors", args = 3)]
    pub fn set_colors_3(
        self,
        in_colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetColors", args = 4)]
    pub fn set_colors_4(
        self,
        in_colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetColors", args = 3)]
    pub fn set_colors_5(
        self,
        in_colors: ::unity2::Array<crate::unity_engine::color::Color>,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetColors", args = 4)]
    pub fn set_colors_6(
        self,
        in_colors: ::unity2::Array<crate::unity_engine::color::Color>,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetColors", args = 1)]
    pub fn get_colors_3(
        self,
        colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
    ) -> ();

    #[method(name = "SetColors", args = 1)]
    pub fn set_colors_7(
        self,
        in_colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
    ) -> ();

    #[method(name = "SetColors", args = 3)]
    pub fn set_colors_8(
        self,
        in_colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetColors", args = 4)]
    pub fn set_colors_9(
        self,
        in_colors: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color32::Color32,
        >,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetColors", args = 1)]
    pub fn set_colors_10(
        self,
        in_colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
    ) -> ();

    #[method(name = "SetColors", args = 3)]
    pub fn set_colors_11(
        self,
        in_colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetColors", args = 4)]
    pub fn set_colors_12(
        self,
        in_colors: ::unity2::Array<crate::unity_engine::color32::Color32>,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetUVs", args = 2)]
    pub fn set_u_vs(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector2::Vector2,
        >,
    ) -> ();

    #[method(name = "SetUVs", args = 2)]
    pub fn set_u_vs_2(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
    ) -> ();

    #[method(name = "SetUVs", args = 2)]
    pub fn set_u_vs_3(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "SetUVs", args = 4)]
    pub fn set_u_vs_4(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector2::Vector2,
        >,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetUVs", args = 5)]
    pub fn set_u_vs_5(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector2::Vector2,
        >,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetUVs", args = 4)]
    pub fn set_u_vs_6(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetUVs", args = 5)]
    pub fn set_u_vs_7(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetUVs", args = 4)]
    pub fn set_u_vs_8(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetUVs", args = 5)]
    pub fn set_u_vs_9(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetUvsImpl", args = 6)]
    pub fn set_uvs_impl(
        self,
        uv_index: i32,
        dim: i32,
        uvs: ::unity2::IlInstance,
        array_start: i32,
        array_size: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetUVs", args = 2)]
    pub fn set_u_vs_10(
        self,
        channel: i32,
        uvs: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
    ) -> ();

    #[method(name = "SetUVs", args = 2)]
    pub fn set_u_vs_11(
        self,
        channel: i32,
        uvs: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> ();

    #[method(name = "SetUVs", args = 2)]
    pub fn set_u_vs_12(
        self,
        channel: i32,
        uvs: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    ) -> ();

    #[method(name = "SetUVs", args = 4)]
    pub fn set_u_vs_13(
        self,
        channel: i32,
        uvs: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetUVs", args = 5)]
    pub fn set_u_vs_14(
        self,
        channel: i32,
        uvs: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetUVs", args = 4)]
    pub fn set_u_vs_15(
        self,
        channel: i32,
        uvs: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetUVs", args = 5)]
    pub fn set_u_vs_16(
        self,
        channel: i32,
        uvs: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetUVs", args = 4)]
    pub fn set_u_vs_17(
        self,
        channel: i32,
        uvs: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
        start: i32,
        length: i32,
    ) -> ();

    #[method(name = "SetUVs", args = 5)]
    pub fn set_u_vs_18(
        self,
        channel: i32,
        uvs: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
        start: i32,
        length: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetUVs", args = 2)]
    pub fn get_u_vs(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector2::Vector2,
        >,
    ) -> ();

    #[method(name = "GetUVs", args = 2)]
    pub fn get_u_vs_2(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector3::Vector3,
        >,
    ) -> ();

    #[method(name = "GetUVs", args = 2)]
    pub fn get_u_vs_3(
        self,
        channel: i32,
        uvs: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "get_vertexAttributeCount", args = 0)]
    pub fn get_vertex_attribute_count(self) -> i32;

    #[method(name = "GetVertexAttributes", args = 0)]
    pub fn get_vertex_attributes(
        self,
    ) -> ::unity2::Array<
        crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor,
    >;

    #[method(name = "GetVertexAttributes", args = 1)]
    pub fn get_vertex_attributes_2(
        self,
        attributes: ::unity2::Array<
            crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor,
        >,
    ) -> i32;

    #[method(name = "GetVertexAttributes", args = 1)]
    pub fn get_vertex_attributes_3(
        self,
        attributes: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor,
        >,
    ) -> i32;

    #[method(name = "SetVertexBufferParams", args = 2)]
    pub fn set_vertex_buffer_params(
        self,
        vertex_count: i32,
        attributes: ::unity2::Array<
            crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor,
        >,
    ) -> ();

    #[method(name = "AcquireReadOnlyMeshData", args = 1)]
    pub fn acquire_read_only_mesh_data(
        mesh: crate::unity_engine::mesh::Mesh,
    ) -> crate::unity_engine::mesh::Mesh_MeshDataArray;

    #[method(name = "AcquireReadOnlyMeshData", args = 1)]
    pub fn acquire_read_only_mesh_data_2(
        meshes: ::unity2::Array<crate::unity_engine::mesh::Mesh>,
    ) -> crate::unity_engine::mesh::Mesh_MeshDataArray;

    #[method(name = "AcquireReadOnlyMeshData", args = 1)]
    pub fn acquire_read_only_mesh_data_3(
        meshes: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::mesh::Mesh,
        >,
    ) -> crate::unity_engine::mesh::Mesh_MeshDataArray;

    #[method(name = "AllocateWritableMeshData", args = 1)]
    pub fn allocate_writable_mesh_data(
        mesh_count: i32,
    ) -> crate::unity_engine::mesh::Mesh_MeshDataArray;

    #[method(name = "ApplyAndDisposeWritableMeshData", args = 3)]
    pub fn apply_and_dispose_writable_mesh_data(
        data: crate::unity_engine::mesh::Mesh_MeshDataArray,
        mesh: crate::unity_engine::mesh::Mesh,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "ApplyAndDisposeWritableMeshData", args = 3)]
    pub fn apply_and_dispose_writable_mesh_data_2(
        data: crate::unity_engine::mesh::Mesh_MeshDataArray,
        meshes: ::unity2::Array<crate::unity_engine::mesh::Mesh>,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "ApplyAndDisposeWritableMeshData", args = 3)]
    pub fn apply_and_dispose_writable_mesh_data_3(
        data: crate::unity_engine::mesh::Mesh_MeshDataArray,
        meshes: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::mesh::Mesh,
        >,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "PrintErrorCantAccessIndices", args = 0)]
    pub fn print_error_cant_access_indices(self) -> ();

    #[method(name = "CheckCanAccessSubmesh", args = 2)]
    pub fn check_can_access_submesh(self, submesh: i32, error_about_triangles: bool) -> bool;

    #[method(name = "CheckCanAccessSubmeshTriangles", args = 1)]
    pub fn check_can_access_submesh_triangles(self, submesh: i32) -> bool;

    #[method(name = "CheckCanAccessSubmeshIndices", args = 1)]
    pub fn check_can_access_submesh_indices(self, submesh: i32) -> bool;

    #[method(name = "get_triangles", args = 0)]
    pub fn get_triangles(self) -> ::unity2::Array<i32>;

    #[method(name = "set_triangles", args = 1)]
    pub fn set_triangles(self, value: ::unity2::Array<i32>) -> ();

    #[method(name = "GetTriangles", args = 1)]
    pub fn get_triangles_2(self, submesh: i32) -> ::unity2::Array<i32>;

    #[method(name = "GetTriangles", args = 2)]
    pub fn get_triangles_3(self, submesh: i32, apply_base_vertex: bool) -> ::unity2::Array<i32>;

    #[method(name = "GetTriangles", args = 2)]
    pub fn get_triangles_4(
        self,
        triangles: crate::system::collections::generic::list_1::List_1<i32>,
        submesh: i32,
    ) -> ();

    #[method(name = "GetTriangles", args = 3)]
    pub fn get_triangles_5(
        self,
        triangles: crate::system::collections::generic::list_1::List_1<i32>,
        submesh: i32,
        apply_base_vertex: bool,
    ) -> ();

    #[method(name = "GetTriangles", args = 3)]
    pub fn get_triangles_6(
        self,
        triangles: crate::system::collections::generic::list_1::List_1<u16>,
        submesh: i32,
        apply_base_vertex: bool,
    ) -> ();

    #[method(name = "GetIndices", args = 1)]
    pub fn get_indices(self, submesh: i32) -> ::unity2::Array<i32>;

    #[method(name = "GetIndices", args = 2)]
    pub fn get_indices_2(self, submesh: i32, apply_base_vertex: bool) -> ::unity2::Array<i32>;

    #[method(name = "GetIndices", args = 2)]
    pub fn get_indices_3(
        self,
        indices: crate::system::collections::generic::list_1::List_1<i32>,
        submesh: i32,
    ) -> ();

    #[method(name = "GetIndices", args = 3)]
    pub fn get_indices_4(
        self,
        indices: crate::system::collections::generic::list_1::List_1<i32>,
        submesh: i32,
        apply_base_vertex: bool,
    ) -> ();

    #[method(name = "GetIndices", args = 3)]
    pub fn get_indices_5(
        self,
        indices: crate::system::collections::generic::list_1::List_1<u16>,
        submesh: i32,
        apply_base_vertex: bool,
    ) -> ();

    #[method(name = "GetIndexStart", args = 1)]
    pub fn get_index_start(self, submesh: i32) -> u32;

    #[method(name = "GetIndexCount", args = 1)]
    pub fn get_index_count(self, submesh: i32) -> u32;

    #[method(name = "GetBaseVertex", args = 1)]
    pub fn get_base_vertex(self, submesh: i32) -> u32;

    #[method(name = "CheckIndicesArrayRange", args = 3)]
    pub fn check_indices_array_range(self, values_length: i32, start: i32, length: i32) -> ();

    #[method(name = "SetTrianglesImpl", args = 8)]
    pub fn set_triangles_impl(
        self,
        submesh: i32,
        indices_format: crate::unity_engine::rendering::indexformat::IndexFormat,
        triangles: ::unity2::IlInstance,
        triangles_array_length: i32,
        start: i32,
        length: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetTriangles", args = 2)]
    pub fn set_triangles_2(self, triangles: ::unity2::Array<i32>, submesh: i32) -> ();

    #[method(name = "SetTriangles", args = 3)]
    pub fn set_triangles_3(
        self,
        triangles: ::unity2::Array<i32>,
        submesh: i32,
        calculate_bounds: bool,
    ) -> ();

    #[method(name = "SetTriangles", args = 4)]
    pub fn set_triangles_4(
        self,
        triangles: ::unity2::Array<i32>,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetTriangles", args = 6)]
    pub fn set_triangles_5(
        self,
        triangles: ::unity2::Array<i32>,
        triangles_start: i32,
        triangles_length: i32,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetTriangles", args = 4)]
    pub fn set_triangles_6(
        self,
        triangles: ::unity2::Array<u16>,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetTriangles", args = 6)]
    pub fn set_triangles_7(
        self,
        triangles: ::unity2::Array<u16>,
        triangles_start: i32,
        triangles_length: i32,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetTriangles", args = 2)]
    pub fn set_triangles_8(
        self,
        triangles: crate::system::collections::generic::list_1::List_1<i32>,
        submesh: i32,
    ) -> ();

    #[method(name = "SetTriangles", args = 3)]
    pub fn set_triangles_9(
        self,
        triangles: crate::system::collections::generic::list_1::List_1<i32>,
        submesh: i32,
        calculate_bounds: bool,
    ) -> ();

    #[method(name = "SetTriangles", args = 4)]
    pub fn set_triangles_10(
        self,
        triangles: crate::system::collections::generic::list_1::List_1<i32>,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetTriangles", args = 6)]
    pub fn set_triangles_11(
        self,
        triangles: crate::system::collections::generic::list_1::List_1<i32>,
        triangles_start: i32,
        triangles_length: i32,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetTriangles", args = 4)]
    pub fn set_triangles_12(
        self,
        triangles: crate::system::collections::generic::list_1::List_1<u16>,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetTriangles", args = 6)]
    pub fn set_triangles_13(
        self,
        triangles: crate::system::collections::generic::list_1::List_1<u16>,
        triangles_start: i32,
        triangles_length: i32,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetIndices", args = 3)]
    pub fn set_indices(
        self,
        indices: ::unity2::Array<i32>,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
    ) -> ();

    #[method(name = "SetIndices", args = 4)]
    pub fn set_indices_2(
        self,
        indices: ::unity2::Array<i32>,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
        calculate_bounds: bool,
    ) -> ();

    #[method(name = "SetIndices", args = 5)]
    pub fn set_indices_3(
        self,
        indices: ::unity2::Array<i32>,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetIndices", args = 7)]
    pub fn set_indices_4(
        self,
        indices: ::unity2::Array<i32>,
        indices_start: i32,
        indices_length: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetIndices", args = 5)]
    pub fn set_indices_5(
        self,
        indices: ::unity2::Array<u16>,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetIndices", args = 7)]
    pub fn set_indices_6(
        self,
        indices: ::unity2::Array<u16>,
        indices_start: i32,
        indices_length: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetIndices", args = 5)]
    pub fn set_indices_7(
        self,
        indices: crate::system::collections::generic::list_1::List_1<i32>,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetIndices", args = 7)]
    pub fn set_indices_8(
        self,
        indices: crate::system::collections::generic::list_1::List_1<i32>,
        indices_start: i32,
        indices_length: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetIndices", args = 5)]
    pub fn set_indices_9(
        self,
        indices: crate::system::collections::generic::list_1::List_1<u16>,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetIndices", args = 7)]
    pub fn set_indices_10(
        self,
        indices: crate::system::collections::generic::list_1::List_1<u16>,
        indices_start: i32,
        indices_length: i32,
        topology: crate::unity_engine::meshtopology::MeshTopology,
        submesh: i32,
        calculate_bounds: bool,
        base_vertex: i32,
    ) -> ();

    #[method(name = "SetSubMeshes", args = 4)]
    pub fn set_sub_meshes(
        self,
        desc: ::unity2::Array<crate::unity_engine::rendering::submeshdescriptor::SubMeshDescriptor>,
        start: i32,
        count: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetSubMeshes", args = 2)]
    pub fn set_sub_meshes_2(
        self,
        desc: ::unity2::Array<crate::unity_engine::rendering::submeshdescriptor::SubMeshDescriptor>,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetSubMeshes", args = 4)]
    pub fn set_sub_meshes_3(
        self,
        desc: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rendering::submeshdescriptor::SubMeshDescriptor,
        >,
        start: i32,
        count: i32,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "SetSubMeshes", args = 2)]
    pub fn set_sub_meshes_4(
        self,
        desc: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::rendering::submeshdescriptor::SubMeshDescriptor,
        >,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetBindposes", args = 1)]
    pub fn get_bindposes_2(
        self,
        bindposes: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "GetBoneWeights", args = 1)]
    pub fn get_bone_weights(
        self,
        bone_weights: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::boneweight::BoneWeight,
        >,
    ) -> ();

    #[method(name = "get_boneWeights", args = 0)]
    pub fn get_bone_weights_2(self)
        -> ::unity2::Array<crate::unity_engine::boneweight::BoneWeight>;

    #[method(name = "set_boneWeights", args = 1)]
    pub fn set_bone_weights(
        self,
        value: ::unity2::Array<crate::unity_engine::boneweight::BoneWeight>,
    ) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, keep_vertex_layout: bool) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear_2(self) -> ();

    #[method(name = "RecalculateBounds", args = 0)]
    pub fn recalculate_bounds(self) -> ();

    #[method(name = "RecalculateNormals", args = 0)]
    pub fn recalculate_normals(self) -> ();

    #[method(name = "RecalculateTangents", args = 0)]
    pub fn recalculate_tangents(self) -> ();

    #[method(name = "RecalculateBounds", args = 1)]
    pub fn recalculate_bounds_2(
        self,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "RecalculateNormals", args = 1)]
    pub fn recalculate_normals_2(
        self,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "RecalculateTangents", args = 1)]
    pub fn recalculate_tangents_2(
        self,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "RecalculateUVDistributionMetric", args = 2)]
    pub fn recalculate_uv_distribution_metric(
        self,
        uv_set_index: i32,
        uv_area_threshold: f32,
    ) -> ();

    #[method(name = "RecalculateUVDistributionMetrics", args = 1)]
    pub fn recalculate_uv_distribution_metrics(self, uv_area_threshold: f32) -> ();

    #[method(name = "MarkDynamic", args = 0)]
    pub fn mark_dynamic(self) -> ();

    #[method(name = "UploadMeshData", args = 1)]
    pub fn upload_mesh_data(self, mark_no_longer_readable: bool) -> ();

    #[method(name = "Optimize", args = 0)]
    pub fn optimize(self) -> ();

    #[method(name = "OptimizeIndexBuffers", args = 0)]
    pub fn optimize_index_buffers(self) -> ();

    #[method(name = "OptimizeReorderVertexBuffer", args = 0)]
    pub fn optimize_reorder_vertex_buffer(self) -> ();

    #[method(name = "GetTopology", args = 1)]
    pub fn get_topology(self, submesh: i32) -> crate::unity_engine::meshtopology::MeshTopology;

    #[method(name = "CombineMeshes", args = 4)]
    pub fn combine_meshes(
        self,
        combine: ::unity2::Array<crate::unity_engine::combineinstance::CombineInstance>,
        merge_sub_meshes: bool,
        use_matrices: bool,
        has_lightmap_data: bool,
    ) -> ();

    #[method(name = "CombineMeshes", args = 3)]
    pub fn combine_meshes_2(
        self,
        combine: ::unity2::Array<crate::unity_engine::combineinstance::CombineInstance>,
        merge_sub_meshes: bool,
        use_matrices: bool,
    ) -> ();

    #[method(name = "CombineMeshes", args = 2)]
    pub fn combine_meshes_3(
        self,
        combine: ::unity2::Array<crate::unity_engine::combineinstance::CombineInstance>,
        merge_sub_meshes: bool,
    ) -> ();

    #[method(name = "CombineMeshes", args = 1)]
    pub fn combine_meshes_4(
        self,
        combine: ::unity2::Array<crate::unity_engine::combineinstance::CombineInstance>,
    ) -> ();

    #[method(name = "GetVertexAttribute_Injected", args = 2)]
    pub fn get_vertex_attribute_injected(
        self,
        index: i32,
        ret: crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor,
    ) -> ();

    #[method(name = "SetSubMesh_Injected", args = 3)]
    pub fn set_sub_mesh_injected(
        self,
        index: i32,
        desc: crate::unity_engine::rendering::submeshdescriptor::SubMeshDescriptor,
        flags: crate::unity_engine::rendering::meshupdateflags::MeshUpdateFlags,
    ) -> ();

    #[method(name = "GetSubMesh_Injected", args = 2)]
    pub fn get_sub_mesh_injected(
        self,
        index: i32,
        ret: crate::unity_engine::rendering::submeshdescriptor::SubMeshDescriptor,
    ) -> ();

    #[method(name = "get_bounds_Injected", args = 1)]
    pub fn get_bounds_injected(self, ret: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "set_bounds_Injected", args = 1)]
    pub fn set_bounds_injected(self, value: crate::unity_engine::bounds::Bounds) -> ();
}

#[cfg(feature = "unity_engine-mesh")]
impl Mesh {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Mesh),
                ::core::stringify!(new),
            )
        });
        <Self as IMeshMethods>::ctor(this);
        this
    }
}
