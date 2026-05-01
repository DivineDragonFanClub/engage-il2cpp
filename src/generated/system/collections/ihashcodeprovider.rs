
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/ihashcodeprovider/IHashCodeProvider.md")))]
#[::unity2::class(namespace = "System.Collections", name = "IHashCodeProvider")]
pub struct IHashCodeProvider {}

#[cfg(feature = "system-collections-ihashcodeprovider")]
#[::unity2::methods]
impl IHashCodeProvider {
    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(self, obj: crate::system::object::Object) -> i32;
}
