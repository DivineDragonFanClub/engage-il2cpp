
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/input/Input.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Input")]
#[parent(crate::system::object::Object)]
pub struct Input {}

#[cfg(feature = "unity_engine-input")]
#[::unity2::methods]
impl Input {
    #[method(name = "GetKeyInt", args = 1)]
    pub fn get_key_int(key: crate::unity_engine::keycode::KeyCode) -> bool;

    #[method(name = "GetKeyUpInt", args = 1)]
    pub fn get_key_up_int(key: crate::unity_engine::keycode::KeyCode) -> bool;

    #[method(name = "GetKeyDownInt", args = 1)]
    pub fn get_key_down_int(key: crate::unity_engine::keycode::KeyCode) -> bool;

    #[method(name = "GetAxis", args = 1)]
    pub fn get_axis(axis_name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetAxisRaw", args = 1)]
    pub fn get_axis_raw(axis_name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetButton", args = 1)]
    pub fn get_button(button_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetButtonDown", args = 1)]
    pub fn get_button_down(button_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetButtonUp", args = 1)]
    pub fn get_button_up(button_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetMouseButton", args = 1)]
    pub fn get_mouse_button(button: i32) -> bool;

    #[method(name = "GetMouseButtonDown", args = 1)]
    pub fn get_mouse_button_down(button: i32) -> bool;

    #[method(name = "GetMouseButtonUp", args = 1)]
    pub fn get_mouse_button_up(button: i32) -> bool;

    #[method(name = "GetTouch", args = 1)]
    pub fn get_touch(index: i32) -> crate::unity_engine::touch::Touch;

    #[method(name = "GetKey", args = 1)]
    pub fn get_key(key: crate::unity_engine::keycode::KeyCode) -> bool;

    #[method(name = "GetKeyUp", args = 1)]
    pub fn get_key_up(key: crate::unity_engine::keycode::KeyCode) -> bool;

    #[method(name = "GetKeyDown", args = 1)]
    pub fn get_key_down(key: crate::unity_engine::keycode::KeyCode) -> bool;

    #[method(name = "get_mousePosition", args = 0)]
    pub fn get_mouse_position() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_mouseScrollDelta", args = 0)]
    pub fn get_mouse_scroll_delta() -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_imeCompositionMode", args = 0)]
    pub fn get_ime_composition_mode() -> crate::unity_engine::imecompositionmode::IMECompositionMode;

    #[method(name = "set_imeCompositionMode", args = 1)]
    pub fn set_ime_composition_mode(
        value: crate::unity_engine::imecompositionmode::IMECompositionMode,
    ) -> ();

    #[method(name = "get_compositionString", args = 0)]
    pub fn get_composition_string() -> ::unity2::Il2CppString;

    #[method(name = "get_compositionCursorPos", args = 0)]
    pub fn get_composition_cursor_pos() -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_compositionCursorPos", args = 1)]
    pub fn set_composition_cursor_pos(value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_mousePresent", args = 0)]
    pub fn get_mouse_present() -> bool;

    #[method(name = "get_touchCount", args = 0)]
    pub fn get_touch_count() -> i32;

    #[method(name = "get_touchSupported", args = 0)]
    pub fn get_touch_supported() -> bool;

    #[method(name = "get_touches", args = 0)]
    pub fn get_touches() -> ::unity2::Array<crate::unity_engine::touch::Touch>;

    #[method(name = "GetTouch_Injected", args = 2)]
    pub fn get_touch_injected(index: i32, ret: crate::unity_engine::touch::Touch) -> ();

    #[method(name = "get_mousePosition_Injected", args = 1)]
    pub fn get_mouse_position_injected(ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_mouseScrollDelta_Injected", args = 1)]
    pub fn get_mouse_scroll_delta_injected(ret: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_compositionCursorPos_Injected", args = 1)]
    pub fn get_composition_cursor_pos_injected(ret: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "set_compositionCursorPos_Injected", args = 1)]
    pub fn set_composition_cursor_pos_injected(value: crate::unity_engine::vector2::Vector2) -> ();
}
