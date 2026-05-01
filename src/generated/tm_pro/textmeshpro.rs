
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::tm_pro::tmp_text::ITMP_Text;
use crate::tm_pro::tmp_text::TMP_Text;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::ui::graphic::Graphic;
use crate::unity_engine::ui::graphic::IGraphic;
use crate::unity_engine::ui::maskablegraphic::IMaskableGraphic;
use crate::unity_engine::ui::maskablegraphic::MaskableGraphic;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/textmeshpro/TextMeshPro.md")))]
#[::unity2::class(namespace = "TMPro", name = "TextMeshPro")]
#[parent(crate::tm_pro::tmp_text::TMP_Text)]
pub struct TextMeshPro {
    #[rename(name = "m_hasFontAssetChanged")]
    pub m_has_font_asset_changed: bool,
    #[rename(name = "m_previousLossyScaleY")]
    pub m_previous_lossy_scale_y: f32,
    #[rename(name = "m_renderer")]
    pub m_renderer: crate::unity_engine::renderer::Renderer,
    #[rename(name = "m_meshFilter")]
    pub m_mesh_filter: crate::unity_engine::meshfilter::MeshFilter,
    #[rename(name = "m_isFirstAllocation")]
    pub m_is_first_allocation: bool,
    #[rename(name = "m_max_characters")]
    pub m_max_characters: i32,
    #[rename(name = "m_max_numberOfLines")]
    pub m_max_number_of_lines: i32,
    #[rename(name = "m_subTextObjects")]
    pub m_sub_text_objects: ::unity2::Array<crate::tm_pro::tmp_submesh::TMP_SubMesh>,
    #[rename(name = "m_maskType")]
    pub m_mask_type: crate::tm_pro::maskingtypes::MaskingTypes,
    #[rename(name = "m_EnvMapMatrix")]
    pub m_env_map_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    #[rename(name = "m_RectTransformCorners")]
    pub m_rect_transform_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_isRegisteredForEvents")]
    pub m_is_registered_for_events: bool,
    #[rename(name = "_SortingLayer")]
    pub sorting_layer: i32,
    #[rename(name = "OnPreRenderText")]
    pub on_pre_render_text:
        crate::system::action_1::Action_1<crate::tm_pro::tmp_textinfo::TMP_TextInfo>,
    #[rename(name = "m_currentAutoSizeMode")]
    pub m_current_auto_size_mode: bool,
}

#[cfg(feature = "tm_pro-textmeshpro")]
#[::unity2::methods]
impl TextMeshPro {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "LoadFontAsset", args = 0)]
    pub fn load_font_asset(self) -> ();

    #[method(name = "UpdateEnvMapMatrix", args = 0)]
    pub fn update_env_map_matrix(self) -> ();

    #[method(name = "SetMask", args = 1)]
    pub fn set_mask(self, mask_type: crate::tm_pro::maskingtypes::MaskingTypes) -> ();

    #[method(name = "SetMaskCoordinates", args = 1)]
    pub fn set_mask_coordinates(self, coords: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "SetMaskCoordinates", args = 3)]
    pub fn set_mask_coordinates_2(
        self,
        coords: crate::unity_engine::vector4::Vector4,
        soft_x: f32,
        soft_y: f32,
    ) -> ();

    #[method(name = "EnableMasking", args = 0)]
    pub fn enable_masking(self) -> ();

    #[method(name = "DisableMasking", args = 0)]
    pub fn disable_masking(self) -> ();

    #[method(name = "UpdateMask", args = 0)]
    pub fn update_mask(self) -> ();

    #[method(name = "GetMaterial", args = 1)]
    pub fn get_material(
        self,
        mat: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "GetMaterials", args = 1)]
    pub fn get_materials(
        self,
        mats: ::unity2::Array<crate::unity_engine::material::Material>,
    ) -> ::unity2::Array<crate::unity_engine::material::Material>;

    #[method(name = "SetSharedMaterial", args = 1)]
    pub fn set_shared_material(self, mat: crate::unity_engine::material::Material) -> ();

    #[method(name = "GetSharedMaterials", args = 0)]
    pub fn get_shared_materials(self) -> ::unity2::Array<crate::unity_engine::material::Material>;

    #[method(name = "SetSharedMaterials", args = 1)]
    pub fn set_shared_materials(
        self,
        materials: ::unity2::Array<crate::unity_engine::material::Material>,
    ) -> ();

    #[method(name = "SetOutlineThickness", args = 1)]
    pub fn set_outline_thickness(self, thickness: f32) -> ();

    #[method(name = "SetFaceColor", args = 1)]
    pub fn set_face_color(self, color: crate::unity_engine::color32::Color32) -> ();

    #[method(name = "SetOutlineColor", args = 1)]
    pub fn set_outline_color(self, color: crate::unity_engine::color32::Color32) -> ();

    #[method(name = "CreateMaterialInstance", args = 0)]
    pub fn create_material_instance(self) -> ();

    #[method(name = "SetShaderDepth", args = 0)]
    pub fn set_shader_depth(self) -> ();

    #[method(name = "SetCulling", args = 0)]
    pub fn set_culling(self) -> ();

    #[method(name = "SetPerspectiveCorrection", args = 0)]
    pub fn set_perspective_correction(self) -> ();

    #[method(name = "SetArraySizes", args = 1)]
    pub fn set_array_sizes(
        self,
        unicode_chars: ::unity2::Array<crate::tm_pro::tmp_text::TMP_Text_UnicodeChar>,
    ) -> i32;

    #[method(name = "ComputeMarginSize", args = 0)]
    pub fn compute_margin_size(self) -> ();

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();

    #[method(name = "OnTransformParentChanged", args = 0)]
    pub fn on_transform_parent_changed(self) -> ();

    #[method(name = "OnRectTransformDimensionsChange", args = 0)]
    pub fn on_rect_transform_dimensions_change(self) -> ();

    #[method(name = "InternalUpdate", args = 0)]
    pub fn internal_update(self) -> ();

    #[method(name = "OnPreRenderObject", args = 0)]
    pub fn on_pre_render_object(self) -> ();

    #[method(name = "GenerateTextMesh", args = 0)]
    pub fn generate_text_mesh(self) -> ();

    #[method(name = "GetTextContainerLocalCorners", args = 0)]
    pub fn get_text_container_local_corners(
        self,
    ) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "SetMeshFilters", args = 1)]
    pub fn set_mesh_filters(self, state: bool) -> ();

    #[method(name = "SetActiveSubMeshes", args = 1)]
    pub fn set_active_sub_meshes(self, state: bool) -> ();

    #[method(name = "SetActiveSubTextObjectRenderers", args = 1)]
    pub fn set_active_sub_text_object_renderers(self, state: bool) -> ();

    #[method(name = "DestroySubMeshObjects", args = 0)]
    pub fn destroy_sub_mesh_objects(self) -> ();

    #[method(name = "UpdateSubMeshSortingLayerID", args = 1)]
    pub fn update_sub_mesh_sorting_layer_id(self, id: i32) -> ();

    #[method(name = "UpdateSubMeshSortingOrder", args = 1)]
    pub fn update_sub_mesh_sorting_order(self, order: i32) -> ();

    #[method(name = "GetCompoundBounds", args = 0)]
    pub fn get_compound_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "UpdateSDFScale", args = 1)]
    pub fn update_sdf_scale(self, scale_delta: f32) -> ();

    #[method(name = "get_sortingLayerID", args = 0)]
    pub fn get_sorting_layer_id(self) -> i32;

    #[method(name = "set_sortingLayerID", args = 1)]
    pub fn set_sorting_layer_id(self, value: i32) -> ();

    #[method(name = "get_sortingOrder", args = 0)]
    pub fn get_sorting_order(self) -> i32;

    #[method(name = "set_sortingOrder", args = 1)]
    pub fn set_sorting_order(self, value: i32) -> ();

    #[method(name = "get_autoSizeTextContainer", args = 0)]
    pub fn get_auto_size_text_container(self) -> bool;

    #[method(name = "set_autoSizeTextContainer", args = 1)]
    pub fn set_auto_size_text_container(self, value: bool) -> ();

    #[method(name = "get_textContainer", args = 0)]
    pub fn get_text_container(self) -> crate::tm_pro::textcontainer::TextContainer;

    #[method(name = "get_transform", args = 0)]
    pub fn get_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_renderer", args = 0)]
    pub fn get_renderer(self) -> crate::unity_engine::renderer::Renderer;

    #[method(name = "get_mesh", args = 0)]
    pub fn get_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "get_meshFilter", args = 0)]
    pub fn get_mesh_filter(self) -> crate::unity_engine::meshfilter::MeshFilter;

    #[method(name = "get_maskType", args = 0)]
    pub fn get_mask_type(self) -> crate::tm_pro::maskingtypes::MaskingTypes;

    #[method(name = "set_maskType", args = 1)]
    pub fn set_mask_type(self, value: crate::tm_pro::maskingtypes::MaskingTypes) -> ();

    #[method(name = "SetMask", args = 2)]
    pub fn set_mask_2(
        self,
        r#type: crate::tm_pro::maskingtypes::MaskingTypes,
        mask_coords: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetMask", args = 4)]
    pub fn set_mask_3(
        self,
        r#type: crate::tm_pro::maskingtypes::MaskingTypes,
        mask_coords: crate::unity_engine::vector4::Vector4,
        softness_x: f32,
        softness_y: f32,
    ) -> ();

    #[method(name = "SetVerticesDirty", args = 0)]
    pub fn set_vertices_dirty(self) -> ();

    #[method(name = "SetLayoutDirty", args = 0)]
    pub fn set_layout_dirty(self) -> ();

    #[method(name = "SetMaterialDirty", args = 0)]
    pub fn set_material_dirty(self) -> ();

    #[method(name = "SetAllDirty", args = 0)]
    pub fn set_all_dirty(self) -> ();

    #[method(name = "Rebuild", args = 1)]
    pub fn rebuild(self, update: crate::unity_engine::ui::canvasupdate::CanvasUpdate) -> ();

    #[method(name = "UpdateMaterial", args = 0)]
    pub fn update_material(self) -> ();

    #[method(name = "UpdateMeshPadding", args = 0)]
    pub fn update_mesh_padding(self) -> ();

    #[method(name = "ForceMeshUpdate", args = 2)]
    pub fn force_mesh_update(self, ignore_active_state: bool, force_text_reparsing: bool) -> ();

    #[method(name = "GetTextInfo", args = 1)]
    pub fn get_text_info(
        self,
        text: ::unity2::Il2CppString,
    ) -> crate::tm_pro::tmp_textinfo::TMP_TextInfo;

    #[method(name = "ClearMesh", args = 1)]
    pub fn clear_mesh(self, update_mesh: bool) -> ();

    #[method(name = "add_OnPreRenderText", args = 1)]
    pub fn add_on_pre_render_text(
        self,
        value: crate::system::action_1::Action_1<crate::tm_pro::tmp_textinfo::TMP_TextInfo>,
    ) -> ();

    #[method(name = "remove_OnPreRenderText", args = 1)]
    pub fn remove_on_pre_render_text(
        self,
        value: crate::system::action_1::Action_1<crate::tm_pro::tmp_textinfo::TMP_TextInfo>,
    ) -> ();

    #[method(name = "UpdateGeometry", args = 2)]
    pub fn update_geometry(self, mesh: crate::unity_engine::mesh::Mesh, index: i32) -> ();

    #[method(name = "UpdateVertexData", args = 1)]
    pub fn update_vertex_data(
        self,
        flags: crate::tm_pro::tmp_vertexdataupdateflags::TMP_VertexDataUpdateFlags,
    ) -> ();

    #[method(name = "UpdateVertexData", args = 0)]
    pub fn update_vertex_data_2(self) -> ();

    #[method(name = "UpdateFontAsset", args = 0)]
    pub fn update_font_asset(self) -> ();

    #[method(name = "CalculateLayoutInputHorizontal", args = 0)]
    pub fn calculate_layout_input_horizontal(self) -> ();

    #[method(name = "CalculateLayoutInputVertical", args = 0)]
    pub fn calculate_layout_input_vertical(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "tm_pro-textmeshpro")]
impl TextMeshPro {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextMeshPro),
                ::core::stringify!(new),
            )
        });
        <Self as ITextMeshProMethods>::ctor(this);
        this
    }
}
