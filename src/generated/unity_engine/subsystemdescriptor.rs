
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/subsystemdescriptor/SubsystemDescriptor.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "SubsystemDescriptor")]
#[parent(crate::system::object::Object)]
pub struct SubsystemDescriptor {}

#[cfg(feature = "unity_engine-subsystemdescriptor")]
#[::unity2::methods]
impl SubsystemDescriptor {
    #[method(name = "get_id", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-subsystemdescriptor")]
impl SubsystemDescriptor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SubsystemDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as ISubsystemDescriptorMethods>::ctor(this);
        this
    }
}
