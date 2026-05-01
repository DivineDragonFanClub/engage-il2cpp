
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographallmenucontent/PhotographAllMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "PhotographAllMenuContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct PhotographAllMenuContent {
    #[static_field]
    #[rename(name = "s_MenuPrefabPath")]
    pub s_menu_prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_CameraKeyHelp")]
    pub m_camera_key_help: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-photographallmenucontent")]
#[::unity2::methods]
impl PhotographAllMenuContent {
    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::photographallmenucontent::PhotographAllMenuContent;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(
        menu_content: crate::app::photographallmenucontent::PhotographAllMenuContent,
    ) -> ();

    #[method(name = "SetMyCardIsVisible", args = 1)]
    pub fn set_my_card_is_visible(self, is_visible: bool) -> ();

    #[method(name = "SetCameraKeyHelpIsVisible", args = 1)]
    pub fn set_camera_key_help_is_visible(self, is_visible: bool) -> ();

    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource() -> ();

    #[method(name = "IsLoadingResource", args = 0)]
    pub fn is_loading_resource() -> bool;

    #[method(name = "UnloadResource", args = 0)]
    pub fn unload_resource() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-photographallmenucontent")]
impl PhotographAllMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographAllMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographAllMenuContentMethods>::ctor(this);
        this
    }
}
