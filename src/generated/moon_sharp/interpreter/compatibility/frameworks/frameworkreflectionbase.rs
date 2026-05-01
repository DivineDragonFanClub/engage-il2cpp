
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkbase::FrameworkBase;
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkbase::IFrameworkBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/compatibility/frameworks/frameworkreflectionbase/FrameworkReflectionBase.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Compatibility.Frameworks",
    name = "FrameworkReflectionBase"
)]
#[parent(crate::moon_sharp::interpreter::compatibility::frameworks::frameworkbase::FrameworkBase)]
pub struct FrameworkReflectionBase {}

#[cfg(feature = "moon_sharp-interpreter-compatibility-frameworks-frameworkreflectionbase")]
#[::unity2::methods]
impl FrameworkReflectionBase {
    #[method(name = "GetTypeInfoFromType", args = 1)]
    pub fn get_type_info_from_type(self, t: ::unity2::SystemType) -> ::unity2::SystemType;

    #[method(name = "GetAssembly", args = 1)]
    pub fn get_assembly(
        self,
        t: ::unity2::SystemType,
    ) -> crate::system::reflection::assembly::Assembly;

    #[method(name = "GetBaseType", args = 1)]
    pub fn get_base_type(self, t: ::unity2::SystemType) -> ::unity2::SystemType;

    #[method(name = "IsValueType", args = 1)]
    pub fn is_value_type(self, t: ::unity2::SystemType) -> bool;

    #[method(name = "IsInterface", args = 1)]
    pub fn is_interface(self, t: ::unity2::SystemType) -> bool;

    #[method(name = "IsNestedPublic", args = 1)]
    pub fn is_nested_public(self, t: ::unity2::SystemType) -> bool;

    #[method(name = "IsAbstract", args = 1)]
    pub fn is_abstract(self, t: ::unity2::SystemType) -> bool;

    #[method(name = "IsEnum", args = 1)]
    pub fn is_enum(self, t: ::unity2::SystemType) -> bool;

    #[method(name = "IsGenericTypeDefinition", args = 1)]
    pub fn is_generic_type_definition(self, t: ::unity2::SystemType) -> bool;

    #[method(name = "IsGenericType", args = 1)]
    pub fn is_generic_type(self, t: ::unity2::SystemType) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-compatibility-frameworks-frameworkreflectionbase")]
impl FrameworkReflectionBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FrameworkReflectionBase),
                ::core::stringify!(new),
            )
        });
        <Self as IFrameworkReflectionBaseMethods>::ctor(this);
        this
    }
}
