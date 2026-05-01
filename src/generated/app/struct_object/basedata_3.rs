
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/struct_object/basedata_3/BaseData_3.md")))]
#[::unity2::class(namespace = "App.StructObject", name = "BaseData`3")]
pub struct BaseData_3<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
    T2: ::unity2::ClassIdentity,
> {
    #[static_field]
    #[rename(name = "instance")]
    pub instance: T0,
    #[rename(name = "IndexKey")]
    pub index_key: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        i32,
    >,
    #[rename(name = "Items")]
    pub items: crate::system::collections::generic::list_1::List_1<T1>,
}

#[cfg(feature = "app-struct_object-basedata_3")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity, T2: ::unity2::ClassIdentity>
    BaseData_3<T0, T1, T2>
{
    #[method(name = "Load", args = 1)]
    pub fn load(path: ::unity2::Il2CppString) -> i32;

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "AddItem", args = 1)]
    pub fn add_item(self, item: T1) -> ();

    #[method(name = "SetMaxItem", args = 1)]
    pub fn set_max_item(self, piece: T2) -> ();

    #[method(name = "SetMinItem", args = 1)]
    pub fn set_min_item(self, piece: T2) -> ();

    #[method(name = "SetChgItem", args = 1)]
    pub fn set_chg_item(self, piece: T2) -> ();

    #[method(name = "CreateDictionary", args = 0)]
    pub fn create_dictionary(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "GetIndex", args = 1)]
    pub fn get_index(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetKey", args = 1)]
    pub fn get_key(index: i32) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 1)]
    pub fn get(index: i32) -> T1;

    #[method(name = "Get", args = 1)]
    pub fn get_2(name: ::unity2::Il2CppString) -> T1;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(index: i32) -> T1;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get_2(name: ::unity2::Il2CppString) -> T1;

    #[method(name = "GetItemCount", args = 0)]
    pub fn get_item_count() -> i32;

    #[method(name = "GetItemList", args = 0)]
    pub fn get_item_list() -> crate::system::collections::generic::list_1::List_1<T1>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-struct_object-basedata_3")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity, T2: ::unity2::ClassIdentity>
    BaseData_3<T0, T1, T2>
{
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseData_3),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseData_3Methods<T0, T1, T2>>::ctor(this);
        this
    }
}
