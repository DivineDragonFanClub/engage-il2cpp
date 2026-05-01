
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
use crate::unity_engine::ui::graphic::Graphic;
use crate::unity_engine::ui::graphic::IGraphic;
use crate::unity_engine::ui::maskablegraphic::IMaskableGraphic;
use crate::unity_engine::ui::maskablegraphic::MaskableGraphic;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_submeshui/TMP_SubMeshUI.md")))]
#[::unity2::class(namespace = "TMPro", name = "TMP_SubMeshUI")]
#[parent(crate::unity_engine::ui::maskablegraphic::MaskableGraphic)]
pub struct TMP_SubMeshUI {
    #[rename(name = "m_fontAsset")]
    pub m_font_asset: crate::tm_pro::tmp_fontasset::TMP_FontAsset,
    #[rename(name = "m_spriteAsset")]
    pub m_sprite_asset: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset,
    #[rename(name = "m_material")]
    pub m_material: crate::unity_engine::material::Material,
    #[rename(name = "m_sharedMaterial")]
    pub m_shared_material: crate::unity_engine::material::Material,
    #[rename(name = "m_fallbackMaterial")]
    pub m_fallback_material: crate::unity_engine::material::Material,
    #[rename(name = "m_fallbackSourceMaterial")]
    pub m_fallback_source_material: crate::unity_engine::material::Material,
    #[rename(name = "m_isDefaultMaterial")]
    pub m_is_default_material: bool,
    #[rename(name = "m_padding")]
    pub m_padding: f32,
    #[rename(name = "m_mesh")]
    pub m_mesh: crate::unity_engine::mesh::Mesh,
    #[rename(name = "m_TextComponent")]
    pub m_text_component: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_isRegisteredForEvents")]
    pub m_is_registered_for_events: bool,
    #[rename(name = "m_materialDirty")]
    pub m_material_dirty: bool,
    #[rename(name = "m_materialReferenceIndex")]
    pub m_material_reference_index: i32,
    #[rename(name = "m_RootCanvasTransform")]
    pub m_root_canvas_transform: crate::unity_engine::transform::Transform,
}

#[cfg(feature = "tm_pro-tmp_submeshui")]
#[::unity2::methods]
impl TMP_SubMeshUI {
    #[method(name = "get_fontAsset", args = 0)]
    pub fn get_font_asset(self) -> crate::tm_pro::tmp_fontasset::TMP_FontAsset;

    #[method(name = "set_fontAsset", args = 1)]
    pub fn set_font_asset(self, value: crate::tm_pro::tmp_fontasset::TMP_FontAsset) -> ();

    #[method(name = "get_spriteAsset", args = 0)]
    pub fn get_sprite_asset(self) -> crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset;

    #[method(name = "set_spriteAsset", args = 1)]
    pub fn set_sprite_asset(self, value: crate::tm_pro::tmp_spriteasset::TMP_SpriteAsset) -> ();

    #[method(name = "get_mainTexture", args = 0)]
    pub fn get_main_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "get_material", args = 0)]
    pub fn get_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "set_material", args = 1)]
    pub fn set_material(self, value: crate::unity_engine::material::Material) -> ();

    #[method(name = "get_sharedMaterial", args = 0)]
    pub fn get_shared_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "set_sharedMaterial", args = 1)]
    pub fn set_shared_material(self, value: crate::unity_engine::material::Material) -> ();

    #[method(name = "get_fallbackMaterial", args = 0)]
    pub fn get_fallback_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "set_fallbackMaterial", args = 1)]
    pub fn set_fallback_material(self, value: crate::unity_engine::material::Material) -> ();

    #[method(name = "get_fallbackSourceMaterial", args = 0)]
    pub fn get_fallback_source_material(self) -> crate::unity_engine::material::Material;

    #[method(name = "set_fallbackSourceMaterial", args = 1)]
    pub fn set_fallback_source_material(self, value: crate::unity_engine::material::Material)
        -> ();

    #[method(name = "get_materialForRendering", args = 0)]
    pub fn get_material_for_rendering(self) -> crate::unity_engine::material::Material;

    #[method(name = "get_isDefaultMaterial", args = 0)]
    pub fn get_is_default_material(self) -> bool;

    #[method(name = "set_isDefaultMaterial", args = 1)]
    pub fn set_is_default_material(self, value: bool) -> ();

    #[method(name = "get_padding", args = 0)]
    pub fn get_padding(self) -> f32;

    #[method(name = "set_padding", args = 1)]
    pub fn set_padding(self, value: f32) -> ();

    #[method(name = "get_mesh", args = 0)]
    pub fn get_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "set_mesh", args = 1)]
    pub fn set_mesh(self, value: crate::unity_engine::mesh::Mesh) -> ();

    #[method(name = "get_textComponent", args = 0)]
    pub fn get_text_component(self) -> crate::tm_pro::tmp_text::TMP_Text;

    #[method(name = "AddSubTextObject", args = 2)]
    pub fn add_sub_text_object(
        text_component: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        material_reference: crate::tm_pro::materialreference::MaterialReference,
    ) -> crate::tm_pro::tmp_submeshui::TMP_SubMeshUI;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "OnTransformParentChanged", args = 0)]
    pub fn on_transform_parent_changed(self) -> ();

    #[method(name = "GetModifiedMaterial", args = 1)]
    pub fn get_modified_material(
        self,
        base_material: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "GetPaddingForMaterial", args = 0)]
    pub fn get_padding_for_material(self) -> f32;

    #[method(name = "GetPaddingForMaterial", args = 1)]
    pub fn get_padding_for_material_2(self, mat: crate::unity_engine::material::Material) -> f32;

    #[method(name = "UpdateMeshPadding", args = 2)]
    pub fn update_mesh_padding(self, is_extra_padding: bool, is_using_bold: bool) -> ();

    #[method(name = "SetAllDirty", args = 0)]
    pub fn set_all_dirty(self) -> ();

    #[method(name = "SetVerticesDirty", args = 0)]
    pub fn set_vertices_dirty(self) -> ();

    #[method(name = "SetLayoutDirty", args = 0)]
    pub fn set_layout_dirty(self) -> ();

    #[method(name = "SetMaterialDirty", args = 0)]
    pub fn set_material_dirty(self) -> ();

    #[method(name = "SetPivotDirty", args = 0)]
    pub fn set_pivot_dirty(self) -> ();

    #[method(name = "GetRootCanvasTransform", args = 0)]
    pub fn get_root_canvas_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "Cull", args = 2)]
    pub fn cull(self, clip_rect: crate::unity_engine::rect::Rect, valid_rect: bool) -> ();

    #[method(name = "UpdateGeometry", args = 0)]
    pub fn update_geometry(self) -> ();

    #[method(name = "Rebuild", args = 1)]
    pub fn rebuild(self, update: crate::unity_engine::ui::canvasupdate::CanvasUpdate) -> ();

    #[method(name = "RefreshMaterial", args = 0)]
    pub fn refresh_material(self) -> ();

    #[method(name = "UpdateMaterial", args = 0)]
    pub fn update_material(self) -> ();

    #[method(name = "RecalculateClipping", args = 0)]
    pub fn recalculate_clipping(self) -> ();

    #[method(name = "GetMaterial", args = 1)]
    pub fn get_material_2(
        self,
        mat: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "CreateMaterialInstance", args = 1)]
    pub fn create_material_instance(
        self,
        source: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::material::Material;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-tmp_submeshui")]
impl TMP_SubMeshUI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TMP_SubMeshUI),
                ::core::stringify!(new),
            )
        });
        <Self as ITMP_SubMeshUIMethods>::ctor(this);
        this
    }
}
