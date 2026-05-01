
use crate::moon_sharp::interpreter::interpreterexception::IInterpreterException;
use crate::moon_sharp::interpreter::interpreterexception::InterpreterException;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/internalerrorexception/InternalErrorException.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "InternalErrorException")]
#[parent(crate::moon_sharp::interpreter::interpreterexception::InterpreterException)]
pub struct InternalErrorException {}

#[cfg(feature = "moon_sharp-interpreter-internalerrorexception")]
#[::unity2::methods]
impl InternalErrorException {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, message: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-internalerrorexception")]
impl InternalErrorException {
    pub fn new(message: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InternalErrorException),
                ::core::stringify!(new),
            )
        });
        <Self as IInternalErrorExceptionMethods>::ctor(this, message);
        this
    }

    pub fn new_2(
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InternalErrorException),
                ::core::stringify!(new_2),
            )
        });
        <Self as IInternalErrorExceptionMethods>::ctor_2(this, format, args);
        this
    }
}
