
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitutil/UnitUtil.md")))]
#[::unity2::class(namespace = "App", name = "UnitUtil")]
#[parent(crate::system::object::Object)]
pub struct UnitUtil {}

#[cfg(feature = "app-unitutil")]
#[::unity2::methods]
impl UnitUtil {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "ResetUnitForce", args = 0)]
    pub fn reset_unit_force() -> ();

    #[method(name = "JoinUnit", args = 1)]
    pub fn join_unit(pid: ::unity2::Il2CppString) -> crate::app::unit::Unit;

    #[method(name = "JoinUnit", args = 1)]
    pub fn join_unit_2(person: crate::app::persondata::PersonData) -> crate::app::unit::Unit;

    #[method(name = "JoinMessage", args = 4)]
    pub fn join_message(
        super_: crate::app::procinst::ProcInst,
        person1: crate::app::persondata::PersonData,
        person2: crate::app::persondata::PersonData,
        person3: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "IsJoinedUnit", args = 1)]
    pub fn is_joined_unit(person: crate::app::persondata::PersonData) -> bool;

    #[method(name = "BandActivate", args = 1)]
    pub fn band_activate(unit: crate::app::unit::Unit) -> ();

    #[method(name = "RefineWeapon", args = 3)]
    pub fn refine_weapon(
        unit: crate::app::unit::Unit,
        owner_item_index: i32,
        refine_level: i32,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "EvolveWeapon", args = 3)]
    pub fn evolve_weapon(
        unit: crate::app::unit::Unit,
        owner_item_index: i32,
        evolve_iid: ::unity2::Il2CppString,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "EngraveWeapon", args = 3)]
    pub fn engrave_weapon(
        unit: crate::app::unit::Unit,
        owner_item_index: i32,
        god_data: crate::app::goddata::GodData,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "ClearEngraveWeapon", args = 2)]
    pub fn clear_engrave_weapon(unit: crate::app::unit::Unit, owner_item_index: i32) -> ();

    #[method(name = "ItemDisposalForDead", args = 1)]
    pub fn item_disposal_for_dead(unit: crate::app::unit::Unit) -> ();

    #[method(name = "IsAttackRange", args = 8)]
    pub fn is_attack_range(
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        attack_x: i32,
        attack_z: i32,
        attack_size: i32,
        target_x: i32,
        target_z: i32,
        target_size: i32,
    ) -> bool;

    #[method(name = "IsAttackRange", args = 3)]
    pub fn is_attack_range_2(
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
        range: i32,
    ) -> bool;

    #[method(name = "CanChainAttack", args = 4)]
    pub fn can_chain_attack(
        unit: crate::app::unit::Unit,
        chain: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "CanAutoEquipWhenAddingItem", args = 1)]
    pub fn can_auto_equip_when_adding_item(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CanAutoEquipWhenAddingItem", args = 2)]
    pub fn can_auto_equip_when_adding_item_2(
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
    ) -> bool;

    #[method(name = "CanAutoEquipWhenAddingItem", args = 2)]
    pub fn can_auto_equip_when_adding_item_3(
        unit: crate::app::unit::Unit,
        unit_item: crate::app::unititem::UnitItem,
    ) -> bool;

    #[method(name = "GetRoundGrow", args = 1)]
    pub fn get_round_grow(percent: i32) -> i32;

    #[method(name = "GetVisionPersonImpl", args = 0)]
    pub fn get_vision_person_impl() -> crate::app::persondata::PersonData;

    #[method(name = "GetVisionCountImpl", args = 1)]
    pub fn get_vision_count_impl(owner: crate::app::unit::Unit) -> i32;

    #[method(name = "CanVisionCreateImpl", args = 3)]
    pub fn can_vision_create_impl(owner: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "ForEachVision", args = 2)]
    pub fn for_each_vision(
        owner: crate::app::unit::Unit,
        func: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    ) -> ();

    #[method(name = "GetVisionOwner", args = 1)]
    pub fn get_vision_owner(vision_unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "IsVisionUnit", args = 2)]
    pub fn is_vision_unit(owner: crate::app::unit::Unit, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "HasVisionUnit", args = 1)]
    pub fn has_vision_unit(owner: crate::app::unit::Unit) -> bool;

    #[method(name = "CanVisionCreate", args = 1)]
    pub fn can_vision_create(owner: crate::app::unit::Unit) -> bool;

    #[method(name = "GetVisionOffsets", args = 0)]
    pub fn get_vision_offsets() -> ::unity2::Array<crate::unity_engine::vector2int::Vector2Int>;

    #[method(name = "ForEachCreatableVision", args = 4)]
    pub fn for_each_creatable_vision(
        owner: crate::app::unit::Unit,
        owner_x: i32,
        owner_z: i32,
        func: crate::system::action_3::Action_3<crate::app::skilldata::SkillData, i32, i32>,
    ) -> ();

    #[method(name = "GetCreatableVisionCount", args = 3)]
    pub fn get_creatable_vision_count(
        owner: crate::app::unit::Unit,
        owner_x: i32,
        owner_z: i32,
    ) -> i32;

    #[method(name = "VisionCreate", args = 1)]
    pub fn vision_create(owner: crate::app::unit::Unit) -> ();

    #[method(name = "CanVisionDelete", args = 1)]
    pub fn can_vision_delete(owner: crate::app::unit::Unit) -> bool;

    #[method(name = "VisionDelete", args = 1)]
    pub fn vision_delete(owner: crate::app::unit::Unit) -> ();

    #[method(name = "GetSummonUnit", args = 1)]
    pub fn get_summon_unit(owner: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "SummonCreate", args = 3)]
    pub fn summon_create(
        owner: crate::app::unit::Unit,
        rank: crate::app::persondata::PersonData_Ranks,
        person: crate::app::persondata::PersonData,
    ) -> crate::app::unit::Unit;

    #[method(name = "SummonDelete", args = 1)]
    pub fn summon_delete(owner: crate::app::unit::Unit) -> ();

    #[method(name = "VisionDeleteImpl", args = 1)]
    pub fn vision_delete_impl(unit: crate::app::unit::Unit) -> ();

    #[method(name = "SummonDeleteImpl", args = 1)]
    pub fn summon_delete_impl(unit: crate::app::unit::Unit) -> ();

    #[method(name = "UnitUpdate", args = 0)]
    pub fn unit_update() -> ();

    #[method(name = "ForceGathter", args = 3)]
    pub fn force_gathter(
        x: i32,
        z: i32,
        types: ::unity2::Array<crate::app::force::Force_Type>,
    ) -> ();

    #[method(name = "GetFreePoint", args = 3)]
    pub fn get_free_point(unit: crate::app::unit::Unit, tx: i32, tz: i32) -> bool;

    #[method(name = "TryAddReliance", args = 3)]
    pub fn try_add_reliance(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
        name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "TryAddReliance", args = 3)]
    pub fn try_add_reliance_2(
        unit_a: crate::app::unit::Unit,
        unit_b: crate::app::unit::Unit,
        value: i32,
    ) -> bool;

    #[method(name = "TryAddExp", args = 3)]
    pub fn try_add_exp(
        unit: crate::app::unit::Unit,
        god_unit: crate::app::godunit::GodUnit,
        exp: i32,
    ) -> bool;

    #[method(name = "GetDieType", args = 1)]
    pub fn get_die_type(unit: crate::app::unit::Unit) -> crate::app::unitutil::UnitUtil_DieType;

    #[method(name = "TryGetExistMessage", args = 3)]
    pub fn try_get_exist_message(
        mid: ::unity2::Il2CppString,
        label: ::unity2::Il2CppString,
        footer: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "GetDieMessage", args = 2)]
    pub fn get_die_message(
        unit: crate::app::unit::Unit,
        ascii: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetDieMessage", args = 1)]
    pub fn get_die_message_2(unit: crate::app::unit::Unit) -> ::unity2::Il2CppString;

    #[method(name = "GetDieGodMessage", args = 1)]
    pub fn get_die_god_message(unit: crate::app::unit::Unit) -> ::unity2::Il2CppString;

    #[method(name = "GetVoiceID", args = 1)]
    pub fn get_voice_id(unit: crate::app::unit::Unit) -> ::unity2::Il2CppString;

    #[method(name = "IsSameUnit", args = 2)]
    pub fn is_same_unit(unit: crate::app::unit::Unit, target: crate::app::unit::Unit) -> bool;

    #[method(name = "IsWeakness", args = 1)]
    pub fn is_weakness(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "ResetChapter", args = 1)]
    pub fn reset_chapter(chapter: crate::app::chapterdata::ChapterData) -> ();

    #[method(name = "GetSummonPersons", args = 2)]
    pub fn get_summon_persons(
        rank: crate::app::persondata::PersonData_Ranks,
        color: crate::app::persondata::PersonData_Colors,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::persondata::PersonData>;

    #[method(name = "GetSummonRank", args = 0)]
    pub fn get_summon_rank() -> crate::app::persondata::PersonData_Ranks;

    #[method(name = "CalcSummon", args = 5)]
    pub fn calc_summon(
        person: crate::app::persondata::PersonData,
        rank: crate::app::persondata::PersonData_Ranks,
        skill: crate::app::skilldata::SkillData,
        color: crate::app::persondata::PersonData_Colors,
        dbg_rank: crate::app::persondata::PersonData_Ranks,
    ) -> bool;

    #[method(name = "IsMultiChangeGod", args = 1)]
    pub fn is_multi_change_god(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsOutOfView", args = 1)]
    pub fn is_out_of_view(position: crate::unity_engine::vector3::Vector3) -> bool;

    #[method(name = "TryMultiChangeGod", args = 1)]
    pub fn try_multi_change_god(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "GetHelpKey", args = 1)]
    pub fn get_help_key(unit: crate::app::unit::Unit) -> ::unity2::Il2CppString;

    #[method(name = "GodSaveEquip", args = 0)]
    pub fn god_save_equip() -> ();

    #[method(name = "GodLoadEquip", args = 0)]
    pub fn god_load_equip() -> ();

    #[method(name = "GetMoveFirstAdd", args = 1)]
    pub fn get_move_first_add(unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetMoveFirstAdd", args = 3)]
    pub fn get_move_first_add_2(unit: crate::app::unit::Unit, x: i32, z: i32) -> i32;

    #[method(name = "GetMoveFirstAdd", args = 4)]
    pub fn get_move_first_add_3(
        move_type: crate::app::jobdata::JobData_MoveTypes,
        move_power: i32,
        x: i32,
        z: i32,
    ) -> i32;

    #[method(name = "GetTotalItemCount", args = 1)]
    pub fn get_total_item_count(iid: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetTotalItemCount", args = 1)]
    pub fn get_total_item_count_2(item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "ClearEnhanceAll", args = 0)]
    pub fn clear_enhance_all() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitutil")]
impl UnitUtil {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitUtil),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitUtilMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitutil/UnitUtil_DieType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UnitUtil_DieType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UnitUtil_DieType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "UnitUtil.DieType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UnitUtil_DieType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UnitUtil_DieType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn casual() -> Self {
        Self { value: 1 }
    }

    pub fn exist() -> Self {
        Self { value: 2 }
    }

    pub fn die() -> Self {
        Self { value: 3 }
    }
}
