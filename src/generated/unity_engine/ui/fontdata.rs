
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/fontdata/FontData.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "FontData")]
#[parent(crate::system::object::Object)]
pub struct FontData {
    #[rename(name = "m_Font")]
    pub m_font: crate::unity_engine::font::Font,
    #[rename(name = "m_FontSize")]
    pub m_font_size: i32,
    #[rename(name = "m_FontStyle")]
    pub m_font_style: crate::unity_engine::fontstyle::FontStyle,
    #[rename(name = "m_BestFit")]
    pub m_best_fit: bool,
    #[rename(name = "m_MinSize")]
    pub m_min_size: i32,
    #[rename(name = "m_MaxSize")]
    pub m_max_size: i32,
    #[rename(name = "m_Alignment")]
    pub m_alignment: crate::unity_engine::textanchor::TextAnchor,
    #[rename(name = "m_AlignByGeometry")]
    pub m_align_by_geometry: bool,
    #[rename(name = "m_RichText")]
    pub m_rich_text: bool,
    #[rename(name = "m_HorizontalOverflow")]
    pub m_horizontal_overflow: crate::unity_engine::horizontalwrapmode::HorizontalWrapMode,
    #[rename(name = "m_VerticalOverflow")]
    pub m_vertical_overflow: crate::unity_engine::verticalwrapmode::VerticalWrapMode,
    #[rename(name = "m_LineSpacing")]
    pub m_line_spacing: f32,
}

#[cfg(feature = "unity_engine-ui-fontdata")]
#[::unity2::methods]
impl FontData {
    #[method(name = "get_defaultFontData", args = 0)]
    pub fn get_default_font_data() -> crate::unity_engine::ui::fontdata::FontData;

    #[method(name = "get_font", args = 0)]
    pub fn get_font(self) -> crate::unity_engine::font::Font;

    #[method(name = "set_font", args = 1)]
    pub fn set_font(self, value: crate::unity_engine::font::Font) -> ();

    #[method(name = "get_fontSize", args = 0)]
    pub fn get_font_size(self) -> i32;

    #[method(name = "set_fontSize", args = 1)]
    pub fn set_font_size(self, value: i32) -> ();

    #[method(name = "get_fontStyle", args = 0)]
    pub fn get_font_style(self) -> crate::unity_engine::fontstyle::FontStyle;

    #[method(name = "set_fontStyle", args = 1)]
    pub fn set_font_style(self, value: crate::unity_engine::fontstyle::FontStyle) -> ();

    #[method(name = "get_bestFit", args = 0)]
    pub fn get_best_fit(self) -> bool;

    #[method(name = "set_bestFit", args = 1)]
    pub fn set_best_fit(self, value: bool) -> ();

    #[method(name = "get_minSize", args = 0)]
    pub fn get_min_size(self) -> i32;

    #[method(name = "set_minSize", args = 1)]
    pub fn set_min_size(self, value: i32) -> ();

    #[method(name = "get_maxSize", args = 0)]
    pub fn get_max_size(self) -> i32;

    #[method(name = "set_maxSize", args = 1)]
    pub fn set_max_size(self, value: i32) -> ();

    #[method(name = "get_alignment", args = 0)]
    pub fn get_alignment(self) -> crate::unity_engine::textanchor::TextAnchor;

    #[method(name = "set_alignment", args = 1)]
    pub fn set_alignment(self, value: crate::unity_engine::textanchor::TextAnchor) -> ();

    #[method(name = "get_alignByGeometry", args = 0)]
    pub fn get_align_by_geometry(self) -> bool;

    #[method(name = "set_alignByGeometry", args = 1)]
    pub fn set_align_by_geometry(self, value: bool) -> ();

    #[method(name = "get_richText", args = 0)]
    pub fn get_rich_text(self) -> bool;

    #[method(name = "set_richText", args = 1)]
    pub fn set_rich_text(self, value: bool) -> ();

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

    #[method(
        name = "UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize",
        args = 0
    )]
    pub fn unity_engine_i_serialization_callback_receiver_on_before_serialize(self) -> ();

    #[method(
        name = "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
        args = 0
    )]
    pub fn unity_engine_i_serialization_callback_receiver_on_after_deserialize(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-ui-fontdata")]
impl FontData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FontData),
                ::core::stringify!(new),
            )
        });
        <Self as IFontDataMethods>::ctor(this);
        this
    }
}
