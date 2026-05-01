
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/resourcehandle_2/ResourceHandle_2.md")))]
#[::unity2::class(namespace = "App", name = "ResourceHandle")]
#[parent(crate::system::object::Object)]
pub struct ResourceHandle_2 {
# [rename (name = "m_Path")] pub m_path : :: unity2 :: Il2CppString ,
# [rename (name = "m_Handle")] pub m_handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ,
}

#[cfg(feature = "app-resourcehandle_2")]
#[::unity2::methods]
impl ResourceHandle_2 {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Reset", args = 1)]
    pub fn reset(self, path: ::unity2::Il2CppString) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "LoadSceneAsync", args = 2)]
    pub fn load_scene_async(
        self,
        path: ::unity2::Il2CppString,
        mode: crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
    ) -> ();

    #[method(name = "UnloadSceneAsync", args = 0)]
    pub fn unload_scene_async(self) -> ();

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "get_Path", args = 0)]
    pub fn get_path(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-resourcehandle_2")]
impl ResourceHandle_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourceHandle_2),
                ::core::stringify!(new),
            )
        });
        <Self as IResourceHandle_2Methods>::ctor(this);
        this
    }
}
