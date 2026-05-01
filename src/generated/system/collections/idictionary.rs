
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/idictionary/IDictionary.md")))]
#[::unity2::class(namespace = "System.Collections", name = "IDictionary")]
pub struct IDictionary {}

#[cfg(feature = "system-collections-idictionary")]
#[::unity2::methods]
impl IDictionary {
    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, key: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, key: crate::system::object::Object) -> bool;

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsFixedSize", args = 0)]
    pub fn get_is_fixed_size(self) -> bool;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::idictionaryenumerator::IDictionaryEnumerator;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: crate::system::object::Object) -> ();
}
