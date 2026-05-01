
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortconstant/SortConstant.md")))]
#[::unity2::class(namespace = "App", name = "SortConstant")]
#[parent(crate::system::object::Object)]
pub struct SortConstant {
    #[static_field]
    #[rename(name = "MergeSortThreshold")]
    pub merge_sort_threshold: i32,
}

#[cfg(feature = "app-sortconstant")]
#[::unity2::methods]
impl SortConstant {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortconstant")]
impl SortConstant {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortConstant),
                ::core::stringify!(new),
            )
        });
        <Self as ISortConstantMethods>::ctor(this);
        this
    }
}
