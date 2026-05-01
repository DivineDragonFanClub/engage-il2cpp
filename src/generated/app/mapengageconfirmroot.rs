
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapengageconfirmroot/MapEngageConfirmRoot.md")))]
#[::unity2::class(namespace = "App", name = "MapEngageConfirmRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MapEngageConfirmRoot {
    #[rename(name = "m_unitName")]
    pub m_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_godName")]
    pub m_god_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_unitNameR")]
    pub m_unit_name_r: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_item")]
    pub m_item: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_skill")]
    pub m_skill: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "app-mapengageconfirmroot")]
#[::unity2::methods]
impl MapEngageConfirmRoot {
    #[method(name = "Setup", args = 2)]
    pub fn setup(self, unit: crate::app::unit::Unit, god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "Setup", args = 2)]
    pub fn setup_2(self, unit: crate::app::unit::Unit, target: crate::app::unit::Unit) -> ();

    #[method(name = "SetUnitName", args = 2)]
    pub fn set_unit_name(
        self,
        obj: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetGodName", args = 2)]
    pub fn set_god_name(
        self,
        obj: crate::unity_engine::gameobject::GameObject,
        god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "SetGodData", args = 3)]
    pub fn set_god_data(
        self,
        god: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
        link_target: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "SetSkill", args = 3)]
    pub fn set_skill(
        self,
        unit: crate::app::unit::Unit,
        obj: crate::unity_engine::gameobject::GameObject,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapengageconfirmroot")]
impl MapEngageConfirmRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapEngageConfirmRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IMapEngageConfirmRootMethods>::ctor(this);
        this
    }
}
