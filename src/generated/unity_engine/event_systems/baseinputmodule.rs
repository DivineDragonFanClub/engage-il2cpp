
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/baseinputmodule/BaseInputModule.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "BaseInputModule")]
#[parent(crate::unity_engine::event_systems::uibehaviour::UIBehaviour)]
pub struct BaseInputModule {
    #[rename(name = "m_RaycastResultCache")]
    pub m_raycast_result_cache: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::event_systems::raycastresult::RaycastResult,
    >,
    #[rename(name = "m_AxisEventData")]
    pub m_axis_event_data: crate::unity_engine::event_systems::axiseventdata::AxisEventData,
    #[rename(name = "m_EventSystem")]
    pub m_event_system: crate::unity_engine::event_systems::eventsystem::EventSystem,
    #[rename(name = "m_BaseEventData")]
    pub m_base_event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    #[rename(name = "m_InputOverride")]
    pub m_input_override: crate::unity_engine::event_systems::baseinput::BaseInput,
    #[rename(name = "m_DefaultInput")]
    pub m_default_input: crate::unity_engine::event_systems::baseinput::BaseInput,
}

#[cfg(feature = "unity_engine-event_systems-baseinputmodule")]
#[::unity2::methods]
impl BaseInputModule {
    #[method(name = "get_input", args = 0)]
    pub fn get_input(self) -> crate::unity_engine::event_systems::baseinput::BaseInput;

    #[method(name = "get_inputOverride", args = 0)]
    pub fn get_input_override(self) -> crate::unity_engine::event_systems::baseinput::BaseInput;

    #[method(name = "set_inputOverride", args = 1)]
    pub fn set_input_override(
        self,
        value: crate::unity_engine::event_systems::baseinput::BaseInput,
    ) -> ();

    #[method(name = "get_eventSystem", args = 0)]
    pub fn get_event_system(self) -> crate::unity_engine::event_systems::eventsystem::EventSystem;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "Process", args = 0)]
    pub fn process(self) -> ();

    #[method(name = "FindFirstRaycast", args = 1)]
    pub fn find_first_raycast(
        candidates: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::event_systems::raycastresult::RaycastResult,
        >,
    ) -> crate::unity_engine::event_systems::raycastresult::RaycastResult;

    #[method(name = "DetermineMoveDirection", args = 2)]
    pub fn determine_move_direction(
        x: f32,
        y: f32,
    ) -> crate::unity_engine::event_systems::movedirection::MoveDirection;

    #[method(name = "DetermineMoveDirection", args = 3)]
    pub fn determine_move_direction_2(
        x: f32,
        y: f32,
        dead_zone: f32,
    ) -> crate::unity_engine::event_systems::movedirection::MoveDirection;

    #[method(name = "FindCommonRoot", args = 2)]
    pub fn find_common_root(
        g1: crate::unity_engine::gameobject::GameObject,
        g2: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "HandlePointerExitAndEnter", args = 2)]
    pub fn handle_pointer_exit_and_enter(
        self,
        current_pointer_data : crate :: unity_engine :: event_systems :: pointereventdata :: PointerEventData,
        new_enter_target: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "GetAxisEventData", args = 3)]
    pub fn get_axis_event_data(
        self,
        x: f32,
        y: f32,
        move_dead_zone: f32,
    ) -> crate::unity_engine::event_systems::axiseventdata::AxisEventData;

    #[method(name = "GetBaseEventData", args = 0)]
    pub fn get_base_event_data(
        self,
    ) -> crate::unity_engine::event_systems::baseeventdata::BaseEventData;

    #[method(name = "IsPointerOverGameObject", args = 1)]
    pub fn is_pointer_over_game_object(self, pointer_id: i32) -> bool;

    #[method(name = "ShouldActivateModule", args = 0)]
    pub fn should_activate_module(self) -> bool;

    #[method(name = "DeactivateModule", args = 0)]
    pub fn deactivate_module(self) -> ();

    #[method(name = "ActivateModule", args = 0)]
    pub fn activate_module(self) -> ();

    #[method(name = "UpdateModule", args = 0)]
    pub fn update_module(self) -> ();

    #[method(name = "IsModuleSupported", args = 0)]
    pub fn is_module_supported(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-event_systems-baseinputmodule")]
impl BaseInputModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseInputModule),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseInputModuleMethods>::ctor(this);
        this
    }
}
