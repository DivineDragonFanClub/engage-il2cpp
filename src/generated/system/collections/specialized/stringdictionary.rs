
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/stringdictionary/StringDictionary.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "StringDictionary"
)]
#[parent(crate::system::object::Object)]
pub struct StringDictionary {
    #[rename(name = "contents")]
    pub contents: crate::system::collections::hashtable::Hashtable,
}

#[cfg(feature = "system-collections-specialized-stringdictionary")]
#[::unity2::methods]
impl StringDictionary {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(self, key: ::unity2::Il2CppString, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;
}

#[cfg(feature = "system-collections-specialized-stringdictionary")]
impl StringDictionary {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StringDictionary),
                ::core::stringify!(new),
            )
        });
        <Self as IStringDictionaryMethods>::ctor(this);
        this
    }
}
