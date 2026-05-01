
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkbase::FrameworkBase;
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkbase::IFrameworkBase;
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkreflectionbase::FrameworkReflectionBase;
use crate::moon_sharp::interpreter::compatibility::frameworks::frameworkreflectionbase::IFrameworkReflectionBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/compatibility/frameworks/frameworkclrbase/FrameworkClrBase.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Compatibility.Frameworks",
    name = "FrameworkClrBase"
)]
# [parent (crate :: moon_sharp :: interpreter :: compatibility :: frameworks :: frameworkreflectionbase :: FrameworkReflectionBase)]
pub struct FrameworkClrBase {
    #[rename(name = "BINDINGFLAGS_MEMBER")]
    pub bindingflags_member: crate::system::reflection::bindingflags::BindingFlags,
    #[rename(name = "BINDINGFLAGS_INNERCLASS")]
    pub bindingflags_innerclass: crate::system::reflection::bindingflags::BindingFlags,
}

#[cfg(feature = "moon_sharp-interpreter-compatibility-frameworks-frameworkclrbase")]
#[::unity2::methods]
impl FrameworkClrBase {
    #[method(name = "GetTypeInfoFromType", args = 1)]
    pub fn get_type_info_from_type(self, t: ::unity2::SystemType) -> ::unity2::SystemType;

    #[method(name = "GetAddMethod", args = 1)]
    pub fn get_add_method(
        self,
        ei: crate::system::reflection::eventinfo::EventInfo,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetConstructors", args = 1)]
    pub fn get_constructors(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::system::reflection::constructorinfo::ConstructorInfo>;

    #[method(name = "GetEvents", args = 1)]
    pub fn get_events(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::system::reflection::eventinfo::EventInfo>;

    #[method(name = "GetFields", args = 1)]
    pub fn get_fields(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::system::reflection::fieldinfo::FieldInfo>;

    #[method(name = "GetGenericArguments", args = 1)]
    pub fn get_generic_arguments(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetGetMethod", args = 1)]
    pub fn get_get_method(
        self,
        pi: crate::system::reflection::propertyinfo::PropertyInfo,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetInterfaces", args = 1)]
    pub fn get_interfaces(self, t: ::unity2::SystemType) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetMethod", args = 2)]
    pub fn get_method(
        self,
        r#type: ::unity2::SystemType,
        name: ::unity2::Il2CppString,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetMethods", args = 1)]
    pub fn get_methods(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::system::reflection::methodinfo::MethodInfo>;

    #[method(name = "GetNestedTypes", args = 1)]
    pub fn get_nested_types(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetProperties", args = 1)]
    pub fn get_properties(
        self,
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::system::reflection::propertyinfo::PropertyInfo>;

    #[method(name = "GetProperty", args = 2)]
    pub fn get_property(
        self,
        r#type: ::unity2::SystemType,
        name: ::unity2::Il2CppString,
    ) -> crate::system::reflection::propertyinfo::PropertyInfo;

    #[method(name = "GetRemoveMethod", args = 1)]
    pub fn get_remove_method(
        self,
        ei: crate::system::reflection::eventinfo::EventInfo,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetSetMethod", args = 1)]
    pub fn get_set_method(
        self,
        pi: crate::system::reflection::propertyinfo::PropertyInfo,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "IsAssignableFrom", args = 2)]
    pub fn is_assignable_from(
        self,
        current: ::unity2::SystemType,
        to_compare: ::unity2::SystemType,
    ) -> bool;

    #[method(name = "IsInstanceOfType", args = 2)]
    pub fn is_instance_of_type(
        self,
        t: ::unity2::SystemType,
        o: crate::system::object::Object,
    ) -> bool;

    #[method(name = "GetMethod", args = 3)]
    pub fn get_method_2(
        self,
        resources_type: ::unity2::SystemType,
        name: ::unity2::Il2CppString,
        types: ::unity2::Array<::unity2::SystemType>,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetAssemblyTypes", args = 1)]
    pub fn get_assembly_types(
        self,
        asm: crate::system::reflection::assembly::Assembly,
    ) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-compatibility-frameworks-frameworkclrbase")]
impl FrameworkClrBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FrameworkClrBase),
                ::core::stringify!(new),
            )
        });
        <Self as IFrameworkClrBaseMethods>::ctor(this);
        this
    }
}
