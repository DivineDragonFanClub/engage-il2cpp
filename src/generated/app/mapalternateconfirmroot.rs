
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapalternateconfirmroot/MapAlternateConfirmRoot.md")))]
#[::unity2::class(namespace = "App", name = "MapAlternateConfirmRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MapAlternateConfirmRoot {
    #[rename(name = "m_beforeGodName")]
    pub m_before_god_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_afterGodName")]
    pub m_after_god_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_skillBefore")]
    pub m_skill_before: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_skillAfter")]
    pub m_skill_after: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "app-mapalternateconfirmroot")]
#[::unity2::methods]
impl MapAlternateConfirmRoot {
    #[method(name = "Setup", args = 1)]
    pub fn setup(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetBeforeGodData", args = 2)]
    pub fn set_before_god_data(
        self,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "SetAfterGodData", args = 2)]
    pub fn set_after_god_data(
        self,
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
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

#[cfg(feature = "app-mapalternateconfirmroot")]
impl MapAlternateConfirmRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAlternateConfirmRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAlternateConfirmRootMethods>::ctor(this);
        this
    }
}
