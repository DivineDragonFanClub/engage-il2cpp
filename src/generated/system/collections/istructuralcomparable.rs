
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/istructuralcomparable/IStructuralComparable.md")))]
#[::unity2::class(namespace = "System.Collections", name = "IStructuralComparable")]
pub struct IStructuralComparable {}

#[cfg(feature = "system-collections-istructuralcomparable")]
#[::unity2::methods]
impl IStructuralComparable {
    #[method(name = "CompareTo", args = 2)]
    pub fn compare_to(
        self,
        other: crate::system::object::Object,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> i32;
}
