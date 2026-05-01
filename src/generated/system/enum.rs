
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/enum/Enum.md")))]
#[::unity2::class(namespace = "System", name = "Enum")]
#[parent(crate::system::valuetype::ValueType)]
pub struct Enum {
    #[static_field]
    #[rename(name = "enumSeperatorCharArray")]
    pub enum_seperator_char_array: ::unity2::Array<u16>,
    #[static_field]
    #[rename(name = "enumSeperator")]
    pub enum_seperator: ::unity2::Il2CppString,
}

#[cfg(feature = "system-enum")]
#[::unity2::methods]
impl Enum {
    #[method(name = "InternalFormattedHexString", args = 1)]
    pub fn internal_formatted_hex_string(
        value: crate::system::object::Object,
    ) -> ::unity2::Il2CppString;

    #[method(name = "InternalFormat", args = 2)]
    pub fn internal_format(
        e_t: crate::system::runtimetype::RuntimeType,
        value: crate::system::object::Object,
    ) -> ::unity2::Il2CppString;

    #[method(name = "InternalFlagsFormat", args = 2)]
    pub fn internal_flags_format(
        e_t: crate::system::runtimetype::RuntimeType,
        value: crate::system::object::Object,
    ) -> ::unity2::Il2CppString;

    #[method(name = "ToUInt64", args = 1)]
    pub fn to_u_int64(value: crate::system::object::Object) -> u64;

    #[method(name = "InternalCompareTo", args = 2)]
    pub fn internal_compare_to(
        o1: crate::system::object::Object,
        o2: crate::system::object::Object,
    ) -> i32;

    #[method(name = "InternalGetUnderlyingType", args = 1)]
    pub fn internal_get_underlying_type(
        enum_type: crate::system::runtimetype::RuntimeType,
    ) -> crate::system::runtimetype::RuntimeType;

    #[method(name = "GetEnumValuesAndNames", args = 3)]
    pub fn get_enum_values_and_names(
        enum_type: crate::system::runtimetype::RuntimeType,
        values: ::unity2::Array<u64>,
        names: ::unity2::Array<::unity2::Il2CppString>,
    ) -> bool;

    #[method(name = "InternalBoxEnum", args = 2)]
    pub fn internal_box_enum(
        enum_type: crate::system::runtimetype::RuntimeType,
        value: i64,
    ) -> crate::system::object::Object;

    #[method(name = "Parse", args = 3)]
    pub fn parse(
        enum_type: ::unity2::SystemType,
        value: ::unity2::Il2CppString,
        ignore_case: bool,
    ) -> crate::system::object::Object;

    #[method(name = "GetUnderlyingType", args = 1)]
    pub fn get_underlying_type(enum_type: ::unity2::SystemType) -> ::unity2::SystemType;

    #[method(name = "GetValues", args = 1)]
    pub fn get_values(enum_type: ::unity2::SystemType) -> ::unity2::IlInstance;

    #[method(name = "InternalGetValues", args = 1)]
    pub fn internal_get_values(
        enum_type: crate::system::runtimetype::RuntimeType,
    ) -> ::unity2::Array<u64>;

    #[method(name = "GetName", args = 2)]
    pub fn get_name(
        enum_type: ::unity2::SystemType,
        value: crate::system::object::Object,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetNames", args = 1)]
    pub fn get_names(enum_type: ::unity2::SystemType) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "InternalGetNames", args = 1)]
    pub fn internal_get_names(
        enum_type: crate::system::runtimetype::RuntimeType,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object(
        enum_type: ::unity2::SystemType,
        value: crate::system::object::Object,
    ) -> crate::system::object::Object;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(
        enum_type: ::unity2::SystemType,
        value: crate::system::object::Object,
    ) -> bool;

    #[method(name = "get_value", args = 0)]
    pub fn get_value(self) -> crate::system::object::Object;

    #[method(name = "InternalHasFlag", args = 1)]
    pub fn internal_has_flag(self, flags: crate::system::r#enum::Enum) -> bool;

    #[method(name = "get_hashcode", args = 0)]
    pub fn get_hashcode(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to(self, target: crate::system::object::Object) -> i32;

    #[method(name = "ToString", args = 1)]
    pub fn to_string_2(self, format: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "HasFlag", args = 1)]
    pub fn has_flag(self, flag: crate::system::r#enum::Enum) -> bool;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object_2(enum_type: ::unity2::SystemType, value: i8)
        -> crate::system::object::Object;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object_3(
        enum_type: ::unity2::SystemType,
        value: i16,
    ) -> crate::system::object::Object;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object_4(
        enum_type: ::unity2::SystemType,
        value: i32,
    ) -> crate::system::object::Object;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object_5(enum_type: ::unity2::SystemType, value: u8)
        -> crate::system::object::Object;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object_6(
        enum_type: ::unity2::SystemType,
        value: u16,
    ) -> crate::system::object::Object;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object_7(
        enum_type: ::unity2::SystemType,
        value: u32,
    ) -> crate::system::object::Object;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object_8(
        enum_type: ::unity2::SystemType,
        value: i64,
    ) -> crate::system::object::Object;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object_9(
        enum_type: ::unity2::SystemType,
        value: u64,
    ) -> crate::system::object::Object;

    #[method(name = "ToObject", args = 2)]
    pub fn to_object_10(
        enum_type: ::unity2::SystemType,
        value: bool,
    ) -> crate::system::object::Object;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-enum")]
impl Enum {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Enum),
                ::core::stringify!(new),
            )
        });
        <Self as IEnumMethods>::ctor(this);
        this
    }
}
