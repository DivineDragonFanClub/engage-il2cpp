
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::methodbase::IMethodBase;
use crate::system::reflection::methodbase::MethodBase;
use crate::system::reflection::methodinfo::IMethodInfo;
use crate::system::reflection::methodinfo::MethodInfo;
use crate::system::reflection::runtimemethodinfo::IRuntimeMethodInfo;
use crate::system::reflection::runtimemethodinfo::RuntimeMethodInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monomethod/MonoMethod.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoMethod")]
#[parent(crate::system::reflection::runtimemethodinfo::RuntimeMethodInfo)]
pub struct MonoMethod {
    #[rename(name = "mhandle")]
    pub mhandle: ::unity2::IntPtr,
    #[rename(name = "name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "reftype")]
    pub reftype: ::unity2::SystemType,
}

#[cfg(feature = "system-reflection-monomethod")]
#[::unity2::methods]
impl MonoMethod {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_name", args = 1)]
    pub fn get_name(
        method: crate::system::reflection::methodbase::MethodBase,
    ) -> ::unity2::Il2CppString;

    #[method(name = "get_base_method", args = 2)]
    pub fn get_base_method(
        method: crate::system::reflection::monomethod::MonoMethod,
        definition: bool,
    ) -> crate::system::reflection::monomethod::MonoMethod;

    #[method(name = "GetBaseMethod", args = 0)]
    pub fn get_base_method_2(self) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "get_ReturnType", args = 0)]
    pub fn get_return_type(self) -> ::unity2::SystemType;

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

    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(self) -> crate::system::reflection::methodattributes::MethodAttributes;

    #[method(name = "get_CallingConvention", args = 0)]
    pub fn get_calling_convention(
        self,
    ) -> crate::system::reflection::callingconventions::CallingConventions;

    #[method(name = "get_ReflectedType", args = 0)]
    pub fn get_reflected_type(self) -> ::unity2::SystemType;

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name_2(self) -> ::unity2::Il2CppString;

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

    #[method(name = "GetPInvoke", args = 3)]
    pub fn get_p_invoke(
        self,
        flags: crate::system::reflection::pinvokeattributes::PInvokeAttributes,
        entry_point: ::unity2::Il2CppString,
        dll_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetPseudoCustomAttributes", args = 0)]
    pub fn get_pseudo_custom_attributes(self) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "MakeGenericMethod", args = 1)]
    pub fn make_generic_method(
        self,
        method_instantiation: ::unity2::Array<::unity2::SystemType>,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "MakeGenericMethod_impl", args = 1)]
    pub fn make_generic_method_impl(
        self,
        types: ::unity2::Array<::unity2::SystemType>,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetGenericArguments", args = 0)]
    pub fn get_generic_arguments(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetGenericMethodDefinition_impl", args = 0)]
    pub fn get_generic_method_definition_impl(
        self,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetGenericMethodDefinition", args = 0)]
    pub fn get_generic_method_definition(self)
        -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "get_IsGenericMethodDefinition", args = 0)]
    pub fn get_is_generic_method_definition(self) -> bool;

    #[method(name = "get_IsGenericMethod", args = 0)]
    pub fn get_is_generic_method(self) -> bool;

    #[method(name = "get_ContainsGenericParameters", args = 0)]
    pub fn get_contains_generic_parameters(self) -> bool;

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
}

#[cfg(feature = "system-reflection-monomethod")]
impl MonoMethod {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoMethod),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoMethodMethods>::ctor(this);
        this
    }
}
