
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::event_systems::abstracteventdata::AbstractEventData;
use crate::unity_engine::event_systems::abstracteventdata::IAbstractEventData;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/baseeventdata/BaseEventData.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "BaseEventData")]
#[parent(crate::unity_engine::event_systems::abstracteventdata::AbstractEventData)]
pub struct BaseEventData {
    #[rename(name = "m_EventSystem")]
    pub m_event_system: crate::unity_engine::event_systems::eventsystem::EventSystem,
}

#[cfg(feature = "unity_engine-event_systems-baseeventdata")]
#[::unity2::methods]
impl BaseEventData {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_system: crate::unity_engine::event_systems::eventsystem::EventSystem,
    ) -> ();

    #[method(name = "get_currentInputModule", args = 0)]
    pub fn get_current_input_module(
        self,
    ) -> crate::unity_engine::event_systems::baseinputmodule::BaseInputModule;

    #[method(name = "get_selectedObject", args = 0)]
    pub fn get_selected_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_selectedObject", args = 1)]
    pub fn set_selected_object(self, value: crate::unity_engine::gameobject::GameObject) -> ();
}

#[cfg(feature = "unity_engine-event_systems-baseeventdata")]
impl BaseEventData {
    pub fn new(event_system: crate::unity_engine::event_systems::eventsystem::EventSystem) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseEventData),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseEventDataMethods>::ctor(this, event_system);
        this
    }
}
