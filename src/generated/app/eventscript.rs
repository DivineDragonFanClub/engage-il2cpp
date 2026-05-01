
use crate::moon_sharp::interpreter::script::IScript;
use crate::moon_sharp::interpreter::script::Script;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventscript/EventScript.md")))]
#[::unity2::class(namespace = "App", name = "EventScript")]
#[parent(crate::moon_sharp::interpreter::script::Script)]
pub struct EventScript {
    #[static_field]
    #[rename(name = "s_Stack")]
    pub s_stack:
        crate::system::collections::generic::stack_1::Stack_1<crate::app::eventscript::EventScript>,
}

#[cfg(feature = "app-eventscript")]
#[::unity2::methods]
impl EventScript {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetFunc", args = 1)]
    pub fn get_func(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Call", args = 2)]
    pub fn call(
        self,
        name: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "TryCreateCoroutine", args = 1)]
    pub fn try_create_coroutine(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "TryCreateCoroutine", args = 1)]
    pub fn try_create_coroutine_2(
        self,
        function: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "IsDead", args = 1)]
    pub fn is_dead(self, coroutine: crate::moon_sharp::interpreter::dynvalue::DynValue) -> bool;

    #[method(name = "DoCoroutine", args = 2)]
    pub fn do_coroutine(
        self,
        coroutine: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "RegistFunction", args = 2)]
    pub fn regist_function(
        self,
        func: crate::app::eventscript::EventScript_FunctionArgs,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "RegistAction", args = 2)]
    pub fn regist_action(
        self,
        func: crate::app::eventscript::EventScript_ActionArgs,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Regist", args = 0)]
    pub fn regist(self) -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "LoadImpl", args = 1)]
    pub fn load_impl(self, path: ::unity2::Il2CppString) -> ();

    #[method(name = "UnloadImpl", args = 0)]
    pub fn unload_impl(self) -> ();

    #[method(name = "Load", args = 1)]
    pub fn load(path: ::unity2::Il2CppString) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> crate::app::eventscript::EventScript;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-eventscript")]
impl EventScript {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventScript),
                ::core::stringify!(new),
            )
        });
        <Self as IEventScriptMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventscript/EventScript_FunctionArgs.md")))]
#[::unity2::class(namespace = "App", name = "EventScript.FunctionArgs")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventScript_FunctionArgs {}

#[cfg(feature = "app-eventscript")]
#[::unity2::methods]
impl EventScript_FunctionArgs {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;
}

#[cfg(feature = "app-eventscript")]
impl EventScript_FunctionArgs {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventScript_FunctionArgs),
                ::core::stringify!(new),
            )
        });
        <Self as IEventScript_FunctionArgsMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventscript/EventScript_ActionArgs.md")))]
#[::unity2::class(namespace = "App", name = "EventScript.ActionArgs")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventScript_ActionArgs {}

#[cfg(feature = "app-eventscript")]
#[::unity2::methods]
impl EventScript_ActionArgs {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();
}

#[cfg(feature = "app-eventscript")]
impl EventScript_ActionArgs {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventScript_ActionArgs),
                ::core::stringify!(new),
            )
        });
        <Self as IEventScript_ActionArgsMethods>::ctor(this, object, method);
        this
    }
}
