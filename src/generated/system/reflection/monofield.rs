
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::fieldinfo::FieldInfo;
use crate::system::reflection::fieldinfo::IFieldInfo;
use crate::system::reflection::memberinfo::IMemberInfo;
use crate::system::reflection::memberinfo::MemberInfo;
use crate::system::reflection::rtfieldinfo::IRtFieldInfo;
use crate::system::reflection::rtfieldinfo::RtFieldInfo;
use crate::system::reflection::runtimefieldinfo::IRuntimeFieldInfo;
use crate::system::reflection::runtimefieldinfo::RuntimeFieldInfo;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monofield/MonoField.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoField")]
#[parent(crate::system::reflection::rtfieldinfo::RtFieldInfo)]
pub struct MonoField {
    #[rename(name = "klass")]
    pub klass: ::unity2::IntPtr,
    #[rename(name = "name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "type")]
    pub r#type: ::unity2::SystemType,
    #[rename(name = "attrs")]
    pub attrs: crate::system::reflection::fieldattributes::FieldAttributes,
}

#[cfg(feature = "system-reflection-monofield")]
#[::unity2::methods]
impl MonoField {
    #[method(name = "get_Attributes", args = 0)]
    pub fn get_attributes(self) -> crate::system::reflection::fieldattributes::FieldAttributes;

    #[method(name = "ResolveType", args = 0)]
    pub fn resolve_type(self) -> ::unity2::SystemType;

    #[method(name = "get_FieldType", args = 0)]
    pub fn get_field_type(self) -> ::unity2::SystemType;

    #[method(name = "GetParentType", args = 1)]
    pub fn get_parent_type(self, declaring: bool) -> ::unity2::SystemType;

    #[method(name = "get_ReflectedType", args = 0)]
    pub fn get_reflected_type(self) -> ::unity2::SystemType;

    #[method(name = "get_DeclaringType", args = 0)]
    pub fn get_declaring_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

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

    #[method(name = "GetFieldOffset", args = 0)]
    pub fn get_field_offset(self) -> i32;

    #[method(name = "GetValueInternal", args = 1)]
    pub fn get_value_internal(
        self,
        obj: crate::system::object::Object,
    ) -> crate::system::object::Object;

    #[method(name = "GetValue", args = 1)]
    pub fn get_value(self, obj: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "SetValueInternal", args = 3)]
    pub fn set_value_internal(
        fi: crate::system::reflection::fieldinfo::FieldInfo,
        obj: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetRawConstantValue", args = 0)]
    pub fn get_raw_constant_value(self) -> crate::system::object::Object;

    #[method(name = "GetCustomAttributesData", args = 0)]
    pub fn get_custom_attributes_data(
        self,
    ) -> crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
        crate::system::reflection::customattributedata::CustomAttributeData,
    >;

    #[method(name = "CheckGeneric", args = 0)]
    pub fn check_generic(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-monofield")]
impl MonoField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoField),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoFieldMethods>::ctor(this);
        this
    }
}
