
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionaryhashhelpers/DictionaryHashHelpers.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "DictionaryHashHelpers"
)]
#[parent(crate::system::object::Object)]
pub struct DictionaryHashHelpers {}

#[cfg(feature = "system-collections-generic-dictionaryhashhelpers")]
#[::unity2::methods]
impl DictionaryHashHelpers {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
