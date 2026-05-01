
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusviolationtypemenucontent/VersusViolationTypeMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "VersusViolationTypeMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct VersusViolationTypeMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
}

#[cfg(feature = "app-versusviolationtypemenucontent")]
#[::unity2::methods]
impl VersusViolationTypeMenuContent {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 1)]
    pub fn create(
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> crate::app::versusviolationtypemenucontent::VersusViolationTypeMenuContent;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(
        content: crate::app::versusviolationtypemenucontent::VersusViolationTypeMenuContent,
    ) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-versusviolationtypemenucontent")]
impl VersusViolationTypeMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusViolationTypeMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusViolationTypeMenuContentMethods>::ctor(this);
        this
    }
}
