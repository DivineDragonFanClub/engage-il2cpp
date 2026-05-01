
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guiskin/GUISkin.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUISkin")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct GUISkin {
    #[rename(name = "m_Font")]
    pub m_font: crate::unity_engine::font::Font,
    #[rename(name = "m_box")]
    pub m_box: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_button")]
    pub m_button: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_toggle")]
    pub m_toggle: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_label")]
    pub m_label: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_textField")]
    pub m_text_field: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_textArea")]
    pub m_text_area: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_window")]
    pub m_window: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_horizontalSlider")]
    pub m_horizontal_slider: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_horizontalSliderThumb")]
    pub m_horizontal_slider_thumb: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_horizontalSliderThumbExtent")]
    pub m_horizontal_slider_thumb_extent: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_verticalSlider")]
    pub m_vertical_slider: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_verticalSliderThumb")]
    pub m_vertical_slider_thumb: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_verticalSliderThumbExtent")]
    pub m_vertical_slider_thumb_extent: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_SliderMixed")]
    pub m_slider_mixed: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_horizontalScrollbar")]
    pub m_horizontal_scrollbar: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_horizontalScrollbarThumb")]
    pub m_horizontal_scrollbar_thumb: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_horizontalScrollbarLeftButton")]
    pub m_horizontal_scrollbar_left_button: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_horizontalScrollbarRightButton")]
    pub m_horizontal_scrollbar_right_button: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_verticalScrollbar")]
    pub m_vertical_scrollbar: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_verticalScrollbarThumb")]
    pub m_vertical_scrollbar_thumb: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_verticalScrollbarUpButton")]
    pub m_vertical_scrollbar_up_button: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_verticalScrollbarDownButton")]
    pub m_vertical_scrollbar_down_button: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_ScrollView")]
    pub m_scroll_view: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_CustomStyles")]
    pub m_custom_styles: ::unity2::Array<crate::unity_engine::guistyle::GUIStyle>,
    #[rename(name = "m_Settings")]
    pub m_settings: crate::unity_engine::guisettings::GUISettings,
    #[static_field]
    #[rename(name = "ms_Error")]
    pub ms_error: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "m_Styles")]
    pub m_styles: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::unity_engine::guistyle::GUIStyle,
    >,
    #[static_field]
    #[rename(name = "m_SkinChanged")]
    pub m_skin_changed: crate::unity_engine::guiskin::GUISkin_SkinChangedDelegate,
    #[static_field]
    #[rename(name = "current")]
    pub current: crate::unity_engine::guiskin::GUISkin,
}

#[cfg(feature = "unity_engine-guiskin")]
#[::unity2::methods]
impl GUISkin {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "CleanupRoots", args = 0)]
    pub fn cleanup_roots() -> ();

    #[method(name = "get_font", args = 0)]
    pub fn get_font(self) -> crate::unity_engine::font::Font;

    #[method(name = "set_font", args = 1)]
    pub fn set_font(self, value: crate::unity_engine::font::Font) -> ();

    #[method(name = "get_box", args = 0)]
    pub fn get_box(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_box", args = 1)]
    pub fn set_box(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_label", args = 0)]
    pub fn get_label(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_label", args = 1)]
    pub fn set_label(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_textField", args = 0)]
    pub fn get_text_field(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_textField", args = 1)]
    pub fn set_text_field(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_textArea", args = 0)]
    pub fn get_text_area(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_textArea", args = 1)]
    pub fn set_text_area(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_button", args = 0)]
    pub fn get_button(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_button", args = 1)]
    pub fn set_button(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_toggle", args = 0)]
    pub fn get_toggle(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_toggle", args = 1)]
    pub fn set_toggle(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_window", args = 0)]
    pub fn get_window(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_window", args = 1)]
    pub fn set_window(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_horizontalSlider", args = 0)]
    pub fn get_horizontal_slider(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_horizontalSlider", args = 1)]
    pub fn set_horizontal_slider(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_horizontalSliderThumb", args = 0)]
    pub fn get_horizontal_slider_thumb(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_horizontalSliderThumb", args = 1)]
    pub fn set_horizontal_slider_thumb(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_horizontalSliderThumbExtent", args = 0)]
    pub fn get_horizontal_slider_thumb_extent(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_horizontalSliderThumbExtent", args = 1)]
    pub fn set_horizontal_slider_thumb_extent(
        self,
        value: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "get_sliderMixed", args = 0)]
    pub fn get_slider_mixed(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_sliderMixed", args = 1)]
    pub fn set_slider_mixed(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_verticalSlider", args = 0)]
    pub fn get_vertical_slider(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_verticalSlider", args = 1)]
    pub fn set_vertical_slider(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_verticalSliderThumb", args = 0)]
    pub fn get_vertical_slider_thumb(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_verticalSliderThumb", args = 1)]
    pub fn set_vertical_slider_thumb(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_verticalSliderThumbExtent", args = 0)]
    pub fn get_vertical_slider_thumb_extent(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_verticalSliderThumbExtent", args = 1)]
    pub fn set_vertical_slider_thumb_extent(
        self,
        value: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "get_horizontalScrollbar", args = 0)]
    pub fn get_horizontal_scrollbar(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_horizontalScrollbar", args = 1)]
    pub fn set_horizontal_scrollbar(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_horizontalScrollbarThumb", args = 0)]
    pub fn get_horizontal_scrollbar_thumb(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_horizontalScrollbarThumb", args = 1)]
    pub fn set_horizontal_scrollbar_thumb(
        self,
        value: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "get_horizontalScrollbarLeftButton", args = 0)]
    pub fn get_horizontal_scrollbar_left_button(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_horizontalScrollbarLeftButton", args = 1)]
    pub fn set_horizontal_scrollbar_left_button(
        self,
        value: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "get_horizontalScrollbarRightButton", args = 0)]
    pub fn get_horizontal_scrollbar_right_button(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_horizontalScrollbarRightButton", args = 1)]
    pub fn set_horizontal_scrollbar_right_button(
        self,
        value: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "get_verticalScrollbar", args = 0)]
    pub fn get_vertical_scrollbar(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_verticalScrollbar", args = 1)]
    pub fn set_vertical_scrollbar(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_verticalScrollbarThumb", args = 0)]
    pub fn get_vertical_scrollbar_thumb(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_verticalScrollbarThumb", args = 1)]
    pub fn set_vertical_scrollbar_thumb(self, value: crate::unity_engine::guistyle::GUIStyle)
        -> ();

    #[method(name = "get_verticalScrollbarUpButton", args = 0)]
    pub fn get_vertical_scrollbar_up_button(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_verticalScrollbarUpButton", args = 1)]
    pub fn set_vertical_scrollbar_up_button(
        self,
        value: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "get_verticalScrollbarDownButton", args = 0)]
    pub fn get_vertical_scrollbar_down_button(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_verticalScrollbarDownButton", args = 1)]
    pub fn set_vertical_scrollbar_down_button(
        self,
        value: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "get_scrollView", args = 0)]
    pub fn get_scroll_view(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_scrollView", args = 1)]
    pub fn set_scroll_view(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_customStyles", args = 0)]
    pub fn get_custom_styles(self) -> ::unity2::Array<crate::unity_engine::guistyle::GUIStyle>;

    #[method(name = "set_customStyles", args = 1)]
    pub fn set_custom_styles(
        self,
        value: ::unity2::Array<crate::unity_engine::guistyle::GUIStyle>,
    ) -> ();

    #[method(name = "get_settings", args = 0)]
    pub fn get_settings(self) -> crate::unity_engine::guisettings::GUISettings;

    #[method(name = "get_error", args = 0)]
    pub fn get_error() -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "Apply", args = 0)]
    pub fn apply(self) -> ();

    #[method(name = "BuildStyleCache", args = 0)]
    pub fn build_style_cache(self) -> ();

    #[method(name = "GetStyle", args = 1)]
    pub fn get_style(
        self,
        style_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "FindStyle", args = 1)]
    pub fn find_style(
        self,
        style_name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "MakeCurrent", args = 0)]
    pub fn make_current(self) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;
}

#[cfg(feature = "unity_engine-guiskin")]
impl GUISkin {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUISkin),
                ::core::stringify!(new),
            )
        });
        <Self as IGUISkinMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guiskin/GUISkin_SkinChangedDelegate.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUISkin.SkinChangedDelegate")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct GUISkin_SkinChangedDelegate {}

#[cfg(feature = "unity_engine-guiskin")]
#[::unity2::methods]
impl GUISkin_SkinChangedDelegate {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "unity_engine-guiskin")]
impl GUISkin_SkinChangedDelegate {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUISkin_SkinChangedDelegate),
                ::core::stringify!(new),
            )
        });
        <Self as IGUISkin_SkinChangedDelegateMethods>::ctor(this, object, method);
        this
    }
}
