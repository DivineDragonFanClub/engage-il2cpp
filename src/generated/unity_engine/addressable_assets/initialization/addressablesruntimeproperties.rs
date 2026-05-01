
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/initialization/addressablesruntimeproperties/AddressablesRuntimeProperties.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets.Initialization",
    name = "AddressablesRuntimeProperties"
)]
#[parent(crate::system::object::Object)]
pub struct AddressablesRuntimeProperties {
    #[static_field]
    #[rename(name = "s_TokenStack")]
    pub s_token_stack:
        crate::system::collections::generic::stack_1::Stack_1<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "s_TokenStartStack")]
    pub s_token_start_stack: crate::system::collections::generic::stack_1::Stack_1<i32>,
    #[static_field]
    #[rename(name = "s_CachedValues")]
    pub s_cached_values: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >,
}

#[cfg(feature = "unity_engine-addressable_assets-initialization-addressablesruntimeproperties")]
#[::unity2::methods]
impl AddressablesRuntimeProperties {
    #[method(name = "GetAssemblies", args = 0)]
    pub fn get_assemblies() -> ::unity2::Array<crate::system::reflection::assembly::Assembly>;

    #[method(name = "GetCachedValueCount", args = 0)]
    pub fn get_cached_value_count() -> i32;

    #[method(name = "SetPropertyValue", args = 2)]
    pub fn set_property_value(name: ::unity2::Il2CppString, val: ::unity2::Il2CppString) -> ();

    #[method(name = "ClearCachedPropertyValues", args = 0)]
    pub fn clear_cached_property_values() -> ();

    #[method(name = "EvaluateProperty", args = 1)]
    pub fn evaluate_property(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "EvaluateString", args = 1)]
    pub fn evaluate_string(input: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "EvaluateString", args = 4)]
    pub fn evaluate_string_2(
        input_string: ::unity2::Il2CppString,
        start_delimiter: u16,
        end_delimiter: u16,
        var_func: crate::system::func_2::Func_2<::unity2::Il2CppString, ::unity2::Il2CppString>,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
