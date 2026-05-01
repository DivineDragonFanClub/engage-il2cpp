
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridelocatorsetting/DragonRideLocatorSetting.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideLocatorSetting")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DragonRideLocatorSetting {
    #[rename(name = "m_PatternType")]
    pub m_pattern_type: ::unity2::Il2CppString,
    #[rename(name = "m_BillboardTypes")]
    pub m_billboard_types: crate::app::dragon_ride::billboardtypes::BillboardTypes,
}

#[cfg(feature = "app-dragonridelocatorsetting")]
#[::unity2::methods]
impl DragonRideLocatorSetting {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dragonridelocatorsetting")]
impl DragonRideLocatorSetting {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideLocatorSetting),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideLocatorSettingMethods>::ctor(this);
        this
    }
}
