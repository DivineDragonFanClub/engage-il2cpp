
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/canvasgroup/CanvasGroup.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "CanvasGroup")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct CanvasGroup {}

#[cfg(feature = "unity_engine-canvasgroup")]
#[::unity2::methods]
impl CanvasGroup {
    #[method(name = "get_alpha", args = 0)]
    pub fn get_alpha(self) -> f32;

    #[method(name = "set_alpha", args = 1)]
    pub fn set_alpha(self, value: f32) -> ();

    #[method(name = "get_interactable", args = 0)]
    pub fn get_interactable(self) -> bool;

    #[method(name = "set_interactable", args = 1)]
    pub fn set_interactable(self, value: bool) -> ();

    #[method(name = "get_blocksRaycasts", args = 0)]
    pub fn get_blocks_raycasts(self) -> bool;

    #[method(name = "set_blocksRaycasts", args = 1)]
    pub fn set_blocks_raycasts(self, value: bool) -> ();

    #[method(name = "get_ignoreParentGroups", args = 0)]
    pub fn get_ignore_parent_groups(self) -> bool;

    #[method(name = "set_ignoreParentGroups", args = 1)]
    pub fn set_ignore_parent_groups(self, value: bool) -> ();

    #[method(name = "IsRaycastLocationValid", args = 2)]
    pub fn is_raycast_location_valid(
        self,
        sp: crate::unity_engine::vector2::Vector2,
        event_camera: crate::unity_engine::camera::Camera,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-canvasgroup")]
impl CanvasGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CanvasGroup),
                ::core::stringify!(new),
            )
        });
        <Self as ICanvasGroupMethods>::ctor(this);
        this
    }
}
