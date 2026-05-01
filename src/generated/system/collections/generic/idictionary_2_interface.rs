
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/idictionary_2_interface/IDictionary_2_Interface.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "IDictionary`2")]
pub struct IDictionary_2_Interface<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-idictionary_2_interface")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> IDictionary_2_Interface<T0, T1> {
    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::generic::icollection_1::ICollection_1<T0>;

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(
        self,
    ) -> crate::system::collections::generic::icollection_1::ICollection_1<T1>;

    #[method(name = "Add", args = 2)]
    pub fn add(self, key: T0, value: T1) -> ();
}
