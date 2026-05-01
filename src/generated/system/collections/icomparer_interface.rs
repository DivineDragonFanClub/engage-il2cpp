
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/icomparer_interface/IComparer_Interface.md")))]
#[::unity2::class(namespace = "System.Collections", name = "IComparer")]
pub struct IComparer_Interface {}

#[cfg(feature = "system-collections-icomparer_interface")]
#[::unity2::methods]
impl IComparer_Interface {
    #[method(name = "Compare", args = 2)]
    pub fn compare(self, x: crate::system::object::Object, y: crate::system::object::Object)
        -> i32;
}
