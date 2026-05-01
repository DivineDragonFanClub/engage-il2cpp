
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aivaluestringmanager/AIValueStringManager.md")))]
#[::unity2::class(namespace = "App", name = "AIValueStringManager")]
#[parent(crate::system::object::Object)]
pub struct AIValueStringManager {
    #[static_field]
    #[rename(name = "s_List")]
    pub s_list: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "s_AIDataStrings")]
    pub s_ai_data_strings:
        crate::system::collections::generic::hashset_1::HashSet_1<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "s_IsLoadingAIData")]
    pub s_is_loading_ai_data: bool,
}

#[cfg(feature = "app-aivaluestringmanager")]
#[::unity2::methods]
impl AIValueStringManager {
    #[method(name = "BeginLoadAIData", args = 0)]
    pub fn begin_load_ai_data() -> ();

    #[method(name = "EndLoadAIData", args = 0)]
    pub fn end_load_ai_data() -> ();

    #[method(name = "Register", args = 1)]
    pub fn register(str: ::unity2::Il2CppString) -> i32;

    #[method(name = "Reset", args = 0)]
    pub fn reset() -> ();

    #[method(name = "ClearAll", args = 0)]
    pub fn clear_all() -> ();

    #[method(name = "GetString", args = 1)]
    pub fn get_string(index: i32) -> ::unity2::Il2CppString;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "GetIndex", args = 1)]
    pub fn get_index(str: ::unity2::Il2CppString) -> i32;

    #[method(name = "Dump", args = 1)]
    pub fn dump(msg: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-aivaluestringmanager")]
impl AIValueStringManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIValueStringManager),
                ::core::stringify!(new),
            )
        });
        <Self as IAIValueStringManagerMethods>::ctor(this);
        this
    }
}
