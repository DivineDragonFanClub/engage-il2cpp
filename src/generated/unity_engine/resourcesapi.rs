
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resourcesapi/ResourcesAPI.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ResourcesAPI")]
#[parent(crate::system::object::Object)]
pub struct ResourcesAPI {
    #[static_field]
    #[rename(name = "s_DefaultAPI")]
    pub s_default_api: crate::unity_engine::resourcesapi::ResourcesAPI,
}

#[cfg(feature = "unity_engine-resourcesapi")]
#[::unity2::methods]
impl ResourcesAPI {
    #[method(name = "get_ActiveAPI", args = 0)]
    pub fn get_active_api() -> crate::unity_engine::resourcesapi::ResourcesAPI;

    #[method(name = "get_overrideAPI", args = 0)]
    pub fn get_override_api() -> crate::unity_engine::resourcesapi::ResourcesAPI;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "FindObjectsOfTypeAll", args = 1)]
    pub fn find_objects_of_type_all(
        self,
        system_type_instance: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::object_2::Object_2>;

    #[method(name = "FindShaderByName", args = 1)]
    pub fn find_shader_by_name(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::shader::Shader;

    #[method(name = "Load", args = 2)]
    pub fn load(
        self,
        path: ::unity2::Il2CppString,
        system_type_instance: ::unity2::SystemType,
    ) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "LoadAll", args = 2)]
    pub fn load_all(
        self,
        path: ::unity2::Il2CppString,
        system_type_instance: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::object_2::Object_2>;

    #[method(name = "LoadAsync", args = 2)]
    pub fn load_async(
        self,
        path: ::unity2::Il2CppString,
        system_type_instance: ::unity2::SystemType,
    ) -> crate::unity_engine::resourcerequest::ResourceRequest;

    #[method(name = "UnloadAsset", args = 1)]
    pub fn unload_asset(self, asset_to_unload: crate::unity_engine::object_2::Object_2) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-resourcesapi")]
impl ResourcesAPI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourcesAPI),
                ::core::stringify!(new),
            )
        });
        <Self as IResourcesAPIMethods>::ctor(this);
        this
    }
}
