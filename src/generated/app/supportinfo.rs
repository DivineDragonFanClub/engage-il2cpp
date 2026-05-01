
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/supportinfo/SupportInfo.md")))]
#[::unity2::class(namespace = "App", name = "SupportInfo")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct SupportInfo {
    #[rename(name = "m_SideType")]
    pub m_side_type: crate::app::battleside::BattleSide_Type,
    #[rename(name = "m_Hit")]
    pub m_hit: i32,
    #[rename(name = "m_Avoid")]
    pub m_avoid: i32,
    #[rename(name = "m_Critical")]
    pub m_critical: i32,
    #[rename(name = "m_Secure")]
    pub m_secure: i32,
    #[rename(name = "m_SupportUnits")]
    pub m_support_units: ::unity2::Array<crate::app::unit::Unit>,
    #[rename(name = "m_SkillArray")]
    pub m_skill_array: crate::app::skillarray::SkillArray,
}

#[cfg(feature = "app-supportinfo")]
#[::unity2::methods]
impl SupportInfo {
    #[method(name = "SetBattleInfo", args = 2)]
    pub fn set_battle_info(
        self,
        info: crate::app::battleinfo::BattleInfo,
        side_type: crate::app::battleside::BattleSide_Type,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-supportinfo")]
impl SupportInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SupportInfo),
                ::core::stringify!(new),
            )
        });
        <Self as ISupportInfoMethods>::ctor(this);
        this
    }
}
