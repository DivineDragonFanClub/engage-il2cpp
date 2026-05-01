
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/debugging/debuggeraction/DebuggerAction.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Debugging", name = "DebuggerAction")]
#[parent(crate::system::object::Object)]
pub struct DebuggerAction {}

#[cfg(feature = "moon_sharp-interpreter-debugging-debuggeraction")]
#[::unity2::methods]
impl DebuggerAction {
    #[method(name = "get_Action", args = 0)]
    pub fn get_action(
        self,
    ) -> crate::moon_sharp::interpreter::debugging::debuggeraction::DebuggerAction_ActionType;

    #[method(name = "set_Action", args = 1)]
    pub fn set_action(
        self,
        value: crate::moon_sharp::interpreter::debugging::debuggeraction::DebuggerAction_ActionType,
    ) -> ();

    #[method(name = "get_SourceID", args = 0)]
    pub fn get_source_id(self) -> i32;

    #[method(name = "set_SourceID", args = 1)]
    pub fn set_source_id(self, value: i32) -> ();

    #[method(name = "get_SourceLine", args = 0)]
    pub fn get_source_line(self) -> i32;

    #[method(name = "set_SourceLine", args = 1)]
    pub fn set_source_line(self, value: i32) -> ();

    #[method(name = "get_SourceCol", args = 0)]
    pub fn get_source_col(self) -> i32;

    #[method(name = "set_SourceCol", args = 1)]
    pub fn set_source_col(self, value: i32) -> ();

    #[method(name = "get_Lines", args = 0)]
    pub fn get_lines(self) -> ::unity2::Array<i32>;

    #[method(name = "set_Lines", args = 1)]
    pub fn set_lines(self, value: ::unity2::Array<i32>) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "moon_sharp-interpreter-debugging-debuggeraction")]
impl DebuggerAction {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebuggerAction),
                ::core::stringify!(new),
            )
        });
        <Self as IDebuggerActionMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/debugging/debuggeraction/DebuggerAction_ActionType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DebuggerAction_ActionType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DebuggerAction_ActionType {
    const NAMESPACE: &'static str = "MoonSharp.Interpreter.Debugging";

    const NAME: &'static str = "DebuggerAction.ActionType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DebuggerAction_ActionType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DebuggerAction_ActionType {
    pub fn byte_code_step_in() -> Self {
        Self { value: 0 }
    }

    pub fn byte_code_step_over() -> Self {
        Self { value: 1 }
    }

    pub fn byte_code_step_out() -> Self {
        Self { value: 2 }
    }

    pub fn step_in() -> Self {
        Self { value: 3 }
    }

    pub fn step_over() -> Self {
        Self { value: 4 }
    }

    pub fn step_out() -> Self {
        Self { value: 5 }
    }

    pub fn run() -> Self {
        Self { value: 6 }
    }

    pub fn toggle_breakpoint() -> Self {
        Self { value: 7 }
    }

    pub fn set_breakpoint() -> Self {
        Self { value: 8 }
    }

    pub fn clear_breakpoint() -> Self {
        Self { value: 9 }
    }

    pub fn reset_breakpoints() -> Self {
        Self { value: 10 }
    }

    pub fn refresh() -> Self {
        Self { value: 11 }
    }

    pub fn hard_refresh() -> Self {
        Self { value: 12 }
    }

    pub fn none() -> Self {
        Self { value: 13 }
    }
}
