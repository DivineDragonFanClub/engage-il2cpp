
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/ireadonlydictionary_2/IReadOnlyDictionary_2.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "IReadOnlyDictionary`2"
)]
pub struct IReadOnlyDictionary_2<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-ireadonlydictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> IReadOnlyDictionary_2<T0, T1> {
    #[method(name = "TryGetValue", args = 2)]
    pub fn try_get_value(self, key: T0, value: T1) -> bool;
}
