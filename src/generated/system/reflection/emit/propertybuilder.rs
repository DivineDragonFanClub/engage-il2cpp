
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::propertyinfo::IPropertyInfo;
use crate::system::reflection::propertyinfo::PropertyInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/emit/propertybuilder/PropertyBuilder.md")))]
#[::unity2::class(namespace = "System.Reflection.Emit", name = "PropertyBuilder")]
#[parent(crate::system::reflection::propertyinfo::PropertyInfo)]
pub struct PropertyBuilder {}

#[cfg(feature = "system-reflection-emit-propertybuilder")]
#[::unity2::methods]
impl PropertyBuilder {
    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(
        self,
    ) -> crate::system::reflection::propertyattributes::PropertyAttributes;

    #[method(name = "get_CanRead", args = 0)]
    pub fn get_can_read(self) -> bool;

    #[method(name = "get_CanWrite", args = 0)]
    pub fn get_can_write(self) -> bool;

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_PropertyType", args = 0)]
    pub fn get_property_type(self) -> ::unity2::SystemType;

    #[method(name = "GetIndexParameters", args = 0)]
    pub fn get_index_parameters(
        self,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

    #[method(name = "get_ReflectedType", args = 0)]
    pub fn get_reflected_type(self) -> ::unity2::SystemType;

    #[method(name = "GetAccessors", args = 1)]
    pub fn get_accessors(
        self,
        non_public: bool,
    ) -> ::unity2::Array<crate::system::reflection::methodinfo::MethodInfo>;

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

    #[method(name = "GetGetMethod", args = 1)]
    pub fn get_get_method(
        self,
        non_public: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetSetMethod", args = 1)]
    pub fn get_set_method(
        self,
        non_public: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;
}
