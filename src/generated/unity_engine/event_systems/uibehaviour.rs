
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/uibehaviour/UIBehaviour.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "UIBehaviour")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct UIBehaviour {}

#[cfg(feature = "unity_engine-event_systems-uibehaviour")]
#[::unity2::methods]
impl UIBehaviour {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "IsActive", args = 0)]
    pub fn is_active(self) -> bool;

    #[method(name = "OnRectTransformDimensionsChange", args = 0)]
    pub fn on_rect_transform_dimensions_change(self) -> ();

    #[method(name = "OnBeforeTransformParentChanged", args = 0)]
    pub fn on_before_transform_parent_changed(self) -> ();

    #[method(name = "OnTransformParentChanged", args = 0)]
    pub fn on_transform_parent_changed(self) -> ();

    #[method(name = "OnDidApplyAnimationProperties", args = 0)]
    pub fn on_did_apply_animation_properties(self) -> ();

    #[method(name = "OnCanvasGroupChanged", args = 0)]
    pub fn on_canvas_group_changed(self) -> ();

    #[method(name = "OnCanvasHierarchyChanged", args = 0)]
    pub fn on_canvas_hierarchy_changed(self) -> ();

    #[method(name = "IsDestroyed", args = 0)]
    pub fn is_destroyed(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-event_systems-uibehaviour")]
impl UIBehaviour {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UIBehaviour),
                ::core::stringify!(new),
            )
        });
        <Self as IUIBehaviourMethods>::ctor(this);
        this
    }
}
