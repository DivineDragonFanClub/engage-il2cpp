
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/arraysorthelper_2/ArraySortHelper_2.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "ArraySortHelper`2")]
pub struct ArraySortHelper_2<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_defaultArraySortHelper")]
    pub s_default_array_sort_helper:
        crate::system::collections::generic::arraysorthelper_2::ArraySortHelper_2<T0, T1>,
}

#[cfg(feature = "system-collections-generic-arraysorthelper_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ArraySortHelper_2<T0, T1> {
    #[method(name = "get_Default", args = 0)]
    pub fn get_default(
    ) -> crate::system::collections::generic::arraysorthelper_2::ArraySortHelper_2<T0, T1>;

    #[method(name = "CreateArraySortHelper", args = 0)]
    pub fn create_array_sort_helper(
    ) -> crate::system::collections::generic::arraysorthelper_2::ArraySortHelper_2<T0, T1>;

    #[method(name = "Sort", args = 5)]
    pub fn sort(
        self,
        keys: ::unity2::Array<T0>,
        values: ::unity2::Array<T1>,
        index: i32,
        length: i32,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = "SwapIfGreaterWithItems", args = 5)]
    pub fn swap_if_greater_with_items(
        keys: ::unity2::Array<T0>,
        values: ::unity2::Array<T1>,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
        a: i32,
        b: i32,
    ) -> ();

    #[method(name = "Swap", args = 4)]
    pub fn swap(keys: ::unity2::Array<T0>, values: ::unity2::Array<T1>, i: i32, j: i32) -> ();

    #[method(name = "IntrospectiveSort", args = 5)]
    pub fn introspective_sort(
        keys: ::unity2::Array<T0>,
        values: ::unity2::Array<T1>,
        left: i32,
        length: i32,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = "IntroSort", args = 6)]
    pub fn intro_sort(
        keys: ::unity2::Array<T0>,
        values: ::unity2::Array<T1>,
        lo: i32,
        hi: i32,
        depth_limit: i32,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = "PickPivotAndPartition", args = 5)]
    pub fn pick_pivot_and_partition(
        keys: ::unity2::Array<T0>,
        values: ::unity2::Array<T1>,
        lo: i32,
        hi: i32,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> i32;

    #[method(name = "Heapsort", args = 5)]
    pub fn heapsort(
        keys: ::unity2::Array<T0>,
        values: ::unity2::Array<T1>,
        lo: i32,
        hi: i32,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = "DownHeap", args = 6)]
    pub fn down_heap(
        keys: ::unity2::Array<T0>,
        values: ::unity2::Array<T1>,
        i: i32,
        n: i32,
        lo: i32,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = "InsertionSort", args = 5)]
    pub fn insertion_sort(
        keys: ::unity2::Array<T0>,
        values: ::unity2::Array<T1>,
        lo: i32,
        hi: i32,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-generic-arraysorthelper_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ArraySortHelper_2<T0, T1> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArraySortHelper_2),
                ::core::stringify!(new),
            )
        });
        <Self as IArraySortHelper_2Methods<T0, T1>>::ctor(this);
        this
    }
}
