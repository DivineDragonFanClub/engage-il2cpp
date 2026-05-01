
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/memberinfo/MemberInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MemberInfo")]
#[parent(crate::system::object::Object)]
pub struct MemberInfo {}

#[cfg(feature = "system-reflection-memberinfo")]
#[::unity2::methods]
impl MemberInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_MemberType", args = 0)]
    pub fn get_member_type(self) -> crate::system::reflection::membertypes::MemberTypes;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "get_ReflectedType", args = 0)]
    pub fn get_reflected_type(self) -> ::unity2::SystemType;

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

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = "GetCustomAttributesData", args = 0)]
    pub fn get_custom_attributes_data(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = "get_MetadataToken", args = 0)]
    pub fn get_metadata_token(self) -> i32;

    #[method(name = "get_Module", args = 0)]
    pub fn get_module(self) -> crate::system::reflection::module::Module;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        left: crate::system::reflection::memberinfo::MemberInfo,
        right: crate::system::reflection::memberinfo::MemberInfo,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        left: crate::system::reflection::memberinfo::MemberInfo,
        right: crate::system::reflection::memberinfo::MemberInfo,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}

#[cfg(feature = "system-reflection-memberinfo")]
impl MemberInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MemberInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IMemberInfoMethods>::ctor(this);
        this
    }
}
