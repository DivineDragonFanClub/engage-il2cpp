
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monomethodinfo/MonoMethodInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MonoMethodInfo {
    pub parent: ::unity2::SystemType,
    pub ret: ::unity2::SystemType,
    pub attrs: crate::system::reflection::methodattributes::MethodAttributes,
    pub iattrs: crate::system::reflection::methodimplattributes::MethodImplAttributes,
    pub callconv: crate::system::reflection::callingconventions::CallingConventions,
}

impl ::unity2::ClassIdentity for MonoMethodInfo {
    const NAMESPACE: &'static str = "System.Reflection";

    const NAME: &'static str = "MonoMethodInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MonoMethodInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-reflection-monomethodinfo")]
#[::unity2::methods(value)]
impl MonoMethodInfo {
    #[method(name = "get_method_info", args = 2)]
    pub fn get_method_info(
        handle: ::unity2::IntPtr,
        info: crate::system::reflection::monomethodinfo::MonoMethodInfo,
    ) -> ();

    #[method(name = "get_method_attributes", args = 1)]
    pub fn get_method_attributes(handle: ::unity2::IntPtr) -> i32;

    #[method(name = "GetMethodInfo", args = 1)]
    pub fn get_method_info_2(
        handle: ::unity2::IntPtr,
    ) -> crate::system::reflection::monomethodinfo::MonoMethodInfo;

    #[method(name = "GetDeclaringType", args = 1)]
    pub fn get_declaring_type(handle: ::unity2::IntPtr) -> ::unity2::SystemType;

    #[method(name = "GetReturnType", args = 1)]
    pub fn get_return_type(handle: ::unity2::IntPtr) -> ::unity2::SystemType;

    #[method(name = "GetAttributes", args = 1)]
    pub fn get_attributes(
        handle: ::unity2::IntPtr,
    ) -> crate::system::reflection::methodattributes::MethodAttributes;

    #[method(name = "GetCallingConvention", args = 1)]
    pub fn get_calling_convention(
        handle: ::unity2::IntPtr,
    ) -> crate::system::reflection::callingconventions::CallingConventions;

    #[method(name = "GetMethodImplementationFlags", args = 1)]
    pub fn get_method_implementation_flags(
        handle: ::unity2::IntPtr,
    ) -> crate::system::reflection::methodimplattributes::MethodImplAttributes;

    #[method(name = "get_parameter_info", args = 2)]
    pub fn get_parameter_info(
        handle: ::unity2::IntPtr,
        member: crate::system::reflection::memberinfo::MemberInfo,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

    #[method(name = "GetParametersInfo", args = 2)]
    pub fn get_parameters_info(
        handle: ::unity2::IntPtr,
        member: crate::system::reflection::memberinfo::MemberInfo,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;
}
