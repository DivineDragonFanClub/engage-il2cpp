
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/investmentmappoint/InvestmentMapPoint.md")))]
#[::unity2::class(namespace = "App", name = "InvestmentMapPoint")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct InvestmentMapPoint {
    #[rename(name = "m_mapCid")]
    pub m_map_cid: ::unity2::Il2CppString,
    #[rename(name = "m_balloonRoot")]
    pub m_balloon_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_battleIcon")]
    pub m_battle_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_animalIcon")]
    pub m_animal_icon: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-investmentmappoint")]
#[::unity2::methods]
impl InvestmentMapPoint {
    #[method(name = "GetRareAnimal", args = 1)]
    pub fn get_rare_animal(
        self,
        nation: crate::app::hubnationdata::HubNationData,
    ) -> crate::app::animaldata::AnimalData;

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-investmentmappoint")]
impl InvestmentMapPoint {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvestmentMapPoint),
                ::core::stringify!(new),
            )
        });
        <Self as IInvestmentMapPointMethods>::ctor(this);
        this
    }
}
