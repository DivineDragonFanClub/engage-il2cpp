
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/methodbase/MethodBase.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MethodBase")]
#[parent(crate::system::reflection::memberinfo::MemberInfo)]
pub struct MethodBase {}

#[cfg(feature = "system-reflection-methodbase")]
#[::unity2::methods]
impl MethodBase {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        left: crate::system::reflection::methodbase::MethodBase,
        right: crate::system::reflection::methodbase::MethodBase,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        left: crate::system::reflection::methodbase::MethodBase,
        right: crate::system::reflection::methodbase::MethodBase,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "GetParametersNoCopy", args = 0)]
    pub fn get_parameters_no_copy(
        self,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

    #[method(name = "GetParameters", args = 0)]
    pub fn get_parameters(
        self,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

    #[method(name = "GetMethodImplementationFlags", args = 0)]
    pub fn get_method_implementation_flags(
        self,
    ) -> crate::system::reflection::methodimplattributes::MethodImplAttributes;

    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(self) -> crate::system::reflection::methodattributes::MethodAttributes;

    #[method(name = "get_CallingConvention", args = 0)]
    pub fn get_calling_convention(
        self,
    ) -> crate::system::reflection::callingconventions::CallingConventions;

    #[method(name = "GetGenericArguments", args = 0)]
    pub fn get_generic_arguments(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "get_IsGenericMethodDefinition", args = 0)]
    pub fn get_is_generic_method_definition(self) -> bool;

    #[method(name = "get_ContainsGenericParameters", args = 0)]
    pub fn get_contains_generic_parameters(self) -> bool;

    #[method(name = "get_IsGenericMethod", args = 0)]
    pub fn get_is_generic_method(self) -> bool;

    #[method(name = "get_IsSecurityCritical", args = 0)]
    pub fn get_is_security_critical(self) -> bool;

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        obj: crate::system::object::Object,
        parameters: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::system::object::Object;

    #[method(name = "get_IsPublic", args = 0)]
    pub fn get_is_public(self) -> bool;

    #[method(name = "get_IsPrivate", args = 0)]
    pub fn get_is_private(self) -> bool;

    #[method(name = "get_IsFamily", args = 0)]
    pub fn get_is_family(self) -> bool;

    #[method(name = "get_IsAssembly", args = 0)]
    pub fn get_is_assembly(self) -> bool;

    #[method(name = "get_IsFamilyAndAssembly", args = 0)]
    pub fn get_is_family_and_assembly(self) -> bool;

    #[method(name = "get_IsFamilyOrAssembly", args = 0)]
    pub fn get_is_family_or_assembly(self) -> bool;

    #[method(name = "get_IsStatic", args = 0)]
    pub fn get_is_static(self) -> bool;

    #[method(name = "get_IsVirtual", args = 0)]
    pub fn get_is_virtual(self) -> bool;

    #[method(name = "get_IsAbstract", args = 0)]
    pub fn get_is_abstract(self) -> bool;

    #[method(name = "get_IsSpecialName", args = 0)]
    pub fn get_is_special_name(self) -> bool;

    #[method(name = "get_IsConstructor", args = 0)]
    pub fn get_is_constructor(self) -> bool;

    #[method(name = "ConstructParameters", args = 3)]
    pub fn construct_parameters(
        parameter_types: ::unity2::Array<::unity2::SystemType>,
        calling_convention: crate::system::reflection::callingconventions::CallingConventions,
        serialization: bool,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FormatNameAndSig", args = 1)]
    pub fn format_name_and_sig(self, serialization: bool) -> ::unity2::Il2CppString;

    #[method(name = "GetParameterTypes", args = 0)]
    pub fn get_parameter_types(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetParametersInternal", args = 0)]
    pub fn get_parameters_internal(
        self,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

    #[method(name = "GetParametersCount", args = 0)]
    pub fn get_parameters_count(self) -> i32;

    #[method(name = "GetMethodFromHandleInternalType", args = 2)]
    pub fn get_method_from_handle_internal_type(
        method_handle: ::unity2::IntPtr,
        type_handle: ::unity2::IntPtr,
    ) -> crate::system::reflection::methodbase::MethodBase;

    #[method(name = "GetMethodFromHandleInternalType_native", args = 3)]
    pub fn get_method_from_handle_internal_type_native(
        method_handle: ::unity2::IntPtr,
        type_handle: ::unity2::IntPtr,
        generic_check: bool,
    ) -> crate::system::reflection::methodbase::MethodBase;
}

#[cfg(feature = "system-reflection-methodbase")]
impl MethodBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MethodBase),
                ::core::stringify!(new),
            )
        });
        <Self as IMethodBaseMethods>::ctor(this);
        this
    }
}
