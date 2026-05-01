
use crate::app::sortconstant::ISortConstant;
use crate::app::sortconstant::SortConstant;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sort/Sort_ElementComparer.md")))]
#[::unity2::class(namespace = "App", name = "Sort.ElementComparer")]
#[parent(crate::system::object::Object)]
pub struct Sort_ElementComparer {}

#[cfg(feature = "app-sort")]
#[::unity2::methods]
impl Sort_ElementComparer {
    #[method(name = "Compare", args = 2)]
    pub fn compare(self, a: crate::system::object::Object, b: crate::system::object::Object)
        -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sort")]
impl Sort_ElementComparer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Sort_ElementComparer),
                ::core::stringify!(new),
            )
        });
        <Self as ISort_ElementComparerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sort/Sort.md")))]
#[::unity2::class(namespace = "App", name = "Sort")]
#[parent(crate::app::sortconstant::SortConstant)]
pub struct Sort {}

#[cfg(feature = "app-sort")]
#[::unity2::methods]
impl Sort {
    #[method(name = "InsertionSort", args = 1)]
    pub fn insertion_sort(list: crate::system::collections::ilist::IList) -> ();

    #[method(name = "InsertionSort", args = 2)]
    pub fn insertion_sort_2(
        list: crate::system::collections::ilist::IList,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> ();

    #[method(name = "InsertionSortPartly", args = 4)]
    pub fn insertion_sort_partly(
        list: crate::system::collections::ilist::IList,
        first: i32,
        last: i32,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> ();

    #[method(name = "MergeSort", args = 1)]
    pub fn merge_sort(list: crate::system::collections::ilist::IList) -> ();

    #[method(name = "MergeSort", args = 2)]
    pub fn merge_sort_2(
        list: crate::system::collections::ilist::IList,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> ();

    #[method(name = "MergeSortPartly", args = 5)]
    pub fn merge_sort_partly(
        list: crate::system::collections::ilist::IList,
        begin: i32,
        end: i32,
        work: ::unity2::Array<crate::system::object::Object>,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> ();

    #[method(name = "MergeSortMerge", args = 6)]
    pub fn merge_sort_merge(
        list: crate::system::collections::ilist::IList,
        begin: i32,
        middle: i32,
        end: i32,
        work: ::unity2::Array<crate::system::object::Object>,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sort")]
impl Sort {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Sort),
                ::core::stringify!(new),
            )
        });
        <Self as ISortMethods>::ctor(this);
        this
    }
}
