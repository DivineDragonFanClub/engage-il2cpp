
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/debugaction/DebugAction.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DebugAction {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DebugAction {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "DebugAction";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DebugAction {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DebugAction {
    pub fn enable_debug_menu() -> Self {
        Self { value: 0 }
    }

    pub fn previous_debug_panel() -> Self {
        Self { value: 1 }
    }

    pub fn next_debug_panel() -> Self {
        Self { value: 2 }
    }

    pub fn action() -> Self {
        Self { value: 3 }
    }

    pub fn make_persistent() -> Self {
        Self { value: 4 }
    }

    pub fn move_vertical() -> Self {
        Self { value: 5 }
    }

    pub fn move_horizontal() -> Self {
        Self { value: 6 }
    }

    pub fn multiplier() -> Self {
        Self { value: 7 }
    }

    pub fn reset_all() -> Self {
        Self { value: 8 }
    }

    pub fn debug_action_count() -> Self {
        Self { value: 9 }
    }
}
