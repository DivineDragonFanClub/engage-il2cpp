
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/standard_descriptors/eventfacade/EventFacade.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.StandardDescriptors",
    name = "EventFacade"
)]
#[parent(crate::system::object::Object)]
pub struct EventFacade {
    #[rename(name = "m_AddCallback")]
    pub m_add_callback: crate::system::func_4::Func_4<
        crate::system::object::Object,
        crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        crate::moon_sharp::interpreter::dynvalue::DynValue,
    >,
    #[rename(name = "m_RemoveCallback")]
    pub m_remove_callback: crate::system::func_4::Func_4<
        crate::system::object::Object,
        crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        crate::moon_sharp::interpreter::dynvalue::DynValue,
    >,
    #[rename(name = "m_Object")]
    pub m_object: ::unity2::IlInstance,
}

#[cfg(feature = "moon_sharp-interpreter-interop-standard_descriptors-eventfacade")]
#[::unity2::methods]
impl EventFacade {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        parent : crate :: moon_sharp :: interpreter :: interop :: eventmemberdescriptor :: EventMemberDescriptor,
        obj: crate::system::object::Object,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        add_callback: crate::system::func_4::Func_4<
            crate::system::object::Object,
            crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
            crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
        remove_callback: crate::system::func_4::Func_4<
            crate::system::object::Object,
            crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
            crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
        obj: crate::system::object::Object,
    ) -> ();

    #[method(name = "Index", args = 3)]
    pub fn index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_direct_indexing: bool,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "SetIndex", args = 4)]
    pub fn set_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        index: crate::moon_sharp::interpreter::dynvalue::DynValue,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
        is_direct_indexing: bool,
    ) -> bool;

    #[method(name = "MetaIndex", args = 2)]
    pub fn meta_index(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        metaname: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;
}

#[cfg(feature = "moon_sharp-interpreter-interop-standard_descriptors-eventfacade")]
impl EventFacade {
    pub fn new(
        parent : crate :: moon_sharp :: interpreter :: interop :: eventmemberdescriptor :: EventMemberDescriptor,
        obj: crate::system::object::Object,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventFacade),
                ::core::stringify!(new),
            )
        });
        <Self as IEventFacadeMethods>::ctor(this, parent, obj);
        this
    }

    pub fn new_2(
        add_callback: crate::system::func_4::Func_4<
            crate::system::object::Object,
            crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
            crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
        remove_callback: crate::system::func_4::Func_4<
            crate::system::object::Object,
            crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
            crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
        obj: crate::system::object::Object,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventFacade),
                ::core::stringify!(new_2),
            )
        });
        <Self as IEventFacadeMethods>::ctor_2(this, add_callback, remove_callback, obj);
        this
    }
}
