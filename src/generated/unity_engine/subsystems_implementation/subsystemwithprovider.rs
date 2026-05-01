
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/subsystems_implementation/subsystemwithprovider/SubsystemWithProvider.md")))]
#[::unity2::class(
    namespace = "UnityEngine.SubsystemsImplementation",
    name = "SubsystemWithProvider"
)]
#[parent(crate::system::object::Object)]
pub struct SubsystemWithProvider {}

#[cfg(feature = "unity_engine-subsystems_implementation-subsystemwithprovider")]
#[::unity2::methods]
impl SubsystemWithProvider {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-subsystems_implementation-subsystemwithprovider")]
impl SubsystemWithProvider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SubsystemWithProvider),
                ::core::stringify!(new),
            )
        });
        <Self as ISubsystemWithProviderMethods>::ctor(this);
        this
    }
}
