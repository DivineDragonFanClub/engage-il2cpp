
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::events::unityevent_1::IUnityEvent_1;
use crate::unity_engine::events::unityevent_1::UnityEvent_1;
use crate::unity_engine::events::unityeventbase::IUnityEventBase;
use crate::unity_engine::events::unityeventbase::UnityEventBase;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/eventtrigger/EventTrigger_TriggerEvent.md")))]
#[::unity2::class(
    namespace = "UnityEngine.EventSystems",
    name = "EventTrigger.TriggerEvent"
)]
# [parent (crate :: unity_engine :: events :: unityevent_1 :: UnityEvent_1 < crate :: unity_engine :: event_systems :: baseeventdata :: BaseEventData >)]
pub struct EventTrigger_TriggerEvent {}

#[cfg(feature = "unity_engine-event_systems-eventtrigger")]
#[::unity2::methods]
impl EventTrigger_TriggerEvent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-event_systems-eventtrigger")]
impl EventTrigger_TriggerEvent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventTrigger_TriggerEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IEventTrigger_TriggerEventMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/eventtrigger/EventTrigger.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "EventTrigger")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct EventTrigger {
    #[rename(name = "m_Delegates")]
    pub m_delegates: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::event_systems::eventtrigger::EventTrigger_Entry,
    >,
}

#[cfg(feature = "unity_engine-event_systems-eventtrigger")]
#[::unity2::methods]
impl EventTrigger {
    #[method(name = "get_delegates", args = 0)]
    pub fn get_delegates(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::event_systems::eventtrigger::EventTrigger_Entry,
    >;

    #[method(name = "set_delegates", args = 1)]
    pub fn set_delegates(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::event_systems::eventtrigger::EventTrigger_Entry,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_triggers", args = 0)]
    pub fn get_triggers(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::event_systems::eventtrigger::EventTrigger_Entry,
    >;

    #[method(name = "set_triggers", args = 1)]
    pub fn set_triggers(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::event_systems::eventtrigger::EventTrigger_Entry,
        >,
    ) -> ();

    #[method(name = "Execute", args = 2)]
    pub fn execute(
        self,
        id: crate::unity_engine::event_systems::eventtriggertype::EventTriggerType,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "OnPointerEnter", args = 1)]
    pub fn on_pointer_enter(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnPointerExit", args = 1)]
    pub fn on_pointer_exit(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnDrag", args = 1)]
    pub fn on_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnDrop", args = 1)]
    pub fn on_drop(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnPointerDown", args = 1)]
    pub fn on_pointer_down(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnPointerUp", args = 1)]
    pub fn on_pointer_up(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnPointerClick", args = 1)]
    pub fn on_pointer_click(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnSelect", args = 1)]
    pub fn on_select(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "OnDeselect", args = 1)]
    pub fn on_deselect(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "OnScroll", args = 1)]
    pub fn on_scroll(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnMove", args = 1)]
    pub fn on_move(
        self,
        event_data: crate::unity_engine::event_systems::axiseventdata::AxisEventData,
    ) -> ();

    #[method(name = "OnUpdateSelected", args = 1)]
    pub fn on_update_selected(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "OnInitializePotentialDrag", args = 1)]
    pub fn on_initialize_potential_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnBeginDrag", args = 1)]
    pub fn on_begin_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnEndDrag", args = 1)]
    pub fn on_end_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "OnSubmit", args = 1)]
    pub fn on_submit(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "OnCancel", args = 1)]
    pub fn on_cancel(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();
}

#[cfg(feature = "unity_engine-event_systems-eventtrigger")]
impl EventTrigger {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventTrigger),
                ::core::stringify!(new),
            )
        });
        <Self as IEventTriggerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/eventtrigger/EventTrigger_Entry.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "EventTrigger.Entry")]
#[parent(crate::system::object::Object)]
pub struct EventTrigger_Entry {
    #[rename(name = "eventID")]
    pub event_id: crate::unity_engine::event_systems::eventtriggertype::EventTriggerType,
    #[rename(name = "callback")]
    pub callback: crate::unity_engine::event_systems::eventtrigger::EventTrigger_TriggerEvent,
}

#[cfg(feature = "unity_engine-event_systems-eventtrigger")]
#[::unity2::methods]
impl EventTrigger_Entry {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-event_systems-eventtrigger")]
impl EventTrigger_Entry {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventTrigger_Entry),
                ::core::stringify!(new),
            )
        });
        <Self as IEventTrigger_EntryMethods>::ctor(this);
        this
    }
}
