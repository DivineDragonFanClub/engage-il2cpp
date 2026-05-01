
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fieldloader/FieldLoader.md")))]
#[::unity2::class(namespace = "Combat", name = "FieldLoader")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FieldLoader {
    #[rename(name = "m_MapHandle")]
    pub m_map_handle: crate::app::resourcehandle_2::ResourceHandle_2,
}

#[cfg(feature = "combat-fieldloader")]
#[::unity2::methods]
impl FieldLoader {
    #[method(name = "get_IsLoading", args = 0)]
    pub fn get_is_loading(self) -> bool;

    #[method(name = "set_IsLoading", args = 1)]
    pub fn set_is_loading(self, value: bool) -> ();

    #[method(name = "GetSceneNames", args = 0)]
    pub fn get_scene_names(
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "LoadScene", args = 2)]
    pub fn load_scene(
        self,
        map_name: ::unity2::Il2CppString,
        callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "UnloadScene", args = 0)]
    pub fn unload_scene(self) -> ();

    #[method(name = "FieldNameToPath", args = 1)]
    pub fn field_name_to_path(field_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "LoadMapImpl", args = 0)]
    pub fn load_map_impl() -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-fieldloader")]
impl FieldLoader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FieldLoader),
                ::core::stringify!(new),
            )
        });
        <Self as IFieldLoaderMethods>::ctor(this);
        this
    }
}
