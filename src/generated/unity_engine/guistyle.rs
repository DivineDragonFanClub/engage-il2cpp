
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guistyle/GUIStyle.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUIStyle")]
#[parent(crate::system::object::Object)]
pub struct GUIStyle {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_Normal")]
    pub m_normal: crate::unity_engine::guistylestate::GUIStyleState,
    #[rename(name = "m_Hover")]
    pub m_hover: crate::unity_engine::guistylestate::GUIStyleState,
    #[rename(name = "m_Active")]
    pub m_active: crate::unity_engine::guistylestate::GUIStyleState,
    #[rename(name = "m_Focused")]
    pub m_focused: crate::unity_engine::guistylestate::GUIStyleState,
    #[rename(name = "m_OnNormal")]
    pub m_on_normal: crate::unity_engine::guistylestate::GUIStyleState,
    #[rename(name = "m_OnHover")]
    pub m_on_hover: crate::unity_engine::guistylestate::GUIStyleState,
    #[rename(name = "m_OnActive")]
    pub m_on_active: crate::unity_engine::guistylestate::GUIStyleState,
    #[rename(name = "m_OnFocused")]
    pub m_on_focused: crate::unity_engine::guistylestate::GUIStyleState,
    #[rename(name = "m_Border")]
    pub m_border: crate::unity_engine::rectoffset::RectOffset,
    #[rename(name = "m_Padding")]
    pub m_padding: crate::unity_engine::rectoffset::RectOffset,
    #[rename(name = "m_Margin")]
    pub m_margin: crate::unity_engine::rectoffset::RectOffset,
    #[rename(name = "m_Overflow")]
    pub m_overflow: crate::unity_engine::rectoffset::RectOffset,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "showKeyboardFocus")]
    pub show_keyboard_focus: bool,
    #[static_field]
    #[rename(name = "s_None")]
    pub s_none: crate::unity_engine::guistyle::GUIStyle,
}

#[cfg(feature = "unity_engine-guistyle")]
#[::unity2::methods]
impl GUIStyle {
    #[method(name = "get_rawName", args = 0)]
    pub fn get_raw_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_rawName", args = 1)]
    pub fn set_raw_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_font", args = 1)]
    pub fn set_font(self, value: crate::unity_engine::font::Font) -> ();

    #[method(name = "get_imagePosition", args = 0)]
    pub fn get_image_position(self) -> crate::unity_engine::imageposition::ImagePosition;

    #[method(name = "set_alignment", args = 1)]
    pub fn set_alignment(self, value: crate::unity_engine::textanchor::TextAnchor) -> ();

    #[method(name = "get_wordWrap", args = 0)]
    pub fn get_word_wrap(self) -> bool;

    #[method(name = "set_clipping", args = 1)]
    pub fn set_clipping(self, value: crate::unity_engine::textclipping::TextClipping) -> ();

    #[method(name = "set_contentOffset", args = 1)]
    pub fn set_content_offset(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_fixedWidth", args = 0)]
    pub fn get_fixed_width(self) -> f32;

    #[method(name = "get_fixedHeight", args = 0)]
    pub fn get_fixed_height(self) -> f32;

    #[method(name = "set_fixedHeight", args = 1)]
    pub fn set_fixed_height(self, value: f32) -> ();

    #[method(name = "get_stretchWidth", args = 0)]
    pub fn get_stretch_width(self) -> bool;

    #[method(name = "set_stretchWidth", args = 1)]
    pub fn set_stretch_width(self, value: bool) -> ();

    #[method(name = "get_stretchHeight", args = 0)]
    pub fn get_stretch_height(self) -> bool;

    #[method(name = "set_stretchHeight", args = 1)]
    pub fn set_stretch_height(self, value: bool) -> ();

    #[method(name = "get_fontSize", args = 0)]
    pub fn get_font_size(self) -> i32;

    #[method(name = "set_fontSize", args = 1)]
    pub fn set_font_size(self, value: i32) -> ();

    #[method(name = "set_fontStyle", args = 1)]
    pub fn set_font_style(self, value: crate::unity_engine::fontstyle::FontStyle) -> ();

    #[method(name = "Internal_Create", args = 1)]
    pub fn internal_create(self_: crate::unity_engine::guistyle::GUIStyle) -> ::unity2::IntPtr;

    #[method(name = "Internal_Copy", args = 2)]
    pub fn internal_copy(
        self_: crate::unity_engine::guistyle::GUIStyle,
        other: crate::unity_engine::guistyle::GUIStyle,
    ) -> ::unity2::IntPtr;

    #[method(name = "Internal_Destroy", args = 1)]
    pub fn internal_destroy(self_: ::unity2::IntPtr) -> ();

    #[method(name = "GetStyleStatePtr", args = 1)]
    pub fn get_style_state_ptr(self, idx: i32) -> ::unity2::IntPtr;

    #[method(name = "AssignStyleState", args = 2)]
    pub fn assign_style_state(self, idx: i32, src_style_state: ::unity2::IntPtr) -> ();

    #[method(name = "GetRectOffsetPtr", args = 1)]
    pub fn get_rect_offset_ptr(self, idx: i32) -> ::unity2::IntPtr;

    #[method(name = "Internal_Draw", args = 6)]
    pub fn internal_draw(
        self,
        screen_rect: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        is_hover: bool,
        is_active: bool,
        on: bool,
        has_keyboard_focus: bool,
    ) -> ();

    #[method(name = "Internal_Draw2", args = 4)]
    pub fn internal_draw2(
        self,
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        control_id: i32,
        on: bool,
    ) -> ();

    #[method(name = "Internal_CalcSize", args = 1)]
    pub fn internal_calc_size(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "Internal_CalcSizeWithConstraints", args = 2)]
    pub fn internal_calc_size_with_constraints(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
        max_size: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "Internal_CalcHeight", args = 2)]
    pub fn internal_calc_height(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
        width: f32,
    ) -> f32;

    #[method(name = "Internal_CalcMinMaxWidth", args = 1)]
    pub fn internal_calc_min_max_width(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "SetMouseTooltip", args = 2)]
    pub fn set_mouse_tooltip(
        tooltip: ::unity2::Il2CppString,
        screen_rect: crate::unity_engine::rect::Rect,
    ) -> ();

    #[method(name = "IsTooltipActive", args = 1)]
    pub fn is_tooltip_active(tooltip: ::unity2::Il2CppString) -> bool;

    #[method(name = "SetDefaultFont", args = 1)]
    pub fn set_default_font(font: crate::unity_engine::font::Font) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, other: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "get_name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_normal", args = 0)]
    pub fn get_normal(self) -> crate::unity_engine::guistylestate::GUIStyleState;

    #[method(name = "set_normal", args = 1)]
    pub fn set_normal(self, value: crate::unity_engine::guistylestate::GUIStyleState) -> ();

    #[method(name = "get_margin", args = 0)]
    pub fn get_margin(self) -> crate::unity_engine::rectoffset::RectOffset;

    #[method(name = "get_padding", args = 0)]
    pub fn get_padding(self) -> crate::unity_engine::rectoffset::RectOffset;

    #[method(name = "Draw", args = 5)]
    pub fn draw(
        self,
        position: crate::unity_engine::rect::Rect,
        is_hover: bool,
        is_active: bool,
        on: bool,
        has_keyboard_focus: bool,
    ) -> ();

    #[method(name = "Draw", args = 6)]
    pub fn draw_2(
        self,
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        is_hover: bool,
        is_active: bool,
        on: bool,
        has_keyboard_focus: bool,
    ) -> ();

    #[method(name = "Draw", args = 3)]
    pub fn draw_3(
        self,
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        control_id: i32,
    ) -> ();

    #[method(name = "Draw", args = 5)]
    pub fn draw_4(
        self,
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        control_id: i32,
        on: bool,
        hover: bool,
    ) -> ();

    #[method(name = "Draw", args = 7)]
    pub fn draw_5(
        self,
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        control_id: i32,
        is_hover: bool,
        is_active: bool,
        on: bool,
        has_keyboard_focus: bool,
    ) -> ();

    #[method(name = "get_none", args = 0)]
    pub fn get_none() -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "CalcSize", args = 1)]
    pub fn calc_size(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "CalcSizeWithConstraints", args = 2)]
    pub fn calc_size_with_constraints(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
        constraints: crate::unity_engine::vector2::Vector2,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "CalcHeight", args = 2)]
    pub fn calc_height(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
        width: f32,
    ) -> f32;

    #[method(name = "get_isHeightDependantOnWidth", args = 0)]
    pub fn get_is_height_dependant_on_width(self) -> bool;

    #[method(name = "CalcMinMaxWidth", args = 3)]
    pub fn calc_min_max_width(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
        min_width: f32,
        max_width: f32,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "set_contentOffset_Injected", args = 1)]
    pub fn set_content_offset_injected(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "Internal_Draw_Injected", args = 6)]
    pub fn internal_draw_injected(
        self,
        screen_rect: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        is_hover: bool,
        is_active: bool,
        on: bool,
        has_keyboard_focus: bool,
    ) -> ();

    #[method(name = "Internal_Draw2_Injected", args = 4)]
    pub fn internal_draw2_injected(
        self,
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        control_id: i32,
        on: bool,
    ) -> ();

    #[method(name = "Internal_CalcSize_Injected", args = 2)]
    pub fn internal_calc_size_injected(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
        ret: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "Internal_CalcSizeWithConstraints_Injected", args = 3)]
    pub fn internal_calc_size_with_constraints_injected(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
        max_size: crate::unity_engine::vector2::Vector2,
        ret: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "Internal_CalcMinMaxWidth_Injected", args = 2)]
    pub fn internal_calc_min_max_width_injected(
        self,
        content: crate::unity_engine::guicontent::GUIContent,
        ret: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "SetMouseTooltip_Injected", args = 2)]
    pub fn set_mouse_tooltip_injected(
        tooltip: ::unity2::Il2CppString,
        screen_rect: crate::unity_engine::rect::Rect,
    ) -> ();
}

#[cfg(feature = "unity_engine-guistyle")]
impl GUIStyle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIStyle),
                ::core::stringify!(new),
            )
        });
        <Self as IGUIStyleMethods>::ctor(this);
        this
    }

    pub fn new_2(other: crate::unity_engine::guistyle::GUIStyle) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIStyle),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGUIStyleMethods>::ctor_2(this, other);
        this
    }
}
