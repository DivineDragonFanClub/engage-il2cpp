
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procscenesequence_1/ProcSceneSequence_1.md")))]
#[::unity2::class(namespace = "App", name = "ProcSceneSequence`1")]
pub struct ProcSceneSequence_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-procscenesequence_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ProcSceneSequence_1<T0> {
    #[method(name = "get_SceneName", args = 0)]
    pub fn get_scene_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SceneName", args = 1)]
    pub fn set_scene_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SceneMode", args = 0)]
    pub fn get_scene_mode(
        self,
    ) -> crate::unity_engine::scene_management::loadscenemode::LoadSceneMode;

    #[method(name = "set_SceneMode", args = 1)]
    pub fn set_scene_mode(
        self,
        value: crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
    ) -> ();

    #[method(name = "LoadScene", args = 0)]
    pub fn load_scene(self) -> ();

    #[method(name = "LoadScene", args = 2)]
    pub fn load_scene_2(
        self,
        name: ::unity2::Il2CppString,
        mode: crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
    ) -> ();

    #[method(name = "UnloadScene", args = 0)]
    pub fn unload_scene(self) -> ();

    #[method(name = "UnloadScene", args = 1)]
    pub fn unload_scene_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "BindLoading", args = 0)]
    pub fn bind_loading(self) -> ();

    #[method(name = "UnbindLoading", args = 0)]
    pub fn unbind_loading(self) -> ();

    #[method(name = "get_LoadingMode", args = 0)]
    pub fn get_loading_mode(self) -> crate::app::loadingmanager::LoadingManager_Modes;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-procscenesequence_1")]
impl<T0: ::unity2::ClassIdentity> ProcSceneSequence_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcSceneSequence_1),
                ::core::stringify!(new),
            )
        });
        <Self as IProcSceneSequence_1Methods<T0>>::ctor(this);
        this
    }
}
