
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/ienumerable_1/IEnumerable_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "IEnumerable`1")]
pub struct IEnumerable_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-ienumerable_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> IEnumerable_1<T0> {
    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::ienumerator_1::IEnumerator_1<T0>;
}
