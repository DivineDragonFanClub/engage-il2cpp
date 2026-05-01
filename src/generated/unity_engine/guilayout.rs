
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::gui::GUI_Scope;
use crate::unity_engine::gui::IGUI_Scope;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guilayout/GUILayout_AreaScope.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUILayout.AreaScope")]
#[parent(crate::unity_engine::gui::GUI_Scope)]
pub struct GUILayout_AreaScope {}

#[cfg(feature = "unity_engine-guilayout")]
#[::unity2::methods]
impl GUILayout_AreaScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, screen_rect: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "CloseScope", args = 0)]
    pub fn close_scope(self) -> ();
}

#[cfg(feature = "unity_engine-guilayout")]
impl GUILayout_AreaScope {
    pub fn new(screen_rect: crate::unity_engine::rect::Rect) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUILayout_AreaScope),
                ::core::stringify!(new),
            )
        });
        <Self as IGUILayout_AreaScopeMethods>::ctor(this, screen_rect);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guilayout/GUILayout.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUILayout")]
#[parent(crate::system::object::Object)]
pub struct GUILayout {}

#[cfg(feature = "unity_engine-guilayout")]
#[::unity2::methods]
impl GUILayout {
    #[method(name = "Label", args = 2)]
    pub fn label(
        text: ::unity2::Il2CppString,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "Label", args = 3)]
    pub fn label_2(
        text: ::unity2::Il2CppString,
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "DoLabel", args = 3)]
    pub fn do_label(
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "HorizontalSlider", args = 4)]
    pub fn horizontal_slider(
        value: f32,
        left_value: f32,
        right_value: f32,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> f32;

    #[method(name = "DoHorizontalSlider", args = 6)]
    pub fn do_horizontal_slider(
        value: f32,
        left_value: f32,
        right_value: f32,
        slider: crate::unity_engine::guistyle::GUIStyle,
        thumb: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> f32;

    #[method(name = "BeginHorizontal", args = 1)]
    pub fn begin_horizontal(
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "BeginHorizontal", args = 3)]
    pub fn begin_horizontal_2(
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "EndHorizontal", args = 0)]
    pub fn end_horizontal() -> ();

    #[method(name = "BeginArea", args = 1)]
    pub fn begin_area(screen_rect: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "BeginArea", args = 3)]
    pub fn begin_area_2(
        screen_rect: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "EndArea", args = 0)]
    pub fn end_area() -> ();

    #[method(name = "BeginScrollView", args = 4)]
    pub fn begin_scroll_view(
        scroll_position: crate::unity_engine::vector2::Vector2,
        always_show_horizontal: bool,
        always_show_vertical: bool,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "BeginScrollView", args = 7)]
    pub fn begin_scroll_view_2(
        scroll_position: crate::unity_engine::vector2::Vector2,
        always_show_horizontal: bool,
        always_show_vertical: bool,
        horizontal_scrollbar: crate::unity_engine::guistyle::GUIStyle,
        vertical_scrollbar: crate::unity_engine::guistyle::GUIStyle,
        background: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "EndScrollView", args = 1)]
    pub fn end_scroll_view(handle_scroll_wheel: bool) -> ();

    #[method(name = "Window", args = 5)]
    pub fn window(
        id: i32,
        screen_rect: crate::unity_engine::rect::Rect,
        func: crate::unity_engine::gui::GUI_WindowFunction,
        text: ::unity2::Il2CppString,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = "DoWindow", args = 6)]
    pub fn do_window(
        id: i32,
        screen_rect: crate::unity_engine::rect::Rect,
        func: crate::unity_engine::gui::GUI_WindowFunction,
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = "Width", args = 1)]
    pub fn width(width: f32) -> crate::unity_engine::guilayoutoption::GUILayoutOption;

    #[method(name = "Height", args = 1)]
    pub fn height(height: f32) -> crate::unity_engine::guilayoutoption::GUILayoutOption;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guilayout/GUILayout_HorizontalScope.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUILayout.HorizontalScope")]
#[parent(crate::unity_engine::gui::GUI_Scope)]
pub struct GUILayout_HorizontalScope {}

#[cfg(feature = "unity_engine-guilayout")]
#[::unity2::methods]
impl GUILayout_HorizontalScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "CloseScope", args = 0)]
    pub fn close_scope(self) -> ();
}

#[cfg(feature = "unity_engine-guilayout")]
impl GUILayout_HorizontalScope {
    pub fn new(
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUILayout_HorizontalScope),
                ::core::stringify!(new),
            )
        });
        <Self as IGUILayout_HorizontalScopeMethods>::ctor(this, options);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guilayout/GUILayout_LayoutedWindow.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUILayout.LayoutedWindow")]
#[parent(crate::system::object::Object)]
pub struct GUILayout_LayoutedWindow {
    #[rename(name = "m_Func")]
    pub m_func: crate::unity_engine::gui::GUI_WindowFunction,
    #[rename(name = "m_ScreenRect")]
    pub m_screen_rect: crate::unity_engine::rect::Rect,
    #[rename(name = "m_Options")]
    pub m_options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    #[rename(name = "m_Style")]
    pub m_style: crate::unity_engine::guistyle::GUIStyle,
}

#[cfg(feature = "unity_engine-guilayout")]
#[::unity2::methods]
impl GUILayout_LayoutedWindow {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        f: crate::unity_engine::gui::GUI_WindowFunction,
        screen_rect: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = "DoWindow", args = 1)]
    pub fn do_window(self, window_id: i32) -> ();
}

#[cfg(feature = "unity_engine-guilayout")]
impl GUILayout_LayoutedWindow {
    pub fn new(
        f: crate::unity_engine::gui::GUI_WindowFunction,
        screen_rect: crate::unity_engine::rect::Rect,
        content: crate::unity_engine::guicontent::GUIContent,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUILayout_LayoutedWindow),
                ::core::stringify!(new),
            )
        });
        <Self as IGUILayout_LayoutedWindowMethods>::ctor(
            this,
            f,
            screen_rect,
            content,
            options,
            style,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guilayout/GUILayout_ScrollViewScope.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUILayout.ScrollViewScope")]
#[parent(crate::unity_engine::gui::GUI_Scope)]
pub struct GUILayout_ScrollViewScope {}

#[cfg(feature = "unity_engine-guilayout")]
#[::unity2::methods]
impl GUILayout_ScrollViewScope {
    #[method(name = "get_scrollPosition", args = 0)]
    pub fn get_scroll_position(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_scrollPosition", args = 1)]
    pub fn set_scroll_position(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_handleScrollWheel", args = 0)]
    pub fn get_handle_scroll_wheel(self) -> bool;

    #[method(name = "set_handleScrollWheel", args = 1)]
    pub fn set_handle_scroll_wheel(self, value: bool) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        scroll_position: crate::unity_engine::vector2::Vector2,
        always_show_horizontal: bool,
        always_show_vertical: bool,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "CloseScope", args = 0)]
    pub fn close_scope(self) -> ();
}

#[cfg(feature = "unity_engine-guilayout")]
impl GUILayout_ScrollViewScope {
    pub fn new(
        scroll_position: crate::unity_engine::vector2::Vector2,
        always_show_horizontal: bool,
        always_show_vertical: bool,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUILayout_ScrollViewScope),
                ::core::stringify!(new),
            )
        });
        <Self as IGUILayout_ScrollViewScopeMethods>::ctor(
            this,
            scroll_position,
            always_show_horizontal,
            always_show_vertical,
            options,
        );
        this
    }
}
