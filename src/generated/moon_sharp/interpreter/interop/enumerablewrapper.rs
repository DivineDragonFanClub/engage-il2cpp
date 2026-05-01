
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/enumerablewrapper/EnumerableWrapper.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "EnumerableWrapper"
)]
#[parent(crate::system::object::Object)]
pub struct EnumerableWrapper {
    #[rename(name = "m_Enumerator")]
    pub m_enumerator: crate::system::collections::ienumerator::IEnumerator,
    #[rename(name = "m_Script")]
    pub m_script: crate::moon_sharp::interpreter::script::Script,
    #[rename(name = "m_Prev")]
    pub m_prev: crate::moon_sharp::interpreter::dynvalue::DynValue,
    #[rename(name = "m_HasTurnOnce")]
    pub m_has_turn_once: bool,
}

#[cfg(feature = "moon_sharp-interpreter-interop-enumerablewrapper")]
#[::unity2::methods]
impl EnumerableWrapper {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        enumerator: crate::system::collections::ienumerator::IEnumerator,
    ) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "GetNext", args = 1)]
    pub fn get_next(
        self,
        prev: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "LuaIteratorCallback", args = 2)]
    pub fn lua_iterator_callback(
        self,
        execution_context : crate :: moon_sharp :: interpreter :: scriptexecutioncontext :: ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ConvertIterator", args = 2)]
    pub fn convert_iterator(
        script: crate::moon_sharp::interpreter::script::Script,
        enumerator: crate::system::collections::ienumerator::IEnumerator,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ConvertTable", args = 1)]
    pub fn convert_table(
        table: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

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

#[cfg(feature = "moon_sharp-interpreter-interop-enumerablewrapper")]
impl EnumerableWrapper {
    pub fn new(
        script: crate::moon_sharp::interpreter::script::Script,
        enumerator: crate::system::collections::ienumerator::IEnumerator,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EnumerableWrapper),
                ::core::stringify!(new),
            )
        });
        <Self as IEnumerableWrapperMethods>::ctor(this, script, enumerator);
        this
    }
}
