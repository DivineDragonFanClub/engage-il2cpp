
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unititem/UnitItem.md")))]
#[::unity2::class(namespace = "App", name = "UnitItem")]
#[parent(crate::system::object::Object)]
pub struct UnitItem {
    #[static_field]
    #[rename(name = "NoItemIndex")]
    pub no_item_index: i32,
    #[static_field]
    #[rename(name = "EngageStockIndex")]
    pub engage_stock_index: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_Item")]
    pub m_item: crate::app::itemdata::ItemData,
    #[rename(name = "m_Endurance")]
    pub m_endurance: u8,
    #[rename(name = "m_RefineLevel")]
    pub m_refine_level: u8,
    #[rename(name = "m_Flags")]
    pub m_flags: crate::app::unititem::UnitItem_Flags,
    #[rename(name = "m_Engrave")]
    pub m_engrave: crate::app::goddata::GodData,
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
    #[static_field]
    #[rename(name = "s_EnchantHash")]
    pub s_enchant_hash: i32,
}

#[cfg(feature = "app-unititem")]
#[::unity2::methods]
impl UnitItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, iid: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_4(self, index: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_5(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(p: crate::app::unititem::UnitItem) -> crate::app::itemdata::ItemData;

    #[method(name = "New", args = 1)]
    pub fn new(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "New", args = 1)]
    pub fn new_2(self, iid: ::unity2::Il2CppString) -> ();

    #[method(name = "New", args = 1)]
    pub fn new_3(self, index: i32) -> ();

    #[method(name = "New", args = 1)]
    pub fn new_4(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Expend", args = 0)]
    pub fn expend(self) -> bool;

    #[method(name = "DoTrade", args = 0)]
    pub fn do_trade(self) -> ();

    #[method(name = "DoTransporter", args = 0)]
    pub fn do_transporter(self) -> ();

    #[method(name = "DoDrop", args = 0)]
    pub fn do_drop(self) -> ();

    #[method(name = "DoTransfer", args = 0)]
    pub fn do_transfer(self) -> ();

    #[method(name = "CanExpend", args = 0)]
    pub fn can_expend(self) -> bool;

    #[method(name = "GetExpend", args = 0)]
    pub fn get_expend(self) -> i32;

    #[method(name = "CanEnchant", args = 0)]
    pub fn can_enchant(self) -> bool;

    #[method(name = "CanEnchant", args = 1)]
    pub fn can_enchant_2(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsEmpty", args = 0)]
    pub fn is_empty(self) -> bool;

    #[method(name = "IsExist", args = 0)]
    pub fn is_exist(self) -> bool;

    #[method(name = "IsIntegrate", args = 1)]
    pub fn is_integrate(self, other_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "IsLongRange", args = 0)]
    pub fn is_long_range(self) -> bool;

    #[method(name = "GetData", args = 0)]
    pub fn get_data(self) -> crate::app::itemdata::ItemData;

    #[method(name = "GetPrice", args = 0)]
    pub fn get_price(self) -> i32;

    #[method(name = "GetSelling", args = 0)]
    pub fn get_selling(self) -> i32;

    #[method(name = "IsFlag", args = 1)]
    pub fn is_flag(self, flags: crate::app::itemdata::ItemData_Flags) -> bool;

    #[method(name = "NotFlag", args = 1)]
    pub fn not_flag(self, flags: crate::app::itemdata::ItemData_Flags) -> bool;

    #[method(name = "IsNone", args = 0)]
    pub fn is_none(self) -> bool;

    #[method(name = "IsInventory", args = 0)]
    pub fn is_inventory(self) -> bool;

    #[method(name = "get_Kind", args = 0)]
    pub fn get_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "get_UseType", args = 0)]
    pub fn get_use_type(self) -> crate::app::itemdata::ItemData_UseTypes;

    #[method(name = "get_RodType", args = 0)]
    pub fn get_rod_type(self) -> crate::app::itemdata::ItemData_RodTypes;

    #[method(name = "IsRangeTarget", args = 0)]
    pub fn is_range_target(self) -> bool;

    #[method(name = "IsRangeHeal", args = 0)]
    pub fn is_range_heal(self) -> bool;

    #[method(name = "IsRangeRestHeal", args = 0)]
    pub fn is_range_rest_heal(self) -> bool;

    #[method(name = "IsRangeAgain", args = 0)]
    pub fn is_range_again(self) -> bool;

    #[method(name = "IsRangeEngageAdd", args = 0)]
    pub fn is_range_engage_add(self) -> bool;

    #[method(name = "IsBless", args = 0)]
    pub fn is_bless(self) -> bool;

    #[method(name = "IsBullet", args = 0)]
    pub fn is_bullet(self) -> bool;

    #[method(name = "GetShootEffect", args = 0)]
    pub fn get_shoot_effect(self) -> crate::app::effectdata::EffectData;

    #[method(name = "GetHitEffect", args = 0)]
    pub fn get_hit_effect(self) -> crate::app::effectdata::EffectData;

    #[method(name = "GetCannonEffect", args = 0)]
    pub fn get_cannon_effect(self) -> crate::app::effectsequence::EffectSequence;

    #[method(name = "GetAttr", args = 0)]
    pub fn get_attr(self) -> crate::app::itemdata::ItemData_Attrs;

    #[method(name = "GetWeaponAttr", args = 0)]
    pub fn get_weapon_attr(self) -> crate::app::itemdata::ItemData_WeaponAttrs;

    #[method(name = "IsWeapon", args = 0)]
    pub fn is_weapon(self) -> bool;

    #[method(name = "IsPhysical", args = 0)]
    pub fn is_physical(self) -> bool;

    #[method(name = "IsMagic", args = 0)]
    pub fn is_magic(self) -> bool;

    #[method(name = "IsBreath", args = 0)]
    pub fn is_breath(self) -> bool;

    #[method(name = "IsSurehit", args = 0)]
    pub fn is_surehit(self) -> bool;

    #[method(name = "IsRod", args = 0)]
    pub fn is_rod(self) -> bool;

    #[method(name = "IsSingleRod", args = 0)]
    pub fn is_single_rod(self) -> bool;

    #[method(name = "IsEfficacy", args = 1)]
    pub fn is_efficacy(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "GetName", args = 1)]
    pub fn get_name(self, with_refine_level: bool) -> ::unity2::Il2CppString;

    #[method(name = "GetOriginalName", args = 0)]
    pub fn get_original_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPower", args = 0)]
    pub fn get_power(self) -> i32;

    #[method(name = "GetWeight", args = 0)]
    pub fn get_weight(self) -> i32;

    #[method(name = "GetHit", args = 0)]
    pub fn get_hit(self) -> i32;

    #[method(name = "GetCritical", args = 0)]
    pub fn get_critical(self) -> i32;

    #[method(name = "GetAvoid", args = 0)]
    pub fn get_avoid(self) -> i32;

    #[method(name = "GetSecure", args = 0)]
    pub fn get_secure(self) -> i32;

    #[method(name = "GetEnhance", args = 0)]
    pub fn get_enhance(self) -> crate::app::capabilitysbyte::CapabilitySbyte;

    #[method(name = "GetTimes", args = 0)]
    pub fn get_times(self) -> i32;

    #[method(name = "GetEquipSkills", args = 0)]
    pub fn get_equip_skills(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "GetSort", args = 0)]
    pub fn get_sort(self) -> i64;

    #[method(name = "IsRefined", args = 0)]
    pub fn is_refined(self) -> bool;

    #[method(name = "GetMaxRefineLevel", args = 0)]
    pub fn get_max_refine_level(self) -> i32;

    #[method(name = "IsExistRefineData", args = 0)]
    pub fn is_exist_refine_data(self) -> bool;

    #[method(name = "GetCurrentRefineData", args = 0)]
    pub fn get_current_refine_data(self) -> crate::app::itemrefinedata::ItemRefineData;

    #[method(name = "GetNextRefineData", args = 0)]
    pub fn get_next_refine_data(self) -> crate::app::itemrefinedata::ItemRefineData;

    #[method(name = "GetRefineDataList", args = 0)]
    pub fn get_refine_data_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::itemrefinedata::ItemRefineData,
    >;

    #[method(name = "CanEvolve", args = 0)]
    pub fn can_evolve(self) -> bool;

    #[method(name = "IsExistEvolveData", args = 0)]
    pub fn is_exist_evolve_data(self) -> bool;

    #[method(name = "GetEvolveDataList", args = 0)]
    pub fn get_evolve_data_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::itemevolvedata::ItemEvolveData,
    >;

    #[method(name = "CanEngrave", args = 0)]
    pub fn can_engrave(self) -> bool;

    #[method(name = "IsEngraved", args = 0)]
    pub fn is_engraved(self) -> bool;

    #[method(name = "SetEngrave", args = 1)]
    pub fn set_engrave(self, god_data: crate::app::goddata::GodData) -> ();

    #[method(name = "GetEngrave", args = 0)]
    pub fn get_engrave(self) -> crate::app::goddata::GodData;

    #[method(name = "GetMenuText", args = 4)]
    pub fn get_menu_text(
        self,
        icon_text: ::unity2::Il2CppString,
        name_text: ::unity2::Il2CppString,
        endurance_text: ::unity2::Il2CppString,
        with_max_endurance: bool,
    ) -> ();

    #[method(name = "GetEngageWeaponGod", args = 0)]
    pub fn get_engage_weapon_god(self) -> crate::app::godunit::GodUnit;

    #[method(name = "SetEngageWeaponGod", args = 1)]
    pub fn set_engage_weapon_god(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetEngageWeaponGod", args = 1)]
    pub fn set_engage_weapon_god_2(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "ClearEngageWeaponGod", args = 0)]
    pub fn clear_engage_weapon_god(self) -> ();

    #[method(name = "UpdateEngageWeaponParam", args = 0)]
    pub fn update_engage_weapon_param(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "IsFlags", args = 1)]
    pub fn is_flags(self, flags: crate::app::unititem::UnitItem_Flags) -> bool;

    #[method(name = "SetFlags", args = 1)]
    pub fn set_flags(self, flags: crate::app::unititem::UnitItem_Flags) -> ();

    #[method(name = "ClearFlags", args = 1)]
    pub fn clear_flags(self, flags: crate::app::unititem::UnitItem_Flags) -> ();

    #[method(name = "SetFlags", args = 2)]
    pub fn set_flags_2(self, flags: crate::app::unititem::UnitItem_Flags, enable: bool) -> ();

    #[method(name = "get_IsEquipped", args = 0)]
    pub fn get_is_equipped(self) -> bool;

    #[method(name = "set_IsEquipped", args = 1)]
    pub fn set_is_equipped(self, value: bool) -> ();

    #[method(name = "get_IsDrop", args = 0)]
    pub fn get_is_drop(self) -> bool;

    #[method(name = "set_IsDrop", args = 1)]
    pub fn set_is_drop(self, value: bool) -> ();

    #[method(name = "get_IsSkipLog", args = 0)]
    pub fn get_is_skip_log(self) -> bool;

    #[method(name = "set_IsSkipLog", args = 1)]
    pub fn set_is_skip_log(self, value: bool) -> ();

    #[method(name = "get_IsEnchant", args = 0)]
    pub fn get_is_enchant(self) -> bool;

    #[method(name = "ResetEnchantItem", args = 0)]
    pub fn reset_enchant_item() -> ();

    #[method(name = "SetEnchantItem", args = 1)]
    pub fn set_enchant_item(item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "SetEnchantHash", args = 1)]
    pub fn set_enchant_hash(hash: i32) -> ();

    #[method(name = "GetEnchantHash", args = 0)]
    pub fn get_enchant_hash() -> i32;

    #[method(name = "get_FlagsValue", args = 0)]
    pub fn get_flags_value(self) -> crate::app::unititem::UnitItem_Flags;

    #[method(name = "set_FlagsValue", args = 1)]
    pub fn set_flags_value(self, value: crate::app::unititem::UnitItem_Flags) -> ();

    #[method(name = "get_Endurance", args = 0)]
    pub fn get_endurance(self) -> i32;

    #[method(name = "set_Endurance", args = 1)]
    pub fn set_endurance(self, value: i32) -> ();

    #[method(name = "get_RefineLevel", args = 0)]
    pub fn get_refine_level(self) -> i32;

    #[method(name = "set_RefineLevel", args = 1)]
    pub fn set_refine_level(self, value: i32) -> ();

    #[method(name = "SortCompare", args = 3)]
    pub fn sort_compare(
        a: crate::app::unititem::UnitItem,
        b: crate::app::unititem::UnitItem,
        arg: ::unity2::IlInstance,
    ) -> bool;

    #[method(name = "GetGiveSkills", args = 0)]
    pub fn get_give_skills(self) -> crate::app::skillarray::SkillArray;

    #[method(name = "GetFontColor", args = 1)]
    pub fn get_font_color(self, is_active: bool) -> crate::unity_engine::color::Color;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::app::unititem::UnitItem,
        rhs: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::app::unititem::UnitItem,
        rhs: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, rhs_obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, rhs: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "EqualsImpl", args = 1)]
    pub fn equals_impl(self, rhs: crate::app::unititem::UnitItem) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-unititem")]
impl UnitItem {
    pub fn new_5(unit_item: crate::app::unititem::UnitItem) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitItem),
                ::core::stringify!(new_5),
            )
        });
        <Self as IUnitItemMethods>::ctor_5(this, unit_item);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unititem/UnitItem_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitItem_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitItem_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitItem.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitItem_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitItem_Flags {
    pub fn equipped() -> Self {
        Self { value: 1 }
    }

    pub fn drop() -> Self {
        Self { value: 2 }
    }

    pub fn skip_log() -> Self {
        Self { value: 4 }
    }
}
