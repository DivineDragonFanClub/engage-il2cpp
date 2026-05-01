
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/exceptionhandlingclause/ExceptionHandlingClause.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "ExceptionHandlingClause")]
#[parent(crate::system::object::Object)]
pub struct ExceptionHandlingClause {
    #[rename(name = "catch_type")]
    pub catch_type: ::unity2::SystemType,
    #[rename(name = "filter_offset")]
    pub filter_offset: i32,
    #[rename(name = "flags")]
    pub flags:
        crate::system::reflection::exceptionhandlingclauseoptions::ExceptionHandlingClauseOptions,
    #[rename(name = "try_offset")]
    pub try_offset: i32,
    #[rename(name = "try_length")]
    pub try_length: i32,
    #[rename(name = "handler_offset")]
    pub handler_offset: i32,
    #[rename(name = "handler_length")]
    pub handler_length: i32,
}

#[cfg(feature = "system-reflection-exceptionhandlingclause")]
#[::unity2::methods]
impl ExceptionHandlingClause {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "system-reflection-exceptionhandlingclause")]
impl ExceptionHandlingClause {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExceptionHandlingClause),
                ::core::stringify!(new),
            )
        });
        <Self as IExceptionHandlingClauseMethods>::ctor(this);
        this
    }
}
