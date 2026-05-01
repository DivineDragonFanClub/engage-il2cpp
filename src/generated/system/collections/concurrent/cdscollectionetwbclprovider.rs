
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/concurrent/cdscollectionetwbclprovider/CDSCollectionETWBCLProvider.md")))]
#[::unity2::class(
    namespace = "System.Collections.Concurrent",
    name = "CDSCollectionETWBCLProvider"
)]
pub struct CDSCollectionETWBCLProvider {
# [static_field] # [rename (name = "Log")] pub log : crate :: system :: collections :: concurrent :: cdscollectionetwbclprovider :: CDSCollectionETWBCLProvider ,
}

#[cfg(feature = "system-collections-concurrent-cdscollectionetwbclprovider")]
#[::unity2::methods]
impl CDSCollectionETWBCLProvider {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ConcurrentDictionary_AcquiringAllLocks", args = 1)]
    pub fn concurrent_dictionary_acquiring_all_locks(self, num_of_buckets: i32) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-collections-concurrent-cdscollectionetwbclprovider")]
impl CDSCollectionETWBCLProvider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CDSCollectionETWBCLProvider),
                ::core::stringify!(new),
            )
        });
        <Self as ICDSCollectionETWBCLProviderMethods>::ctor(this);
        this
    }
}
