
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guiutility/GUIUtility.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUIUtility")]
#[parent(crate::system::object::Object)]
pub struct GUIUtility {
    #[static_field]
    #[rename(name = "s_SkinMode")]
    pub s_skin_mode: i32,
    #[static_field]
    #[rename(name = "s_OriginalID")]
    pub s_original_id: i32,
    #[static_field]
    #[rename(name = "releaseCapture")]
    pub release_capture: crate::system::action::Action,
    #[static_field]
    #[rename(name = "guiChanged")]
    pub gui_changed: crate::system::action::Action,
    #[static_field]
    #[rename(name = "s_HasCurrentWindowKeyFocusFunc")]
    pub s_has_current_window_key_focus_func: crate::system::func_1::Func_1<bool>,
}

#[cfg(feature = "unity_engine-guiutility")]
#[::unity2::methods]
impl GUIUtility {
    #[method(name = "get_pixelsPerPoint", args = 0)]
    pub fn get_pixels_per_point() -> f32;

    #[method(name = "get_guiDepth", args = 0)]
    pub fn get_gui_depth() -> i32;

    #[method(name = "set_mouseUsed", args = 1)]
    pub fn set_mouse_used(value: bool) -> ();

    #[method(name = "get_systemCopyBuffer", args = 0)]
    pub fn get_system_copy_buffer() -> ::unity2::Il2CppString;

    #[method(name = "set_systemCopyBuffer", args = 1)]
    pub fn set_system_copy_buffer(value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetControlID", args = 3)]
    pub fn get_control_id(
        hint: i32,
        focus_type: crate::unity_engine::focustype::FocusType,
        rect: crate::unity_engine::rect::Rect,
    ) -> i32;

    #[method(name = "Internal_GetHotControl", args = 0)]
    pub fn internal_get_hot_control() -> i32;

    #[method(name = "Internal_GetKeyboardControl", args = 0)]
    pub fn internal_get_keyboard_control() -> i32;

    #[method(name = "Internal_SetHotControl", args = 1)]
    pub fn internal_set_hot_control(value: i32) -> ();

    #[method(name = "Internal_GetDefaultSkin", args = 1)]
    pub fn internal_get_default_skin(skin_mode: i32) -> crate::system::object::Object;

    #[method(name = "Internal_ExitGUI", args = 0)]
    pub fn internal_exit_gui() -> ();

    #[method(name = "MarkGUIChanged", args = 0)]
    pub fn mark_gui_changed() -> ();

    #[method(name = "GetControlID", args = 2)]
    pub fn get_control_id_2(hint: i32, focus: crate::unity_engine::focustype::FocusType) -> i32;

    #[method(name = "GetStateObject", args = 2)]
    pub fn get_state_object(
        t: ::unity2::SystemType,
        control_id: i32,
    ) -> crate::system::object::Object;

    #[method(name = "get_guiIsExiting", args = 0)]
    pub fn get_gui_is_exiting() -> bool;

    #[method(name = "set_guiIsExiting", args = 1)]
    pub fn set_gui_is_exiting(value: bool) -> ();

    #[method(name = "get_hotControl", args = 0)]
    pub fn get_hot_control() -> i32;

    #[method(name = "set_hotControl", args = 1)]
    pub fn set_hot_control(value: i32) -> ();

    #[method(name = "TakeCapture", args = 0)]
    pub fn take_capture_fn() -> ();

    #[method(name = "RemoveCapture", args = 0)]
    pub fn remove_capture() -> ();

    #[method(name = "get_keyboardControl", args = 0)]
    pub fn get_keyboard_control() -> i32;

    #[method(name = "HasKeyFocus", args = 1)]
    pub fn has_key_focus(control_id: i32) -> bool;

    #[method(name = "GetDefaultSkin", args = 0)]
    pub fn get_default_skin() -> crate::unity_engine::guiskin::GUISkin;

    #[method(name = "ProcessEvent", args = 3)]
    pub fn process_event_fn(
        instance_id: i32,
        native_event_ptr: ::unity2::IntPtr,
        result: bool,
    ) -> ();

    #[method(name = "BeginGUI", args = 3)]
    pub fn begin_gui(skin_mode: i32, instance_id: i32, use_gui_layout: i32) -> ();

    #[method(name = "EndGUI", args = 1)]
    pub fn end_gui(layout_type: i32) -> ();

    #[method(name = "ResetGlobalState", args = 0)]
    pub fn reset_global_state() -> ();

    #[method(name = "CheckOnGUI", args = 0)]
    pub fn check_on_gui() -> ();

    #[method(name = "HitTest", args = 3)]
    pub fn hit_test(
        rect: crate::unity_engine::rect::Rect,
        point: crate::unity_engine::vector2::Vector2,
        offset: i32,
    ) -> bool;

    #[method(name = "HitTest", args = 3)]
    pub fn hit_test_2(
        rect: crate::unity_engine::rect::Rect,
        point: crate::unity_engine::vector2::Vector2,
        is_direct_manipulation_device: bool,
    ) -> bool;

    #[method(name = "HitTest", args = 2)]
    pub fn hit_test_3(
        rect: crate::unity_engine::rect::Rect,
        evt: crate::unity_engine::event::Event,
    ) -> bool;

    #[method(name = "GetControlID_Injected", args = 3)]
    pub fn get_control_id_injected(
        hint: i32,
        focus_type: crate::unity_engine::focustype::FocusType,
        rect: crate::unity_engine::rect::Rect,
    ) -> i32;
}
