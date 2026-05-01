
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/propertyinfo/PropertyInfo.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "PropertyInfo")]
#[parent(crate::system::reflection::memberinfo::MemberInfo)]
pub struct PropertyInfo {}

#[cfg(feature = "system-reflection-propertyinfo")]
#[::unity2::methods]
impl PropertyInfo {
    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(
        self,
    ) -> crate::system::reflection::propertyattributes::PropertyAttributes;

    #[method(name = "get_CanRead", args = 0)]
    pub fn get_can_read(self) -> bool;

    #[method(name = "get_CanWrite", args = 0)]
    pub fn get_can_write(self) -> bool;

    #[method(name = "get_IsSpecialName", args = 0)]
    pub fn get_is_special_name(self) -> bool;

    #[method(name = "get_MemberType", args = 0)]
    pub fn get_member_type(self) -> crate::system::reflection::membertypes::MemberTypes;

    #[method(name = "get_PropertyType", args = 0)]
    pub fn get_property_type(self) -> ::unity2::SystemType;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetAccessors", args = 1)]
    pub fn get_accessors(
        self,
        non_public: bool,
    ) -> ::unity2::Array<crate::system::reflection::methodinfo::MethodInfo>;

    #[method(name = "GetGetMethod", args = 1)]
    pub fn get_get_method(
        self,
        non_public: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetIndexParameters", args = 0)]
    pub fn get_index_parameters(
        self,
    ) -> ::unity2::Array<crate::system::reflection::parameterinfo::ParameterInfo>;

    #[method(name = "GetSetMethod", args = 0)]
    pub fn get_set_method(self) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetSetMethod", args = 1)]
    pub fn get_set_method_2(
        self,
        non_public: bool,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetValue", args = 2)]
    pub fn get_value(
        self,
        obj: crate::system::object::Object,
        index: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::system::object::Object;

    #[method(name = "GetValue", args = 1)]
    pub fn get_value_2(self, obj: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "SetValue", args = 3)]
    pub fn set_value(
        self,
        obj: crate::system::object::Object,
        value: crate::system::object::Object,
        index: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "SetValue", args = 2)]
    pub fn set_value_2(
        self,
        obj: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetOptionalCustomModifiers", args = 0)]
    pub fn get_optional_custom_modifiers(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetRequiredCustomModifiers", args = 0)]
    pub fn get_required_custom_modifiers(self) -> ::unity2::Array<::unity2::SystemType>;

    #[method(name = "GetConstantValue", args = 0)]
    pub fn get_constant_value(self) -> crate::system::object::Object;

    #[method(name = "GetRawConstantValue", args = 0)]
    pub fn get_raw_constant_value(self) -> crate::system::object::Object;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        left: crate::system::reflection::propertyinfo::PropertyInfo,
        right: crate::system::reflection::propertyinfo::PropertyInfo,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        left: crate::system::reflection::propertyinfo::PropertyInfo,
        right: crate::system::reflection::propertyinfo::PropertyInfo,
    ) -> bool;

    #[method(name = "internal_from_handle_type", args = 2)]
    pub fn internal_from_handle_type(
        event_handle: ::unity2::IntPtr,
        type_handle: ::unity2::IntPtr,
    ) -> crate::system::reflection::propertyinfo::PropertyInfo;
}

#[cfg(feature = "system-reflection-propertyinfo")]
impl PropertyInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PropertyInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IPropertyInfoMethods>::ctor(this);
        this
    }
}
