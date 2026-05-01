
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/subsystems_implementation/subsystemdescriptorstore/SubsystemDescriptorStore.md")))]
#[::unity2::class(
    namespace = "UnityEngine.SubsystemsImplementation",
    name = "SubsystemDescriptorStore"
)]
#[parent(crate::system::object::Object)]
pub struct SubsystemDescriptorStore {
# [static_field] # [rename (name = "s_IntegratedDescriptors")] pub s_integrated_descriptors : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: integratedsubsystemdescriptor :: IntegratedSubsystemDescriptor > ,
# [static_field] # [rename (name = "s_StandaloneDescriptors")] pub s_standalone_descriptors : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: subsystems_implementation :: subsystemdescriptorwithprovider :: SubsystemDescriptorWithProvider > ,
# [static_field] # [rename (name = "s_DeprecatedDescriptors")] pub s_deprecated_descriptors : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: subsystemdescriptor :: SubsystemDescriptor > ,
}

#[cfg(feature = "unity_engine-subsystems_implementation-subsystemdescriptorstore")]
#[::unity2::methods]
impl SubsystemDescriptorStore {
    #[method(name = "InitializeManagedDescriptor", args = 2)]
    pub fn initialize_managed_descriptor(
        ptr: ::unity2::IntPtr,
        desc: crate::unity_engine::integratedsubsystemdescriptor::IntegratedSubsystemDescriptor,
    ) -> ();

    #[method(name = "ClearManagedDescriptors", args = 0)]
    pub fn clear_managed_descriptors() -> ();

    #[method(name = "ReportSingleSubsystemAnalytics", args = 1)]
    pub fn report_single_subsystem_analytics(id: ::unity2::Il2CppString) -> ();

    #[method(name = "RegisterDeprecatedDescriptor", args = 1)]
    pub fn register_deprecated_descriptor(
        descriptor: crate::unity_engine::subsystemdescriptor::SubsystemDescriptor,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
