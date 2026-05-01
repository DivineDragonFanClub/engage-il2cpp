
use crate::system::collections::specialized::nameobjectcollectionbase::INameObjectCollectionBase;
use crate::system::collections::specialized::nameobjectcollectionbase::NameObjectCollectionBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/namevaluecollection/NameValueCollection.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "NameValueCollection"
)]
#[parent(
    crate::system::collections::specialized::nameobjectcollectionbase::NameObjectCollectionBase
)]
pub struct NameValueCollection {
    #[rename(name = "_all")]
    pub all: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "_allKeys")]
    pub all_keys: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "system-collections-specialized-namevaluecollection")]
#[::unity2::methods]
impl NameValueCollection {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        capacity: i32,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> ();

    #[method(name = "InvalidateCachedArrays", args = 0)]
    pub fn invalidate_cached_arrays(self) -> ();

    #[method(name = "GetAsOneString", args = 1)]
    pub fn get_as_one_string(
        list: crate::system::collections::arraylist::ArrayList,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetAsStringArray", args = 1)]
    pub fn get_as_string_array(
        list: crate::system::collections::arraylist::ArrayList,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "Add", args = 2)]
    pub fn add(self, name: ::unity2::Il2CppString, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(self, name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetValues", args = 1)]
    pub fn get_values(
        self,
        name: ::unity2::Il2CppString,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "Set", args = 2)]
    pub fn set(self, name: ::unity2::Il2CppString, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, name: ::unity2::Il2CppString, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get_2(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetKey", args = 1)]
    pub fn get_key(self, index: i32) -> ::unity2::Il2CppString;
}

#[cfg(feature = "system-collections-specialized-namevaluecollection")]
impl NameValueCollection {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NameValueCollection),
                ::core::stringify!(new),
            )
        });
        <Self as INameValueCollectionMethods>::ctor(this);
        this
    }

    pub fn new_2(
        capacity: i32,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NameValueCollection),
                ::core::stringify!(new_2),
            )
        });
        <Self as INameValueCollectionMethods>::ctor_2(this, capacity, equality_comparer);
        this
    }
}
