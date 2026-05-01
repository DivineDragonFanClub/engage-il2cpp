
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/mousepositiondebug/MousePositionDebug.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "MousePositionDebug")]
#[parent(crate::system::object::Object)]
pub struct MousePositionDebug {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: crate::unity_engine::rendering::mousepositiondebug::MousePositionDebug,
}

#[cfg(feature = "unity_engine-rendering-mousepositiondebug")]
#[::unity2::methods]
impl MousePositionDebug {
    #[method(name = "get_instance", args = 0)]
    pub fn get_instance() -> crate::unity_engine::rendering::mousepositiondebug::MousePositionDebug;

    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "GetMousePosition", args = 2)]
    pub fn get_mouse_position(
        self,
        screen_height: f32,
        scene_view: bool,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetInputMousePosition", args = 0)]
    pub fn get_input_mouse_position(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetMouseClickPosition", args = 1)]
    pub fn get_mouse_click_position(
        self,
        screen_height: f32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-mousepositiondebug")]
impl MousePositionDebug {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MousePositionDebug),
                ::core::stringify!(new),
            )
        });
        <Self as IMousePositionDebugMethods>::ctor(this);
        this
    }
}
