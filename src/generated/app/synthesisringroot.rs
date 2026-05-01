
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/synthesisringroot/SynthesisRingRoot.md")))]
#[::unity2::class(namespace = "App", name = "SynthesisRingRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct SynthesisRingRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_SynthesisRingBaseRingMenuObject")]
    pub m_synthesis_ring_base_ring_menu_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SynthesisRingInfoWindowObject")]
    pub m_synthesis_ring_info_window_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SynthesisRingInfoWindow")]
    pub m_synthesis_ring_info_window: crate::app::synthesisringinfowindow::SynthesisRingInfoWindow,
}

#[cfg(feature = "app-synthesisringroot")]
#[::unity2::methods]
impl SynthesisRingRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateRoot", args = 0)]
    pub fn create_root() -> crate::app::synthesisringroot::SynthesisRingRoot;

    #[method(name = "Create", args = 0)]
    pub fn create(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "SetData", args = 1)]
    pub fn set_data(self, ring_data: crate::app::ringdata::RingData) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-synthesisringroot")]
impl SynthesisRingRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SynthesisRingRoot),
                ::core::stringify!(new),
            )
        });
        <Self as ISynthesisRingRootMethods>::ctor(this);
        this
    }
}
