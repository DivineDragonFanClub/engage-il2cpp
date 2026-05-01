
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/data_structs/slice_1/Slice_1.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.DataStructs", name = "Slice`1")]
pub struct Slice_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_SourceList")]
    pub m_source_list:
        crate::system::collections::generic::ilist_1_interface::IList_1_Interface<T0>,
    #[rename(name = "m_From")]
    pub m_from: i32,
    #[rename(name = "m_Length")]
    pub m_length: i32,
    #[rename(name = "m_Reversed")]
    pub m_reversed: bool,
}

#[cfg(feature = "moon_sharp-interpreter-data_structs-slice_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Slice_1<T0> {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        list: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<T0>,
        from: i32,
        length: i32,
        reversed: bool,
    ) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> T0;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: T0) -> ();

    #[method(name = "get_From", args = 0)]
    pub fn get_from(self) -> i32;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Reversed", args = 0)]
    pub fn get_reversed(self) -> bool;

    #[method(name = "CalcRealIndex", args = 1)]
    pub fn calc_real_index(self, index: i32) -> i32;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::ienumerator_1::IEnumerator_1<T0>;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ToArray", args = 0)]
    pub fn to_array(self) -> ::unity2::Array<T0>;

    #[method(name = "ToList", args = 0)]
    pub fn to_list(self) -> crate::system::collections::generic::list_1::List_1<T0>;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, item: T0) -> i32;

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, index: i32, item: T0) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, item: T0) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, item: T0) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::Array<T0>, array_index: i32) -> ();

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, item: T0) -> bool;
}

#[cfg(feature = "moon_sharp-interpreter-data_structs-slice_1")]
impl<T0: ::unity2::ClassIdentity> Slice_1<T0> {
    pub fn new(
        list: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<T0>,
        from: i32,
        length: i32,
        reversed: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Slice_1),
                ::core::stringify!(new),
            )
        });
        <Self as ISlice_1Methods<T0>>::ctor(this, list, from, length, reversed);
        this
    }
}
