
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/caseinsensitivehashcodeprovider/CaseInsensitiveHashCodeProvider.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "CaseInsensitiveHashCodeProvider"
)]
#[parent(crate::system::object::Object)]
pub struct CaseInsensitiveHashCodeProvider {}

#[cfg(feature = "system-collections-caseinsensitivehashcodeprovider")]
#[::unity2::methods]
impl CaseInsensitiveHashCodeProvider {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Default", args = 0)]
    pub fn get_default(
    ) -> crate::system::collections::caseinsensitivehashcodeprovider::CaseInsensitiveHashCodeProvider;

    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(self, obj: crate::system::object::Object) -> i32;
}

#[cfg(feature = "system-collections-caseinsensitivehashcodeprovider")]
impl CaseInsensitiveHashCodeProvider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CaseInsensitiveHashCodeProvider),
                ::core::stringify!(new),
            )
        });
        <Self as ICaseInsensitiveHashCodeProviderMethods>::ctor(this);
        this
    }
}
