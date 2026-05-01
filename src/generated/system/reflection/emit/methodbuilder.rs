
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::methodbase::IMethodBase;
use crate::system::reflection::methodbase::MethodBase;
use crate::system::reflection::methodinfo::IMethodInfo;
use crate::system::reflection::methodinfo::MethodInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/emit/methodbuilder/MethodBuilder.md")))]
#[::unity2::class(namespace = "System.Reflection.Emit", name = "MethodBuilder")]
#[parent(crate::system::reflection::methodinfo::MethodInfo)]
pub struct MethodBuilder {}

#[cfg(feature = "system-reflection-emit-methodbuilder")]
#[::unity2::methods]
impl MethodBuilder {
    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(self) -> crate::system::reflection::methodattributes::MethodAttributes;

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetParameters", args = 0)]
    pub fn get_parameters(
        self,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

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

    #[method(name = "GetMethodImplementationFlags", args = 0)]
    pub fn get_method_implementation_flags(
        self,
    ) -> crate::system::reflection::methodimplattributes::MethodImplAttributes;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;
}
