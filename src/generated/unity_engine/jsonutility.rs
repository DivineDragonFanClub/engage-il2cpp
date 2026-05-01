
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/jsonutility/JsonUtility.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "JsonUtility")]
#[parent(crate::system::object::Object)]
pub struct JsonUtility {}

#[cfg(feature = "unity_engine-jsonutility")]
#[::unity2::methods]
impl JsonUtility {
    #[method(name = "ToJsonInternal", args = 2)]
    pub fn to_json_internal(
        obj: crate::system::object::Object,
        pretty_print: bool,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FromJsonInternal", args = 3)]
    pub fn from_json_internal(
        json: ::unity2::Il2CppString,
        object_to_overwrite: crate::system::object::Object,
        r#type: ::unity2::SystemType,
    ) -> crate::system::object::Object;

    #[method(name = "ToJson", args = 1)]
    pub fn to_json(obj: crate::system::object::Object) -> ::unity2::Il2CppString;

    #[method(name = "ToJson", args = 2)]
    pub fn to_json_2(
        obj: crate::system::object::Object,
        pretty_print: bool,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FromJson", args = 2)]
    pub fn from_json(
        json: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> crate::system::object::Object;
}
