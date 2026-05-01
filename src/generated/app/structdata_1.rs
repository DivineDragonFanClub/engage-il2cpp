
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structdata_1/StructData_1_EachFuncVoid.md")))]
#[::unity2::class(namespace = "App", name = "StructData`1.EachFuncVoid")]
pub struct StructData_1_EachFuncVoid<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-structdata_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> StructData_1_EachFuncVoid<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, data: T0) -> ();
}

#[cfg(feature = "app-structdata_1")]
impl<T0: ::unity2::ClassIdentity> StructData_1_EachFuncVoid<T0> {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructData_1_EachFuncVoid),
                ::core::stringify!(new),
            )
        });
        <Self as IStructData_1_EachFuncVoidMethods<T0>>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structdata_1/StructData_1.md")))]
#[::unity2::class(namespace = "App", name = "StructData`1")]
pub struct StructData_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_list")]
    pub s_list: crate::app::structlist_1::StructList_1<T0>,
    #[static_field]
    #[rename(name = "s_loaded")]
    pub s_loaded: bool,
}

#[cfg(feature = "app-structdata_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> StructData_1<T0> {
    #[method(name = "AddPublicLabel", args = 1)]
    pub fn add_public_label(instance: T0) -> ();

    #[method(name = "get_PublicNames", args = 0)]
    pub fn get_public_names() -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "Import", args = 3)]
    pub fn import(
        data: ::unity2::Array<u8>,
        path: ::unity2::Il2CppString,
        sheet_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Load", args = 3)]
    pub fn load(
        path: ::unity2::Il2CppString,
        sheet_name: ::unity2::Il2CppString,
        completed: crate::system::action::Action,
    ) -> ();

    #[method(name = "Completed", args = 0)]
    pub fn completed() -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(name: ::unity2::Il2CppString) -> T0;

    #[method(name = "Get", args = 1)]
    pub fn get_2(index: i32) -> T0;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(name: ::unity2::Il2CppString) -> T0;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get_2(index: i32) -> T0;

    #[method(name = "TryGetFromHash", args = 1)]
    pub fn try_get_from_hash(hash: i32) -> T0;

    #[method(name = "TryGet", args = 2)]
    pub fn try_get_3(name: ::unity2::Il2CppString, value: T0) -> bool;

    #[method(name = "TryGet", args = 2)]
    pub fn try_get_4(index: i32, value: T0) -> bool;

    #[method(name = "UnsafeGet", args = 1)]
    pub fn unsafe_get(index: i32) -> T0;

    #[method(name = "GetIndex", args = 1)]
    pub fn get_index(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "TryGetIndex", args = 1)]
    pub fn try_get_index(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetIndex", args = 1)]
    pub fn get_index_2(data: T0) -> i32;

    #[method(name = "GetCount", args = 0)]
    pub fn get_count() -> i32;

    #[method(name = "GetList", args = 0)]
    pub fn get_list() -> crate::system::collections::generic::list_1::List_1<T0>;

    #[method(name = "ForEach", args = 1)]
    pub fn for_each(func: crate::app::structdata_1::StructData_1_EachFuncVoid<T0>) -> ();

    #[method(name = "ForEach", args = 1)]
    pub fn for_each_2(func: crate::app::structdata_1::StructData_1_EachFuncBool<T0>) -> T0;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-structdata_1")]
impl<T0: ::unity2::ClassIdentity> StructData_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructData_1),
                ::core::stringify!(new),
            )
        });
        <Self as IStructData_1Methods<T0>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structdata_1/StructData_1_EachFuncBool.md")))]
#[::unity2::class(namespace = "App", name = "StructData`1.EachFuncBool")]
pub struct StructData_1_EachFuncBool<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-structdata_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> StructData_1_EachFuncBool<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, data: T0) -> bool;
}

#[cfg(feature = "app-structdata_1")]
impl<T0: ::unity2::ClassIdentity> StructData_1_EachFuncBool<T0> {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructData_1_EachFuncBool),
                ::core::stringify!(new),
            )
        });
        <Self as IStructData_1_EachFuncBoolMethods<T0>>::ctor(this, object, method);
        this
    }
}
