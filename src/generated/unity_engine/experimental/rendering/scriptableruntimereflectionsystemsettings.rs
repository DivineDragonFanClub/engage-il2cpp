
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/scriptableruntimereflectionsystemsettings/ScriptableRuntimeReflectionSystemSettings.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering",
    name = "ScriptableRuntimeReflectionSystemSettings"
)]
#[parent(crate::system::object::Object)]
pub struct ScriptableRuntimeReflectionSystemSettings {
# [static_field] # [rename (name = "s_Instance")] pub s_instance : crate :: unity_engine :: experimental :: rendering :: scriptableruntimereflectionsystemwrapper :: ScriptableRuntimeReflectionSystemWrapper ,
}

#[cfg(feature = "unity_engine-experimental-rendering-scriptableruntimereflectionsystemsettings")]
#[::unity2::methods]
impl ScriptableRuntimeReflectionSystemSettings {
    #[method(
        name = "set_Internal_ScriptableRuntimeReflectionSystemSettings_system",
        args = 1
    )]
    pub fn set_internal_scriptable_runtime_reflection_system_settings_system(
        value : crate :: unity_engine :: experimental :: rendering :: iscriptableruntimereflectionsystem :: IScriptableRuntimeReflectionSystem,
    ) -> ();

    #[method(
        name = "get_Internal_ScriptableRuntimeReflectionSystemSettings_instance",
        args = 0
    )]
    pub fn get_internal_scriptable_runtime_reflection_system_settings_instance () -> crate :: unity_engine :: experimental :: rendering :: scriptableruntimereflectionsystemwrapper :: ScriptableRuntimeReflectionSystemWrapper ;

    #[method(name = "ScriptingDirtyReflectionSystemInstance", args = 0)]
    pub fn scripting_dirty_reflection_system_instance() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
