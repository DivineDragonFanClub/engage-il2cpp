
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/utility/serializationutilities/SerializationUtilities_ObjectType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SerializationUtilities_ObjectType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SerializationUtilities_ObjectType {
    const NAMESPACE: &'static str = "UnityEngine.AddressableAssets.Utility";

    const NAME: &'static str = "SerializationUtilities.ObjectType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SerializationUtilities_ObjectType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SerializationUtilities_ObjectType {
    pub fn ascii_string() -> Self {
        Self { value: 0 }
    }

    pub fn unicode_string() -> Self {
        Self { value: 1 }
    }

    pub fn u_int16() -> Self {
        Self { value: 2 }
    }

    pub fn u_int32() -> Self {
        Self { value: 3 }
    }

    pub fn int32() -> Self {
        Self { value: 4 }
    }

    pub fn hash128() -> Self {
        Self { value: 5 }
    }

    pub fn r#type() -> Self {
        Self { value: 6 }
    }

    pub fn json_object() -> Self {
        Self { value: 7 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/utility/serializationutilities/SerializationUtilities.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets.Utility",
    name = "SerializationUtilities"
)]
#[parent(crate::system::object::Object)]
pub struct SerializationUtilities {}

#[cfg(feature = "unity_engine-addressable_assets-utility-serializationutilities")]
#[::unity2::methods]
impl SerializationUtilities {
    #[method(name = "ReadInt32FromByteArray", args = 2)]
    pub fn read_int32_from_byte_array(data: ::unity2::Array<u8>, offset: i32) -> i32;

    #[method(name = "WriteInt32ToByteArray", args = 3)]
    pub fn write_int32_to_byte_array(data: ::unity2::Array<u8>, val: i32, offset: i32) -> i32;

    #[method(name = "ReadObjectFromByteArray", args = 2)]
    pub fn read_object_from_byte_array(
        key_data: ::unity2::Array<u8>,
        data_index: i32,
    ) -> crate::system::object::Object;

    #[method(name = "WriteObjectToByteList", args = 2)]
    pub fn write_object_to_byte_list(
        obj: crate::system::object::Object,
        buffer: crate::system::collections::generic::list_1::List_1<u8>,
    ) -> i32;
}
