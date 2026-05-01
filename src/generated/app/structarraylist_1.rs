
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structarraylist_1/StructArrayList_1.md")))]
#[::unity2::class(namespace = "App", name = "StructArrayList`1")]
pub struct StructArrayList_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-structarraylist_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> StructArrayList_1<T0> {
    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "Completed", args = 0)]
    pub fn completed(self) -> ();

    #[method(name = "CompletedEnd", args = 0)]
    pub fn completed_end(self) -> bool;

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-structarraylist_1")]
impl<T0: ::unity2::ClassIdentity> StructArrayList_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructArrayList_1),
                ::core::stringify!(new),
            )
        });
        <Self as IStructArrayList_1Methods<T0>>::ctor(this);
        this
    }
}
