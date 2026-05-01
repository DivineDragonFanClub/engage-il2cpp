
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/subsystemmanager/SubsystemManager.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "SubsystemManager")]
#[parent(crate::system::object::Object)]
pub struct SubsystemManager {
# [static_field] # [rename (name = "beforeReloadSubsystems")] pub before_reload_subsystems : crate :: system :: action :: Action ,
# [static_field] # [rename (name = "afterReloadSubsystems")] pub after_reload_subsystems : crate :: system :: action :: Action ,
# [static_field] # [rename (name = "s_IntegratedSubsystems")] pub s_integrated_subsystems : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: integratedsubsystem :: IntegratedSubsystem > ,
# [static_field] # [rename (name = "s_StandaloneSubsystems")] pub s_standalone_subsystems : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: subsystems_implementation :: subsystemwithprovider :: SubsystemWithProvider > ,
# [static_field] # [rename (name = "s_DeprecatedSubsystems")] pub s_deprecated_subsystems : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: subsystem :: Subsystem > ,
# [static_field] # [rename (name = "reloadSubsytemsStarted")] pub reload_subsytems_started : crate :: system :: action :: Action ,
# [static_field] # [rename (name = "reloadSubsytemsCompleted")] pub reload_subsytems_completed : crate :: system :: action :: Action ,
}

#[cfg(feature = "unity_engine-subsystemmanager")]
#[::unity2::methods]
impl SubsystemManager {
    #[method(name = "ReloadSubsystemsStarted", args = 0)]
    pub fn reload_subsystems_started() -> ();

    #[method(name = "ReloadSubsystemsCompleted", args = 0)]
    pub fn reload_subsystems_completed() -> ();

    #[method(name = "InitializeIntegratedSubsystem", args = 2)]
    pub fn initialize_integrated_subsystem(
        ptr: ::unity2::IntPtr,
        subsystem: crate::unity_engine::integratedsubsystem::IntegratedSubsystem,
    ) -> ();

    #[method(name = "ClearSubsystems", args = 0)]
    pub fn clear_subsystems() -> ();

    #[method(name = "StaticConstructScriptingClassMap", args = 0)]
    pub fn static_construct_scripting_class_map() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "GetIntegratedSubsystemByPtr", args = 1)]
    pub fn get_integrated_subsystem_by_ptr(
        ptr: ::unity2::IntPtr,
    ) -> crate::unity_engine::integratedsubsystem::IntegratedSubsystem;
}
