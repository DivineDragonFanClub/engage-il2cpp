
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/scene_management/scenemanagerapi/SceneManagerAPI.md")))]
#[::unity2::class(namespace = "UnityEngine.SceneManagement", name = "SceneManagerAPI")]
#[parent(crate::system::object::Object)]
pub struct SceneManagerAPI {
    #[static_field]
    #[rename(name = "s_DefaultAPI")]
    pub s_default_api: crate::unity_engine::scene_management::scenemanagerapi::SceneManagerAPI,
}

#[cfg(feature = "unity_engine-scene_management-scenemanagerapi")]
#[::unity2::methods]
impl SceneManagerAPI {
    #[method(name = "get_ActiveAPI", args = 0)]
    pub fn get_active_api(
    ) -> crate::unity_engine::scene_management::scenemanagerapi::SceneManagerAPI;

    #[method(name = "get_overrideAPI", args = 0)]
    pub fn get_override_api(
    ) -> crate::unity_engine::scene_management::scenemanagerapi::SceneManagerAPI;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "LoadSceneAsyncByNameOrIndex", args = 4)]
    pub fn load_scene_async_by_name_or_index(
        self,
        scene_name: ::unity2::Il2CppString,
        scene_build_index: i32,
        parameters: crate::unity_engine::scene_management::loadsceneparameters::LoadSceneParameters,
        must_complete_next_frame: bool,
    ) -> crate::unity_engine::asyncoperation::AsyncOperation;

    #[method(name = "LoadFirstScene", args = 1)]
    pub fn load_first_scene(
        self,
        must_load_async: bool,
    ) -> crate::unity_engine::asyncoperation::AsyncOperation;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-scene_management-scenemanagerapi")]
impl SceneManagerAPI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SceneManagerAPI),
                ::core::stringify!(new),
            )
        });
        <Self as ISceneManagerAPIMethods>::ctor(this);
        this
    }
}
