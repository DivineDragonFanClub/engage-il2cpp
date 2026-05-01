
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::integratedsubsystemdescriptor::IIntegratedSubsystemDescriptor;
use crate::unity_engine::integratedsubsystemdescriptor::IntegratedSubsystemDescriptor;
use crate::unity_engine::integratedsubsystemdescriptor_1::IIntegratedSubsystemDescriptor_1;
use crate::unity_engine::integratedsubsystemdescriptor_1::IntegratedSubsystemDescriptor_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/xr/xrmeshsubsystemdescriptor/XRMeshSubsystemDescriptor.md")))]
#[::unity2::class(namespace = "UnityEngine.XR", name = "XRMeshSubsystemDescriptor")]
# [parent (crate :: unity_engine :: integratedsubsystemdescriptor_1 :: IntegratedSubsystemDescriptor_1 < crate :: unity_engine :: xr :: xrmeshsubsystem :: XRMeshSubsystem >)]
pub struct XRMeshSubsystemDescriptor {}

#[cfg(feature = "unity_engine-xr-xrmeshsubsystemdescriptor")]
#[::unity2::methods]
impl XRMeshSubsystemDescriptor {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-xr-xrmeshsubsystemdescriptor")]
impl XRMeshSubsystemDescriptor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(XRMeshSubsystemDescriptor),
                ::core::stringify!(new),
            )
        });
        <Self as IXRMeshSubsystemDescriptorMethods>::ctor(this);
        this
    }
}
