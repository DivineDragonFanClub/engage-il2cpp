
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamevariable/GameVariable_Value.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct GameVariable_Value {
    pub number: i32,
    pub string: ::unity2::Il2CppString,
}

impl ::unity2::ClassIdentity for GameVariable_Value {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameVariable.Value";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameVariable_Value {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-gamevariable")]
#[::unity2::methods(value)]
impl GameVariable_Value {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, num: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, str: ::unity2::Il2CppString) -> ();

    #[method(name = "IsNumber", args = 0)]
    pub fn is_number(self) -> bool;

    #[method(name = "IsString", args = 0)]
    pub fn is_string(self) -> bool;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamevariable/GameVariable.md")))]
#[::unity2::class(namespace = "App", name = "GameVariable")]
#[parent(crate::system::object::Object)]
pub struct GameVariable {
    #[static_field]
    #[rename(name = "Type_Number")]
    pub type_number: u8,
    #[static_field]
    #[rename(name = "Type_String")]
    pub type_string: u8,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "VersionForOnline")]
    pub version_for_online: i32,
    #[rename(name = "m_Capacity")]
    pub m_capacity: i32,
    #[rename(name = "m_Dictionary")]
    pub m_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::gamevariable::GameVariable_Value,
    >,
    #[rename(name = "m_Rewindable")]
    pub m_rewindable:
        crate::system::collections::generic::hashset_1::HashSet_1<::unity2::Il2CppString>,
    #[rename(name = "m_Monitors")]
    pub m_monitors: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
}

#[cfg(feature = "app-gamevariable")]
#[::unity2::methods]
impl GameVariable {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, capacity: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Capacity", args = 0)]
    pub fn get_capacity(self) -> i32;

    #[method(name = "CanRwind", args = 1)]
    pub fn can_rwind(self, key: ::unity2::Il2CppString) -> bool;

    #[method(name = "Entry", args = 2)]
    pub fn entry(self, key: ::unity2::Il2CppString, num: i32) -> bool;

    #[method(name = "Entry", args = 2)]
    pub fn entry_2(self, key: ::unity2::Il2CppString, str: ::unity2::Il2CppString) -> bool;

    #[method(name = "EntryNoRewind", args = 2)]
    pub fn entry_no_rewind(self, key: ::unity2::Il2CppString, num: i32) -> bool;

    #[method(name = "EntryNoRewind", args = 2)]
    pub fn entry_no_rewind_2(
        self,
        key: ::unity2::Il2CppString,
        str: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "EntryImpl", args = 3)]
    pub fn entry_impl(
        self,
        key: ::unity2::Il2CppString,
        value: crate::app::gamevariable::GameVariable_Value,
        rewindable: bool,
    ) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: ::unity2::Il2CppString) -> bool;

    #[method(name = "RemoveImpl", args = 1)]
    pub fn remove_impl(self, key: ::unity2::Il2CppString) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(self, key: ::unity2::Il2CppString) -> i32;

    #[method(name = "IsString", args = 1)]
    pub fn is_string(self, key: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetNumber", args = 1)]
    pub fn get_number(self, key: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetBool", args = 1)]
    pub fn get_bool(self, key: ::unity2::Il2CppString) -> bool;

    #[method(name = "TryGetBool", args = 1)]
    pub fn try_get_bool(self, key: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetString", args = 1)]
    pub fn get_string(self, key: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "TryGet", args = 2)]
    pub fn try_get(self, key: ::unity2::Il2CppString, empty: i32) -> i32;

    #[method(name = "Set", args = 2)]
    pub fn set(self, key: ::unity2::Il2CppString, enable: bool) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_2(self, key: ::unity2::Il2CppString, num: i32) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_3(self, key: ::unity2::Il2CppString, str: ::unity2::Il2CppString) -> ();

    #[method(name = "SetNumber", args = 2)]
    pub fn set_number(self, key: ::unity2::Il2CppString, num: i32) -> ();

    #[method(name = "SetString", args = 2)]
    pub fn set_string(self, key: ::unity2::Il2CppString, str: ::unity2::Il2CppString) -> ();

    #[method(name = "Add", args = 4)]
    pub fn add(self, key: ::unity2::Il2CppString, count: i32, min: i32, max: i32) -> i32;

    #[method(name = "ClearAll", args = 0)]
    pub fn clear_all(self) -> ();

    #[method(name = "ClearLocal", args = 0)]
    pub fn clear_local(self) -> ();

    #[method(name = "ClearChapter", args = 0)]
    pub fn clear_chapter(self) -> ();

    #[method(name = "ClearHub", args = 0)]
    pub fn clear_hub(self) -> ();

    #[method(name = "ClearStartsWith", args = 1)]
    pub fn clear_starts_with(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "FindStartsWith", args = 1)]
    pub fn find_starts_with(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "ClearKeys", args = 1)]
    pub fn clear_keys(
        self,
        keys: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist(self, key: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsLocal", args = 1)]
    pub fn is_local(key: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsChapter", args = 1)]
    pub fn is_chapter(key: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsHub", args = 1)]
    pub fn is_hub(key: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsGlobal", args = 1)]
    pub fn is_global(key: ::unity2::Il2CppString) -> bool;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SerializeKeyValue", args = 2)]
    pub fn serialize_key_value(
        self,
        stream: crate::app::stream_2::Stream_2,
        key_value: crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<
            ::unity2::Il2CppString,
            crate::app::gamevariable::GameVariable_Value,
        >,
    ) -> ();

    #[method(name = "DeserializeKeyValue", args = 1)]
    pub fn deserialize_key_value(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SerializeForOnline", args = 1)]
    pub fn serialize_for_online(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeForOnline", args = 1)]
    pub fn deserialize_for_online(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "FindKeys", args = 1)]
    pub fn find_keys(
        self,
        header: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "GetName", args = 1)]
    pub fn get_name(self, key: ::unity2::Il2CppString) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-gamevariable")]
impl GameVariable {
    pub fn new(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameVariable),
                ::core::stringify!(new),
            )
        });
        <Self as IGameVariableMethods>::ctor(this, capacity);
        this
    }
}
