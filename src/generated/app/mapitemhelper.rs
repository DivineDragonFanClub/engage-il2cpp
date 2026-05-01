
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemhelper/MapItemHelper.md")))]
#[::unity2::class(namespace = "App", name = "MapItemHelper")]
#[parent(crate::system::object::Object)]
pub struct MapItemHelper {}

#[cfg(feature = "app-mapitemhelper")]
#[::unity2::methods]
impl MapItemHelper {
    #[method(name = "CanUse", args = 8)]
    pub fn can_use(
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
        target_x: i32,
        target_z: i32,
        unit_x: i32,
        unit_z: i32,
        flag: crate::app::mapitemhelper::MapItemHelper_Flag,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "GetUseTargetUnit", args = 4)]
    pub fn get_use_target_unit(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
        x: i32,
        z: i32,
    ) -> crate::app::unit::Unit;

    #[method(name = "CanUse", args = 8)]
    pub fn can_use_2(
        attack_unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        atk_pos_x: i32,
        atk_pos_z: i32,
        target_pos_x: i32,
        target_pos_z: i32,
        flag: crate::app::mapitemhelper::MapItemHelper_Flag,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "IsTargetAllied", args = 1)]
    pub fn is_target_allied(r#type: crate::app::itemdata::ItemData_UseTypes) -> bool;

    #[method(name = "CanDirectTarget", args = 1)]
    pub fn can_direct_target(item: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "GetBlessFlags", args = 3)]
    pub fn get_bless_flags(
        item: crate::app::itemdata::ItemData,
        target: crate::app::unit::Unit,
        use_type: crate::app::itemdata::ItemData_UseTypes,
    ) -> crate::app::mapitemhelper::MapItemHelper_BlessFlags;

    #[method(name = "GetBlessFlags", args = 2)]
    pub fn get_bless_flags_2(
        item: crate::app::itemdata::ItemData,
        target: crate::app::unit::Unit,
    ) -> crate::app::mapitemhelper::MapItemHelper_BlessFlags;

    #[method(name = "CanUseImpl", args = 5)]
    pub fn can_use_impl(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
        target: crate::app::unit::Unit,
        use_type: crate::app::itemdata::ItemData_UseTypes,
        give_skills: crate::app::skillarray::SkillArray,
    ) -> bool;

    #[method(name = "CanUseEnchant", args = 4)]
    pub fn can_use_enchant(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
        target: crate::app::unit::Unit,
        cancelable: bool,
    ) -> bool;

    #[method(name = "CanUseTarget", args = 3)]
    pub fn can_use_target(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
        target: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "GetItemIndex", args = 6)]
    pub fn get_item_index(
        unit: crate::app::unit::Unit,
        target_x: i32,
        target_z: i32,
        unit_x: i32,
        unit_z: i32,
        flag: crate::app::mapitemhelper::MapItemHelper_Flag,
    ) -> i32;

    #[method(name = "ForEachRodTarget", args = 6)]
    pub fn for_each_rod_target(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
        target_x: i32,
        target_z: i32,
        skill: crate::app::skilldata::SkillData,
        func: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    ) -> ();

    #[method(name = "ForEachRodRange", args = 5)]
    pub fn for_each_rod_range(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
        target_x: i32,
        target_z: i32,
        func: crate::system::action_2::Action_2<i32, i32>,
    ) -> ();

    #[method(name = "TryCreation", args = 5)]
    pub fn try_creation(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        target_x: i32,
        target_z: i32,
        tid: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "GetRescuePosition", args = 6)]
    pub fn get_rescue_position(
        dst_x: i32,
        dst_z: i32,
        target: crate::app::unit::Unit,
        src_x: i32,
        src_z: i32,
        is_here: bool,
    ) -> bool;

    #[method(name = "GetRescuePosition", args = 7)]
    pub fn get_rescue_position_2(
        rod_unit: crate::app::unit::Unit,
        dst_x: i32,
        dst_z: i32,
        target: crate::app::unit::Unit,
        src_x: i32,
        src_z: i32,
        is_here: bool,
    ) -> bool;

    #[method(name = "CanAttack", args = 3)]
    pub fn can_attack(
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        flag: crate::app::mapitemhelper::MapItemHelper_Flag,
    ) -> bool;

    #[method(name = "CanRod", args = 4)]
    pub fn can_rod(
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
        flag: crate::app::mapitemhelper::MapItemHelper_Flag,
    ) -> bool;

    #[method(name = "GetFireCannonItem", args = 0)]
    pub fn get_fire_cannon_item() -> crate::app::itemdata::ItemData;

    #[method(name = "GetFireCannonRangeIO", args = 3)]
    pub fn get_fire_cannon_range_io(unit: crate::app::unit::Unit, range_i: i32, range_o: i32)
        -> ();

    #[method(name = "GetHealPower", args = 3)]
    pub fn get_heal_power(
        unit: crate::app::unit::Unit,
        use_type: crate::app::itemdata::ItemData_UseTypes,
        power: i32,
    ) -> i32;

    #[method(name = "GetHealPower", args = 3)]
    pub fn get_heal_power_2(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
        is_enchant: bool,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapitemhelper")]
impl MapItemHelper {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemHelper),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemHelperMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemhelper/MapItemHelper_BlessFlags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapItemHelper_BlessFlags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapItemHelper_BlessFlags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapItemHelper.BlessFlags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapItemHelper_BlessFlags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapItemHelper_BlessFlags {
    pub fn heal() -> Self {
        Self { value: 1 }
    }

    pub fn stock() -> Self {
        Self { value: 2 }
    }

    pub fn disorder() -> Self {
        Self { value: 4 }
    }

    pub fn engage_turn() -> Self {
        Self { value: 8 }
    }

    pub fn engage_count() -> Self {
        Self { value: 16 }
    }

    pub fn max_heal() -> Self {
        Self { value: 32 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemhelper/MapItemHelper_Flag.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapItemHelper_Flag {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapItemHelper_Flag {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapItemHelper.Flag";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapItemHelper_Flag {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapItemHelper_Flag {
    pub fn attack() -> Self {
        Self { value: 1 }
    }

    pub fn rod() -> Self {
        Self { value: 2 }
    }

    pub fn destroy() -> Self {
        Self { value: 4 }
    }

    pub fn offense() -> Self {
        Self { value: 8 }
    }

    pub fn direct_rod() -> Self {
        Self { value: 16 }
    }
}
