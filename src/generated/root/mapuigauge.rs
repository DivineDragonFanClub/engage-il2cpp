
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/root/mapuigauge/MapUIGauge.md")))]
#[::unity2::class(namespace = "", name = "MapUIGauge")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: root :: mapuigauge :: MapUIGauge >)]
pub struct MapUIGauge {
    #[static_field]
    #[rename(name = "IconNames")]
    pub icon_names: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_AlphaCurve")]
    pub m_alpha_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_ShineCurve")]
    pub m_shine_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_CanBreakCurve")]
    pub m_can_break_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_CountMaxScaleCurve")]
    pub m_count_max_scale_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_CountMaxAlphaCurve")]
    pub m_count_max_alpha_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_SpriteAtlas")]
    pub m_sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_WaitFrameForAfterEvent")]
    pub m_wait_frame_for_after_event: i32,
    #[rename(name = "m_Sprites")]
    pub m_sprites: ::unity2::Array<crate::unity_engine::sprite::Sprite>,
    #[rename(name = "m_Dictionary")]
    pub m_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::unity_engine::sprite::Sprite,
    >,
    #[rename(name = "m_Time")]
    pub m_time: f32,
    #[rename(name = "m_WaitFrame")]
    pub m_wait_frame: f32,
    #[rename(name = "m_IsVisible")]
    pub m_is_visible: bool,
    #[rename(name = "m_IsExec")]
    pub m_is_exec: bool,
    #[rename(name = "m_IsCalc")]
    pub m_is_calc: bool,
    #[rename(name = "m_IsForceQuit")]
    pub m_is_force_quit: bool,
    #[rename(name = "m_NeedsForecastOneself")]
    pub m_needs_forecast_oneself: bool,
    #[rename(name = "m_BattleInfo")]
    pub m_battle_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_BattleCalc")]
    pub m_battle_calc: crate::app::battlecalculator::BattleCalculator,
    #[rename(name = "m_Performer")]
    pub m_performer: crate::app::unit::Unit,
    #[rename(name = "m_ReserveUnit")]
    pub m_reserve_unit: crate::app::unit::Unit,
    #[rename(name = "m_PerformerHpDiff")]
    pub m_performer_hp_diff: i32,
    #[rename(name = "m_IsDoomed")]
    pub m_is_doomed: bool,
    #[rename(name = "m_Mind")]
    pub m_mind: crate::app::mapmind::MapMind_Type,
    #[rename(name = "m_CommandSkill")]
    pub m_command_skill: crate::app::skilldata::SkillData,
    #[rename(name = "m_HpAfterBattle")]
    pub m_hp_after_battle: i32,
    #[rename(name = "m_TargetList")]
    pub m_target_list: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_InfluencerList")]
    pub m_influencer_list:
        crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_TempList")]
    pub m_temp_list: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_InitList")]
    pub m_init_list: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_TargetIndex")]
    pub m_target_index: i32,
    #[rename(name = "m_IsPriorWeaponRange")]
    pub m_is_prior_weapon_range: bool,
    #[rename(name = "m_MoveImage")]
    pub m_move_image: crate::app::mapimagecoresbyte::MapImageCoreSbyte,
    #[rename(name = "m_AttackImage")]
    pub m_attack_image: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_HealImage")]
    pub m_heal_image: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_RangeImage")]
    pub m_range_image: crate::app::mapimagecorebit::MapImageCoreBit,
    #[rename(name = "m_CannonImage")]
    pub m_cannon_image: crate::app::mapimagecorebit::MapImageCoreBit,
}

#[cfg(feature = "root-mapuigauge")]
#[::unity2::methods]
impl MapUIGauge {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "RegisterSprites", args = 1)]
    pub fn register_sprites(
        self,
        sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    ) -> ();

    #[method(name = "GetSprite", args = 1)]
    pub fn get_sprite(self, index: i32) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetSprite", args = 1)]
    pub fn get_sprite_2(self, name: ::unity2::Il2CppString) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "UpdateVisible", args = 0)]
    pub fn update_visible(self) -> ();

    #[method(name = "InitHpForecast", args = 1)]
    pub fn init_hp_forecast(self, is_all: bool) -> ();

    #[method(name = "SetHpForecast", args = 3)]
    pub fn set_hp_forecast(
        self,
        unit: crate::app::unit::Unit,
        needs_forecast_oneself: bool,
        is_prior_weapon_range: bool,
    ) -> ();

    #[method(name = "UpdateHpForecast", args = 0)]
    pub fn update_hp_forecast(self) -> ();

    #[method(name = "PrepareHpForecastTarget", args = 0)]
    pub fn prepare_hp_forecast_target(self) -> ();

    #[method(name = "CanUseItemForHealForecast", args = 1)]
    pub fn can_use_item_for_heal_forecast(self, item_data: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "GetImage", args = 1)]
    pub fn get_image(self, performer: crate::app::unit::Unit) -> ();

    #[method(name = "IsUnitTargetForDamageForecast", args = 1)]
    pub fn is_unit_target_for_damage_forecast(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "CanAttack", args = 2)]
    pub fn can_attack(
        self,
        attacker: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "IsUnitTargetForHealForecast", args = 1)]
    pub fn is_unit_target_for_heal_forecast(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "IsUnitInfluencerForDamageForecast", args = 1)]
    pub fn is_unit_influencer_for_damage_forecast(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsUnitInfluencerForHealForecast", args = 1)]
    pub fn is_unit_influencer_for_heal_forecast(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CalcHpForecast", args = 0)]
    pub fn calc_hp_forecast(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetBreathSkill", args = 2)]
    pub fn get_breath_skill(
        self,
        target: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "CalcBattleInfoForBreath", args = 3)]
    pub fn calc_battle_info_for_breath(
        self,
        target: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        breath_skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CalcBattleInfoForEngageWait", args = 1)]
    pub fn calc_battle_info_for_engage_wait(self, target: crate::app::unit::Unit) -> ();

    #[method(name = "CalcBattleInfoForHealItem", args = 2)]
    pub fn calc_battle_info_for_heal_item(
        self,
        target: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "CalcBattleInfoForNormal", args = 2)]
    pub fn calc_battle_info_for_normal(
        self,
        target: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "IsHealItemUsed", args = 2)]
    pub fn is_heal_item_used(
        self,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "CalcHpHealItem", args = 4)]
    pub fn calc_hp_heal_item(
        self,
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        before_hp: i32,
        after_hp: i32,
    ) -> ();

    #[method(name = "GetBattleInfoFlag", args = 2)]
    pub fn get_battle_info_flag(
        self,
        performer: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
    ) -> crate::app::battleinfo::BattleInfo_Flags;

    #[method(name = "StackHpDiff", args = 2)]
    pub fn stack_hp_diff(self, hp_before_battle: i32, hp_after_battle: i32) -> ();

    #[method(name = "GetItem", args = 1)]
    pub fn get_item(self, target: crate::app::unit::Unit) -> crate::app::unititem::UnitItem;

    #[method(name = "get_Sprites", args = 0)]
    pub fn get_sprites(self) -> ::unity2::Array<crate::unity_engine::sprite::Sprite>;

    #[method(name = "get_Time", args = 0)]
    pub fn get_time(self) -> f32;

    #[method(name = "get_WaitFrame", args = 0)]
    pub fn get_wait_frame(self) -> f32;

    #[method(name = "get_WaitFrameForAfterEvent", args = 0)]
    pub fn get_wait_frame_for_after_event(self) -> f32;

    #[method(name = "get_IsVisible", args = 0)]
    pub fn get_is_visible(self) -> bool;

    #[method(name = "get_AlphaCurve", args = 0)]
    pub fn get_alpha_curve(self) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "get_ShineCurve", args = 0)]
    pub fn get_shine_curve(self) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "get_CanBreakCurve", args = 0)]
    pub fn get_can_break_curve(self) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "get_CountMaxScaleCurve", args = 0)]
    pub fn get_count_max_scale_curve(self) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "get_CountMaxAlphaCurve", args = 0)]
    pub fn get_count_max_alpha_curve(self) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "root-mapuigauge")]
impl MapUIGauge {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUIGauge),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUIGaugeMethods>::ctor(this);
        this
    }
}
