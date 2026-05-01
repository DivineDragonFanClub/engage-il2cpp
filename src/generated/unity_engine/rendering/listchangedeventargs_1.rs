
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/listchangedeventargs_1/ListChangedEventArgs_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "ListChangedEventArgs`1")]
pub struct ListChangedEventArgs_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "index")]
    pub index: i32,
    #[rename(name = "item")]
    pub item: T0,
}

#[cfg(feature = "unity_engine-rendering-listchangedeventargs_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ListChangedEventArgs_1<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, index: i32, item: T0) -> ();
}

#[cfg(feature = "unity_engine-rendering-listchangedeventargs_1")]
impl<T0: ::unity2::ClassIdentity> ListChangedEventArgs_1<T0> {
    pub fn new(index: i32, item: T0) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ListChangedEventArgs_1),
                ::core::stringify!(new),
            )
        });
        <Self as IListChangedEventArgs_1Methods<T0>>::ctor(this, index, item);
        this
    }
}
