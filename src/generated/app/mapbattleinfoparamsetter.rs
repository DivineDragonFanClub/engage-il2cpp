
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapbattleinfoparamsetter/MapBattleInfoParamSetter.md")))]
#[::unity2::class(namespace = "App", name = "MapBattleInfoParamSetter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MapBattleInfoParamSetter {
    #[rename(name = "ParamClamp")]
    pub param_clamp: i32,
    #[rename(name = "m_InfoRoot")]
    pub m_info_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CharaNameRoot")]
    pub m_chara_name_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CharaName")]
    pub m_chara_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GodNameRoot")]
    pub m_god_name_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_GodName")]
    pub m_god_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_NameOnlyCharaNameRoot")]
    pub m_name_only_chara_name_root: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_NameOnlyCharaName")]
    pub m_name_only_chara_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_NameOnlyGodNameRoot")]
    pub m_name_only_god_name_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NameOnlyGodName")]
    pub m_name_only_god_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_MaxHpGaugeWidth")]
    pub m_max_hp_gauge_width: i32,
    #[rename(name = "m_MinHpGaugeWidth")]
    pub m_min_hp_gauge_width: i32,
    #[rename(name = "m_HpGaugeMax")]
    pub m_hp_gauge_max: i32,
    #[rename(name = "m_HpRoot")]
    pub m_hp_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_HpGaugeRoot")]
    pub m_hp_gauge_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NowHp")]
    pub m_now_hp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AfterHpRoot")]
    pub m_after_hp_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AfterHp")]
    pub m_after_hp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AfterHpHealRoot")]
    pub m_after_hp_heal_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AfterHpHeal")]
    pub m_after_hp_heal: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_HpGaugeAfter")]
    pub m_hp_gauge_after: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_HpGaugeAdd")]
    pub m_hp_gauge_add: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DamageMaterial")]
    pub m_damage_material: crate::unity_engine::material::Material,
    #[rename(name = "m_HealMaterial")]
    pub m_heal_material: crate::unity_engine::material::Material,
    #[rename(name = "m_EngageMaterial")]
    pub m_engage_material: crate::unity_engine::material::Material,
    #[rename(name = "m_HpStockRoot")]
    pub m_hp_stock_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_HpStock")]
    pub m_hp_stock: ::unity2::Array<crate::unity_engine::ui::image::Image>,
    #[rename(name = "m_HpStockSprites")]
    pub m_hp_stock_sprites: ::unity2::Array<crate::unity_engine::sprite::Sprite>,
    #[rename(name = "m_HpStockAdd")]
    pub m_hp_stock_add: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DamageSpace")]
    pub m_damage_space: f32,
    #[rename(name = "m_StatusRoot")]
    pub m_status_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BtlAtkHelp")]
    pub m_btl_atk_help: crate::app::helpitemfixedtext::HelpItemFixedText,
    #[rename(name = "m_BtlAtkTitle")]
    pub m_btl_atk_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BtlAtk")]
    pub m_btl_atk: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BtlHit")]
    pub m_btl_hit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BtlCrit")]
    pub m_btl_crit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ChainBtlAtk")]
    pub m_chain_btl_atk: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ChainBtlHitRoot")]
    pub m_chain_btl_hit_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ChainBtlHit")]
    pub m_chain_btl_hit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ChainBtlCritRoot")]
    pub m_chain_btl_crit_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ChainBtlCrit")]
    pub m_chain_btl_crit: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_WeaponRoot")]
    pub m_weapon_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WeaponIconRoot")]
    pub m_weapon_icon_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WeaponIcon")]
    pub m_weapon_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_WeaponArrow")]
    pub m_weapon_arrow: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_WeaponName")]
    pub m_weapon_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_WeaponNothing")]
    pub m_weapon_nothing: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WeaponEndurance")]
    pub m_weapon_endurance: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_WeaponChangeL")]
    pub m_weapon_change_l: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_WeaponChangeR")]
    pub m_weapon_change_r: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ItemListRoot")]
    pub m_item_list_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SpaceRoot")]
    pub m_space_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_forceTexture")]
    pub m_force_texture: crate::system::collections::generic::list_1::List_1<
        crate::app::forcetexturesetter::ForceTextureSetter,
    >,
}

#[cfg(feature = "app-mapbattleinfoparamsetter")]
#[::unity2::methods]
impl MapBattleInfoParamSetter {
    #[method(name = "get_SideType", args = 0)]
    pub fn get_side_type(self) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "set_SideType", args = 1)]
    pub fn set_side_type(self, value: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "get_BattleInfo", args = 0)]
    pub fn get_battle_info(self) -> crate::app::battleinfo::BattleInfo;

    #[method(name = "set_BattleInfo", args = 1)]
    pub fn set_battle_info(self, value: crate::app::battleinfo::BattleInfo) -> ();

    #[method(name = "get_BattleSceneList", args = 0)]
    pub fn get_battle_scene_list(self) -> crate::app::battlescenelist::BattleSceneList;

    #[method(name = "set_BattleSceneList", args = 1)]
    pub fn set_battle_scene_list(self, value: crate::app::battlescenelist::BattleSceneList) -> ();

    #[method(name = "get_Side", args = 0)]
    pub fn get_side(self) -> crate::app::battleinfoside::BattleInfoSide;

    #[method(name = "set_Side", args = 1)]
    pub fn set_side(self, value: crate::app::battleinfoside::BattleInfoSide) -> ();

    #[method(name = "get_ReverseSide", args = 0)]
    pub fn get_reverse_side(self) -> crate::app::battleinfoside::BattleInfoSide;

    #[method(name = "set_ReverseSide", args = 1)]
    pub fn set_reverse_side(self, value: crate::app::battleinfoside::BattleInfoSide) -> ();

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "get_UnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "get_MaxHp", args = 0)]
    pub fn get_max_hp(self) -> i32;

    #[method(name = "SetWeaponChangeVisible", args = 1)]
    pub fn set_weapon_change_visible(self, is_visible: bool) -> ();

    #[method(name = "SetBattleInfo", args = 4)]
    pub fn set_battle_info_2(
        self,
        side_type: crate::app::battleside::BattleSide_Type,
        b_show_wdw: bool,
        info: crate::app::battleinfo::BattleInfo,
        scene_list: crate::app::battlescenelist::BattleSceneList,
    ) -> ();

    #[method(name = "SetBattleInfoForBattle", args = 0)]
    pub fn set_battle_info_for_battle(self) -> ();

    #[method(name = "SetBattleInfoForTrade", args = 0)]
    pub fn set_battle_info_for_trade(self) -> ();

    #[method(name = "SetBattleInfoForDestroy", args = 0)]
    pub fn set_battle_info_for_destroy(self) -> ();

    #[method(name = "SetBattleInfoForNoParam", args = 2)]
    pub fn set_battle_info_for_no_param(self, is_weapon: bool, is_god_name: bool) -> ();

    #[method(name = "GetHpGaugeMaxWidth", args = 1)]
    pub fn get_hp_gauge_max_width(self, max_hp: i32) -> f32;

    #[method(name = "TryUpdateDamageGauge", args = 3)]
    pub fn try_update_damage_gauge(
        self,
        gauge_obj: crate::unity_engine::gameobject::GameObject,
        value: i32,
        max: i32,
    ) -> ();

    #[method(name = "TrySetChildImageMaterial", args = 2)]
    pub fn try_set_child_image_material(
        self,
        game_object: crate::unity_engine::gameobject::GameObject,
        mat: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "SetUnitName", args = 1)]
    pub fn set_unit_name(self, unit_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetUnitName", args = 3)]
    pub fn set_unit_name_2(self, unit: crate::app::unit::Unit, b_weapon: bool, b_god: bool) -> ();

    #[method(name = "ShowDestroyName", args = 0)]
    pub fn show_destroy_name(self) -> ();

    #[method(name = "ShowWeapon", args = 0)]
    pub fn show_weapon(self) -> ();

    #[method(name = "HideWeapon", args = 0)]
    pub fn hide_weapon(self) -> ();

    #[method(name = "ShowHp", args = 0)]
    pub fn show_hp(self) -> ();

    #[method(name = "ShowDestroyHp", args = 0)]
    pub fn show_destroy_hp(self) -> ();

    #[method(name = "HideHp", args = 0)]
    pub fn hide_hp(self) -> ();

    #[method(name = "SetAfterHpGauge", args = 1)]
    pub fn set_after_hp_gauge(self, hp: i32) -> ();

    #[method(name = "SetGaugeSide", args = 1)]
    pub fn set_gauge_side(self, gauge: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetInitParams", args = 1)]
    pub fn set_init_params(self, b_show: bool) -> ();

    #[method(name = "SetAtkParam", args = 3)]
    pub fn set_atk_param(
        self,
        side_type: crate::app::battleside::BattleSide_Type,
        info: crate::app::battleinfo::BattleInfo,
        scene_list: crate::app::battlescenelist::BattleSceneList,
    ) -> ();

    #[method(name = "SetChainAtkParam", args = 0)]
    pub fn set_chain_atk_param(self) -> ();

    #[method(name = "IsShowWeapon", args = 0)]
    pub fn is_show_weapon(self) -> bool;

    #[method(name = "IsShowHP", args = 0)]
    pub fn is_show_hp(self) -> bool;

    #[method(name = "IsFriendCommandSkill", args = 0)]
    pub fn is_friend_command_skill(self) -> bool;

    #[method(name = "GetForceType", args = 0)]
    pub fn get_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "ShowDisorderIcon", args = 1)]
    pub fn show_disorder_icon(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ShowAddHpStock", args = 0)]
    pub fn show_add_hp_stock(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapbattleinfoparamsetter")]
impl MapBattleInfoParamSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapBattleInfoParamSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IMapBattleInfoParamSetterMethods>::ctor(this);
        this
    }
}
