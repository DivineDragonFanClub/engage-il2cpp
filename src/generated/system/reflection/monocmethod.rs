
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::constructorinfo::ConstructorInfo;
use crate::system::reflection::constructorinfo::IConstructorInfo;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::methodbase::IMethodBase;
use crate::system::reflection::methodbase::MethodBase;
use crate::system::reflection::runtimeconstructorinfo::IRuntimeConstructorInfo;
use crate::system::reflection::runtimeconstructorinfo::RuntimeConstructorInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monocmethod/MonoCMethod.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoCMethod")]
#[parent(crate::system::reflection::runtimeconstructorinfo::RuntimeConstructorInfo)]
pub struct MonoCMethod {
    #[rename(name = "mhandle")]
    pub mhandle: ::unity2::IntPtr,
    #[rename(name = "name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "reftype")]
    pub reftype: ::unity2::SystemType,
}

#[cfg(feature = "system-reflection-monocmethod")]
#[::unity2::methods]
impl MonoCMethod {
    #[method(name = "GetMethodImplementationFlags", args = 0)]
    pub fn get_method_implementation_flags(
        self,
    ) -> crate::system::reflection::methodimplattributes::MethodImplAttributes;

    #[method(name = "GetParameters", args = 0)]
    pub fn get_parameters(
        self,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

    #[method(name = "GetParametersInternal", args = 0)]
    pub fn get_parameters_internal(
        self,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

    #[method(name = "GetParametersCount", args = 0)]
    pub fn get_parameters_count(self) -> i32;

    #[method(name = "InternalInvoke", args = 2)]
    pub fn internal_invoke(
        self,
        obj: crate::system::object::Object,
        parameters: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::system::object::Object;

    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(self) -> crate::system::reflection::methodattributes::MethodAttributes;

    #[method(name = "get_CallingConvention", args = 0)]
    pub fn get_calling_convention(
        self,
    ) -> crate::system::reflection::callingconventions::CallingConventions;

    #[method(name = "get_ContainsGenericParameters", args = 0)]
    pub fn get_contains_generic_parameters(self) -> bool;

    #[method(name = "get_ReflectedType", args = 0)]
    pub fn get_reflected_type(self) -> ::unity2::SystemType;

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = "GetCustomAttributes", args = 1)]
    pub fn get_custom_attributes(
        self,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "GetCustomAttributes", args = 2)]
    pub fn get_custom_attributes_2(
        self,
        attribute_type: ::unity2::SystemType,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCustomAttributesData", args = 0)]
    pub fn get_custom_attributes_data(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = "get_core_clr_security_level", args = 0)]
    pub fn get_core_clr_security_level() -> i32;

    #[method(name = "get_IsSecurityCritical", args = 0)]
    pub fn get_is_security_critical(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-monocmethod")]
impl MonoCMethod {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoCMethod),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoCMethodMethods>::ctor(this);
        this
    }
}
