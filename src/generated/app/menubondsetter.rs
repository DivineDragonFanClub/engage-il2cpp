
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/menubondsetter/MenuBondSetter.md")))]
#[::unity2::class(namespace = "App", name = "MenuBondSetter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MenuBondSetter {
    #[rename(name = "m_UnitName")]
    pub m_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GodName")]
    pub m_god_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BondLv")]
    pub m_bond_lv: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BondGauge")]
    pub m_bond_gauge: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-menubondsetter")]
#[::unity2::methods]
impl MenuBondSetter {
    #[method(name = "SetData", args = 2)]
    pub fn set_data(self, unit: crate::app::unit::Unit, god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "SetData", args = 3)]
    pub fn set_data_2(
        self,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "SetStatus", args = 3)]
    pub fn set_status(
        self,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "GetGodName", args = 2)]
    pub fn get_god_name(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-menubondsetter")]
impl MenuBondSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MenuBondSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IMenuBondSetterMethods>::ctor(this);
        this
    }
}
