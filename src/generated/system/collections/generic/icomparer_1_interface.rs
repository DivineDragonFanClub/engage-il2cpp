
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/icomparer_1_interface/IComparer_1_Interface.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "IComparer`1")]
pub struct IComparer_1_Interface<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-icomparer_1_interface")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> IComparer_1_Interface<T0> {
    #[method(name = "Compare", args = 2)]
    pub fn compare(self, x: T0, y: T0) -> i32;
}
