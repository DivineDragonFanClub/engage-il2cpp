
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event/Event.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Event")]
#[parent(crate::system::object::Object)]
pub struct Event {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[static_field]
    #[rename(name = "s_Current")]
    pub s_current: crate::unity_engine::event::Event,
    #[static_field]
    #[rename(name = "s_MasterEvent")]
    pub s_master_event: crate::unity_engine::event::Event,
}

#[cfg(feature = "unity_engine-event")]
#[::unity2::methods]
impl Event {
    #[method(name = "get_rawType", args = 0)]
    pub fn get_raw_type(self) -> crate::unity_engine::eventtype::EventType;

    #[method(name = "get_mousePosition", args = 0)]
    pub fn get_mouse_position(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_delta", args = 0)]
    pub fn get_delta(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_pointerType", args = 0)]
    pub fn get_pointer_type(self) -> crate::unity_engine::pointertype::PointerType;

    #[method(name = "get_modifiers", args = 0)]
    pub fn get_modifiers(self) -> crate::unity_engine::eventmodifiers::EventModifiers;

    #[method(name = "get_character", args = 0)]
    pub fn get_character(self) -> u16;

    #[method(name = "get_keyCode", args = 0)]
    pub fn get_key_code(self) -> crate::unity_engine::keycode::KeyCode;

    #[method(name = "set_displayIndex", args = 1)]
    pub fn set_display_index(self, value: i32) -> ();

    #[method(name = "get_type", args = 0)]
    pub fn get_type(self) -> crate::unity_engine::eventtype::EventType;

    #[method(name = "get_commandName", args = 0)]
    pub fn get_command_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Internal_Use", args = 0)]
    pub fn internal_use(self) -> ();

    #[method(name = "Internal_Create", args = 1)]
    pub fn internal_create(display_index: i32) -> ::unity2::IntPtr;

    #[method(name = "Internal_Destroy", args = 1)]
    pub fn internal_destroy(ptr: ::unity2::IntPtr) -> ();

    #[method(name = "GetTypeForControl", args = 1)]
    pub fn get_type_for_control(self, control_id: i32)
        -> crate::unity_engine::eventtype::EventType;

    #[method(name = "PopEvent", args = 1)]
    pub fn pop_event(out_event: crate::unity_engine::event::Event) -> bool;

    #[method(name = "Internal_SetNativeEvent", args = 1)]
    pub fn internal_set_native_event(ptr: ::unity2::IntPtr) -> ();

    #[method(name = "Internal_MakeMasterEventCurrent", args = 1)]
    pub fn internal_make_master_event_current(display_index: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, display_index: i32) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "get_shift", args = 0)]
    pub fn get_shift(self) -> bool;

    #[method(name = "get_control", args = 0)]
    pub fn get_control(self) -> bool;

    #[method(name = "get_alt", args = 0)]
    pub fn get_alt(self) -> bool;

    #[method(name = "get_command", args = 0)]
    pub fn get_command(self) -> bool;

    #[method(name = "get_current", args = 0)]
    pub fn get_current() -> crate::unity_engine::event::Event;

    #[method(name = "get_isKey", args = 0)]
    pub fn get_is_key(self) -> bool;

    #[method(name = "get_isMouse", args = 0)]
    pub fn get_is_mouse(self) -> bool;

    #[method(name = "get_isDirectManipulationDevice", args = 0)]
    pub fn get_is_direct_manipulation_device(self) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Use", args = 0)]
    pub fn r#use(self) -> ();

    #[method(name = "get_mousePosition_Injected", args = 1)]
    pub fn get_mouse_position_injected(self, ret: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_delta_Injected", args = 1)]
    pub fn get_delta_injected(self, ret: crate::unity_engine::vector2::Vector2) -> ();
}

#[cfg(feature = "unity_engine-event")]
impl Event {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Event),
                ::core::stringify!(new),
            )
        });
        <Self as IEventMethods>::ctor(this);
        this
    }

    pub fn new_2(display_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Event),
                ::core::stringify!(new_2),
            )
        });
        <Self as IEventMethods>::ctor_2(this, display_index);
        this
    }
}
