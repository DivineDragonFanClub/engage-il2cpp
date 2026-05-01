
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::guilayoutentry::GUILayoutEntry;
use crate::unity_engine::guilayoutentry::IGUILayoutEntry;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guilayoutgroup/GUILayoutGroup.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUILayoutGroup")]
#[parent(crate::unity_engine::guilayoutentry::GUILayoutEntry)]
pub struct GUILayoutGroup {
    #[rename(name = "entries")]
    pub entries: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::guilayoutentry::GUILayoutEntry,
    >,
    #[rename(name = "isVertical")]
    pub is_vertical: bool,
    #[rename(name = "resetCoords")]
    pub reset_coords: bool,
    #[rename(name = "spacing")]
    pub spacing: f32,
    #[rename(name = "sameSize")]
    pub same_size: bool,
    #[rename(name = "isWindow")]
    pub is_window: bool,
    #[rename(name = "windowID")]
    pub window_id: i32,
    #[rename(name = "m_Cursor")]
    pub m_cursor: i32,
    #[rename(name = "m_StretchableCountX")]
    pub m_stretchable_count_x: i32,
    #[rename(name = "m_StretchableCountY")]
    pub m_stretchable_count_y: i32,
    #[rename(name = "m_UserSpecifiedWidth")]
    pub m_user_specified_width: bool,
    #[rename(name = "m_UserSpecifiedHeight")]
    pub m_user_specified_height: bool,
    #[rename(name = "m_ChildMinWidth")]
    pub m_child_min_width: f32,
    #[rename(name = "m_ChildMaxWidth")]
    pub m_child_max_width: f32,
    #[rename(name = "m_ChildMinHeight")]
    pub m_child_min_height: f32,
    #[rename(name = "m_ChildMaxHeight")]
    pub m_child_max_height: f32,
    #[rename(name = "m_MarginLeft")]
    pub m_margin_left: i32,
    #[rename(name = "m_MarginRight")]
    pub m_margin_right: i32,
    #[rename(name = "m_MarginTop")]
    pub m_margin_top: i32,
    #[rename(name = "m_MarginBottom")]
    pub m_margin_bottom: i32,
    #[static_field]
    #[rename(name = "none")]
    pub none: crate::unity_engine::guilayoutentry::GUILayoutEntry,
}

#[cfg(feature = "unity_engine-guilayoutgroup")]
#[::unity2::methods]
impl GUILayoutGroup {
    #[method(name = "get_marginLeft", args = 0)]
    pub fn get_margin_left(self) -> i32;

    #[method(name = "get_marginRight", args = 0)]
    pub fn get_margin_right(self) -> i32;

    #[method(name = "get_marginTop", args = 0)]
    pub fn get_margin_top(self) -> i32;

    #[method(name = "get_marginBottom", args = 0)]
    pub fn get_margin_bottom(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ApplyOptions", args = 1)]
    pub fn apply_options(
        self,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "ApplyStyleSettings", args = 1)]
    pub fn apply_style_settings(self, style: crate::unity_engine::guistyle::GUIStyle) -> ();

    #[method(name = "ResetCursor", args = 0)]
    pub fn reset_cursor(self) -> ();

    #[method(name = "GetNext", args = 0)]
    pub fn get_next(self) -> crate::unity_engine::guilayoutentry::GUILayoutEntry;

    #[method(name = "Add", args = 1)]
    pub fn add(self, e: crate::unity_engine::guilayoutentry::GUILayoutEntry) -> ();

    #[method(name = "CalcWidth", args = 0)]
    pub fn calc_width(self) -> ();

    #[method(name = "SetHorizontal", args = 2)]
    pub fn set_horizontal(self, x: f32, width: f32) -> ();

    #[method(name = "CalcHeight", args = 0)]
    pub fn calc_height(self) -> ();

    #[method(name = "SetVertical", args = 2)]
    pub fn set_vertical(self, y: f32, height: f32) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-guilayoutgroup")]
impl GUILayoutGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUILayoutGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IGUILayoutGroupMethods>::ctor(this);
        this
    }
}
