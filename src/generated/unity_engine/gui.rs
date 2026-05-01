
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/gui/GUI.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUI")]
#[parent(crate::system::object::Object)]
pub struct GUI {
    #[static_field]
    #[rename(name = "s_ScrollControlId")]
    pub s_scroll_control_id: i32,
    #[static_field]
    #[rename(name = "s_HotTextField")]
    pub s_hot_text_field: i32,
    #[static_field]
    #[rename(name = "s_BoxHash")]
    pub s_box_hash: i32,
    #[static_field]
    #[rename(name = "s_ButonHash")]
    pub s_buton_hash: i32,
    #[static_field]
    #[rename(name = "s_RepeatButtonHash")]
    pub s_repeat_button_hash: i32,
    #[static_field]
    #[rename(name = "s_ToggleHash")]
    pub s_toggle_hash: i32,
    #[static_field]
    #[rename(name = "s_ButtonGridHash")]
    pub s_button_grid_hash: i32,
    #[static_field]
    #[rename(name = "s_SliderHash")]
    pub s_slider_hash: i32,
    #[static_field]
    #[rename(name = "s_BeginGroupHash")]
    pub s_begin_group_hash: i32,
    #[static_field]
    #[rename(name = "s_ScrollviewHash")]
    pub s_scrollview_hash: i32,
    #[static_field]
    #[rename(name = "s_Skin")]
    pub s_skin: crate::unity_engine::guiskin::GUISkin,
    #[static_field]
    #[rename(name = "s_ToolTipRect")]
    pub s_tool_tip_rect: crate::unity_engine::rect::Rect,
}

#[cfg(feature = "unity_engine-gui")]
#[::unity2::methods]
impl GUI {
    #[method(name = "get_color", args = 0)]
    pub fn get_color() -> crate::unity_engine::color::Color;

    #[method(name = "set_color", args = 1)]
    pub fn set_color(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_backgroundColor", args = 1)]
    pub fn set_background_color(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_changed", args = 1)]
    pub fn set_changed(value: bool) -> ();

    #[method(name = "set_depth", args = 1)]
    pub fn set_depth(value: i32) -> ();

    #[method(name = "get_usePageScrollbars", args = 0)]
    pub fn get_use_page_scrollbars() -> bool;

    #[method(name = "get_blendMaterial", args = 0)]
    pub fn get_blend_material() -> crate::unity_engine::material::Material;

    #[method(name = "get_blitMaterial", args = 0)]
    pub fn get_blit_material() -> crate::unity_engine::material::Material;

    #[method(name = "get_roundedRectMaterial", args = 0)]
    pub fn get_rounded_rect_material() -> crate::unity_engine::material::Material;

    #[method(name = "get_roundedRectWithColorPerBorderMaterial", args = 0)]
    pub fn get_rounded_rect_with_color_per_border_material(
    ) -> crate::unity_engine::material::Material;

    #[method(name = "GrabMouseControl", args = 1)]
    pub fn grab_mouse_control(id: i32) -> ();

    #[method(name = "HasMouseControl", args = 1)]
    pub fn has_mouse_control(id: i32) -> bool;

    #[method(name = "ReleaseMouseControl", args = 0)]
    pub fn release_mouse_control() -> ();

    #[method(name = "InternalRepaintEditorWindow", args = 0)]
    pub fn internal_repaint_editor_window() -> ();

    #[method(name = "Internal_DoWindow", args = 8)]
    pub fn internal_do_window(
        id: i32,
        instance_id: i32,
        client_rect: crate::unity_engine::rect::Rect,
        func: crate::unity_engine::gui::GUI_WindowFunction,
        title: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        skin: crate::system::object::Object,
        force_rect_on_layout: bool,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "get_scrollTroughSide", args = 0)]
    pub fn get_scroll_trough_side() -> i32;

    #[method(name = "set_scrollTroughSide", args = 1)]
    pub fn set_scroll_trough_side(value: i32) -> ();

    #[method(name = "set_skin", args = 1)]
    pub fn set_skin(value: crate::unity_engine::guiskin::GUISkin) -> ();

    #[method(name = "get_skin", args = 0)]
    pub fn get_skin() -> crate::unity_engine::guiskin::GUISkin;

    #[method(name = "DoSetSkin", args = 1)]
    pub fn do_set_skin(new_skin: crate::unity_engine::guiskin::GUISkin) -> ();

    #[method(name = "set_matrix", args = 1)]
    pub fn set_matrix(value: crate::unity_engine::matrix4x4::Matrix4x4) -> ();

    #[method(name = "Label", args = 3)]
    pub fn label(
        position: crate::unity_engine::rect::Rect,
        text: ::unity2::Il2CppString,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "Label", args = 3)]
    pub fn label_2(
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "DrawTexture", args = 2)]
    pub fn draw_texture(
        position: crate::unity_engine::rect::Rect,
        image: crate::unity_engine::texture::Texture,
    ) -> ();

    #[method(name = "DrawTexture", args = 3)]
    pub fn draw_texture_2(
        position: crate::unity_engine::rect::Rect,
        image: crate::unity_engine::texture::Texture,
        scale_mode: crate::unity_engine::scalemode::ScaleMode,
    ) -> ();

    #[method(name = "DrawTexture", args = 4)]
    pub fn draw_texture_3(
        position: crate::unity_engine::rect::Rect,
        image: crate::unity_engine::texture::Texture,
        scale_mode: crate::unity_engine::scalemode::ScaleMode,
        alpha_blend: bool,
    ) -> ();

    #[method(name = "DrawTexture", args = 5)]
    pub fn draw_texture_4(
        position: crate::unity_engine::rect::Rect,
        image: crate::unity_engine::texture::Texture,
        scale_mode: crate::unity_engine::scalemode::ScaleMode,
        alpha_blend: bool,
        image_aspect: f32,
    ) -> ();

    #[method(name = "DrawTexture", args = 8)]
    pub fn draw_texture_5(
        position: crate::unity_engine::rect::Rect,
        image: crate::unity_engine::texture::Texture,
        scale_mode: crate::unity_engine::scalemode::ScaleMode,
        alpha_blend: bool,
        image_aspect: f32,
        color: crate::unity_engine::color::Color,
        border_width: f32,
        border_radius: f32,
    ) -> ();

    #[method(name = "DrawTexture", args = 8)]
    pub fn draw_texture_6(
        position: crate::unity_engine::rect::Rect,
        image: crate::unity_engine::texture::Texture,
        scale_mode: crate::unity_engine::scalemode::ScaleMode,
        alpha_blend: bool,
        image_aspect: f32,
        color: crate::unity_engine::color::Color,
        border_widths: crate::unity_engine::vector4::Vector4,
        border_radius: f32,
    ) -> ();

    #[method(name = "DrawTexture", args = 8)]
    pub fn draw_texture_7(
        position: crate::unity_engine::rect::Rect,
        image: crate::unity_engine::texture::Texture,
        scale_mode: crate::unity_engine::scalemode::ScaleMode,
        alpha_blend: bool,
        image_aspect: f32,
        color: crate::unity_engine::color::Color,
        border_widths: crate::unity_engine::vector4::Vector4,
        border_radiuses: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "DrawTexture", args = 9)]
    pub fn draw_texture_8(
        position: crate::unity_engine::rect::Rect,
        image: crate::unity_engine::texture::Texture,
        scale_mode: crate::unity_engine::scalemode::ScaleMode,
        alpha_blend: bool,
        image_aspect: f32,
        color: crate::unity_engine::color::Color,
        border_widths: crate::unity_engine::vector4::Vector4,
        border_radiuses: crate::unity_engine::vector4::Vector4,
        draw_smooth_corners: bool,
    ) -> ();

    #[method(name = "DrawTexture", args = 12)]
    pub fn draw_texture_9(
        position: crate::unity_engine::rect::Rect,
        image: crate::unity_engine::texture::Texture,
        scale_mode: crate::unity_engine::scalemode::ScaleMode,
        alpha_blend: bool,
        image_aspect: f32,
        left_color: crate::unity_engine::color::Color,
        top_color: crate::unity_engine::color::Color,
        right_color: crate::unity_engine::color::Color,
        bottom_color: crate::unity_engine::color::Color,
        border_widths: crate::unity_engine::vector4::Vector4,
        border_radiuses: crate::unity_engine::vector4::Vector4,
        draw_smooth_corners: bool,
    ) -> ();

    #[method(name = "CalculateScaledTextureRects", args = 5)]
    pub fn calculate_scaled_texture_rects(
        position: crate::unity_engine::rect::Rect,
        scale_mode: crate::unity_engine::scalemode::ScaleMode,
        image_aspect: f32,
        out_screen_rect: crate::unity_engine::rect::Rect,
        out_source_rect: crate::unity_engine::rect::Rect,
    ) -> bool;

    #[method(name = "Box", args = 3)]
    pub fn r#box(
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "Button", args = 2)]
    pub fn button(position: crate::unity_engine::rect::Rect, text: ::unity2::Il2CppString) -> bool;

    #[method(name = "Button", args = 3)]
    pub fn button_2(
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> bool;

    #[method(name = "Button", args = 4)]
    pub fn button_3(
        position: crate::unity_engine::rect::Rect,
        id: i32,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> bool;

    #[method(name = "DoRepeatButton", args = 4)]
    pub fn do_repeat_button(
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        focus_type: crate::unity_engine::focustype::FocusType,
    ) -> bool;

    #[method(name = "DoControl", args = 6)]
    pub fn do_control(
        position: crate::unity_engine::rect::Rect,
        id: i32,
        on: bool,
        hover: bool,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> bool;

    #[method(name = "DoLabel", args = 3)]
    pub fn do_label(
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "DoButton", args = 4)]
    pub fn do_button(
        position: crate::unity_engine::rect::Rect,
        id: i32,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> bool;

    #[method(name = "HorizontalSlider", args = 6)]
    pub fn horizontal_slider(
        position: crate::unity_engine::rect::Rect,
        value: f32,
        left_value: f32,
        right_value: f32,
        slider: crate::unity_engine::guistyle::GUIStyle,
        thumb: crate::unity_engine::guistyle::GUIStyle,
    ) -> f32;

    #[method(name = "Slider", args = 10)]
    pub fn slider(
        position: crate::unity_engine::rect::Rect,
        value: f32,
        size: f32,
        start: f32,
        end: f32,
        slider: crate::unity_engine::guistyle::GUIStyle,
        thumb: crate::unity_engine::guistyle::GUIStyle,
        horiz: bool,
        id: i32,
        thumb_extent: crate::unity_engine::guistyle::GUIStyle,
    ) -> f32;

    #[method(name = "HorizontalScrollbar", args = 6)]
    pub fn horizontal_scrollbar(
        position: crate::unity_engine::rect::Rect,
        value: f32,
        size: f32,
        left_value: f32,
        right_value: f32,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> f32;

    #[method(name = "ScrollerRepeatButton", args = 3)]
    pub fn scroller_repeat_button(
        scroller_id: i32,
        rect: crate::unity_engine::rect::Rect,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> bool;

    #[method(name = "VerticalScrollbar", args = 6)]
    pub fn vertical_scrollbar(
        position: crate::unity_engine::rect::Rect,
        value: f32,
        size: f32,
        top_value: f32,
        bottom_value: f32,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> f32;

    #[method(name = "Scroller", args = 10)]
    pub fn scroller(
        position: crate::unity_engine::rect::Rect,
        value: f32,
        size: f32,
        left_value: f32,
        right_value: f32,
        slider: crate::unity_engine::guistyle::GUIStyle,
        thumb: crate::unity_engine::guistyle::GUIStyle,
        left_button: crate::unity_engine::guistyle::GUIStyle,
        right_button: crate::unity_engine::guistyle::GUIStyle,
        horiz: bool,
    ) -> f32;

    #[method(name = "BeginGroup", args = 3)]
    pub fn begin_group(
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "BeginGroup", args = 4)]
    pub fn begin_group_2(
        position: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        scroll_offset: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "EndGroup", args = 0)]
    pub fn end_group() -> ();

    #[method(name = "BeginClip", args = 1)]
    pub fn begin_clip(position: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "EndClip", args = 0)]
    pub fn end_clip() -> ();

    #[method(name = "BeginScrollView", args = 8)]
    pub fn begin_scroll_view(
        position: crate::unity_engine::rect::Rect,
        scroll_position: crate::unity_engine::vector2::Vector2,
        view_rect: crate::unity_engine::rect::Rect,
        always_show_horizontal: bool,
        always_show_vertical: bool,
        horizontal_scrollbar: crate::unity_engine::guistyle::GUIStyle,
        vertical_scrollbar: crate::unity_engine::guistyle::GUIStyle,
        background: crate::unity_engine::guistyle::GUIStyle,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "EndScrollView", args = 1)]
    pub fn end_scroll_view(handle_scroll_wheel: bool) -> ();

    #[method(name = "Window", args = 5)]
    pub fn window(
        id: i32,
        client_rect: crate::unity_engine::rect::Rect,
        func: crate::unity_engine::gui::GUI_WindowFunction,
        title: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = "DoWindow", args = 7)]
    pub fn do_window(
        id: i32,
        client_rect: crate::unity_engine::rect::Rect,
        func: crate::unity_engine::gui::GUI_WindowFunction,
        title: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        skin: crate::unity_engine::guiskin::GUISkin,
        force_rect_on_layout: bool,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = "CallWindowDelegate", args = 8)]
    pub fn call_window_delegate(
        func: crate::unity_engine::gui::GUI_WindowFunction,
        id: i32,
        instance_id: i32,
        skin: crate::unity_engine::guiskin::GUISkin,
        force_rect: i32,
        width: f32,
        height: f32,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_color_Injected", args = 1)]
    pub fn get_color_injected(ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_color_Injected", args = 1)]
    pub fn set_color_injected(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_backgroundColor_Injected", args = 1)]
    pub fn set_background_color_injected(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "Internal_DoWindow_Injected", args = 9)]
    pub fn internal_do_window_injected(
        id: i32,
        instance_id: i32,
        client_rect: crate::unity_engine::rect::Rect,
        func: crate::unity_engine::gui::GUI_WindowFunction,
        title: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        skin: crate::system::object::Object,
        force_rect_on_layout: bool,
        ret: crate::unity_engine::rect::Rect,
    ) -> ();
}

#[cfg(feature = "unity_engine-gui")]
impl GUI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUI),
                ::core::stringify!(new),
            )
        });
        <Self as IGUIMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/gui/GUI_WindowFunction.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUI.WindowFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct GUI_WindowFunction {}

#[cfg(feature = "unity_engine-gui")]
#[::unity2::methods]
impl GUI_WindowFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, id: i32) -> ();
}

#[cfg(feature = "unity_engine-gui")]
impl GUI_WindowFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUI_WindowFunction),
                ::core::stringify!(new),
            )
        });
        <Self as IGUI_WindowFunctionMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/gui/GUI_Scope.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUI.Scope")]
#[parent(crate::system::object::Object)]
pub struct GUI_Scope {
    #[rename(name = "m_Disposed")]
    pub m_disposed: bool,
}

#[cfg(feature = "unity_engine-gui")]
#[::unity2::methods]
impl GUI_Scope {
    #[method(name = "Dispose", args = 1)]
    pub fn dispose(self, disposing: bool) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose_2(self) -> ();

    #[method(name = "CloseScope", args = 0)]
    pub fn close_scope(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-gui")]
impl GUI_Scope {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUI_Scope),
                ::core::stringify!(new),
            )
        });
        <Self as IGUI_ScopeMethods>::ctor(this);
        this
    }
}
