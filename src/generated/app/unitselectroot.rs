
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectroot/UnitSelectRoot.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct UnitSelectRoot {
    #[rename(name = "m_UnitListRoot")]
    pub m_unit_list_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_GodImageObject")]
    pub m_god_image_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-unitselectroot")]
#[::unity2::methods]
impl UnitSelectRoot {
    #[method(name = "GetUnitListRoot", args = 0)]
    pub fn get_unit_list_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "UpdateUnitData", args = 0)]
    pub fn update_unit_data(self) -> ();

    #[method(name = "TimerEnd", args = 0)]
    pub fn timer_end(self) -> ();

    #[method(name = "UpdateUnitData", args = 1)]
    pub fn update_unit_data_2(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitselectroot")]
impl UnitSelectRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectRootMethods>::ctor(this);
        this
    }
}
