
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resourcesapiinternal/ResourcesAPIInternal.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ResourcesAPIInternal")]
#[parent(crate::system::object::Object)]
pub struct ResourcesAPIInternal {}

#[cfg(feature = "unity_engine-resourcesapiinternal")]
#[::unity2::methods]
impl ResourcesAPIInternal {
    #[method(name = "FindObjectsOfTypeAll", args = 1)]
    pub fn find_objects_of_type_all(
        r#type: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::object_2::Object_2>;

    #[method(name = "FindShaderByName", args = 1)]
    pub fn find_shader_by_name(name: ::unity2::Il2CppString)
        -> crate::unity_engine::shader::Shader;

    #[method(name = "Load", args = 2)]
    pub fn load(
        path: ::unity2::Il2CppString,
        system_type_instance: ::unity2::SystemType,
    ) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "LoadAll", args = 2)]
    pub fn load_all(
        path: ::unity2::Il2CppString,
        system_type_instance: ::unity2::SystemType,
    ) -> ::unity2::Array<crate::unity_engine::object_2::Object_2>;

    #[method(name = "LoadAsyncInternal", args = 2)]
    pub fn load_async_internal(
        path: ::unity2::Il2CppString,
        r#type: ::unity2::SystemType,
    ) -> crate::unity_engine::resourcerequest::ResourceRequest;

    #[method(name = "UnloadAsset", args = 1)]
    pub fn unload_asset(asset_to_unload: crate::unity_engine::object_2::Object_2) -> ();
}
