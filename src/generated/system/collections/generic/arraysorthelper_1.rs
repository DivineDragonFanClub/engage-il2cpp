
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/arraysorthelper_1/ArraySortHelper_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "ArraySortHelper`1")]
pub struct ArraySortHelper_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-arraysorthelper_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ArraySortHelper_1<T0> {
    #[method(name = "Sort", args = 4)]
    pub fn sort(
        keys: ::unity2::Array<T0>,
        index: i32,
        length: i32,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = "BinarySearch", args = 5)]
    pub fn binary_search(
        array: ::unity2::Array<T0>,
        index: i32,
        length: i32,
        value: T0,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> i32;

    #[method(name = "Sort", args = 4)]
    pub fn sort_2(
        keys: ::unity2::Array<T0>,
        index: i32,
        length: i32,
        comparer: crate::system::comparison_1::Comparison_1<T0>,
    ) -> ();

    #[method(name = "InternalBinarySearch", args = 5)]
    pub fn internal_binary_search(
        array: ::unity2::Array<T0>,
        index: i32,
        length: i32,
        value: T0,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> i32;

    #[method(name = "SwapIfGreater", args = 4)]
    pub fn swap_if_greater(
        keys: ::unity2::Array<T0>,
        comparer: crate::system::comparison_1::Comparison_1<T0>,
        a: i32,
        b: i32,
    ) -> ();

    #[method(name = "Swap", args = 3)]
    pub fn swap(a: ::unity2::Array<T0>, i: i32, j: i32) -> ();

    #[method(name = "IntrospectiveSort", args = 4)]
    pub fn introspective_sort(
        keys: ::unity2::Array<T0>,
        left: i32,
        length: i32,
        comparer: crate::system::comparison_1::Comparison_1<T0>,
    ) -> ();

    #[method(name = "IntroSort", args = 5)]
    pub fn intro_sort(
        keys: ::unity2::Array<T0>,
        lo: i32,
        hi: i32,
        depth_limit: i32,
        comparer: crate::system::comparison_1::Comparison_1<T0>,
    ) -> ();

    #[method(name = "PickPivotAndPartition", args = 4)]
    pub fn pick_pivot_and_partition(
        keys: ::unity2::Array<T0>,
        lo: i32,
        hi: i32,
        comparer: crate::system::comparison_1::Comparison_1<T0>,
    ) -> i32;

    #[method(name = "Heapsort", args = 4)]
    pub fn heapsort(
        keys: ::unity2::Array<T0>,
        lo: i32,
        hi: i32,
        comparer: crate::system::comparison_1::Comparison_1<T0>,
    ) -> ();

    #[method(name = "DownHeap", args = 5)]
    pub fn down_heap(
        keys: ::unity2::Array<T0>,
        i: i32,
        n: i32,
        lo: i32,
        comparer: crate::system::comparison_1::Comparison_1<T0>,
    ) -> ();

    #[method(name = "InsertionSort", args = 4)]
    pub fn insertion_sort(
        keys: ::unity2::Array<T0>,
        lo: i32,
        hi: i32,
        comparer: crate::system::comparison_1::Comparison_1<T0>,
    ) -> ();
}
