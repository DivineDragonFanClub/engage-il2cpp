
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubcontrolmouthsing/HubControlMouthSing.md")))]
#[::unity2::class(namespace = "App", name = "HubControlMouthSing")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubControlMouthSing {
    #[rename(name = "m_animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_layerIndex")]
    pub m_layer_index: i32,
}

#[cfg(feature = "app-hubcontrolmouthsing")]
#[::unity2::methods]
impl HubControlMouthSing {
    #[method(name = "get_UnitController", args = 0)]
    pub fn get_unit_controller(self) -> crate::app::hubunitcontroller::HubUnitController;

    #[method(name = "set_UnitController", args = 1)]
    pub fn set_unit_controller(self, value: crate::app::hubunitcontroller::HubUnitController)
        -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubcontrolmouthsing")]
impl HubControlMouthSing {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubControlMouthSing),
                ::core::stringify!(new),
            )
        });
        <Self as IHubControlMouthSingMethods>::ctor(this);
        this
    }
}
