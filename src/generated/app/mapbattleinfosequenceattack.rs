
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbattleinfosequenceattack/MapBattleInfoSequenceAttack.md")))]
#[::unity2::class(namespace = "App", name = "MapBattleInfoSequenceAttack")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MapBattleInfoSequenceAttack {
    #[rename(name = "m_DefenseDamage")]
    pub m_defense_damage: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_DefenseArrow")]
    pub m_defense_arrow: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_DefenseDead")]
    pub m_defense_dead: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_OffenseDamage")]
    pub m_offense_damage: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_OffenseArrow")]
    pub m_offense_arrow: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_OffenseDead")]
    pub m_offense_dead: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_DefenseResult")]
    pub m_defense_result: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_OffenseResult")]
    pub m_offense_result: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_DamageMaterial")]
    pub m_damage_material: crate::unity_engine::material::Material,
    #[rename(name = "m_HealMaterial")]
    pub m_heal_material: crate::unity_engine::material::Material,
    #[rename(name = "m_ArrowSprite")]
    pub m_arrow_sprite: ::unity2::Array<crate::unity_engine::sprite::Sprite>,
    #[rename(name = "m_InstDamageMaterial")]
    pub m_inst_damage_material: crate::unity_engine::material::Material,
    #[rename(name = "m_InstHealMaterial")]
    pub m_inst_heal_material: crate::unity_engine::material::Material,
}

#[cfg(feature = "app-mapbattleinfosequenceattack")]
#[::unity2::methods]
impl MapBattleInfoSequenceAttack {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Setup", args = 2)]
    pub fn setup(
        self,
        attack: crate::app::mapbattleinfosequence::MapBattleInfoSequence_AttackInfo,
        is_hide: bool,
    ) -> ();

    #[method(name = "SetArrow", args = 2)]
    pub fn set_arrow(self, is_offense: bool, force_type: crate::app::force::Force_Type) -> ();

    #[method(name = "SetReceiveDamage", args = 5)]
    pub fn set_receive_damage(
        self,
        side_type: crate::app::battleside::BattleSide_Type,
        atk_string: ::unity2::Il2CppString,
        is_heal: bool,
        receive_first_damage: i32,
        is_hide: bool,
    ) -> ();

    #[method(name = "SetSendDamage", args = 3)]
    pub fn set_send_damage(
        self,
        side_type: crate::app::battleside::BattleSide_Type,
        atk: i32,
        send_first_damage: i32,
    ) -> ();

    #[method(name = "SetSceneResult", args = 2)]
    pub fn set_scene_result(
        self,
        offense: crate::app::mapbattleinfosequence::MapBattleInfoSequence_AttackInfo_SceneResult,
        defense: crate::app::mapbattleinfosequence::MapBattleInfoSequence_AttackInfo_SceneResult,
    ) -> ();

    #[method(name = "SetDeath", args = 1)]
    pub fn set_death(self, side: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapbattleinfosequenceattack")]
impl MapBattleInfoSequenceAttack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBattleInfoSequenceAttack),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBattleInfoSequenceAttackMethods>::ctor(this);
        this
    }
}
