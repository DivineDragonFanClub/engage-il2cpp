
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/eventsystem/EventSystem.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "EventSystem")]
#[parent(crate::unity_engine::event_systems::uibehaviour::UIBehaviour)]
pub struct EventSystem {
    #[rename(name = "m_SystemInputModules")]
    pub m_system_input_modules: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::event_systems::baseinputmodule::BaseInputModule,
    >,
    #[rename(name = "m_CurrentInputModule")]
    pub m_current_input_module:
        crate::unity_engine::event_systems::baseinputmodule::BaseInputModule,
    #[static_field]
    #[rename(name = "m_EventSystems")]
    pub m_event_systems: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::event_systems::eventsystem::EventSystem,
    >,
    #[rename(name = "m_FirstSelected")]
    pub m_first_selected: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_sendNavigationEvents")]
    pub m_send_navigation_events: bool,
    #[rename(name = "m_DragThreshold")]
    pub m_drag_threshold: i32,
    #[rename(name = "m_CurrentSelected")]
    pub m_current_selected: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_HasFocus")]
    pub m_has_focus: bool,
    #[rename(name = "m_SelectionGuard")]
    pub m_selection_guard: bool,
    #[rename(name = "m_DummyData")]
    pub m_dummy_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    #[static_field]
    #[rename(name = "s_RaycastComparer")]
    pub s_raycast_comparer: crate::system::comparison_1::Comparison_1<
        crate::unity_engine::event_systems::raycastresult::RaycastResult,
    >,
}

#[cfg(feature = "unity_engine-event_systems-eventsystem")]
#[::unity2::methods]
impl EventSystem {
    #[method(name = "get_current", args = 0)]
    pub fn get_current() -> crate::unity_engine::event_systems::eventsystem::EventSystem;

    #[method(name = "set_current", args = 1)]
    pub fn set_current(value: crate::unity_engine::event_systems::eventsystem::EventSystem) -> ();

    #[method(name = "get_sendNavigationEvents", args = 0)]
    pub fn get_send_navigation_events(self) -> bool;

    #[method(name = "set_sendNavigationEvents", args = 1)]
    pub fn set_send_navigation_events(self, value: bool) -> ();

    #[method(name = "get_pixelDragThreshold", args = 0)]
    pub fn get_pixel_drag_threshold(self) -> i32;

    #[method(name = "set_pixelDragThreshold", args = 1)]
    pub fn set_pixel_drag_threshold(self, value: i32) -> ();

    #[method(name = "get_currentInputModule", args = 0)]
    pub fn get_current_input_module(
        self,
    ) -> crate::unity_engine::event_systems::baseinputmodule::BaseInputModule;

    #[method(name = "get_firstSelectedGameObject", args = 0)]
    pub fn get_first_selected_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_firstSelectedGameObject", args = 1)]
    pub fn set_first_selected_game_object(
        self,
        value: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "get_currentSelectedGameObject", args = 0)]
    pub fn get_current_selected_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_lastSelectedGameObject", args = 0)]
    pub fn get_last_selected_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_isFocused", args = 0)]
    pub fn get_is_focused(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "UpdateModules", args = 0)]
    pub fn update_modules(self) -> ();

    #[method(name = "get_alreadySelecting", args = 0)]
    pub fn get_already_selecting(self) -> bool;

    #[method(name = "SetSelectedGameObject", args = 2)]
    pub fn set_selected_game_object(
        self,
        selected: crate::unity_engine::gameobject::GameObject,
        pointer: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = "get_baseEventDataCache", args = 0)]
    pub fn get_base_event_data_cache(
        self,
    ) -> crate::unity_engine::event_systems::baseeventdata::BaseEventData;

    #[method(name = "SetSelectedGameObject", args = 1)]
    pub fn set_selected_game_object_2(
        self,
        selected: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "RaycastComparer", args = 2)]
    pub fn raycast_comparer(
        lhs: crate::unity_engine::event_systems::raycastresult::RaycastResult,
        rhs: crate::unity_engine::event_systems::raycastresult::RaycastResult,
    ) -> i32;

    #[method(name = "RaycastAll", args = 2)]
    pub fn raycast_all(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
        raycast_results: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::event_systems::raycastresult::RaycastResult,
        >,
    ) -> ();

    #[method(name = "IsPointerOverGameObject", args = 0)]
    pub fn is_pointer_over_game_object(self) -> bool;

    #[method(name = "IsPointerOverGameObject", args = 1)]
    pub fn is_pointer_over_game_object_2(self, pointer_id: i32) -> bool;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "TickModules", args = 0)]
    pub fn tick_modules(self) -> ();

    #[method(name = "OnApplicationFocus", args = 1)]
    pub fn on_application_focus(self, has_focus: bool) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "ChangeEventModule", args = 1)]
    pub fn change_event_module(
        self,
        module: crate::unity_engine::event_systems::baseinputmodule::BaseInputModule,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-event_systems-eventsystem")]
impl EventSystem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventSystem),
                ::core::stringify!(new),
            )
        });
        <Self as IEventSystemMethods>::ctor(this);
        this
    }
}
