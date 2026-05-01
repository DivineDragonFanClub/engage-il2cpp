
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interpreterexception/InterpreterException.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "InterpreterException")]
pub struct InterpreterException {}

#[cfg(feature = "moon_sharp-interpreter-interpreterexception")]
#[::unity2::methods]
impl InterpreterException {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, message: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "get_InstructionPtr", args = 0)]
    pub fn get_instruction_ptr(self) -> i32;

    #[method(name = "set_InstructionPtr", args = 1)]
    pub fn set_instruction_ptr(self, value: i32) -> ();

    #[method(name = "get_CallStack", args = 0)]
    pub fn get_call_stack(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::moon_sharp::interpreter::debugging::watchitem::WatchItem,
    >;

    #[method(name = "set_CallStack", args = 1)]
    pub fn set_call_stack(
        self,
        value: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            crate::moon_sharp::interpreter::debugging::watchitem::WatchItem,
        >,
    ) -> ();

    #[method(name = "get_DecoratedMessage", args = 0)]
    pub fn get_decorated_message(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DecoratedMessage", args = 1)]
    pub fn set_decorated_message(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DoNotDecorateMessage", args = 0)]
    pub fn get_do_not_decorate_message(self) -> bool;

    #[method(name = "set_DoNotDecorateMessage", args = 1)]
    pub fn set_do_not_decorate_message(self, value: bool) -> ();

    #[method(name = "DecorateMessage", args = 3)]
    pub fn decorate_message(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        sref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
        ip: i32,
    ) -> ();

    #[method(name = "Rethrow", args = 0)]
    pub fn rethrow(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interpreterexception")]
impl InterpreterException {
    pub fn new(message: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InterpreterException),
                ::core::stringify!(new),
            )
        });
        <Self as IInterpreterExceptionMethods>::ctor(this, message);
        this
    }

    pub fn new_2(
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InterpreterException),
                ::core::stringify!(new_2),
            )
        });
        <Self as IInterpreterExceptionMethods>::ctor_2(this, format, args);
        this
    }
}
