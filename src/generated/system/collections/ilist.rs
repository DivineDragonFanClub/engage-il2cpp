
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/ilist/IList.md")))]
#[::unity2::class(namespace = "System.Collections", name = "IList")]
pub struct IList {}

#[cfg(feature = "system-collections-ilist")]
#[::unity2::methods]
impl IList {
    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, value: crate::system::object::Object) -> i32;

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, value: crate::system::object::Object) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, value: crate::system::object::Object) -> i32;

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, value: crate::system::object::Object) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();
}
