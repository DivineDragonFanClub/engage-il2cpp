
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/infoutil/InfoUtil_HpStockSpriteType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct InfoUtil_HpStockSpriteType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for InfoUtil_HpStockSpriteType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "InfoUtil.HpStockSpriteType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for InfoUtil_HpStockSpriteType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl InfoUtil_HpStockSpriteType {
    pub fn enemy() -> Self {
        Self { value: 0 }
    }

    pub fn rampage() -> Self {
        Self { value: 1 }
    }

    pub fn player() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/infoutil/InfoUtil_StatusSkill.md")))]
#[::unity2::class(namespace = "App", name = "InfoUtil.StatusSkill")]
#[parent(crate::system::object::Object)]
pub struct InfoUtil_StatusSkill {}

#[cfg(feature = "app-infoutil")]
#[::unity2::methods]
impl InfoUtil_StatusSkill {
    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::app::skilldata::SkillData;

    #[method(name = "set_Data", args = 1)]
    pub fn set_data(self, value: crate::app::skilldata::SkillData) -> ();

    #[method(name = "get_IsActive", args = 0)]
    pub fn get_is_active(self) -> bool;

    #[method(name = "set_IsActive", args = 1)]
    pub fn set_is_active(self, value: bool) -> ();

    #[method(name = "get_Category", args = 0)]
    pub fn get_category(self) -> crate::app::skilldata::SkillData_Categorys;

    #[method(name = "set_Category", args = 1)]
    pub fn set_category(self, value: crate::app::skilldata::SkillData_Categorys) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-infoutil")]
impl InfoUtil_StatusSkill {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InfoUtil_StatusSkill),
                ::core::stringify!(new),
            )
        });
        <Self as IInfoUtil_StatusSkillMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/infoutil/InfoUtil.md")))]
#[::unity2::class(namespace = "App", name = "InfoUtil")]
#[parent(crate::system::object::Object)]
pub struct InfoUtil {
    #[static_field]
    #[rename(name = "SKILL_SLOT_MAX")]
    pub skill_slot_max: i32,
}

#[cfg(feature = "app-infoutil")]
#[::unity2::methods]
impl InfoUtil {
    #[method(name = "TrySetText", args = 2)]
    pub fn try_set_text(
        tmp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        str: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "TrySetText", args = 2)]
    pub fn try_set_text_2(tmp: crate::tm_pro::textmeshprougui::TextMeshProUGUI, value: i32) -> ();

    #[method(name = "TrySetText", args = 2)]
    pub fn try_set_text_3(
        game_object: crate::unity_engine::gameobject::GameObject,
        str: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "TrySetColor", args = 2)]
    pub fn try_set_color(
        tmp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "TrySetColor", args = 2)]
    pub fn try_set_color_2(
        game_object: crate::unity_engine::gameobject::GameObject,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "TrySetColor", args = 2)]
    pub fn try_set_color_3(
        image: crate::unity_engine::ui::image::Image,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "TrySetSprite", args = 2)]
    pub fn try_set_sprite(
        game_object: crate::unity_engine::gameobject::GameObject,
        spr: crate::unity_engine::sprite::Sprite,
    ) -> ();

    #[method(name = "TrySetSprite", args = 2)]
    pub fn try_set_sprite_2(
        image: crate::unity_engine::ui::image::Image,
        spr: crate::unity_engine::sprite::Sprite,
    ) -> ();

    #[method(name = "TrySetActive", args = 2)]
    pub fn try_set_active(obj: crate::unity_engine::gameobject::GameObject, is_active: bool) -> ();

    #[method(name = "TrySetActive", args = 2)]
    pub fn try_set_active_2(c: crate::unity_engine::component::Component, is_active: bool) -> ();

    #[method(name = "TrySetParentActive", args = 2)]
    pub fn try_set_parent_active(
        obj: crate::unity_engine::gameobject::GameObject,
        is_active: bool,
    ) -> ();

    #[method(name = "TrySetParentActive", args = 2)]
    pub fn try_set_parent_active_2(
        c: crate::unity_engine::component::Component,
        is_active: bool,
    ) -> ();

    #[method(name = "TrySetActive", args = 3)]
    pub fn try_set_active_3(
        c: crate::unity_engine::component::Component,
        name: ::unity2::Il2CppString,
        is_active: bool,
    ) -> ();

    #[method(name = "TrySetParentActive", args = 3)]
    pub fn try_set_parent_active_3(
        c: crate::unity_engine::component::Component,
        name: ::unity2::Il2CppString,
        is_active: bool,
    ) -> ();

    #[method(name = "TrySetGrandParentActive", args = 2)]
    pub fn try_set_grand_parent_active(
        obj: crate::unity_engine::gameobject::GameObject,
        is_active: bool,
    ) -> ();

    #[method(name = "TrySetGrandParentActive", args = 2)]
    pub fn try_set_grand_parent_active_2(
        c: crate::unity_engine::component::Component,
        is_active: bool,
    ) -> ();

    #[method(name = "TryResetTextAnime", args = 1)]
    pub fn try_reset_text_anime(tmp: crate::tm_pro::textmeshprougui::TextMeshProUGUI) -> ();

    #[method(name = "TrySetParamTitle", args = 2)]
    pub fn try_set_param_title(
        tmp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        title: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "TrySetParamTitle", args = 3)]
    pub fn try_set_param_title_2(
        tmp: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        title: ::unity2::Il2CppString,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "TryUpdateChildGauge", args = 4)]
    pub fn try_update_child_gauge(
        gauge_obj: crate::unity_engine::gameobject::GameObject,
        value: i32,
        max: i32,
        is_hide_when0: bool,
    ) -> ();

    #[method(name = "TryUpdateGauge", args = 3)]
    pub fn try_update_gauge(
        gauge_obj: crate::unity_engine::gameobject::GameObject,
        value: i32,
        max: i32,
    ) -> ();

    #[method(name = "TrySetMaterial", args = 2)]
    pub fn try_set_material(
        img: crate::unity_engine::ui::image::Image,
        mat: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "AddSkill", args = 8)]
    pub fn add_skill(
        list: ::unity2::Array<crate::app::infoutil::InfoUtil_StatusSkill>,
        skill_array: crate::app::skillarray::SkillArray,
        slot_index: i32,
        slot_num: i32,
        is_pack_slot: bool,
        force_disp: bool,
        unit: crate::app::unit::Unit,
        is_view_restriction: bool,
    ) -> i32;

    #[method(name = "GetSkillListForUnitInfo", args = 4)]
    pub fn get_skill_list_for_unit_info(
        unit: crate::app::unit::Unit,
        is_skill_equip: bool,
        is_pack: bool,
        size: i32,
    ) -> ::unity2::Array<crate::app::infoutil::InfoUtil_StatusSkill>;

    #[method(name = "AddSkillArray", args = 5)]
    pub fn add_skill_array(
        unit: crate::app::unit::Unit,
        skill_array: crate::app::skillarray::SkillArray,
        list: ::unity2::Array<crate::app::infoutil::InfoUtil_StatusSkill>,
        index: i32,
        slot_num: i32,
    ) -> ();

    #[method(name = "IsEngageItemAppear", args = 2)]
    pub fn is_engage_item_appear(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
    ) -> bool;

    #[method(name = "GetShowItemIconNum", args = 1)]
    pub fn get_show_item_icon_num(unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetUnitItem", args = 2)]
    pub fn get_unit_item(
        unit: crate::app::unit::Unit,
        frame_index: i32,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "GetEngageItemData", args = 3)]
    pub fn get_engage_item_data(
        unit: crate::app::unit::Unit,
        temp_god: crate::app::godunit::GodUnit,
        frame_index: i32,
    ) -> crate::app::itemdata::ItemData;

    #[method(name = "TrySetItemIconList", args = 2)]
    pub fn try_set_item_icon_list(
        icon_list: ::unity2::Array<crate::unity_engine::gameobject::GameObject>,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "TrySetItemIcon", args = 3)]
    pub fn try_set_item_icon(
        item_root: crate::unity_engine::gameobject::GameObject,
        ui: crate::app::unititem::UnitItem,
        is_equip: bool,
    ) -> ();

    #[method(name = "TrySetColorToItemIcons", args = 3)]
    pub fn try_set_color_to_item_icons(
        item_root: crate::unity_engine::gameobject::GameObject,
        is_valid: bool,
        blend_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "TrySetColorToItemIcon", args = 2)]
    pub fn try_set_color_to_item_icon(
        item_icon_image: crate::unity_engine::ui::image::Image,
        is_valid: bool,
    ) -> ();

    #[method(name = "GetItemIconColor", args = 1)]
    pub fn get_item_icon_color(is_valid: bool) -> crate::unity_engine::color::Color;

    #[method(name = "IsBattleSequence", args = 0)]
    pub fn is_battle_sequence() -> bool;

    #[method(name = "TrySetEngageItemIcon", args = 2)]
    pub fn try_set_engage_item_icon(
        item_root: crate::unity_engine::gameobject::GameObject,
        item: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "TrySetItemIconBase", args = 6)]
    pub fn try_set_item_icon_base(
        item_root: crate::unity_engine::gameobject::GameObject,
        item: crate::app::itemdata::ItemData,
        is_valid: bool,
        is_equip: bool,
        is_drop: bool,
        is_enchant: bool,
    ) -> ();

    #[method(name = "AddDamageString", args = 3)]
    pub fn add_damage_string(
        base_str: ::unity2::Il2CppString,
        damage: i32,
        count: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetBattleAtkParam", args = 4)]
    pub fn get_battle_atk_param(
        side_type: crate::app::battleside::BattleSide_Type,
        scene_list: crate::app::battlescenelist::BattleSceneList,
        total_damage: i32,
        damage_list: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> i32;

    #[method(name = "GetBattleAtkCount", args = 2)]
    pub fn get_battle_atk_count(
        side_type: crate::app::battleside::BattleSide_Type,
        scene_list: crate::app::battlescenelist::BattleSceneList,
    ) -> i32;

    #[method(name = "SetUnitName", args = 2)]
    pub fn set_unit_name(
        unit: crate::app::unit::Unit,
        text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    ) -> ();

    #[method(name = "SetGodName", args = 4)]
    pub fn set_god_name(
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        root: crate::unity_engine::gameobject::GameObject,
        text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    ) -> ();

    #[method(name = "SetHpStock", args = 5)]
    pub fn set_hp_stock(
        unit: crate::app::unit::Unit,
        root: crate::unity_engine::gameobject::GameObject,
        stocks: ::unity2::Array<crate::unity_engine::ui::image::Image>,
        sprites: ::unity2::Array<crate::unity_engine::sprite::Sprite>,
        force: crate::app::force::Force_Type,
    ) -> ();

    #[method(name = "GetChainAtkParam", args = 5)]
    pub fn get_chain_atk_param(
        side_type: crate::app::battleside::BattleSide_Type,
        info: crate::app::battleinfo::BattleInfo,
        damage_str: ::unity2::Il2CppString,
        hit: i32,
        crit: i32,
    ) -> bool;

    #[method(name = "SetBattleAtkTitle", args = 3)]
    pub fn set_battle_atk_title(
        is_heal: bool,
        text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        help: crate::app::helpitemfixedtext::HelpItemFixedText,
    ) -> ();

    #[method(name = "SetupEngageWeaponText", args = 2)]
    pub fn setup_engage_weapon_text(
        contents_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        item: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "SetEnchantText", args = 2)]
    pub fn set_enchant_text(
        enchant_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-infoutil")]
impl InfoUtil {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InfoUtil),
                ::core::stringify!(new),
            )
        });
        <Self as IInfoUtilMethods>::ctor(this);
        this
    }
}
