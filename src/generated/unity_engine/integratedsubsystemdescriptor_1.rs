
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::integratedsubsystemdescriptor::IIntegratedSubsystemDescriptor;
use crate::unity_engine::integratedsubsystemdescriptor::IntegratedSubsystemDescriptor;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/integratedsubsystemdescriptor_1/IntegratedSubsystemDescriptor_1.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "IntegratedSubsystemDescriptor`1")]
pub struct IntegratedSubsystemDescriptor_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "unity_engine-integratedsubsystemdescriptor_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> IntegratedSubsystemDescriptor_1<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-integratedsubsystemdescriptor_1")]
impl<T0: ::unity2::ClassIdentity> IntegratedSubsystemDescriptor_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(IntegratedSubsystemDescriptor_1),
                ::core::stringify!(new),
            )
        });
        <Self as IIntegratedSubsystemDescriptor_1Methods<T0>>::ctor(this);
        this
    }
}
