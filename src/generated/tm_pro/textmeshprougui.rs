
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/textmeshprougui/TextMeshProUGUI.md")))]
#[::unity2::class(namespace = "TMPro", name = "TextMeshProUGUI")]
#[parent(crate::tm_pro::tmp_text::TMP_Text)]
pub struct TextMeshProUGUI {
    #[rename(name = "m_hasFontAssetChanged")]
    pub m_has_font_asset_changed: bool,
    #[rename(name = "m_subTextObjects")]
    pub m_sub_text_objects: ::unity2::Array<crate::tm_pro::tmp_submeshui::TMP_SubMeshUI>,
    #[rename(name = "m_previousLossyScaleY")]
    pub m_previous_lossy_scale_y: f32,
    #[rename(name = "m_RectTransformCorners")]
    pub m_rect_transform_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "m_canvasRenderer")]
    pub m_canvas_renderer: crate::unity_engine::canvasrenderer::CanvasRenderer,
    #[rename(name = "m_canvas")]
    pub m_canvas: crate::unity_engine::canvas::Canvas,
    #[rename(name = "m_CanvasScaleFactor")]
    pub m_canvas_scale_factor: f32,
    #[rename(name = "m_isFirstAllocation")]
    pub m_is_first_allocation: bool,
    #[rename(name = "m_max_characters")]
    pub m_max_characters: i32,
    #[rename(name = "m_baseMaterial")]
    pub m_base_material: crate::unity_engine::material::Material,
    #[rename(name = "m_isScrollRegionSet")]
    pub m_is_scroll_region_set: bool,
    #[rename(name = "m_maskOffset")]
    pub m_mask_offset: crate::unity_engine::vector4::Vector4,
    #[rename(name = "m_EnvMapMatrix")]
    pub m_env_map_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    #[rename(name = "m_isRegisteredForEvents")]
    pub m_is_registered_for_events: bool,
    #[rename(name = "m_isRebuildingLayout")]
    pub m_is_rebuilding_layout: bool,
    #[rename(name = "m_DelayedGraphicRebuild")]
    pub m_delayed_graphic_rebuild: crate::unity_engine::coroutine::Coroutine,
    #[rename(name = "m_DelayedMaterialRebuild")]
    pub m_delayed_material_rebuild: crate::unity_engine::coroutine::Coroutine,
    #[rename(name = "m_ClipRect")]
    pub m_clip_rect: crate::unity_engine::rect::Rect,
    #[rename(name = "m_ValidRect")]
    pub m_valid_rect: bool,
    #[rename(name = "OnPreRenderText")]
    pub on_pre_render_text:
        crate::system::action_1::Action_1<crate::tm_pro::tmp_textinfo::TMP_TextInfo>,
}

#[cfg(feature = "tm_pro-textmeshprougui")]
#[::unity2::methods]
impl TextMeshProUGUI {
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

    #[method(name = "GetCanvas", args = 0)]
    pub fn get_canvas(self) -> crate::unity_engine::canvas::Canvas;

    #[method(name = "UpdateEnvMapMatrix", args = 0)]
    pub fn update_env_map_matrix(self) -> ();

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

    #[method(name = "SetShaderDepth", args = 0)]
    pub fn set_shader_depth(self) -> ();

    #[method(name = "SetCulling", args = 0)]
    pub fn set_culling(self) -> ();

    #[method(name = "SetPerspectiveCorrection", args = 0)]
    pub fn set_perspective_correction(self) -> ();

    #[method(name = "SetMeshArrays", args = 1)]
    pub fn set_mesh_arrays(self, size: i32) -> ();

    #[method(name = "SetArraySizes", args = 1)]
    pub fn set_array_sizes(
        self,
        unicode_chars: ::unity2::Array<crate::tm_pro::tmp_text::TMP_Text_UnicodeChar>,
    ) -> i32;

    #[method(name = "ComputeMarginSize", args = 0)]
    pub fn compute_margin_size(self) -> ();

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();

    #[method(name = "OnCanvasHierarchyChanged", args = 0)]
    pub fn on_canvas_hierarchy_changed(self) -> ();

    #[method(name = "OnTransformParentChanged", args = 0)]
    pub fn on_transform_parent_changed(self) -> ();

    #[method(name = "OnRectTransformDimensionsChange", args = 0)]
    pub fn on_rect_transform_dimensions_change(self) -> ();

    #[method(name = "InternalUpdate", args = 0)]
    pub fn internal_update(self) -> ();

    #[method(name = "OnPreRenderCanvas", args = 0)]
    pub fn on_pre_render_canvas(self) -> ();

    #[method(name = "GenerateTextMesh", args = 0)]
    pub fn generate_text_mesh(self) -> ();

    #[method(name = "GetTextContainerLocalCorners", args = 0)]
    pub fn get_text_container_local_corners(
        self,
    ) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = "SetActiveSubMeshes", args = 1)]
    pub fn set_active_sub_meshes(self, state: bool) -> ();

    #[method(name = "DestroySubMeshObjects", args = 0)]
    pub fn destroy_sub_mesh_objects(self) -> ();

    #[method(name = "GetCompoundBounds", args = 0)]
    pub fn get_compound_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "GetCanvasSpaceClippingRect", args = 0)]
    pub fn get_canvas_space_clipping_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "UpdateSDFScale", args = 1)]
    pub fn update_sdf_scale(self, scale_delta: f32) -> ();

    #[method(name = "get_materialForRendering", args = 0)]
    pub fn get_material_for_rendering(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_autoSizeTextContainer", args = 0)]
    pub fn get_auto_size_text_container(self) -> bool;

    #[method(name = "set_autoSizeTextContainer", args = 1)]
    pub fn set_auto_size_text_container(self, value: bool) -> ();

    #[method(name = "get_mesh", args = 0)]
    pub fn get_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "get_canvasRenderer", args = 0)]
    pub fn get_canvas_renderer(self) -> crate::unity_engine::canvasrenderer::CanvasRenderer;

    #[method(name = "CalculateLayoutInputHorizontal", args = 0)]
    pub fn calculate_layout_input_horizontal(self) -> ();

    #[method(name = "CalculateLayoutInputVertical", args = 0)]
    pub fn calculate_layout_input_vertical(self) -> ();

    #[method(name = "SetVerticesDirty", args = 0)]
    pub fn set_vertices_dirty(self) -> ();

    #[method(name = "SetLayoutDirty", args = 0)]
    pub fn set_layout_dirty(self) -> ();

    #[method(name = "SetMaterialDirty", args = 0)]
    pub fn set_material_dirty(self) -> ();

    #[method(name = "SetAllDirty", args = 0)]
    pub fn set_all_dirty(self) -> ();

    #[method(name = "DelayedGraphicRebuild", args = 0)]
    pub fn delayed_graphic_rebuild(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "DelayedMaterialRebuild", args = 0)]
    pub fn delayed_material_rebuild(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Rebuild", args = 1)]
    pub fn rebuild(self, update: crate::unity_engine::ui::canvasupdate::CanvasUpdate) -> ();

    #[method(name = "UpdateSubObjectPivot", args = 0)]
    pub fn update_sub_object_pivot(self) -> ();

    #[method(name = "GetModifiedMaterial", args = 1)]
    pub fn get_modified_material(
        self,
        base_material: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "UpdateMaterial", args = 0)]
    pub fn update_material(self) -> ();

    #[method(name = "get_maskOffset", args = 0)]
    pub fn get_mask_offset(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "set_maskOffset", args = 1)]
    pub fn set_mask_offset(self, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "RecalculateClipping", args = 0)]
    pub fn recalculate_clipping(self) -> ();

    #[method(name = "Cull", args = 2)]
    pub fn cull(self, clip_rect: crate::unity_engine::rect::Rect, valid_rect: bool) -> ();

    #[method(name = "UpdateCulling", args = 0)]
    pub fn update_culling(self) -> ();

    #[method(name = "UpdateMeshPadding", args = 0)]
    pub fn update_mesh_padding(self) -> ();

    #[method(name = "InternalCrossFadeColor", args = 4)]
    pub fn internal_cross_fade_color(
        self,
        target_color: crate::unity_engine::color::Color,
        duration: f32,
        ignore_time_scale: bool,
        use_alpha: bool,
    ) -> ();

    #[method(name = "InternalCrossFadeAlpha", args = 3)]
    pub fn internal_cross_fade_alpha(
        self,
        alpha: f32,
        duration: f32,
        ignore_time_scale: bool,
    ) -> ();

    #[method(name = "ForceMeshUpdate", args = 2)]
    pub fn force_mesh_update(self, ignore_active_state: bool, force_text_reparsing: bool) -> ();

    #[method(name = "GetTextInfo", args = 1)]
    pub fn get_text_info(
        self,
        text: ::unity2::Il2CppString,
    ) -> crate::tm_pro::tmp_textinfo::TMP_TextInfo;

    #[method(name = "ClearMesh", args = 0)]
    pub fn clear_mesh(self) -> ();

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

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "tm_pro-textmeshprougui")]
impl TextMeshProUGUI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TextMeshProUGUI),
                ::core::stringify!(new),
            )
        });
        <Self as ITextMeshProUGUIMethods>::ctor(this);
        this
    }
}
