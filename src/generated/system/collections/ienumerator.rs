
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/ienumerator/IEnumerator.md")))]
#[::unity2::class(namespace = "System.Collections", name = "IEnumerator")]
pub struct IEnumerator {}

#[cfg(feature = "system-collections-ienumerator")]
#[::unity2::methods]
impl IEnumerator {
    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}
