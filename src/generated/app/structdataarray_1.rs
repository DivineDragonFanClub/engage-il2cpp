
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structdataarray_1/StructDataArray_1.md")))]
#[::unity2::class(namespace = "App", name = "StructDataArray`1")]
pub struct StructDataArray_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_arrayList")]
    pub s_array_list: crate::app::structarraylist_1::StructArrayList_1<T0>,
    #[static_field]
    #[rename(name = "s_loaded")]
    pub s_loaded: bool,
}

#[cfg(feature = "app-structdataarray_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> StructDataArray_1<T0> {
    #[method(name = "get_ArrayName", args = 0)]
    pub fn get_array_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ArrayName", args = 1)]
    pub fn set_array_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ArrayList", args = 0)]
    pub fn get_array_list() -> crate::app::structarraylist_1::StructArrayList_1<T0>;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Load", args = 3)]
    pub fn load(
        path: ::unity2::Il2CppString,
        sheet_name: ::unity2::Il2CppString,
        completed: crate::system::action::Action,
    ) -> ();

    #[method(name = "Import", args = 3)]
    pub fn import(
        data: ::unity2::Array<u8>,
        path: ::unity2::Il2CppString,
        sheet_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "AddArrayList", args = 1)]
    pub fn add_array_list(list: crate::app::structdataarraylist_1::StructDataArrayList_1<T0>)
        -> ();

    #[method(name = "Completed", args = 0)]
    pub fn completed() -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(
        name: ::unity2::Il2CppString,
    ) -> crate::app::structdataarraylist_1::StructDataArrayList_1<T0>;

    #[method(name = "Get", args = 1)]
    pub fn get_2(index: i32) -> crate::app::structdataarraylist_1::StructDataArrayList_1<T0>;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(
        name: ::unity2::Il2CppString,
    ) -> crate::app::structdataarraylist_1::StructDataArrayList_1<T0>;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get_2(index: i32) -> crate::app::structdataarraylist_1::StructDataArrayList_1<T0>;

    #[method(name = "TryGetFromHash", args = 1)]
    pub fn try_get_from_hash(
        hash: i32,
    ) -> crate::app::structdataarraylist_1::StructDataArrayList_1<T0>;

    #[method(name = "GetIndex", args = 1)]
    pub fn get_index(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetKey", args = 1)]
    pub fn get_key(index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetCount", args = 0)]
    pub fn get_count() -> i32;

    #[method(name = "GetList", args = 0)]
    pub fn get_list() -> crate::app::structarraylist_1::StructArrayList_1<T0>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-structdataarray_1")]
impl<T0: ::unity2::ClassIdentity> StructDataArray_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructDataArray_1),
                ::core::stringify!(new),
            )
        });
        <Self as IStructDataArray_1Methods<T0>>::ctor(this);
        this
    }
}
