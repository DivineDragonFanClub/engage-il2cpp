
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::event_systems::baseinputmodule::BaseInputModule;
use crate::unity_engine::event_systems::baseinputmodule::IBaseInputModule;
use crate::unity_engine::event_systems::uibehaviour::IUIBehaviour;
use crate::unity_engine::event_systems::uibehaviour::UIBehaviour;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/pointerinputmodule/PointerInputModule_ButtonState.md")))]
#[::unity2::class(
    namespace = "UnityEngine.EventSystems",
    name = "PointerInputModule.ButtonState"
)]
#[parent(crate::system::object::Object)]
pub struct PointerInputModule_ButtonState {
# [rename (name = "m_Button")] pub m_button : crate :: unity_engine :: event_systems :: pointereventdata :: PointerEventData_InputButton ,
# [rename (name = "m_EventData")] pub m_event_data : crate :: unity_engine :: event_systems :: pointerinputmodule :: PointerInputModule_MouseButtonEventData ,
}

#[cfg(feature = "unity_engine-event_systems-pointerinputmodule")]
#[::unity2::methods]
impl PointerInputModule_ButtonState {
    #[method(name = "get_eventData", args = 0)]
    pub fn get_event_data (self ,) -> crate :: unity_engine :: event_systems :: pointerinputmodule :: PointerInputModule_MouseButtonEventData ;

    #[method(name = "set_eventData", args = 1)]
    pub fn set_event_data(
        self,
        value : crate :: unity_engine :: event_systems :: pointerinputmodule :: PointerInputModule_MouseButtonEventData,
    ) -> ();

    #[method(name = "get_button", args = 0)]
    pub fn get_button(
        self,
    ) -> crate::unity_engine::event_systems::pointereventdata::PointerEventData_InputButton;

    #[method(name = "set_button", args = 1)]
    pub fn set_button(
        self,
        value: crate::unity_engine::event_systems::pointereventdata::PointerEventData_InputButton,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-event_systems-pointerinputmodule")]
impl PointerInputModule_ButtonState {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PointerInputModule_ButtonState),
                ::core::stringify!(new),
            )
        });
        <Self as IPointerInputModule_ButtonStateMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/pointerinputmodule/PointerInputModule.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "PointerInputModule")]
#[parent(crate::unity_engine::event_systems::baseinputmodule::BaseInputModule)]
pub struct PointerInputModule {
    #[static_field]
    #[rename(name = "kMouseLeftId")]
    pub k_mouse_left_id: i32,
    #[static_field]
    #[rename(name = "kMouseRightId")]
    pub k_mouse_right_id: i32,
    #[static_field]
    #[rename(name = "kMouseMiddleId")]
    pub k_mouse_middle_id: i32,
    #[static_field]
    #[rename(name = "kFakeTouchesId")]
    pub k_fake_touches_id: i32,
    #[rename(name = "m_PointerData")]
    pub m_pointer_data: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    >,
    #[rename(name = "m_MouseState")]
    pub m_mouse_state:
        crate::unity_engine::event_systems::pointerinputmodule::PointerInputModule_MouseState,
}

#[cfg(feature = "unity_engine-event_systems-pointerinputmodule")]
#[::unity2::methods]
impl PointerInputModule {
    #[method(name = "GetPointerData", args = 3)]
    pub fn get_pointer_data(
        self,
        id: i32,
        data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
        create: bool,
    ) -> bool;

    #[method(name = "RemovePointerData", args = 1)]
    pub fn remove_pointer_data(
        self,
        data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "GetTouchPointerEventData", args = 3)]
    pub fn get_touch_pointer_event_data(
        self,
        input: crate::unity_engine::touch::Touch,
        pressed: bool,
        released: bool,
    ) -> crate::unity_engine::event_systems::pointereventdata::PointerEventData;

    #[method(name = "CopyFromTo", args = 2)]
    pub fn copy_from_to(
        self,
        from: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
        to: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "StateForMouseButton", args = 1)]
    pub fn state_for_mouse_button(
        self,
        button_id: i32,
    ) -> crate::unity_engine::event_systems::pointereventdata::PointerEventData_FramePressState;

    #[method(name = "GetMousePointerEventData", args = 0)]
    pub fn get_mouse_pointer_event_data(
        self,
    ) -> crate::unity_engine::event_systems::pointerinputmodule::PointerInputModule_MouseState;

    #[method(name = "GetMousePointerEventData", args = 1)]
    pub fn get_mouse_pointer_event_data_2(
        self,
        id: i32,
    ) -> crate::unity_engine::event_systems::pointerinputmodule::PointerInputModule_MouseState;

    #[method(name = "GetLastPointerEventData", args = 1)]
    pub fn get_last_pointer_event_data(
        self,
        id: i32,
    ) -> crate::unity_engine::event_systems::pointereventdata::PointerEventData;

    #[method(name = "ShouldStartDrag", args = 4)]
    pub fn should_start_drag(
        press_pos: crate::unity_engine::vector2::Vector2,
        current_pos: crate::unity_engine::vector2::Vector2,
        threshold: f32,
        use_drag_threshold: bool,
    ) -> bool;

    #[method(name = "ProcessMove", args = 1)]
    pub fn process_move(
        self,
        pointer_event: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "ProcessDrag", args = 1)]
    pub fn process_drag(
        self,
        pointer_event: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = "IsPointerOverGameObject", args = 1)]
    pub fn is_pointer_over_game_object(self, pointer_id: i32) -> bool;

    #[method(name = "ClearSelection", args = 0)]
    pub fn clear_selection(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "DeselectIfSelectionChanged", args = 2)]
    pub fn deselect_if_selection_changed(
        self,
        current_over_go: crate::unity_engine::gameobject::GameObject,
        pointer_event: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-event_systems-pointerinputmodule")]
impl PointerInputModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PointerInputModule),
                ::core::stringify!(new),
            )
        });
        <Self as IPointerInputModuleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/pointerinputmodule/PointerInputModule_MouseButtonEventData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.EventSystems",
    name = "PointerInputModule.MouseButtonEventData"
)]
#[parent(crate::system::object::Object)]
pub struct PointerInputModule_MouseButtonEventData {
    #[rename(name = "buttonState")]
    pub button_state:
        crate::unity_engine::event_systems::pointereventdata::PointerEventData_FramePressState,
    #[rename(name = "buttonData")]
    pub button_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
}

#[cfg(feature = "unity_engine-event_systems-pointerinputmodule")]
#[::unity2::methods]
impl PointerInputModule_MouseButtonEventData {
    #[method(name = "PressedThisFrame", args = 0)]
    pub fn pressed_this_frame(self) -> bool;

    #[method(name = "ReleasedThisFrame", args = 0)]
    pub fn released_this_frame(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-event_systems-pointerinputmodule")]
impl PointerInputModule_MouseButtonEventData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PointerInputModule_MouseButtonEventData),
                ::core::stringify!(new),
            )
        });
        <Self as IPointerInputModule_MouseButtonEventDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/pointerinputmodule/PointerInputModule_MouseState.md")))]
#[::unity2::class(
    namespace = "UnityEngine.EventSystems",
    name = "PointerInputModule.MouseState"
)]
#[parent(crate::system::object::Object)]
pub struct PointerInputModule_MouseState {
    #[rename(name = "m_TrackedButtons")]
    pub m_tracked_buttons: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::event_systems::pointerinputmodule::PointerInputModule_ButtonState,
    >,
}

#[cfg(feature = "unity_engine-event_systems-pointerinputmodule")]
#[::unity2::methods]
impl PointerInputModule_MouseState {
    #[method(name = "AnyPressesThisFrame", args = 0)]
    pub fn any_presses_this_frame(self) -> bool;

    #[method(name = "AnyReleasesThisFrame", args = 0)]
    pub fn any_releases_this_frame(self) -> bool;

    #[method(name = "GetButtonState", args = 1)]
    pub fn get_button_state(
        self,
        button: crate::unity_engine::event_systems::pointereventdata::PointerEventData_InputButton,
    ) -> crate::unity_engine::event_systems::pointerinputmodule::PointerInputModule_ButtonState;

    #[method(name = "SetButtonState", args = 3)]
    pub fn set_button_state(
        self,
        button: crate::unity_engine::event_systems::pointereventdata::PointerEventData_InputButton,
        state_for_mouse_button : crate :: unity_engine :: event_systems :: pointereventdata :: PointerEventData_FramePressState,
        data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-event_systems-pointerinputmodule")]
impl PointerInputModule_MouseState {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PointerInputModule_MouseState),
                ::core::stringify!(new),
            )
        });
        <Self as IPointerInputModule_MouseStateMethods>::ctor(this);
        this
    }
}
