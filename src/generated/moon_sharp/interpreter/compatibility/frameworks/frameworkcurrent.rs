
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkbase::FrameworkBase;
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkbase::IFrameworkBase;
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkclrbase::FrameworkClrBase;
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkclrbase::IFrameworkClrBase;
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkreflectionbase::FrameworkReflectionBase;
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkreflectionbase::IFrameworkReflectionBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/compatibility/frameworks/frameworkcurrent/FrameworkCurrent.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Compatibility.Frameworks",
    name = "FrameworkCurrent"
)]
#[parent(
    crate::moon_sharp::interpreter::compatibility::frameworks::frameworkclrbase::FrameworkClrBase
)]
pub struct FrameworkCurrent {}

#[cfg(feature = "moon_sharp-interpreter-compatibility-frameworks-frameworkcurrent")]
#[::unity2::methods]
impl FrameworkCurrent {
    #[method(name = "IsDbNull", args = 1)]
    pub fn is_db_null(self, o: crate::system::object::Object) -> bool;

    #[method(name = "StringContainsChar", args = 2)]
    pub fn string_contains_char(self, str: ::unity2::Il2CppString, chr: u16) -> bool;

    #[method(name = "GetInterface", args = 2)]
    pub fn get_interface(
        self,
        r#type: ::unity2::SystemType,
        name: ::unity2::Il2CppString,
    ) -> ::unity2::SystemType;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-compatibility-frameworks-frameworkcurrent")]
impl FrameworkCurrent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FrameworkCurrent),
                ::core::stringify!(new),
            )
        });
        <Self as IFrameworkCurrentMethods>::ctor(this);
        this
    }
}
