
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::event_systems::abstracteventdata::AbstractEventData;
use crate::unity_engine::event_systems::abstracteventdata::IAbstractEventData;
use crate::unity_engine::event_systems::baseeventdata::BaseEventData;
use crate::unity_engine::event_systems::baseeventdata::IBaseEventData;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/axiseventdata/AxisEventData.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "AxisEventData")]
#[parent(crate::unity_engine::event_systems::baseeventdata::BaseEventData)]
pub struct AxisEventData {}

#[cfg(feature = "unity_engine-event_systems-axiseventdata")]
#[::unity2::methods]
impl AxisEventData {
    #[method(name = "get_moveVector", args = 0)]
    pub fn get_move_vector(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_moveVector", args = 1)]
    pub fn set_move_vector(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_moveDir", args = 0)]
    pub fn get_move_dir(self) -> crate::unity_engine::event_systems::movedirection::MoveDirection;

    #[method(name = "set_moveDir", args = 1)]
    pub fn set_move_dir(
        self,
        value: crate::unity_engine::event_systems::movedirection::MoveDirection,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_system: crate::unity_engine::event_systems::eventsystem::EventSystem,
    ) -> ();
}

#[cfg(feature = "unity_engine-event_systems-axiseventdata")]
impl AxisEventData {
    pub fn new(event_system: crate::unity_engine::event_systems::eventsystem::EventSystem) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AxisEventData),
                ::core::stringify!(new),
            )
        });
        <Self as IAxisEventDataMethods>::ctor(this, event_system);
        this
    }
}
