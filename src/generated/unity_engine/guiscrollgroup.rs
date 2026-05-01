
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::guilayoutentry::GUILayoutEntry;
use crate::unity_engine::guilayoutentry::IGUILayoutEntry;
use crate::unity_engine::guilayoutgroup::GUILayoutGroup;
use crate::unity_engine::guilayoutgroup::IGUILayoutGroup;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guiscrollgroup/GUIScrollGroup.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUIScrollGroup")]
#[parent(crate::unity_engine::guilayoutgroup::GUILayoutGroup)]
pub struct GUIScrollGroup {
    #[rename(name = "calcMinWidth")]
    pub calc_min_width: f32,
    #[rename(name = "calcMaxWidth")]
    pub calc_max_width: f32,
    #[rename(name = "calcMinHeight")]
    pub calc_min_height: f32,
    #[rename(name = "calcMaxHeight")]
    pub calc_max_height: f32,
    #[rename(name = "clientWidth")]
    pub client_width: f32,
    #[rename(name = "clientHeight")]
    pub client_height: f32,
    #[rename(name = "allowHorizontalScroll")]
    pub allow_horizontal_scroll: bool,
    #[rename(name = "allowVerticalScroll")]
    pub allow_vertical_scroll: bool,
    #[rename(name = "needsHorizontalScrollbar")]
    pub needs_horizontal_scrollbar: bool,
    #[rename(name = "needsVerticalScrollbar")]
    pub needs_vertical_scrollbar: bool,
    #[rename(name = "horizontalScrollbar")]
    pub horizontal_scrollbar: crate::unity_engine::guistyle::GUIStyle,
    #[rename(name = "verticalScrollbar")]
    pub vertical_scrollbar: crate::unity_engine::guistyle::GUIStyle,
}

#[cfg(feature = "unity_engine-guiscrollgroup")]
#[::unity2::methods]
impl GUIScrollGroup {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CalcWidth", args = 0)]
    pub fn calc_width(self) -> ();

    #[method(name = "SetHorizontal", args = 2)]
    pub fn set_horizontal(self, x: f32, width: f32) -> ();

    #[method(name = "CalcHeight", args = 0)]
    pub fn calc_height(self) -> ();

    #[method(name = "SetVertical", args = 2)]
    pub fn set_vertical(self, y: f32, height: f32) -> ();
}

#[cfg(feature = "unity_engine-guiscrollgroup")]
impl GUIScrollGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIScrollGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IGUIScrollGroupMethods>::ctor(this);
        this
    }
}
