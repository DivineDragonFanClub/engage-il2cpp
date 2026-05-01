
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineringtimesselectroot/RefineRingTimesSelectRoot.md")))]
#[::unity2::class(namespace = "App", name = "RefineRingTimesSelectRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RefineRingTimesSelectRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_RefineRingTimesSelectMenuContentObject")]
    pub m_refine_ring_times_select_menu_content_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-refineringtimesselectroot")]
#[::unity2::methods]
impl RefineRingTimesSelectRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateRoot", args = 0)]
    pub fn create_root() -> crate::app::refineringtimesselectroot::RefineRingTimesSelectRoot;

    #[method(name = "Create", args = 0)]
    pub fn create(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetMenuContent", args = 0)]
    pub fn get_menu_content(
        self,
    ) -> crate::app::refineringtimesselectmenucontent::RefineRingTimesSelectMenuContent;
}

#[cfg(feature = "app-refineringtimesselectroot")]
impl RefineRingTimesSelectRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineRingTimesSelectRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineRingTimesSelectRootMethods>::ctor(this);
        this
    }
}
