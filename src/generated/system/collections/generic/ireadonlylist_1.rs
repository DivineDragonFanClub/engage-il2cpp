
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/ireadonlylist_1/IReadOnlyList_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "IReadOnlyList`1")]
pub struct IReadOnlyList_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-ireadonlylist_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> IReadOnlyList_1<T0> {
    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> T0;
}
