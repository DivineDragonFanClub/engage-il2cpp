
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/canvasmanager/CanvasManager.md")))]
#[::unity2::class(namespace = "App", name = "CanvasManager")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CanvasManager {
    #[rename(name = "m_LocalPosition")]
    pub m_local_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_LocalRotation")]
    pub m_local_rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_LocalScale")]
    pub m_local_scale: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_AnchorMin")]
    pub m_anchor_min: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_AnchorMax")]
    pub m_anchor_max: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_AnchoredPosition")]
    pub m_anchored_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_SizeDelta")]
    pub m_size_delta: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_Pivot")]
    pub m_pivot: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_SelfDestroy")]
    pub m_self_destroy: bool,
}

#[cfg(feature = "app-canvasmanager")]
#[::unity2::methods]
impl CanvasManager {
    #[method(name = "SetTargetDisplay", args = 2)]
    pub fn set_target_display(canvas: crate::unity_engine::canvas::Canvas, index: i32) -> ();

    #[method(name = "SetTargetDisplay", args = 2)]
    pub fn set_target_display_2(canvas: crate::unity_engine::canvas::Canvas, enabled: bool) -> ();

    #[method(name = "TryGetRootCanvas", args = 2)]
    pub fn try_get_root_canvas(
        transform: crate::unity_engine::transform::Transform,
        canvas: crate::unity_engine::canvas::Canvas,
    ) -> bool;

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible(self) -> bool;

    #[method(name = "SetVisible", args = 1)]
    pub fn set_visible(self, enabled: bool) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SetVisible", args = 2)]
    pub fn set_visible_2(go: crate::unity_engine::gameobject::GameObject, enabled: bool) -> ();

    #[method(name = "IsVisible", args = 1)]
    pub fn is_visible_2(go: crate::unity_engine::gameobject::GameObject) -> bool;

    #[method(name = "Show", args = 1)]
    pub fn show_2(go: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Hide", args = 1)]
    pub fn hide_2(go: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "OnTransformChildrenChanged", args = 0)]
    pub fn on_transform_children_changed(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-canvasmanager")]
impl CanvasManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CanvasManager),
                ::core::stringify!(new),
            )
        });
        <Self as ICanvasManagerMethods>::ctor(this);
        this
    }
}
