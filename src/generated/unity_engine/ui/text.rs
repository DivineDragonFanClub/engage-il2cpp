
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/text/Text.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "Text")]
#[parent(crate::unity_engine::ui::maskablegraphic::MaskableGraphic)]
pub struct Text {
    #[rename(name = "m_FontData")]
    pub m_font_data: crate::unity_engine::ui::fontdata::FontData,
    #[rename(name = "m_Text")]
    pub m_text: ::unity2::Il2CppString,
    #[rename(name = "m_TextCache")]
    pub m_text_cache: crate::unity_engine::textgenerator::TextGenerator,
    #[rename(name = "m_TextCacheForLayout")]
    pub m_text_cache_for_layout: crate::unity_engine::textgenerator::TextGenerator,
    #[static_field]
    #[rename(name = "s_DefaultText")]
    pub s_default_text: crate::unity_engine::material::Material,
    #[rename(name = "m_DisableFontTextureRebuiltCallback")]
    pub m_disable_font_texture_rebuilt_callback: bool,
    #[rename(name = "m_TempVerts")]
    pub m_temp_verts: ::unity2::Array<crate::unity_engine::uivertex::UIVertex>,
}

#[cfg(feature = "unity_engine-ui-text")]
#[::unity2::methods]
impl Text {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_cachedTextGenerator", args = 0)]
    pub fn get_cached_text_generator(self) -> crate::unity_engine::textgenerator::TextGenerator;

    #[method(name = "get_cachedTextGeneratorForLayout", args = 0)]
    pub fn get_cached_text_generator_for_layout(
        self,
    ) -> crate::unity_engine::textgenerator::TextGenerator;

    #[method(name = "get_mainTexture", args = 0)]
    pub fn get_main_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "FontTextureChanged", args = 0)]
    pub fn font_texture_changed(self) -> ();

    #[method(name = "get_font", args = 0)]
    pub fn get_font(self) -> crate::unity_engine::font::Font;

    #[method(name = "set_font", args = 1)]
    pub fn set_font(self, value: crate::unity_engine::font::Font) -> ();

    #[method(name = "get_text", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "set_text", args = 1)]
    pub fn set_text(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_supportRichText", args = 0)]
    pub fn get_support_rich_text(self) -> bool;

    #[method(name = "set_supportRichText", args = 1)]
    pub fn set_support_rich_text(self, value: bool) -> ();

    #[method(name = "get_resizeTextForBestFit", args = 0)]
    pub fn get_resize_text_for_best_fit(self) -> bool;

    #[method(name = "set_resizeTextForBestFit", args = 1)]
    pub fn set_resize_text_for_best_fit(self, value: bool) -> ();

    #[method(name = "get_resizeTextMinSize", args = 0)]
    pub fn get_resize_text_min_size(self) -> i32;

    #[method(name = "set_resizeTextMinSize", args = 1)]
    pub fn set_resize_text_min_size(self, value: i32) -> ();

    #[method(name = "get_resizeTextMaxSize", args = 0)]
    pub fn get_resize_text_max_size(self) -> i32;

    #[method(name = "set_resizeTextMaxSize", args = 1)]
    pub fn set_resize_text_max_size(self, value: i32) -> ();

    #[method(name = "get_alignment", args = 0)]
    pub fn get_alignment(self) -> crate::unity_engine::textanchor::TextAnchor;

    #[method(name = "set_alignment", args = 1)]
    pub fn set_alignment(self, value: crate::unity_engine::textanchor::TextAnchor) -> ();

    #[method(name = "get_alignByGeometry", args = 0)]
    pub fn get_align_by_geometry(self) -> bool;

    #[method(name = "set_alignByGeometry", args = 1)]
    pub fn set_align_by_geometry(self, value: bool) -> ();

    #[method(name = "get_fontSize", args = 0)]
    pub fn get_font_size(self) -> i32;

    #[method(name = "set_fontSize", args = 1)]
    pub fn set_font_size(self, value: i32) -> ();

    #[method(name = "get_horizontalOverflow", args = 0)]
    pub fn get_horizontal_overflow(
        self,
    ) -> crate::unity_engine::horizontalwrapmode::HorizontalWrapMode;

    #[method(name = "set_horizontalOverflow", args = 1)]
    pub fn set_horizontal_overflow(
        self,
        value: crate::unity_engine::horizontalwrapmode::HorizontalWrapMode,
    ) -> ();

    #[method(name = "get_verticalOverflow", args = 0)]
    pub fn get_vertical_overflow(self) -> crate::unity_engine::verticalwrapmode::VerticalWrapMode;

    #[method(name = "set_verticalOverflow", args = 1)]
    pub fn set_vertical_overflow(
        self,
        value: crate::unity_engine::verticalwrapmode::VerticalWrapMode,
    ) -> ();

    #[method(name = "get_lineSpacing", args = 0)]
    pub fn get_line_spacing(self) -> f32;

    #[method(name = "set_lineSpacing", args = 1)]
    pub fn set_line_spacing(self, value: f32) -> ();

    #[method(name = "get_fontStyle", args = 0)]
    pub fn get_font_style(self) -> crate::unity_engine::fontstyle::FontStyle;

    #[method(name = "set_fontStyle", args = 1)]
    pub fn set_font_style(self, value: crate::unity_engine::fontstyle::FontStyle) -> ();

    #[method(name = "get_pixelsPerUnit", args = 0)]
    pub fn get_pixels_per_unit(self) -> f32;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "UpdateGeometry", args = 0)]
    pub fn update_geometry(self) -> ();

    #[method(name = "AssignDefaultFont", args = 0)]
    pub fn assign_default_font(self) -> ();

    #[method(name = "GetGenerationSettings", args = 1)]
    pub fn get_generation_settings(
        self,
        extents: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::textgenerationsettings::TextGenerationSettings;

    #[method(name = "GetTextAnchorPivot", args = 1)]
    pub fn get_text_anchor_pivot(
        anchor: crate::unity_engine::textanchor::TextAnchor,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "OnPopulateMesh", args = 1)]
    pub fn on_populate_mesh(
        self,
        to_fill: crate::unity_engine::ui::vertexhelper::VertexHelper,
    ) -> ();

    #[method(name = "CalculateLayoutInputHorizontal", args = 0)]
    pub fn calculate_layout_input_horizontal(self) -> ();

    #[method(name = "CalculateLayoutInputVertical", args = 0)]
    pub fn calculate_layout_input_vertical(self) -> ();

    #[method(name = "get_minWidth", args = 0)]
    pub fn get_min_width(self) -> f32;

    #[method(name = "get_preferredWidth", args = 0)]
    pub fn get_preferred_width(self) -> f32;

    #[method(name = "get_flexibleWidth", args = 0)]
    pub fn get_flexible_width(self) -> f32;

    #[method(name = "get_minHeight", args = 0)]
    pub fn get_min_height(self) -> f32;

    #[method(name = "get_preferredHeight", args = 0)]
    pub fn get_preferred_height(self) -> f32;

    #[method(name = "get_flexibleHeight", args = 0)]
    pub fn get_flexible_height(self) -> f32;

    #[method(name = "get_layoutPriority", args = 0)]
    pub fn get_layout_priority(self) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-ui-text")]
impl Text {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Text),
                ::core::stringify!(new),
            )
        });
        <Self as ITextMethods>::ctor(this);
        this
    }
}
