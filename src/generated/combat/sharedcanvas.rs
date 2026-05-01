
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/sharedcanvas/SharedCanvas.md")))]
#[::unity2::class(namespace = "Combat", name = "SharedCanvas")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct SharedCanvas {
    #[static_field]
    #[rename(name = "s_this")]
    pub s_this: crate::combat::sharedcanvas::SharedCanvas,
    #[rename(name = "m_Canvas")]
    pub m_canvas: crate::unity_engine::canvas::Canvas,
    #[rename(name = "m_bShown")]
    pub m_b_shown: crate::combat::editorprefs_bool::EditorPrefs_Bool,
}

#[cfg(feature = "combat-sharedcanvas")]
#[::unity2::methods]
impl SharedCanvas {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> crate::combat::sharedcanvas::SharedCanvas;

    #[method(name = "get_Canvas", args = 0)]
    pub fn get_canvas() -> crate::unity_engine::canvas::Canvas;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "ForceDestroyCanvas", args = 0)]
    pub fn force_destroy_canvas(self) -> ();

    #[method(name = "ReplaceInputModule", args = 0)]
    pub fn replace_input_module() -> bool;

    #[method(name = "get_IsShown", args = 0)]
    pub fn get_is_shown() -> bool;

    #[method(name = "Show", args = 1)]
    pub fn show(self, v: bool) -> ();

    #[method(name = "Reconnect", args = 0)]
    pub fn reconnect(self) -> ();

    #[method(name = "ClearCanvas", args = 0)]
    pub fn clear_canvas(self) -> ();

    #[method(name = "FromLeft", args = 1)]
    pub fn from_left(margin: i32) -> i32;

    #[method(name = "FromRight", args = 1)]
    pub fn from_right(margin: i32) -> i32;

    #[method(name = "FromTop", args = 1)]
    pub fn from_top(margin: i32) -> i32;

    #[method(name = "FromBottom", args = 1)]
    pub fn from_bottom(margin: i32) -> i32;

    #[method(name = "X", args = 2)]
    pub fn x(value: f32, div: f32) -> f32;

    #[method(name = "Y", args = 2)]
    pub fn y(value: f32, div: f32) -> f32;

    #[method(name = "XY", args = 2)]
    pub fn xy(x: i32, y: i32) -> crate::unity_engine::vector2::Vector2;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-sharedcanvas")]
impl SharedCanvas {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SharedCanvas),
                ::core::stringify!(new),
            )
        });
        <Self as ISharedCanvasMethods>::ctor(this);
        this
    }
}
