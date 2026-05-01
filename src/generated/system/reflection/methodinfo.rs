
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::methodbase::IMethodBase;
use crate::system::reflection::methodbase::MethodBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/methodinfo/MethodInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MethodInfo")]
#[parent(crate::system::reflection::methodbase::MethodBase)]
pub struct MethodInfo {}

#[cfg(feature = "system-reflection-methodinfo")]
#[::unity2::methods]
impl MethodInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        left: crate::system::reflection::methodinfo::MethodInfo,
        right: crate::system::reflection::methodinfo::MethodInfo,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        left: crate::system::reflection::methodinfo::MethodInfo,
        right: crate::system::reflection::methodinfo::MethodInfo,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "get_MemberType", args = 0)]
    pub fn get_member_type(self) -> crate::system::reflection::membertypes::MemberTypes;

    #[method(name = "get_ReturnType", args = 0)]
    pub fn get_return_type(self) -> ::unity2::SystemType;

    #[method(name = "GetGenericArguments", args = 0)]
    pub fn get_generic_arguments(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetGenericMethodDefinition", args = 0)]
    pub fn get_generic_method_definition(self)
        -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "MakeGenericMethod", args = 1)]
    pub fn make_generic_method(
        self,
        type_arguments: ::unity2::Array<::unity2::SystemType>,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "CreateDelegate", args = 1)]
    pub fn create_delegate(
        self,
        delegate_type: ::unity2::SystemType,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "CreateDelegate", args = 2)]
    pub fn create_delegate_2(
        self,
        delegate_type: ::unity2::SystemType,
        target: crate::system::object::Object,
    ) -> crate::system::delegate::Delegate;

    #[method(name = "GetBaseMethod", args = 0)]
    pub fn get_base_method(self) -> crate::system::reflection::methodinfo::MethodInfo;
}

#[cfg(feature = "system-reflection-methodinfo")]
impl MethodInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MethodInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IMethodInfoMethods>::ctor(this);
        this
    }
}
