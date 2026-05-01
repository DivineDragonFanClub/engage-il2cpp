
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::event_systems::abstracteventdata::AbstractEventData;
use crate::unity_engine::event_systems::abstracteventdata::IAbstractEventData;
use crate::unity_engine::event_systems::baseeventdata::BaseEventData;
use crate::unity_engine::event_systems::baseeventdata::IBaseEventData;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/pointereventdata/PointerEventData.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "PointerEventData")]
#[parent(crate::unity_engine::event_systems::baseeventdata::BaseEventData)]
pub struct PointerEventData {
    #[rename(name = "m_PointerPress")]
    pub m_pointer_press: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "hovered")]
    pub hovered: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "unity_engine-event_systems-pointereventdata")]
#[::unity2::methods]
impl PointerEventData {
    #[method(name = "get_pointerEnter", args = 0)]
    pub fn get_pointer_enter(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_pointerEnter", args = 1)]
    pub fn set_pointer_enter(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_lastPress", args = 0)]
    pub fn get_last_press(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_lastPress", args = 1)]
    pub fn set_last_press(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_rawPointerPress", args = 0)]
    pub fn get_raw_pointer_press(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_rawPointerPress", args = 1)]
    pub fn set_raw_pointer_press(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_pointerDrag", args = 0)]
    pub fn get_pointer_drag(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_pointerDrag", args = 1)]
    pub fn set_pointer_drag(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_pointerClick", args = 0)]
    pub fn get_pointer_click(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_pointerClick", args = 1)]
    pub fn set_pointer_click(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_pointerCurrentRaycast", args = 0)]
    pub fn get_pointer_current_raycast(
        self,
    ) -> crate::unity_engine::event_systems::raycastresult::RaycastResult;

    #[method(name = "set_pointerCurrentRaycast", args = 1)]
    pub fn set_pointer_current_raycast(
        self,
        value: crate::unity_engine::event_systems::raycastresult::RaycastResult,
    ) -> ();

    #[method(name = "get_pointerPressRaycast", args = 0)]
    pub fn get_pointer_press_raycast(
        self,
    ) -> crate::unity_engine::event_systems::raycastresult::RaycastResult;

    #[method(name = "set_pointerPressRaycast", args = 1)]
    pub fn set_pointer_press_raycast(
        self,
        value: crate::unity_engine::event_systems::raycastresult::RaycastResult,
    ) -> ();

    #[method(name = "get_eligibleForClick", args = 0)]
    pub fn get_eligible_for_click(self) -> bool;

    #[method(name = "set_eligibleForClick", args = 1)]
    pub fn set_eligible_for_click(self, value: bool) -> ();

    #[method(name = "get_pointerId", args = 0)]
    pub fn get_pointer_id(self) -> i32;

    #[method(name = "set_pointerId", args = 1)]
    pub fn set_pointer_id(self, value: i32) -> ();

    #[method(name = "get_position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_position", args = 1)]
    pub fn set_position(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_delta", args = 0)]
    pub fn get_delta(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_delta", args = 1)]
    pub fn set_delta(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_pressPosition", args = 0)]
    pub fn get_press_position(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_pressPosition", args = 1)]
    pub fn set_press_position(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_worldPosition", args = 0)]
    pub fn get_world_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_worldPosition", args = 1)]
    pub fn set_world_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_worldNormal", args = 0)]
    pub fn get_world_normal(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_worldNormal", args = 1)]
    pub fn set_world_normal(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_clickTime", args = 0)]
    pub fn get_click_time(self) -> f32;

    #[method(name = "set_clickTime", args = 1)]
    pub fn set_click_time(self, value: f32) -> ();

    #[method(name = "get_clickCount", args = 0)]
    pub fn get_click_count(self) -> i32;

    #[method(name = "set_clickCount", args = 1)]
    pub fn set_click_count(self, value: i32) -> ();

    #[method(name = "get_scrollDelta", args = 0)]
    pub fn get_scroll_delta(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_scrollDelta", args = 1)]
    pub fn set_scroll_delta(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_useDragThreshold", args = 0)]
    pub fn get_use_drag_threshold(self) -> bool;

    #[method(name = "set_useDragThreshold", args = 1)]
    pub fn set_use_drag_threshold(self, value: bool) -> ();

    #[method(name = "get_dragging", args = 0)]
    pub fn get_dragging(self) -> bool;

    #[method(name = "set_dragging", args = 1)]
    pub fn set_dragging(self, value: bool) -> ();

    #[method(name = "get_button", args = 0)]
    pub fn get_button(
        self,
    ) -> crate::unity_engine::event_systems::pointereventdata::PointerEventData_InputButton;

    #[method(name = "set_button", args = 1)]
    pub fn set_button(
        self,
        value: crate::unity_engine::event_systems::pointereventdata::PointerEventData_InputButton,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        event_system: crate::unity_engine::event_systems::eventsystem::EventSystem,
    ) -> ();

    #[method(name = "IsPointerMoving", args = 0)]
    pub fn is_pointer_moving(self) -> bool;

    #[method(name = "IsScrolling", args = 0)]
    pub fn is_scrolling(self) -> bool;

    #[method(name = "get_enterEventCamera", args = 0)]
    pub fn get_enter_event_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "get_pressEventCamera", args = 0)]
    pub fn get_press_event_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "get_pointerPress", args = 0)]
    pub fn get_pointer_press(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_pointerPress", args = 1)]
    pub fn set_pointer_press(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "unity_engine-event_systems-pointereventdata")]
impl PointerEventData {
    pub fn new(event_system: crate::unity_engine::event_systems::eventsystem::EventSystem) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PointerEventData),
                ::core::stringify!(new),
            )
        });
        <Self as IPointerEventDataMethods>::ctor(this, event_system);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/pointereventdata/PointerEventData_InputButton.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct PointerEventData_InputButton {
    pub value: i32,
}

impl ::unity2::ClassIdentity for PointerEventData_InputButton {
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";

    const NAME: &'static str = "PointerEventData.InputButton";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PointerEventData_InputButton {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl PointerEventData_InputButton {
    pub fn left() -> Self {
        Self { value: 0 }
    }

    pub fn right() -> Self {
        Self { value: 1 }
    }

    pub fn middle() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/pointereventdata/PointerEventData_FramePressState.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct PointerEventData_FramePressState {
    pub value: i32,
}

impl ::unity2::ClassIdentity for PointerEventData_FramePressState {
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";

    const NAME: &'static str = "PointerEventData.FramePressState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PointerEventData_FramePressState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl PointerEventData_FramePressState {
    pub fn pressed() -> Self {
        Self { value: 0 }
    }

    pub fn released() -> Self {
        Self { value: 1 }
    }

    pub fn pressed_and_released() -> Self {
        Self { value: 2 }
    }

    pub fn not_changed() -> Self {
        Self { value: 3 }
    }
}
