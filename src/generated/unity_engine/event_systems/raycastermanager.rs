
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/raycastermanager/RaycasterManager.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "RaycasterManager")]
#[parent(crate::system::object::Object)]
pub struct RaycasterManager {
    #[static_field]
    #[rename(name = "s_Raycasters")]
    pub s_raycasters: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::event_systems::baseraycaster::BaseRaycaster,
    >,
}

#[cfg(feature = "unity_engine-event_systems-raycastermanager")]
#[::unity2::methods]
impl RaycasterManager {
    #[method(name = "AddRaycaster", args = 1)]
    pub fn add_raycaster(
        base_raycaster: crate::unity_engine::event_systems::baseraycaster::BaseRaycaster,
    ) -> ();

    #[method(name = "GetRaycasters", args = 0)]
    pub fn get_raycasters() -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::event_systems::baseraycaster::BaseRaycaster,
    >;

    #[method(name = "RemoveRaycasters", args = 1)]
    pub fn remove_raycasters(
        base_raycaster: crate::unity_engine::event_systems::baseraycaster::BaseRaycaster,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
