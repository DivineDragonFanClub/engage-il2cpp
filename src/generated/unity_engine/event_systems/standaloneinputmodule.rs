
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::baseinputmodule::BaseInputModule;
use crate::unity_engine::event_systems::baseinputmodule::IBaseInputModule;
use crate::unity_engine::event_systems::pointerinputmodule::IPointerInputModule;
use crate::unity_engine::event_systems::pointerinputmodule::PointerInputModule;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/standaloneinputmodule/StandaloneInputModule.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "StandaloneInputModule")]
#[parent(crate::unity_engine::event_systems::pointerinputmodule::PointerInputModule)]
pub struct StandaloneInputModule {
    #[rename(name = "m_PrevActionTime")]
    pub m_prev_action_time: f32,
    #[rename(name = "m_LastMoveVector")]
    pub m_last_move_vector: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_ConsecutiveMoveCount")]
    pub m_consecutive_move_count: i32,
    #[rename(name = "m_LastMousePosition")]
    pub m_last_mouse_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_MousePosition")]
    pub m_mouse_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_CurrentFocusedGameObject")]
    pub m_current_focused_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_InputPointerEvent")]
    pub m_input_pointer_event:
        crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    #[rename(name = "m_HorizontalAxis")]
    pub m_horizontal_axis: ::unity2::Il2CppString,
    #[rename(name = "m_VerticalAxis")]
    pub m_vertical_axis: ::unity2::Il2CppString,
    #[rename(name = "m_SubmitButton")]
    pub m_submit_button: ::unity2::Il2CppString,
    #[rename(name = "m_CancelButton")]
    pub m_cancel_button: ::unity2::Il2CppString,
    #[rename(name = "m_InputActionsPerSecond")]
    pub m_input_actions_per_second: f32,
    #[rename(name = "m_RepeatDelay")]
    pub m_repeat_delay: f32,
    #[rename(name = "m_ForceModuleActive")]
    pub m_force_module_active: bool,
}

#[cfg(feature = "unity_engine-event_systems-standaloneinputmodule")]
#[::unity2::methods]
impl StandaloneInputModule {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_inputMode", args = 0)]
    pub fn get_input_mode(
        self,
    ) -> crate::unity_engine::event_systems::standaloneinputmodule::StandaloneInputModule_InputMode;

    #[method(name = "get_allowActivationOnMobileDevice", args = 0)]
    pub fn get_allow_activation_on_mobile_device(self) -> bool;

    #[method(name = "set_allowActivationOnMobileDevice", args = 1)]
    pub fn set_allow_activation_on_mobile_device(self, value: bool) -> ();

    #[method(name = "get_forceModuleActive", args = 0)]
    pub fn get_force_module_active(self) -> bool;

    #[method(name = "set_forceModuleActive", args = 1)]
    pub fn set_force_module_active(self, value: bool) -> ();

    #[method(name = "get_inputActionsPerSecond", args = 0)]
    pub fn get_input_actions_per_second(self) -> f32;

    #[method(name = "set_inputActionsPerSecond", args = 1)]
    pub fn set_input_actions_per_second(self, value: f32) -> ();

    #[method(name = "get_repeatDelay", args = 0)]
    pub fn get_repeat_delay(self) -> f32;

    #[method(name = "set_repeatDelay", args = 1)]
    pub fn set_repeat_delay(self, value: f32) -> ();

    #[method(name = "get_horizontalAxis", args = 0)]
    pub fn get_horizontal_axis(self) -> ::unity2::Il2CppString;

    #[method(name = "set_horizontalAxis", args = 1)]
    pub fn set_horizontal_axis(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_verticalAxis", args = 0)]
    pub fn get_vertical_axis(self) -> ::unity2::Il2CppString;

    #[method(name = "set_verticalAxis", args = 1)]
    pub fn set_vertical_axis(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_submitButton", args = 0)]
    pub fn get_submit_button(self) -> ::unity2::Il2CppString;

    #[method(name = "set_submitButton", args = 1)]
    pub fn set_submit_button(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_cancelButton", args = 0)]
    pub fn get_cancel_button(self) -> ::unity2::Il2CppString;

    #[method(name = "set_cancelButton", args = 1)]
    pub fn set_cancel_button(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "ShouldIgnoreEventsOnNoFocus", args = 0)]
    pub fn should_ignore_events_on_no_focus(self) -> bool;

    #[method(name = "UpdateModule", args = 0)]
    pub fn update_module(self) -> ();

    #[method(name = "ReleaseMouse", args = 2)]
    pub fn release_mouse(
        self,
        pointer_event: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
        current_over_go: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "IsModuleSupported", args = 0)]
    pub fn is_module_supported(self) -> bool;

    #[method(name = "ShouldActivateModule", args = 0)]
    pub fn should_activate_module(self) -> bool;

    #[method(name = "ActivateModule", args = 0)]
    pub fn activate_module(self) -> ();

    #[method(name = "DeactivateModule", args = 0)]
    pub fn deactivate_module(self) -> ();

    #[method(name = "Process", args = 0)]
    pub fn process(self) -> ();

    #[method(name = "ProcessTouchEvents", args = 0)]
    pub fn process_touch_events(self) -> bool;

    #[method(name = "ProcessTouchPress", args = 3)]
    pub fn process_touch_press(
        self,
        pointer_event: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
        pressed: bool,
        released: bool,
    ) -> ();

    #[method(name = "SendSubmitEventToSelectedObject", args = 0)]
    pub fn send_submit_event_to_selected_object(self) -> bool;

    #[method(name = "GetRawMoveVector", args = 0)]
    pub fn get_raw_move_vector(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "SendMoveEventToSelectedObject", args = 0)]
    pub fn send_move_event_to_selected_object(self) -> bool;

    #[method(name = "ProcessMouseEvent", args = 0)]
    pub fn process_mouse_event(self) -> ();

    #[method(name = "ForceAutoSelect", args = 0)]
    pub fn force_auto_select(self) -> bool;

    #[method(name = "ProcessMouseEvent", args = 1)]
    pub fn process_mouse_event_2(self, id: i32) -> ();

    #[method(name = "SendUpdateEventToSelectedObject", args = 0)]
    pub fn send_update_event_to_selected_object(self) -> bool;

    #[method(name = "ProcessMousePress", args = 1)]
    pub fn process_mouse_press(
        self,
        data : crate :: unity_engine :: event_systems :: pointerinputmodule :: PointerInputModule_MouseButtonEventData,
    ) -> ();

    #[method(name = "GetCurrentFocusedGameObject", args = 0)]
    pub fn get_current_focused_game_object(self) -> crate::unity_engine::gameobject::GameObject;
}

#[cfg(feature = "unity_engine-event_systems-standaloneinputmodule")]
impl StandaloneInputModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StandaloneInputModule),
                ::core::stringify!(new),
            )
        });
        <Self as IStandaloneInputModuleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/standaloneinputmodule/StandaloneInputModule_InputMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct StandaloneInputModule_InputMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for StandaloneInputModule_InputMode {
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";

    const NAME: &'static str = "StandaloneInputModule.InputMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for StandaloneInputModule_InputMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl StandaloneInputModule_InputMode {
    pub fn mouse() -> Self {
        Self { value: 0 }
    }

    pub fn buttons() -> Self {
        Self { value: 1 }
    }
}
