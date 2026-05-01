
use crate::moon_sharp::interpreter::refidobject::IRefIdObject;
use crate::moon_sharp::interpreter::refidobject::RefIdObject;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/callbackfunction/CallbackFunction.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "CallbackFunction")]
#[parent(crate::moon_sharp::interpreter::refidobject::RefIdObject)]
pub struct CallbackFunction {
    #[static_field]
    #[rename(name = "m_DefaultAccessMode")]
    pub m_default_access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
}

#[cfg(feature = "moon_sharp-interpreter-callbackfunction")]
#[::unity2::methods]
impl CallbackFunction {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ClrCallback", args = 0)]
    pub fn get_clr_callback(
        self,
    ) -> crate::system::func_3::Func_3<
        crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        crate::moon_sharp::interpreter::dynvalue::DynValue,
    >;

    #[method(name = "set_ClrCallback", args = 1)]
    pub fn set_clr_callback(
        self,
        value: crate::system::func_3::Func_3<
            crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
            crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        call_back: crate::system::func_3::Func_3<
            crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
            crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
        is_method_call: bool,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "get_DefaultAccessMode", args = 0)]
    pub fn get_default_access_mode(
    ) -> crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode;

    #[method(name = "set_DefaultAccessMode", args = 1)]
    pub fn set_default_access_mode(
        value: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> ();

    #[method(name = "FromDelegate", args = 3)]
    pub fn from_delegate(
        script: crate::moon_sharp::interpreter::script::Script,
        del: crate::system::delegate::Delegate,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> crate::moon_sharp::interpreter::callbackfunction::CallbackFunction;

    #[method(name = "FromMethodInfo", args = 4)]
    pub fn from_method_info(
        script: crate::moon_sharp::interpreter::script::Script,
        mi: crate::system::reflection::methodinfo::MethodInfo,
        obj: ::unity2::IlInstance,
        access_mode: crate::moon_sharp::interpreter::interopaccessmode::InteropAccessMode,
    ) -> crate::moon_sharp::interpreter::callbackfunction::CallbackFunction;

    #[method(name = "get_AdditionalData", args = 0)]
    pub fn get_additional_data(self) -> crate::system::object::Object;

    #[method(name = "set_AdditionalData", args = 1)]
    pub fn set_additional_data(self, value: crate::system::object::Object) -> ();

    #[method(name = "CheckCallbackSignature", args = 2)]
    pub fn check_callback_signature(
        mi: crate::system::reflection::methodinfo::MethodInfo,
        require_public_visibility: bool,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "moon_sharp-interpreter-callbackfunction")]
impl CallbackFunction {
    pub fn new(
        call_back: crate::system::func_3::Func_3<
            crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
            crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
        name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CallbackFunction),
                ::core::stringify!(new),
            )
        });
        <Self as ICallbackFunctionMethods>::ctor(this, call_back, name);
        this
    }
}
