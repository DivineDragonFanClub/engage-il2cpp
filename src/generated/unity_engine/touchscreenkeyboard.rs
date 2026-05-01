
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/touchscreenkeyboard/TouchScreenKeyboard.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "TouchScreenKeyboard")]
#[parent(crate::system::object::Object)]
pub struct TouchScreenKeyboard {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-touchscreenkeyboard")]
#[::unity2::methods]
impl TouchScreenKeyboard {
    #[method(name = "Internal_Destroy", args = 1)]
    pub fn internal_destroy(ptr: ::unity2::IntPtr) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = ".ctor", args = 8)]
    pub fn ctor(
        self,
        text: ::unity2::Il2CppString,
        keyboard_type: crate::unity_engine::touchscreenkeyboardtype::TouchScreenKeyboardType,
        autocorrection: bool,
        multiline: bool,
        secure: bool,
        alert: bool,
        text_placeholder: ::unity2::Il2CppString,
        character_limit: i32,
    ) -> ();

    #[method(name = "TouchScreenKeyboard_InternalConstructorHelper", args = 3)]
    pub fn touch_screen_keyboard_internal_constructor_helper(
        arguments : crate :: unity_engine :: touchscreenkeyboard_internalconstructorhelperarguments :: TouchScreenKeyboard_InternalConstructorHelperArguments,
        text: ::unity2::Il2CppString,
        text_placeholder: ::unity2::Il2CppString,
    ) -> ::unity2::IntPtr;

    #[method(name = "get_isSupported", args = 0)]
    pub fn get_is_supported() -> bool;

    #[method(name = "get_isInPlaceEditingAllowed", args = 0)]
    pub fn get_is_in_place_editing_allowed() -> bool;

    #[method(name = "Open", args = 8)]
    pub fn open(
        text: ::unity2::Il2CppString,
        keyboard_type: crate::unity_engine::touchscreenkeyboardtype::TouchScreenKeyboardType,
        autocorrection: bool,
        multiline: bool,
        secure: bool,
        alert: bool,
        text_placeholder: ::unity2::Il2CppString,
        character_limit: i32,
    ) -> crate::unity_engine::touchscreenkeyboard::TouchScreenKeyboard;

    #[method(name = "get_text", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "set_text", args = 1)]
    pub fn set_text(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_hideInput", args = 1)]
    pub fn set_hide_input(value: bool) -> ();

    #[method(name = "get_active", args = 0)]
    pub fn get_active(self) -> bool;

    #[method(name = "set_active", args = 1)]
    pub fn set_active(self, value: bool) -> ();

    #[method(name = "get_status", args = 0)]
    pub fn get_status(self)
        -> crate::unity_engine::touchscreenkeyboard::TouchScreenKeyboard_Status;

    #[method(name = "set_characterLimit", args = 1)]
    pub fn set_character_limit(self, value: i32) -> ();

    #[method(name = "get_canGetSelection", args = 0)]
    pub fn get_can_get_selection(self) -> bool;

    #[method(name = "get_canSetSelection", args = 0)]
    pub fn get_can_set_selection(self) -> bool;

    #[method(name = "get_selection", args = 0)]
    pub fn get_selection(self) -> crate::unity_engine::rangeint::RangeInt;

    #[method(name = "set_selection", args = 1)]
    pub fn set_selection(self, value: crate::unity_engine::rangeint::RangeInt) -> ();

    #[method(name = "GetSelection", args = 2)]
    pub fn get_selection_2(start: i32, length: i32) -> ();

    #[method(name = "SetSelection", args = 2)]
    pub fn set_selection_2(start: i32, length: i32) -> ();
}

#[cfg(feature = "unity_engine-touchscreenkeyboard")]
impl TouchScreenKeyboard {
    pub fn new(
        text: ::unity2::Il2CppString,
        keyboard_type: crate::unity_engine::touchscreenkeyboardtype::TouchScreenKeyboardType,
        autocorrection: bool,
        multiline: bool,
        secure: bool,
        alert: bool,
        text_placeholder: ::unity2::Il2CppString,
        character_limit: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TouchScreenKeyboard),
                ::core::stringify!(new),
            )
        });
        <Self as ITouchScreenKeyboardMethods>::ctor(
            this,
            text,
            keyboard_type,
            autocorrection,
            multiline,
            secure,
            alert,
            text_placeholder,
            character_limit,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/touchscreenkeyboard/TouchScreenKeyboard_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TouchScreenKeyboard_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TouchScreenKeyboard_Status {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "TouchScreenKeyboard.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TouchScreenKeyboard_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TouchScreenKeyboard_Status {
    pub fn visible() -> Self {
        Self { value: 0 }
    }

    pub fn done() -> Self {
        Self { value: 1 }
    }

    pub fn canceled() -> Self {
        Self { value: 2 }
    }

    pub fn lost_focus() -> Self {
        Self { value: 3 }
    }
}
