
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maptarget/MapTarget_DataSet.md")))]
#[::unity2::class(namespace = "App", name = "MapTarget.DataSet")]
#[parent(crate::system::object::Object)]
pub struct MapTarget_DataSet {
    #[rename(name = "m_List")]
    pub m_list:
        crate::system::collections::generic::list_1::List_1<crate::app::maptarget::MapTarget_Data>,
    #[rename(name = "m_Stack")]
    pub m_stack: crate::system::collections::generic::stack_1::Stack_1<
        crate::app::maptarget::MapTarget_Data,
    >,
}

#[cfg(feature = "app-maptarget")]
#[::unity2::methods]
impl MapTarget_DataSet {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, capacity: i32) -> ();

    #[method(name = "get_List", args = 0)]
    pub fn get_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::maptarget::MapTarget_Data>;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, i: i32) -> crate::app::maptarget::MapTarget_Data;

    #[method(name = "get_ItemMask", args = 0)]
    pub fn get_item_mask(self) -> u32;

    #[method(name = "set_ItemMask", args = 1)]
    pub fn set_item_mask(self, value: u32) -> ();

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "IndexOf", args = 2)]
    pub fn index_of_2(self, x: i32, z: i32) -> i32;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsExist", args = 2)]
    pub fn is_exist_2(self, x: i32, z: i32) -> bool;

    #[method(name = "NewData", args = 0)]
    pub fn new_data(self) -> crate::app::maptarget::MapTarget_Data;

    #[method(name = "NewData", args = 5)]
    pub fn new_data_2(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        item_mask: u32,
        select_item_index: i32,
    ) -> crate::app::maptarget::MapTarget_Data;

    #[method(name = "NewData", args = 4)]
    pub fn new_data_3(
        self,
        x: i32,
        z: i32,
        item_mask: u32,
        select_item_index: i32,
    ) -> crate::app::maptarget::MapTarget_Data;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, from: crate::app::maptarget::MapTarget_DataSet) -> ();

    #[method(name = "Sort", args = 0)]
    pub fn sort(self) -> ();

    #[method(name = "GetDataItemCount", args = 0)]
    pub fn get_data_item_count(self) -> i32;
}

#[cfg(feature = "app-maptarget")]
impl MapTarget_DataSet {
    pub fn new(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapTarget_DataSet),
                ::core::stringify!(new),
            )
        });
        <Self as IMapTarget_DataSetMethods>::ctor(this, capacity);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maptarget/MapTarget_ActionMask.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapTarget_ActionMask {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapTarget_ActionMask {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapTarget.ActionMask";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapTarget_ActionMask {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapTarget_ActionMask {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn attack_only() -> Self {
        Self { value: 1 }
    }

    pub fn magic_only() -> Self {
        Self { value: 2 }
    }

    pub fn rod_only() -> Self {
        Self { value: 3 }
    }

    pub fn cursor_only() -> Self {
        Self { value: 4 }
    }

    pub fn direct_only() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maptarget/MapTarget_Data.md")))]
#[::unity2::class(namespace = "App", name = "MapTarget.Data")]
#[parent(crate::system::object::Object)]
pub struct MapTarget_Data {
    #[rename(name = "m_Index")]
    pub m_index: u8,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_X")]
    pub m_x: i8,
    #[rename(name = "m_Z")]
    pub m_z: i8,
    #[rename(name = "m_X1")]
    pub m_x1: i8,
    #[rename(name = "m_Z1")]
    pub m_z1: i8,
    #[rename(name = "m_X2")]
    pub m_x2: i8,
    #[rename(name = "m_Z2")]
    pub m_z2: i8,
    #[rename(name = "m_ItemMask")]
    pub m_item_mask: u32,
    #[rename(name = "m_SelectItemIndex")]
    pub m_select_item_index: i8,
}

#[cfg(feature = "app-maptarget")]
#[::unity2::methods]
impl MapTarget_Data {
    #[method(name = "Set", args = 3)]
    pub fn set(
        self,
        unit: crate::app::unit::Unit,
        item_mask: u32,
        select_item_index: i32,
    ) -> crate::app::maptarget::MapTarget_Data;

    #[method(name = "Set", args = 5)]
    pub fn set_2(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        item_mask: u32,
        select_item_index: i32,
    ) -> crate::app::maptarget::MapTarget_Data;

    #[method(name = "Set", args = 4)]
    pub fn set_3(
        self,
        x: i32,
        z: i32,
        item_mask: u32,
        select_item_index: i32,
    ) -> crate::app::maptarget::MapTarget_Data;

    #[method(name = "SetRect", args = 4)]
    pub fn set_rect(
        self,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
    ) -> crate::app::maptarget::MapTarget_Data;

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: i32) -> ();

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "get_X1", args = 0)]
    pub fn get_x1(self) -> i32;

    #[method(name = "get_Z1", args = 0)]
    pub fn get_z1(self) -> i32;

    #[method(name = "get_X2", args = 0)]
    pub fn get_x2(self) -> i32;

    #[method(name = "get_Z2", args = 0)]
    pub fn get_z2(self) -> i32;

    #[method(name = "get_ItemMask", args = 0)]
    pub fn get_item_mask(self) -> u32;

    #[method(name = "get_SelectItemIndex", args = 0)]
    pub fn get_select_item_index(self) -> i32;

    #[method(name = "set_SelectItemIndex", args = 1)]
    pub fn set_select_item_index(self, value: i32) -> ();

    #[method(name = "CheckItemMask", args = 1)]
    pub fn check_item_mask(self, index: i32) -> bool;

    #[method(name = "IsOutSide", args = 2)]
    pub fn is_out_side(self, x: i32, z: i32) -> bool;

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, from: crate::app::maptarget::MapTarget_Data) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-maptarget")]
impl MapTarget_Data {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapTarget_Data),
                ::core::stringify!(new),
            )
        });
        <Self as IMapTarget_DataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maptarget/MapTarget_RangeType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapTarget_RangeType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapTarget_RangeType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapTarget.RangeType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapTarget_RangeType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapTarget_RangeType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn unit_items() -> Self {
        Self { value: 1 }
    }

    pub fn specified_item() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/maptarget/MapTarget.md")))]
#[::unity2::class(namespace = "App", name = "MapTarget")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: maptarget :: MapTarget >)]
pub struct MapTarget {
    #[static_field]
    #[rename(name = "WeaponMask")]
    pub weapon_mask: u32,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_X")]
    pub m_x: i8,
    #[rename(name = "m_Z")]
    pub m_z: i8,
    #[rename(name = "m_Mind")]
    pub m_mind: crate::app::mapmind::MapMind_Type,
    #[rename(name = "m_ActionMask")]
    pub m_action_mask: crate::app::maptarget::MapTarget_ActionMask,
    #[rename(name = "m_ActionTemp")]
    pub m_action_temp: crate::app::maptarget::MapTarget_ActionMask,
    #[rename(name = "m_DataSet")]
    pub m_data_set: crate::app::maptarget::MapTarget_DataSet,
    #[rename(name = "m_BufferA")]
    pub m_buffer_a: crate::app::maptarget::MapTarget_DataSet,
    #[rename(name = "m_BufferB")]
    pub m_buffer_b: crate::app::maptarget::MapTarget_DataSet,
    #[rename(name = "m_SelectUnit")]
    pub m_select_unit: crate::app::unit::Unit,
    #[rename(name = "m_SelectX")]
    pub m_select_x: i8,
    #[rename(name = "m_SelectZ")]
    pub m_select_z: i8,
    #[rename(name = "m_SelectItemIndex")]
    pub m_select_item_index: i32,
    #[rename(name = "m_CommandSkill")]
    pub m_command_skill: crate::app::skilldata::SkillData,
    #[rename(name = "m_EnumerateAttackUnitItems")]
    pub m_enumerate_attack_unit_items: crate::app::mapfor::MapFor_TargetFunction,
    #[rename(name = "m_EnumerateAttackSpecifiedItem")]
    pub m_enumerate_attack_specified_item: crate::app::mapfor::MapFor_TargetFunction,
    #[rename(name = "m_EnumerateRodUnitItems")]
    pub m_enumerate_rod_unit_items: crate::app::mapfor::MapFor_TargetFunction,
    #[rename(name = "m_EnumerateRodSpecifiedItem")]
    pub m_enumerate_rod_specified_item: crate::app::mapfor::MapFor_TargetFunction,
    #[static_field]
    #[rename(name = "s_DummyData")]
    pub s_dummy_data: crate::app::maptarget::MapTarget_Data,
}

#[cfg(feature = "app-maptarget")]
#[::unity2::methods]
impl MapTarget {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "BeginTempBuffer", args = 0)]
    pub fn begin_temp_buffer(self) -> ();

    #[method(name = "EndTempBuffer", args = 0)]
    pub fn end_temp_buffer(self) -> ();

    #[method(name = "ResetDataSet", args = 1)]
    pub fn reset_data_set(self, data_set: crate::app::maptarget::MapTarget_DataSet) -> ();

    #[method(name = "ResetDataSet", args = 0)]
    pub fn reset_data_set_2(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Reset", args = 3)]
    pub fn reset_2(
        self,
        unit: crate::app::unit::Unit,
        mind: crate::app::mapmind::MapMind_Type,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "Dump", args = 1)]
    pub fn dump(self, label: ::unity2::Il2CppString) -> ();

    #[method(name = "ReEnumerate", args = 0)]
    pub fn re_enumerate(self) -> ();

    #[method(name = "Enumerate", args = 1)]
    pub fn enumerate(self, mask: crate::app::maptarget::MapTarget_ActionMask) -> ();

    #[method(name = "CheckDataItemMask", args = 1)]
    pub fn check_data_item_mask(self, index: i32) -> bool;

    #[method(name = "SetPosition", args = 2)]
    pub fn set_position(self, x: i32, z: i32) -> ();

    #[method(name = "SetSelectPosition", args = 2)]
    pub fn set_select_position(self, x: i32, z: i32) -> ();

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "set_Mind", args = 1)]
    pub fn set_mind(self, value: crate::app::mapmind::MapMind_Type) -> ();

    #[method(name = "get_IsActive", args = 0)]
    pub fn get_is_active(self) -> bool;

    #[method(name = "get_IsAttack", args = 0)]
    pub fn get_is_attack(self) -> bool;

    #[method(name = "get_ActionMaskValue", args = 0)]
    pub fn get_action_mask_value(self) -> crate::app::maptarget::MapTarget_ActionMask;

    #[method(name = "set_ActionMaskValue", args = 1)]
    pub fn set_action_mask_value(self, value: crate::app::maptarget::MapTarget_ActionMask) -> ();

    #[method(name = "get_Datas", args = 0)]
    pub fn get_datas(self) -> crate::app::maptarget::MapTarget_DataSet;

    #[method(name = "get_DataCount", args = 0)]
    pub fn get_data_count(self) -> i32;

    #[method(name = "get_DataItemMask", args = 0)]
    pub fn get_data_item_mask(self) -> u32;

    #[method(name = "get_DataItemCount", args = 0)]
    pub fn get_data_item_count(self) -> i32;

    #[method(name = "get_SelectUnit", args = 0)]
    pub fn get_select_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_SelectUnit", args = 1)]
    pub fn set_select_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_SelectX", args = 0)]
    pub fn get_select_x(self) -> i32;

    #[method(name = "get_SelectZ", args = 0)]
    pub fn get_select_z(self) -> i32;

    #[method(name = "get_SelectItemIndex", args = 0)]
    pub fn get_select_item_index(self) -> i32;

    #[method(name = "set_SelectItemIndex", args = 1)]
    pub fn set_select_item_index(self, value: i32) -> ();

    #[method(name = "get_CommandSkill", args = 0)]
    pub fn get_command_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "get_SpecifiedItem", args = 0)]
    pub fn get_specified_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "CanAttack", args = 3)]
    pub fn can_attack(
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "CanAttack", args = 4)]
    pub fn can_attack_2(
        mind: crate::app::mapmind::MapMind_Type,
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "PrecheckItemMaskAttack", args = 3)]
    pub fn precheck_item_mask_attack(
        unit: crate::app::unit::Unit,
        mask: crate::app::maptarget::MapTarget_ActionMask,
        skill: crate::app::skilldata::SkillData,
    ) -> u32;

    #[method(name = "CalcAttackRange", args = 6)]
    pub fn calc_attack_range(
        unit: crate::app::unit::Unit,
        mask: crate::app::maptarget::MapTarget_ActionMask,
        skill: crate::app::skilldata::SkillData,
        specified_item: crate::app::unititem::UnitItem,
        min_range: i32,
        max_range: i32,
    ) -> crate::app::maptarget::MapTarget_RangeType;

    #[method(name = "CalcAttackRange", args = 7)]
    pub fn calc_attack_range_2(
        unit: crate::app::unit::Unit,
        mask: crate::app::maptarget::MapTarget_ActionMask,
        skill: crate::app::skilldata::SkillData,
        specified_item: crate::app::unititem::UnitItem,
        min_range: i32,
        max_range: i32,
        item_mask: u32,
    ) -> crate::app::maptarget::MapTarget_RangeType;

    #[method(name = "EnumerateAttackUnitItems", args = 7)]
    pub fn enumerate_attack_unit_items(
        self,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
        target: crate::app::unit::Unit,
        item_mask: u32,
        specified_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "EnumerateAttackSpecifiedItem", args = 7)]
    pub fn enumerate_attack_specified_item(
        self,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
        target: crate::app::unit::Unit,
        item_mask: u32,
        specified_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "EnumerateAttack", args = 1)]
    pub fn enumerate_attack(self, mask: crate::app::maptarget::MapTarget_ActionMask) -> ();

    #[method(name = "EnumerateFullBullet", args = 0)]
    pub fn enumerate_full_bullet(self) -> ();

    #[method(name = "EnumerateRod", args = 1)]
    pub fn enumerate_rod(self, specified_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "EnumerateRod", args = 1)]
    pub fn enumerate_rod_2(self, item_mask: u32) -> ();

    #[method(name = "EnumerateRod", args = 0)]
    pub fn enumerate_rod_3(self) -> ();

    #[method(name = "EnumerateRodUnitItems", args = 7)]
    pub fn enumerate_rod_unit_items(
        self,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
        target: crate::app::unit::Unit,
        item_mask: u32,
        specified_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "EnumerateRodSpecifiedItem", args = 7)]
    pub fn enumerate_rod_specified_item(
        self,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
        target: crate::app::unit::Unit,
        item_mask: u32,
        specified_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "PrecheckItemMaskRod", args = 3)]
    pub fn precheck_item_mask_rod(
        unit: crate::app::unit::Unit,
        mind: crate::app::mapmind::MapMind_Type,
        mask: crate::app::maptarget::MapTarget_ActionMask,
    ) -> u32;

    #[method(name = "EnumerateRodForItemSelect", args = 1)]
    pub fn enumerate_rod_for_item_select(
        self,
        selected_unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "GetSameTypeItemMaskRod", args = 1)]
    pub fn get_same_type_item_mask_rod(
        self,
        source_unit_item: crate::app::unititem::UnitItem,
    ) -> u32;

    #[method(name = "EnumerateSelfOnly", args = 1)]
    pub fn enumerate_self_only(self, item_mask: u32) -> ();

    #[method(name = "EachEnchantTarget", args = 5)]
    pub fn each_enchant_target(
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        item: crate::app::itemdata::ItemData,
        func: crate::system::action_3::Action_3<crate::app::unit::Unit, i32, i32>,
    ) -> ();

    #[method(name = "EachEnchantTarget", args = 6)]
    pub fn each_enchant_target_2(
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        item: crate::app::itemdata::ItemData,
        cancelable: bool,
        func: crate::system::action_3::Action_3<crate::app::unit::Unit, i32, i32>,
    ) -> ();

    #[method(name = "EnumerateEnchant", args = 0)]
    pub fn enumerate_enchant(self) -> ();

    #[method(name = "EnumerateWeapon", args = 0)]
    pub fn enumerate_weapon(self) -> ();

    #[method(name = "EnumerateTalk", args = 0)]
    pub fn enumerate_talk(self) -> ();

    #[method(name = "EnumerateDance", args = 0)]
    pub fn enumerate_dance(self) -> ();

    #[method(name = "EnumerateContract", args = 0)]
    pub fn enumerate_contract(self) -> ();

    #[method(name = "EnumerateTrade", args = 0)]
    pub fn enumerate_trade(self) -> ();

    #[method(name = "PrecheckItemMaskRod", args = 1)]
    pub fn precheck_item_mask_rod_2(self, use_type: crate::app::itemdata::ItemData_UseTypes)
        -> u32;

    #[method(name = "EnumerateRodRewarp", args = 0)]
    pub fn enumerate_rod_rewarp(self) -> ();

    #[method(name = "EnumerateRodTorch", args = 0)]
    pub fn enumerate_rod_torch(self) -> ();

    #[method(name = "TryAddCreation", args = 3)]
    pub fn try_add_creation(self, x: i32, z: i32, item_mask: u32) -> bool;

    #[method(name = "EnumerateRodCreation", args = 0)]
    pub fn enumerate_rod_creation(self) -> ();

    #[method(name = "EnumerateOverlapSkill", args = 0)]
    pub fn enumerate_overlap_skill(self) -> ();

    #[method(name = "EnumerateCommandSkill", args = 0)]
    pub fn enumerate_command_skill(self) -> ();

    #[method(name = "EnumerateDestroy", args = 1)]
    pub fn enumerate_destroy(self, mask: crate::app::maptarget::MapTarget_ActionMask) -> ();

    #[method(name = "EnumerateDestroy", args = 3)]
    pub fn enumerate_destroy_2(self, item_mask: u32, x: i32, z: i32) -> ();

    #[method(name = "EnumerateDestroy", args = 7)]
    pub fn enumerate_destroy_3(
        self,
        item_mask: u32,
        x: i32,
        z: i32,
        x1: i32,
        z1: i32,
        x2: i32,
        z2: i32,
    ) -> ();

    #[method(name = "EnumerateMapInspector", args = 1)]
    pub fn enumerate_map_inspector(self, kind: crate::app::mapinspector::MapInspector_Kind) -> ();

    #[method(name = "EnumerateFireCannon", args = 0)]
    pub fn enumerate_fire_cannon(self) -> ();

    #[method(name = "EnumerateEngageLink", args = 0)]
    pub fn enumerate_engage_link(self) -> ();

    #[method(name = "Sort", args = 0)]
    pub fn sort(self) -> ();

    #[method(name = "GetUsableItemIndex", args = 2)]
    pub fn get_usable_item_index(
        self,
        data: crate::app::maptarget::MapTarget_Data,
        item_index: i32,
    ) -> i32;

    #[method(name = "GetSelectItem", args = 0)]
    pub fn get_select_item(self) -> crate::app::itemdata::ItemData;

    #[method(name = "IsUnnecessaryItemCommand", args = 1)]
    pub fn is_unnecessary_item_command(mind: crate::app::mapmind::MapMind_Type) -> bool;

    #[method(name = "GetDefaultTarget", args = 2)]
    pub fn get_default_target(
        self,
        target_data: crate::app::maptarget::MapTarget_Data,
        item_index: i32,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-maptarget")]
impl MapTarget {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapTarget),
                ::core::stringify!(new),
            )
        });
        <Self as IMapTargetMethods>::ctor(this);
        this
    }
}
