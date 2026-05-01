
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/parameterinfo/ParameterInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "ParameterInfo")]
#[parent(crate::system::object::Object)]
pub struct ParameterInfo {
    #[rename(name = "ClassImpl")]
    pub class_impl: ::unity2::SystemType,
    #[rename(name = "DefaultValueImpl")]
    pub default_value_impl: ::unity2::IlInstance,
    #[rename(name = "MemberImpl")]
    pub member_impl: crate::system::reflection::memberinfo::MemberInfo,
    #[rename(name = "NameImpl")]
    pub name_impl: ::unity2::Il2CppString,
    #[rename(name = "PositionImpl")]
    pub position_impl: i32,
    #[rename(name = "AttrsImpl")]
    pub attrs_impl: crate::system::reflection::parameterattributes::ParameterAttributes,
}

#[cfg(feature = "system-reflection-parameterinfo")]
#[::unity2::methods]
impl ParameterInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ParameterType", args = 0)]
    pub fn get_parameter_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(
        self,
    ) -> crate::system::reflection::parameterattributes::ParameterAttributes;

    #[method(name = "get_IsIn", args = 0)]
    pub fn get_is_in(self) -> bool;

    #[method(name = "get_IsOptional", args = 0)]
    pub fn get_is_optional(self) -> bool;

    #[method(name = "get_IsOut", args = 0)]
    pub fn get_is_out(self) -> bool;

    #[method(name = "get_IsRetval", args = 0)]
    pub fn get_is_retval(self) -> bool;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> i32;

    #[method(name = "GetPseudoCustomAttributes", args = 0)]
    pub fn get_pseudo_custom_attributes(self) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "GetDefaultValueImpl", args = 0)]
    pub fn get_default_value_impl(self) -> crate::system::object::Object;

    #[method(name = "get_DefaultValue", args = 0)]
    pub fn get_default_value(self) -> crate::system::object::Object;

    #[method(name = "GetCustomAttributes", args = 2)]
    pub fn get_custom_attributes(
        self,
        attribute_type: ::unity2::SystemType,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = "New", args = 2)]
    pub fn new(
        pinfo: crate::system::reflection::parameterinfo::ParameterInfo,
        member: crate::system::reflection::memberinfo::MemberInfo,
    ) -> crate::system::reflection::parameterinfo::ParameterInfo;
}
