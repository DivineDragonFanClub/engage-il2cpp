
use crate::moon_sharp::interpreter::interop::functionmemberdescriptorbase::FunctionMemberDescriptorBase;
use crate::moon_sharp::interpreter::interop::functionmemberdescriptorbase::IFunctionMemberDescriptorBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/standard_descriptors/hardwired_descriptors/hardwiredmethodmemberdescriptor/HardwiredMethodMemberDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.StandardDescriptors.HardwiredDescriptors",
    name = "HardwiredMethodMemberDescriptor"
)]
# [parent (crate :: moon_sharp :: interpreter :: interop :: functionmemberdescriptorbase :: FunctionMemberDescriptorBase)]
pub struct HardwiredMethodMemberDescriptor {}

#[cfg(feature = "moon_sharp-interpreter-interop-standard_descriptors-hardwired_descriptors-hardwiredmethodmemberdescriptor")]
#[::unity2::methods]
impl HardwiredMethodMemberDescriptor {
    #[method(name = "Execute", args = 4)]
    pub fn execute(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
        args: crate::moon_sharp::interpreter::callbackarguments::CallbackArguments,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CalcArgsCount", args = 1)]
    pub fn calc_args_count(self, pars: ::unity2::Array<crate::system::object::Object>) -> i32;

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        obj: crate::system::object::Object,
        pars: ::unity2::Array<crate::system::object::Object>,
        argscount: i32,
    ) -> crate::system::object::Object;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-standard_descriptors-hardwired_descriptors-hardwiredmethodmemberdescriptor")]
impl HardwiredMethodMemberDescriptor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HardwiredMethodMemberDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IHardwiredMethodMemberDescriptorMethods>::ctor(this);
        this
    }
}
