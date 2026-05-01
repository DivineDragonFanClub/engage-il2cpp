
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guilayoutentry/GUILayoutEntry.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUILayoutEntry")]
#[parent(crate::system::object::Object)]
pub struct GUILayoutEntry {
    #[rename(name = "minWidth")]
    pub min_width: f32,
    #[rename(name = "maxWidth")]
    pub max_width: f32,
    #[rename(name = "minHeight")]
    pub min_height: f32,
    #[rename(name = "maxHeight")]
    pub max_height: f32,
    #[rename(name = "rect")]
    pub rect: crate::unity_engine::rect::Rect,
    #[rename(name = "stretchWidth")]
    pub stretch_width: i32,
    #[rename(name = "stretchHeight")]
    pub stretch_height: i32,
    #[rename(name = "consideredForMargin")]
    pub considered_for_margin: bool,
    #[rename(name = "m_Style")]
    pub m_style: crate::unity_engine::guistyle::GUIStyle,
    #[static_field]
    #[rename(name = "kDummyRect")]
    pub k_dummy_rect: crate::unity_engine::rect::Rect,
    #[static_field]
    #[rename(name = "indent")]
    pub indent: i32,
}

#[cfg(feature = "unity_engine-guilayoutentry")]
#[::unity2::methods]
impl GUILayoutEntry {
    #[method(name = "get_style", args = 0)]
    pub fn get_style(self) -> crate::unity_engine::guistyle::GUIStyle;

    #[method(name = "set_style", args = 1)]
    pub fn set_style(self, value: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "get_marginLeft", args = 0)]
    pub fn get_margin_left(self) -> i32;

    #[method(name = "get_marginRight", args = 0)]
    pub fn get_margin_right(self) -> i32;

    #[method(name = "get_marginTop", args = 0)]
    pub fn get_margin_top(self) -> i32;

    #[method(name = "get_marginBottom", args = 0)]
    pub fn get_margin_bottom(self) -> i32;

    #[method(name = "get_marginHorizontal", args = 0)]
    pub fn get_margin_horizontal(self) -> i32;

    #[method(name = "get_marginVertical", args = 0)]
    pub fn get_margin_vertical(self) -> i32;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        min_width: f32,
        max_width: f32,
        min_height: f32,
        max_height: f32,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> ();

    #[method(name = ".ctor", args = 6)]
    pub fn ctor_2(
        self,
        min_width: f32,
        max_width: f32,
        min_height: f32,
        max_height: f32,
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "CalcWidth", args = 0)]
    pub fn calc_width(self) -> ();

    #[method(name = "CalcHeight", args = 0)]
    pub fn calc_height(self) -> ();

    #[method(name = "SetHorizontal", args = 2)]
    pub fn set_horizontal(self, x: f32, width: f32) -> ();

    #[method(name = "SetVertical", args = 2)]
    pub fn set_vertical(self, y: f32, height: f32) -> ();

    #[method(name = "ApplyStyleSettings", args = 1)]
    pub fn apply_style_settings(self, style: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "ApplyOptions", args = 1)]
    pub fn apply_options(
        self,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-guilayoutentry")]
impl GUILayoutEntry {
    pub fn new(
        min_width: f32,
        max_width: f32,
        min_height: f32,
        max_height: f32,
        style: crate::unity_engine::guistyle::GUIStyle,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUILayoutEntry),
                ::core::stringify!(new),
            )
        });
        <Self as IGUILayoutEntryMethods>::ctor(
            this, min_width, max_width, min_height, max_height, style,
        );
        this
    }

    pub fn new_2(
        min_width: f32,
        max_width: f32,
        min_height: f32,
        max_height: f32,
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUILayoutEntry),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGUILayoutEntryMethods>::ctor_2(
            this, min_width, max_width, min_height, max_height, style, options,
        );
        this
    }
}
