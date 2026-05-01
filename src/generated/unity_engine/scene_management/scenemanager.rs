
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/scene_management/scenemanager/SceneManager.md")))]
#[::unity2::class(namespace = "UnityEngine.SceneManagement", name = "SceneManager")]
#[parent(crate::system::object::Object)]
pub struct SceneManager {
    #[static_field]
    #[rename(name = "s_AllowLoadScene")]
    pub s_allow_load_scene: bool,
    #[static_field]
    #[rename(name = "sceneLoaded")]
    pub scene_loaded: crate::unity_engine::events::unityaction_2::UnityAction_2<
        crate::unity_engine::scene_management::scene::Scene,
        crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
    >,
    #[static_field]
    #[rename(name = "sceneUnloaded")]
    pub scene_unloaded: crate::unity_engine::events::unityaction_1::UnityAction_1<
        crate::unity_engine::scene_management::scene::Scene,
    >,
    #[static_field]
    #[rename(name = "activeSceneChanged")]
    pub active_scene_changed: crate::unity_engine::events::unityaction_2::UnityAction_2<
        crate::unity_engine::scene_management::scene::Scene,
        crate::unity_engine::scene_management::scene::Scene,
    >,
}

#[cfg(feature = "unity_engine-scene_management-scenemanager")]
#[::unity2::methods]
impl SceneManager {
    #[method(name = "get_sceneCount", args = 0)]
    pub fn get_scene_count() -> i32;

    #[method(name = "GetActiveScene", args = 0)]
    pub fn get_active_scene() -> crate::unity_engine::scene_management::scene::Scene;

    #[method(name = "SetActiveScene", args = 1)]
    pub fn set_active_scene(scene: crate::unity_engine::scene_management::scene::Scene) -> bool;

    #[method(name = "GetSceneByName", args = 1)]
    pub fn get_scene_by_name(
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::scene_management::scene::Scene;

    #[method(name = "GetSceneAt", args = 1)]
    pub fn get_scene_at(index: i32) -> crate::unity_engine::scene_management::scene::Scene;

    #[method(name = "UnloadSceneAsyncInternal", args = 2)]
    pub fn unload_scene_async_internal(
        scene: crate::unity_engine::scene_management::scene::Scene,
        options: crate::unity_engine::scene_management::unloadsceneoptions::UnloadSceneOptions,
    ) -> crate::unity_engine::asyncoperation::AsyncOperation;

    #[method(name = "LoadSceneAsyncNameIndexInternal", args = 4)]
    pub fn load_scene_async_name_index_internal(
        scene_name: ::unity2::Il2CppString,
        scene_build_index: i32,
        parameters: crate::unity_engine::scene_management::loadsceneparameters::LoadSceneParameters,
        must_complete_next_frame: bool,
    ) -> crate::unity_engine::asyncoperation::AsyncOperation;

    #[method(name = "MoveGameObjectToScene", args = 2)]
    pub fn move_game_object_to_scene(
        go: crate::unity_engine::gameobject::GameObject,
        scene: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();

    #[method(name = "LoadFirstScene_Internal", args = 1)]
    pub fn load_first_scene_internal(
        r#async: bool,
    ) -> crate::unity_engine::asyncoperation::AsyncOperation;

    #[method(name = "add_sceneLoaded", args = 1)]
    pub fn add_scene_loaded(
        value: crate::unity_engine::events::unityaction_2::UnityAction_2<
            crate::unity_engine::scene_management::scene::Scene,
            crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
        >,
    ) -> ();

    #[method(name = "remove_sceneLoaded", args = 1)]
    pub fn remove_scene_loaded(
        value: crate::unity_engine::events::unityaction_2::UnityAction_2<
            crate::unity_engine::scene_management::scene::Scene,
            crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
        >,
    ) -> ();

    #[method(name = "add_sceneUnloaded", args = 1)]
    pub fn add_scene_unloaded(
        value: crate::unity_engine::events::unityaction_1::UnityAction_1<
            crate::unity_engine::scene_management::scene::Scene,
        >,
    ) -> ();

    #[method(name = "remove_sceneUnloaded", args = 1)]
    pub fn remove_scene_unloaded(
        value: crate::unity_engine::events::unityaction_1::UnityAction_1<
            crate::unity_engine::scene_management::scene::Scene,
        >,
    ) -> ();

    #[method(name = "add_activeSceneChanged", args = 1)]
    pub fn add_active_scene_changed(
        value: crate::unity_engine::events::unityaction_2::UnityAction_2<
            crate::unity_engine::scene_management::scene::Scene,
            crate::unity_engine::scene_management::scene::Scene,
        >,
    ) -> ();

    #[method(name = "remove_activeSceneChanged", args = 1)]
    pub fn remove_active_scene_changed(
        value: crate::unity_engine::events::unityaction_2::UnityAction_2<
            crate::unity_engine::scene_management::scene::Scene,
            crate::unity_engine::scene_management::scene::Scene,
        >,
    ) -> ();

    #[method(name = "LoadSceneAsync", args = 2)]
    pub fn load_scene_async(
        scene_name: ::unity2::Il2CppString,
        parameters: crate::unity_engine::scene_management::loadsceneparameters::LoadSceneParameters,
    ) -> crate::unity_engine::asyncoperation::AsyncOperation;

    #[method(name = "UnloadSceneAsync", args = 1)]
    pub fn unload_scene_async(
        scene: crate::unity_engine::scene_management::scene::Scene,
    ) -> crate::unity_engine::asyncoperation::AsyncOperation;

    #[method(name = "Internal_SceneLoaded", args = 2)]
    pub fn internal_scene_loaded(
        scene: crate::unity_engine::scene_management::scene::Scene,
        mode: crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
    ) -> ();

    #[method(name = "Internal_SceneUnloaded", args = 1)]
    pub fn internal_scene_unloaded(
        scene: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();

    #[method(name = "Internal_ActiveSceneChanged", args = 2)]
    pub fn internal_active_scene_changed(
        previous_active_scene: crate::unity_engine::scene_management::scene::Scene,
        new_active_scene: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "GetActiveScene_Injected", args = 1)]
    pub fn get_active_scene_injected(
        ret: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();

    #[method(name = "SetActiveScene_Injected", args = 1)]
    pub fn set_active_scene_injected(
        scene: crate::unity_engine::scene_management::scene::Scene,
    ) -> bool;

    #[method(name = "GetSceneByName_Injected", args = 2)]
    pub fn get_scene_by_name_injected(
        name: ::unity2::Il2CppString,
        ret: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();

    #[method(name = "GetSceneAt_Injected", args = 2)]
    pub fn get_scene_at_injected(
        index: i32,
        ret: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();

    #[method(name = "UnloadSceneAsyncInternal_Injected", args = 2)]
    pub fn unload_scene_async_internal_injected(
        scene: crate::unity_engine::scene_management::scene::Scene,
        options: crate::unity_engine::scene_management::unloadsceneoptions::UnloadSceneOptions,
    ) -> crate::unity_engine::asyncoperation::AsyncOperation;

    #[method(name = "MoveGameObjectToScene_Injected", args = 2)]
    pub fn move_game_object_to_scene_injected(
        go: crate::unity_engine::gameobject::GameObject,
        scene: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();
}
