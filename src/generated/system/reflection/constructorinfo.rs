
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::methodbase::IMethodBase;
use crate::system::reflection::methodbase::MethodBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/constructorinfo/ConstructorInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "ConstructorInfo")]
#[parent(crate::system::reflection::methodbase::MethodBase)]
pub struct ConstructorInfo {
    #[static_field]
    #[rename(name = "ConstructorName")]
    pub constructor_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "TypeConstructorName")]
    pub type_constructor_name: ::unity2::Il2CppString,
}

#[cfg(feature = "system-reflection-constructorinfo")]
#[::unity2::methods]
impl ConstructorInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_MemberType", args = 0)]
    pub fn get_member_type(self) -> crate::system::reflection::membertypes::MemberTypes;

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        parameters: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::system::object::Object;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        left: crate::system::reflection::constructorinfo::ConstructorInfo,
        right: crate::system::reflection::constructorinfo::ConstructorInfo,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        left: crate::system::reflection::constructorinfo::ConstructorInfo,
        right: crate::system::reflection::constructorinfo::ConstructorInfo,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-reflection-constructorinfo")]
impl ConstructorInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConstructorInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IConstructorInfoMethods>::ctor(this);
        this
    }
}
