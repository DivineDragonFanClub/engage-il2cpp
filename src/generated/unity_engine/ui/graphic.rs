
use crate::system::object::IObject;
use crate::system::object::Object;
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
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/graphic/Graphic.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Graphic")]
#[parent(crate::unity_engine::event_systems::uibehaviour::UIBehaviour)]
pub struct Graphic {
    #[static_field]
    #[rename(name = "s_DefaultUI")]
    pub s_default_ui: crate::unity_engine::material::Material,
    #[static_field]
    #[rename(name = "s_WhiteTexture")]
    pub s_white_texture: crate::unity_engine::texture2d::Texture2D,
    #[rename(name = "m_Material")]
    pub m_material: crate::unity_engine::material::Material,
    #[rename(name = "m_Color")]
    pub m_color: crate::unity_engine::color::Color,
    #[rename(name = "m_SkipLayoutUpdate")]
    pub m_skip_layout_update: bool,
    #[rename(name = "m_SkipMaterialUpdate")]
    pub m_skip_material_update: bool,
    #[rename(name = "m_RaycastTarget")]
    pub m_raycast_target: bool,
    #[rename(name = "m_RaycastPadding")]
    pub m_raycast_padding: crate::unity_engine::vector4::Vector4,
    #[rename(name = "m_RectTransform")]
    pub m_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_CanvasRenderer")]
    pub m_canvas_renderer: crate::unity_engine::canvasrenderer::CanvasRenderer,
    #[rename(name = "m_Canvas")]
    pub m_canvas: crate::unity_engine::canvas::Canvas,
    #[rename(name = "m_VertsDirty")]
    pub m_verts_dirty: bool,
    #[rename(name = "m_MaterialDirty")]
    pub m_material_dirty: bool,
    #[rename(name = "m_OnDirtyLayoutCallback")]
    pub m_on_dirty_layout_callback: crate::unity_engine::events::unityaction::UnityAction,
    #[rename(name = "m_OnDirtyVertsCallback")]
    pub m_on_dirty_verts_callback: crate::unity_engine::events::unityaction::UnityAction,
    #[rename(name = "m_OnDirtyMaterialCallback")]
    pub m_on_dirty_material_callback: crate::unity_engine::events::unityaction::UnityAction,
    #[static_field]
    #[rename(name = "s_Mesh")]
    pub s_mesh: crate::unity_engine::mesh::Mesh,
    #[static_field]
    #[rename(name = "s_VertexHelper")]
    pub s_vertex_helper: crate::unity_engine::ui::vertexhelper::VertexHelper,
    #[rename(name = "m_CachedMesh")]
    pub m_cached_mesh: crate::unity_engine::mesh::Mesh,
    #[rename(name = "m_CachedUvs")]
    pub m_cached_uvs: ::unity2::Array<crate::unity_engine::vector2::Vector2>,
    #[rename(name = "m_ColorTweenRunner")]
    pub m_color_tween_runner:
        crate::unity_engine::ui::coroutine_tween::tweenrunner_1::TweenRunner_1<
            crate::unity_engine::ui::coroutine_tween::colortween::ColorTween,
        >,
}

#[cfg(feature = "unity_engine-ui-graphic")]
#[::unity2::methods]
impl Graphic {
    #[method(name = "get_defaultGraphicMaterial", args = 0)]
    pub fn get_default_graphic_material() -> crate::unity_engine::material::Material;

    #[method(name = "get_color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_color", args = 1)]
    pub fn set_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_raycastTarget", args = 0)]
    pub fn get_raycast_target(self) -> bool;

    #[method(name = "set_raycastTarget", args = 1)]
    pub fn set_raycast_target(self, value: bool) -> ();

    #[method(name = "get_raycastPadding", args = 0)]
    pub fn get_raycast_padding(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "set_raycastPadding", args = 1)]
    pub fn set_raycast_padding(self, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "get_useLegacyMeshGeneration", args = 0)]
    pub fn get_use_legacy_mesh_generation(self) -> bool;

    #[method(name = "set_useLegacyMeshGeneration", args = 1)]
    pub fn set_use_legacy_mesh_generation(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "SetAllDirty", args = 0)]
    pub fn set_all_dirty(self) -> ();

    #[method(name = "SetLayoutDirty", args = 0)]
    pub fn set_layout_dirty(self) -> ();

    #[method(name = "SetVerticesDirty", args = 0)]
    pub fn set_vertices_dirty(self) -> ();

    #[method(name = "SetMaterialDirty", args = 0)]
    pub fn set_material_dirty(self) -> ();

    #[method(name = "OnRectTransformDimensionsChange", args = 0)]
    pub fn on_rect_transform_dimensions_change(self) -> ();

    #[method(name = "OnBeforeTransformParentChanged", args = 0)]
    pub fn on_before_transform_parent_changed(self) -> ();

    #[method(name = "OnTransformParentChanged", args = 0)]
    pub fn on_transform_parent_changed(self) -> ();

    #[method(name = "get_depth", args = 0)]
    pub fn get_depth(self) -> i32;

    #[method(name = "get_rectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "get_canvas", args = 0)]
    pub fn get_canvas(self) -> crate::unity_engine::canvas::Canvas;

    #[method(name = "CacheCanvas", args = 0)]
    pub fn cache_canvas(self) -> ();

    #[method(name = "get_canvasRenderer", args = 0)]
    pub fn get_canvas_renderer(self) -> crate::unity_engine::canvasrenderer::CanvasRenderer;

    #[method(name = "get_defaultMaterial", args = 0)]
    pub fn get_default_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_material", args = 0)]
    pub fn get_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "set_material", args = 1)]
    pub fn set_material(self, value: crate::unity_engine::material::Material) -> ();

    #[method(name = "get_materialForRendering", args = 0)]
    pub fn get_material_for_rendering(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_mainTexture", args = 0)]
    pub fn get_main_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "OnCanvasHierarchyChanged", args = 0)]
    pub fn on_canvas_hierarchy_changed(self) -> ();

    #[method(name = "OnCullingChanged", args = 0)]
    pub fn on_culling_changed(self) -> ();

    #[method(name = "Rebuild", args = 1)]
    pub fn rebuild(self, update: crate::unity_engine::ui::canvasupdate::CanvasUpdate) -> ();

    #[method(name = "LayoutComplete", args = 0)]
    pub fn layout_complete(self) -> ();

    #[method(name = "GraphicUpdateComplete", args = 0)]
    pub fn graphic_update_complete(self) -> ();

    #[method(name = "UpdateMaterial", args = 0)]
    pub fn update_material(self) -> ();

    #[method(name = "UpdateGeometry", args = 0)]
    pub fn update_geometry(self) -> ();

    #[method(name = "DoMeshGeneration", args = 0)]
    pub fn do_mesh_generation(self) -> ();

    #[method(name = "DoLegacyMeshGeneration", args = 0)]
    pub fn do_legacy_mesh_generation(self) -> ();

    #[method(name = "get_workerMesh", args = 0)]
    pub fn get_worker_mesh() -> crate::unity_engine::mesh::Mesh;

    #[method(name = "OnFillVBO", args = 1)]
    pub fn on_fill_vbo(
        self,
        vbo: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::uivertex::UIVertex,
        >,
    ) -> ();

    #[method(name = "OnPopulateMesh", args = 1)]
    pub fn on_populate_mesh(self, m: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = "OnPopulateMesh", args = 1)]
    pub fn on_populate_mesh_2(self, vh: crate::unity_engine::ui::vertexhelper::VertexHelper) -> ();

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();

    #[method(name = "SetNativeSize", args = 0)]
    pub fn set_native_size(self) -> ();

    #[method(name = "Raycast", args = 2)]
    pub fn raycast(
        self,
        sp: crate::unity_engine::vector2::Vector2,
        event_camera: crate::unity_engine::camera::Camera,
    ) -> bool;

    #[method(name = "PixelAdjustPoint", args = 1)]
    pub fn pixel_adjust_point(
        self,
        point: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetPixelAdjustedRect", args = 0)]
    pub fn get_pixel_adjusted_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "CrossFadeColor", args = 4)]
    pub fn cross_fade_color(
        self,
        target_color: crate::unity_engine::color::Color,
        duration: f32,
        ignore_time_scale: bool,
        use_alpha: bool,
    ) -> ();

    #[method(name = "CrossFadeColor", args = 5)]
    pub fn cross_fade_color_2(
        self,
        target_color: crate::unity_engine::color::Color,
        duration: f32,
        ignore_time_scale: bool,
        use_alpha: bool,
        use_rgb: bool,
    ) -> ();

    #[method(name = "CreateColorFromAlpha", args = 1)]
    pub fn create_color_from_alpha(alpha: f32) -> crate::unity_engine::color::Color;

    #[method(name = "CrossFadeAlpha", args = 3)]
    pub fn cross_fade_alpha(self, alpha: f32, duration: f32, ignore_time_scale: bool) -> ();

    #[method(name = "RegisterDirtyLayoutCallback", args = 1)]
    pub fn register_dirty_layout_callback(
        self,
        action: crate::unity_engine::events::unityaction::UnityAction,
    ) -> ();

    #[method(name = "UnregisterDirtyLayoutCallback", args = 1)]
    pub fn unregister_dirty_layout_callback(
        self,
        action: crate::unity_engine::events::unityaction::UnityAction,
    ) -> ();

    #[method(name = "RegisterDirtyVerticesCallback", args = 1)]
    pub fn register_dirty_vertices_callback(
        self,
        action: crate::unity_engine::events::unityaction::UnityAction,
    ) -> ();

    #[method(name = "UnregisterDirtyVerticesCallback", args = 1)]
    pub fn unregister_dirty_vertices_callback(
        self,
        action: crate::unity_engine::events::unityaction::UnityAction,
    ) -> ();

    #[method(name = "RegisterDirtyMaterialCallback", args = 1)]
    pub fn register_dirty_material_callback(
        self,
        action: crate::unity_engine::events::unityaction::UnityAction,
    ) -> ();

    #[method(name = "UnregisterDirtyMaterialCallback", args = 1)]
    pub fn unregister_dirty_material_callback(
        self,
        action: crate::unity_engine::events::unityaction::UnityAction,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "UnityEngine.UI.ICanvasElement.get_transform", args = 0)]
    pub fn unity_engine_ui_i_canvas_element_get_transform(
        self,
    ) -> crate::unity_engine::transform::Transform;
}

#[cfg(feature = "unity_engine-ui-graphic")]
impl Graphic {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Graphic),
                ::core::stringify!(new),
            )
        });
        <Self as IGraphicMethods>::ctor(this);
        this
    }
}
