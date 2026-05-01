
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillarray/SkillArray.md")))]
#[::unity2::class(namespace = "App", name = "SkillArray")]
#[parent(crate::system::object::Object)]
pub struct SkillArray {
    #[static_field]
    #[rename(name = "MaxCount")]
    pub max_count: i32,
    #[static_field]
    #[rename(name = "Capacity")]
    pub capacity: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_Mask")]
    pub m_mask: crate::app::bitstruct::BitStruct,
    #[rename(name = "m_List")]
    pub m_list: crate::system::collections::generic::list_1::List_1<
        crate::app::skillarray::SkillArray_Entity,
    >,
    #[rename(name = "m_Flags")]
    pub m_flags: crate::app::skilldata::SkillData_Flags,
    #[rename(name = "m_Cycles")]
    pub m_cycles: crate::app::skilldata::SkillData_CycleMasks,
    #[rename(name = "m_Timings")]
    pub m_timings: crate::app::skilldata::SkillData_TimingMasks,
    #[rename(name = "m_Efficacys")]
    pub m_efficacys: crate::app::skilldata::SkillData_Attrs,
    #[rename(name = "m_EfficacyIgnores")]
    pub m_efficacy_ignores: crate::app::skilldata::SkillData_Attrs,
    #[rename(name = "m_BadStates")]
    pub m_bad_states: crate::app::skilldata::SkillData_States,
    #[rename(name = "m_BadIgnore")]
    pub m_bad_ignore: crate::app::skilldata::SkillData_States,
    #[rename(name = "m_WeaponLevels")]
    pub m_weapon_levels: crate::app::weaponlevels::WeaponLevels,
    #[rename(name = "m_IsEquipSkillFirstNull")]
    pub m_is_equip_skill_first_null: bool,
}

#[cfg(feature = "app-skillarray")]
#[::unity2::methods]
impl SkillArray {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, src: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::app::skillarray::SkillArray_Enumerator;

    #[method(name = "get_Flags", args = 0)]
    pub fn get_flags(self) -> crate::app::skilldata::SkillData_Flags;

    #[method(name = "Test", args = 1)]
    pub fn test(self, sid: ::unity2::Il2CppString) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_2(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_3(self, flags: crate::app::skilldata::SkillData_Flags) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_4(self, states: crate::app::skilldata::SkillData_States) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_5(self, timing: crate::app::skilldata::SkillData_Timings) -> bool;

    #[method(name = "Test", args = 2)]
    pub fn test_6(
        self,
        timing1: crate::app::skilldata::SkillData_Timings,
        timing2: crate::app::skilldata::SkillData_Timings,
    ) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_7(self, mask: crate::app::skilldata::SkillData_TimingMasks) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_8(self, cycle: crate::app::skilldata::SkillData_Cycles) -> bool;

    #[method(name = "Test", args = 2)]
    pub fn test_9(
        self,
        cycle1: crate::app::skilldata::SkillData_Cycles,
        cycle2: crate::app::skilldata::SkillData_Cycles,
    ) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_10(self, mask: crate::app::skilldata::SkillData_CycleMasks) -> bool;

    #[method(name = "Test", args = 1)]
    pub fn test_11(self, target: crate::app::skilldata::SkillData_Targets) -> bool;

    #[method(name = "GetBadStates", args = 0)]
    pub fn get_bad_states(self) -> crate::app::skilldata::SkillData_States;

    #[method(name = "Set", args = 2)]
    pub fn set(
        self,
        sid: ::unity2::Il2CppString,
        category: crate::app::skilldata::SkillData_Categorys,
    ) -> bool;

    #[method(name = "Set", args = 2)]
    pub fn set_2(
        self,
        skill: crate::app::skilldata::SkillData,
        category: crate::app::skilldata::SkillData_Categorys,
    ) -> bool;

    #[method(name = "Set", args = 2)]
    pub fn set_3(
        self,
        sids: ::unity2::Array<::unity2::Il2CppString>,
        category: crate::app::skilldata::SkillData_Categorys,
    ) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set_4(self, src: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add(
        self,
        sid: ::unity2::Il2CppString,
        category: crate::app::skilldata::SkillData_Categorys,
        age: i32,
    ) -> bool;

    #[method(name = "Add", args = 3)]
    pub fn add_2(
        self,
        skill: crate::app::skilldata::SkillData,
        category: crate::app::skilldata::SkillData_Categorys,
        age: i32,
    ) -> bool;

    #[method(name = "AddWithoutUpdate", args = 3)]
    pub fn add_without_update(
        self,
        skill: crate::app::skilldata::SkillData,
        category: crate::app::skilldata::SkillData_Categorys,
        age: i32,
    ) -> bool;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_3(
        self,
        sids: ::unity2::Array<::unity2::Il2CppString>,
        category: crate::app::skilldata::SkillData_Categorys,
    ) -> bool;

    #[method(name = "Add", args = 2)]
    pub fn add_4(
        self,
        skills: crate::system::collections::generic::list_1::List_1<
            crate::app::skilldata::SkillData,
        >,
        category: crate::app::skilldata::SkillData_Categorys,
    ) -> bool;

    #[method(name = "Add", args = 1)]
    pub fn add_5(self, src: crate::app::skillarray::SkillArray) -> bool;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, sid: ::unity2::Il2CppString) -> bool;

    #[method(name = "Remove", args = 1)]
    pub fn remove_2(self, src: crate::app::skillarray::SkillArray) -> bool;

    #[method(name = "Remove", args = 1)]
    pub fn remove_3(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "Remove", args = 1)]
    pub fn remove_4(self, flags: crate::app::skilldata::SkillData_Flags) -> bool;

    #[method(name = "Replace", args = 3)]
    pub fn replace(
        self,
        index: i32,
        skill: crate::app::skilldata::SkillData,
        category: crate::app::skilldata::SkillData_Categorys,
    ) -> bool;

    #[method(name = "Replace", args = 1)]
    pub fn replace_2(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "Move", args = 2)]
    pub fn r#move(self, old_index: i32, new_index: i32) -> bool;

    #[method(name = "Sort", args = 0)]
    pub fn sort(self) -> ();

    #[method(name = "GetGiveSkill", args = 1)]
    pub fn get_give_skill(
        self,
        give: crate::app::skilldata::SkillData,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "AddImpl", args = 3)]
    pub fn add_impl(
        self,
        sid: ::unity2::Il2CppString,
        category: crate::app::skilldata::SkillData_Categorys,
        age: i32,
    ) -> bool;

    #[method(name = "AddImpl", args = 3)]
    pub fn add_impl_2(
        self,
        skill: crate::app::skilldata::SkillData,
        category: crate::app::skilldata::SkillData_Categorys,
        age: i32,
    ) -> bool;

    #[method(name = "RemoveImpl", args = 1)]
    pub fn remove_impl(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "RemoveImpl", args = 1)]
    pub fn remove_impl_2(self, index: i32) -> bool;

    #[method(name = "ReplaceImpl", args = 4)]
    pub fn replace_impl(
        self,
        index: i32,
        skill: crate::app::skilldata::SkillData,
        category: crate::app::skilldata::SkillData_Categorys,
        age: i32,
    ) -> bool;

    #[method(name = "MoveImpl", args = 2)]
    pub fn move_impl(self, old_index: i32, new_index: i32) -> bool;

    #[method(name = "AddSyncImpl", args = 3)]
    pub fn add_sync_impl(
        self,
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        category: crate::app::skilldata::SkillData_Categorys,
    ) -> bool;

    #[method(name = "CanGive", args = 1)]
    pub fn can_give(self, give: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "Commit", args = 1)]
    pub fn commit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetItemRangeI", args = 2)]
    pub fn get_item_range_i(
        self,
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
    ) -> i32;

    #[method(name = "GetItemRangeO", args = 2)]
    pub fn get_item_range_o(
        self,
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
    ) -> i32;

    #[method(name = "GetItemDistance", args = 2)]
    pub fn get_item_distance(
        self,
        unit: crate::app::unit::Unit,
        item: crate::app::itemdata::ItemData,
    ) -> i32;

    #[method(name = "GetRodRangeExtend", args = 1)]
    pub fn get_rod_range_extend(self, item: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "UpdateImpl", args = 0)]
    pub fn update_impl(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, src: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "Change", args = 1)]
    pub fn change(self, index: i32) -> ();

    #[method(name = "IsExist", args = 0)]
    pub fn is_exist(self) -> bool;

    #[method(name = "IsIgnore", args = 1)]
    pub fn is_ignore(self, state: crate::app::skilldata::SkillData_States) -> bool;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, sid: ::unity2::Il2CppString) -> i32;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of_2(self, skill: crate::app::skilldata::SkillData) -> i32;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::app::skilldata::SkillData;

    #[method(name = "GetSkill", args = 1)]
    pub fn get_skill(self, index: i32) -> crate::app::skilldata::SkillData;

    #[method(name = "TryGetSkill", args = 1)]
    pub fn try_get_skill(self, index: i32) -> crate::app::skilldata::SkillData;

    #[method(name = "GetEntity", args = 1)]
    pub fn get_entity(self, index: i32) -> crate::app::skillarray::SkillArray_Entity;

    #[method(name = "GetAge", args = 1)]
    pub fn get_age(self, index: i32) -> i32;

    #[method(name = "GetAge", args = 1)]
    pub fn get_age_2(self, skill: crate::app::skilldata::SkillData) -> i32;

    #[method(name = "GetDecay", args = 1)]
    pub fn get_decay(self, index: i32) -> i32;

    #[method(name = "GetCategory", args = 1)]
    pub fn get_category(self, index: i32) -> crate::app::skilldata::SkillData_Categorys;

    #[method(name = "GetCategory", args = 1)]
    pub fn get_category_2(
        self,
        skill: crate::app::skilldata::SkillData,
    ) -> crate::app::skilldata::SkillData_Categorys;

    #[method(name = "Find", args = 1)]
    pub fn find(self, sid: ::unity2::Il2CppString) -> crate::app::skilldata::SkillData;

    #[method(name = "Find", args = 1)]
    pub fn find_2(
        self,
        flags: crate::app::skilldata::SkillData_Flags,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "Find", args = 1)]
    pub fn find_3(
        self,
        states: crate::app::skilldata::SkillData_States,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "Find", args = 1)]
    pub fn find_4(
        self,
        work: crate::app::skilldata::SkillData_Works,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "UpdateAging", args = 2)]
    pub fn update_aging(
        self,
        cycle: crate::app::skilldata::SkillData_Cycles,
        ignore: crate::system::func_2::Func_2<crate::app::skilldata::SkillData, bool>,
    ) -> bool;

    #[method(name = "UpdateAging", args = 3)]
    pub fn update_aging_2(
        self,
        cycle1: crate::app::skilldata::SkillData_Cycles,
        cycle2: crate::app::skilldata::SkillData_Cycles,
        ignore: crate::system::func_2::Func_2<crate::app::skilldata::SkillData, bool>,
    ) -> bool;

    #[method(name = "UpdateAgingImpl", args = 2)]
    pub fn update_aging_impl(
        self,
        mask: crate::app::skilldata::SkillData_CycleMasks,
        ignore: crate::system::func_2::Func_2<crate::app::skilldata::SkillData, bool>,
    ) -> bool;

    #[method(name = "ClearCycles", args = 0)]
    pub fn clear_cycles(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "CalcWork", args = 2)]
    pub fn calc_work(self, value: i32, work: crate::app::skilldata::SkillData_Works) -> i32;

    #[method(name = "GetPower", args = 0)]
    pub fn get_power(self) -> i32;

    #[method(name = "GetEfficacyValue", args = 1)]
    pub fn get_efficacy_value(self, target: crate::app::unit::Unit) -> i32;

    #[method(name = "GetEfficacyMask", args = 1)]
    pub fn get_efficacy_mask(
        self,
        target: crate::app::unit::Unit,
    ) -> crate::app::skilldata::SkillData_Attrs;

    #[method(name = "IsEfficacy", args = 1)]
    pub fn is_efficacy(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "get_WeaponLevels", args = 0)]
    pub fn get_weapon_levels(self) -> crate::app::weaponlevels::WeaponLevels;

    #[method(name = "get_IsEquipSkillFirstNull", args = 0)]
    pub fn get_is_equip_skill_first_null(self) -> bool;

    #[method(name = "set_IsEquipSkillFirstNull", args = 1)]
    pub fn set_is_equip_skill_first_null(self, value: bool) -> ();
}

#[cfg(feature = "app-skillarray")]
impl SkillArray {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillArray),
                ::core::stringify!(new),
            )
        });
        <Self as ISkillArrayMethods>::ctor(this);
        this
    }

    pub fn new_2(src: crate::app::skillarray::SkillArray) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SkillArray),
                ::core::stringify!(new_2),
            )
        });
        <Self as ISkillArrayMethods>::ctor_2(this, src);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillarray/SkillArray_Enumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SkillArray_Enumerator {
    pub m_array: crate::app::skillarray::SkillArray,
    pub m_current: crate::app::skilldata::SkillData,
    pub m_index: i32,
}

impl ::unity2::ClassIdentity for SkillArray_Enumerator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SkillArray.Enumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SkillArray_Enumerator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-skillarray")]
#[::unity2::methods(value)]
impl SkillArray_Enumerator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, skill_array: crate::app::skillarray::SkillArray) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::skilldata::SkillData;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/skillarray/SkillArray_Entity.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SkillArray_Entity {
    pub value: u32,
}

impl ::unity2::ClassIdentity for SkillArray_Entity {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SkillArray.Entity";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SkillArray_Entity {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-skillarray")]
#[::unity2::methods(value)]
impl SkillArray_Entity {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        index: i32,
        group: i32,
        category: crate::app::skilldata::SkillData_Categorys,
        age: i32,
    ) -> ();

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "set_Index", args = 1)]
    pub fn set_index(self, value: i32) -> ();

    #[method(name = "get_Group", args = 0)]
    pub fn get_group(self) -> i32;

    #[method(name = "set_Group", args = 1)]
    pub fn set_group(self, value: i32) -> ();

    #[method(name = "get_Age", args = 0)]
    pub fn get_age(self) -> i32;

    #[method(name = "set_Age", args = 1)]
    pub fn set_age(self, value: i32) -> ();

    #[method(name = "get_Category", args = 0)]
    pub fn get_category(self) -> crate::app::skilldata::SkillData_Categorys;

    #[method(name = "set_Category", args = 1)]
    pub fn set_category(self, value: crate::app::skilldata::SkillData_Categorys) -> ();

    #[method(name = "get_Skill", args = 0)]
    pub fn get_skill(self) -> crate::app::skilldata::SkillData;
}
