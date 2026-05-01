
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guilayoututility/GUILayoutUtility.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUILayoutUtility")]
#[parent(crate::system::object::Object)]
pub struct GUILayoutUtility {
    #[static_field]
    #[rename(name = "s_StoredLayouts")]
    pub s_stored_layouts: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::unity_engine::guilayoututility::GUILayoutUtility_LayoutCache,
    >,
    #[static_field]
    #[rename(name = "s_StoredWindows")]
    pub s_stored_windows: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::unity_engine::guilayoututility::GUILayoutUtility_LayoutCache,
    >,
    #[static_field]
    #[rename(name = "current")]
    pub current: crate::unity_engine::guilayoututility::GUILayoutUtility_LayoutCache,
    #[static_field]
    #[rename(name = "kDummyRect")]
    pub k_dummy_rect: crate::unity_engine::rect::Rect,
}

#[cfg(feature = "unity_engine-guilayoututility")]
#[::unity2::methods]
impl GUILayoutUtility {
    #[method(name = "Internal_GetWindowRect", args = 1)]
    pub fn internal_get_window_rect(window_id: i32) -> crate::unity_engine::rect::Rect;

    #[method(name = "Internal_MoveWindow", args = 2)]
    pub fn internal_move_window(window_id: i32, r: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "SelectIDList", args = 2)]
    pub fn select_id_list(
        instance_id: i32,
        is_window: bool,
    ) -> crate::unity_engine::guilayoututility::GUILayoutUtility_LayoutCache;

    #[method(name = "Begin", args = 1)]
    pub fn begin(instance_id: i32) -> ();

    #[method(name = "BeginWindow", args = 3)]
    pub fn begin_window(
        window_id: i32,
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> ();

    #[method(name = "Layout", args = 0)]
    pub fn layout() -> ();

    #[method(name = "LayoutFromEditorWindow", args = 0)]
    pub fn layout_from_editor_window() -> ();

    #[method(name = "LayoutFreeGroup", args = 1)]
    pub fn layout_free_group(toplevel: crate::unity_engine::guilayoutgroup::GUILayoutGroup) -> ();

    #[method(name = "LayoutSingleGroup", args = 1)]
    pub fn layout_single_group(i: crate::unity_engine::guilayoutgroup::GUILayoutGroup) -> ();

    #[method(name = "CreateGUILayoutGroupInstanceOfType", args = 1)]
    pub fn create_gui_layout_group_instance_of_type(
        layout_type: ::unity2::SystemType,
    ) -> crate::unity_engine::guilayoutgroup::GUILayoutGroup;

    #[method(name = "BeginLayoutGroup", args = 3)]
    pub fn begin_layout_group(
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
        layout_type: ::unity2::SystemType,
    ) -> crate::unity_engine::guilayoutgroup::GUILayoutGroup;

    #[method(name = "EndLayoutGroup", args = 0)]
    pub fn end_layout_group() -> ();

    #[method(name = "BeginLayoutArea", args = 2)]
    pub fn begin_layout_area(
        style: crate::unity_engine::guistyle::GUIStyle,
        layout_type: ::unity2::SystemType,
    ) -> crate::unity_engine::guilayoutgroup::GUILayoutGroup;

    #[method(name = "GetRect", args = 3)]
    pub fn get_rect(
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = "DoGetRect", args = 3)]
    pub fn do_get_rect(
        content: crate::unity_engine::guicontent::GUIContent,
        style: crate::unity_engine::guistyle::GUIStyle,
        options: ::unity2::Array<crate::unity_engine::guilayoutoption::GUILayoutOption>,
    ) -> crate::unity_engine::rect::Rect;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "Internal_GetWindowRect_Injected", args = 2)]
    pub fn internal_get_window_rect_injected(
        window_id: i32,
        ret: crate::unity_engine::rect::Rect,
    ) -> ();

    #[method(name = "Internal_MoveWindow_Injected", args = 2)]
    pub fn internal_move_window_injected(window_id: i32, r: crate::unity_engine::rect::Rect) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guilayoututility/GUILayoutUtility_LayoutCache.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUILayoutUtility.LayoutCache")]
#[parent(crate::system::object::Object)]
pub struct GUILayoutUtility_LayoutCache {
    #[rename(name = "topLevel")]
    pub top_level: crate::unity_engine::guilayoutgroup::GUILayoutGroup,
    #[rename(name = "windows")]
    pub windows: crate::unity_engine::guilayoutgroup::GUILayoutGroup,
}

#[cfg(feature = "unity_engine-guilayoututility")]
#[::unity2::methods]
impl GUILayoutUtility_LayoutCache {
    #[method(name = "set_id", args = 1)]
    pub fn set_id(self, value: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, instance_id: i32) -> ();
}

#[cfg(feature = "unity_engine-guilayoututility")]
impl GUILayoutUtility_LayoutCache {
    pub fn new(instance_id: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUILayoutUtility_LayoutCache),
                ::core::stringify!(new),
            )
        });
        <Self as IGUILayoutUtility_LayoutCacheMethods>::ctor(this, instance_id);
        this
    }
}
