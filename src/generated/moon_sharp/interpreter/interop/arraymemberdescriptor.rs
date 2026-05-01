
use crate::moon_sharp::interpreter::interop::functionmemberdescriptorbase::FunctionMemberDescriptorBase;
use crate::moon_sharp::interpreter::interop::functionmemberdescriptorbase::IFunctionMemberDescriptorBase;
use crate::moon_sharp::interpreter::interop::objectcallbackmemberdescriptor::IObjectCallbackMemberDescriptor;
use crate::moon_sharp::interpreter::interop::objectcallbackmemberdescriptor::ObjectCallbackMemberDescriptor;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/arraymemberdescriptor/ArrayMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "ArrayMemberDescriptor"
)]
# [parent (crate :: moon_sharp :: interpreter :: interop :: objectcallbackmemberdescriptor :: ObjectCallbackMemberDescriptor)]
pub struct ArrayMemberDescriptor {
    #[rename(name = "m_IsSetter")]
    pub m_is_setter: bool,
}

#[cfg(feature = "moon_sharp-interpreter-interop-arraymemberdescriptor")]
#[::unity2::methods]
impl ArrayMemberDescriptor {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        name: ::unity2::Il2CppString,
        is_setter: bool,
        indexer_params : :: unity2 :: Array < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: parameterdescriptor :: ParameterDescriptor >,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString, is_setter: bool) -> ();

    #[method(name = "PrepareForWiring", args = 1)]
    pub fn prepare_for_wiring(self, t: crate::moon_sharp::interpreter::table::Table) -> ();

    #[method(name = "BuildArrayIndices", args = 2)]
    pub fn build_array_indices(
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
        count: i32,
    ) -> ::unity2::Array<i32>;

    #[method(name = "ArrayIndexerSet", args = 3)]
    pub fn array_indexer_set(
        array_obj: crate::system::object::Object,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::system::object::Object;

    #[method(name = "ArrayIndexerGet", args = 3)]
    pub fn array_indexer_get(
        array_obj: crate::system::object::Object,
        ctx: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::system::object::Object;
}

#[cfg(feature = "moon_sharp-interpreter-interop-arraymemberdescriptor")]
impl ArrayMemberDescriptor {
    pub fn new(
        name: ::unity2::Il2CppString,
        is_setter: bool,
        indexer_params : :: unity2 :: Array < crate :: moon_sharp :: interpreter :: interop :: basic_descriptors :: parameterdescriptor :: ParameterDescriptor >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArrayMemberDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IArrayMemberDescriptorMethods>::ctor(this, name, is_setter, indexer_params);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString, is_setter: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArrayMemberDescriptor),
                ::core::stringify!(new_2),
            )
        });
        <Self as IArrayMemberDescriptorMethods>::ctor_2(this, name, is_setter);
        this
    }
}
