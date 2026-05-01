
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/treewalkpredicate_1/TreeWalkPredicate_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "TreeWalkPredicate`1")]
pub struct TreeWalkPredicate_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-treewalkpredicate_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> TreeWalkPredicate_1<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        node: crate::system::collections::generic::sortedset_1::SortedSet_1_Node<T0>,
    ) -> bool;
}

#[cfg(feature = "system-collections-generic-treewalkpredicate_1")]
impl<T0: ::unity2::ClassIdentity> TreeWalkPredicate_1<T0> {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TreeWalkPredicate_1),
                ::core::stringify!(new),
            )
        });
        <Self as ITreeWalkPredicate_1Methods<T0>>::ctor(this, object, method);
        this
    }
}
