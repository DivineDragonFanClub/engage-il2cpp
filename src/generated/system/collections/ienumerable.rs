
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/ienumerable/IEnumerable.md")))]
#[::unity2::class(namespace = "System.Collections", name = "IEnumerable")]
pub struct IEnumerable {}

#[cfg(feature = "system-collections-ienumerable")]
#[::unity2::methods]
impl IEnumerable {
    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;
}
