
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guiclip/GUIClip.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUIClip")]
#[parent(crate::system::object::Object)]
pub struct GUIClip {}

#[cfg(feature = "unity_engine-guiclip")]
#[::unity2::methods]
impl GUIClip {
    #[method(name = "get_visibleRect", args = 0)]
    pub fn get_visible_rect() -> crate::unity_engine::rect::Rect;

    #[method(name = "Internal_Push", args = 4)]
    pub fn internal_push(
        screen_rect: crate::unity_engine::rect::Rect,
        scroll_offset: crate::unity_engine::vector2::Vector2,
        render_offset: crate::unity_engine::vector2::Vector2,
        reset_offset: bool,
    ) -> ();

    #[method(name = "Internal_Pop", args = 0)]
    pub fn internal_pop() -> ();

    #[method(name = "SetMatrix", args = 1)]
    pub fn set_matrix(m: crate::unity_engine::matrix4x4::Matrix4x4) -> ();

    #[method(name = "Push", args = 4)]
    pub fn push(
        screen_rect: crate::unity_engine::rect::Rect,
        scroll_offset: crate::unity_engine::vector2::Vector2,
        render_offset: crate::unity_engine::vector2::Vector2,
        reset_offset: bool,
    ) -> ();

    #[method(name = "Pop", args = 0)]
    pub fn pop() -> ();

    #[method(name = "get_visibleRect_Injected", args = 1)]
    pub fn get_visible_rect_injected(ret: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "Internal_Push_Injected", args = 4)]
    pub fn internal_push_injected(
        screen_rect: crate::unity_engine::rect::Rect,
        scroll_offset: crate::unity_engine::vector2::Vector2,
        render_offset: crate::unity_engine::vector2::Vector2,
        reset_offset: bool,
    ) -> ();

    #[method(name = "SetMatrix_Injected", args = 1)]
    pub fn set_matrix_injected(m: crate::unity_engine::matrix4x4::Matrix4x4) -> ();
}
