
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aienum/AIEnum_SkillRangeEnemyData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AIEnum_SkillRangeEnemyData {
    pub unit: crate::app::unit::Unit,
    pub x: i8,
    pub z: i8,
}

impl ::unity2::ClassIdentity for AIEnum_SkillRangeEnemyData {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AIEnum.SkillRangeEnemyData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AIEnum_SkillRangeEnemyData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aienum/AIEnum_SkillRangeEnemyEnumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AIEnum_SkillRangeEnemyEnumerator {
    pub m_unit: crate::app::unit::Unit,
    pub m_offsets: crate::system::collections::generic::list_1::List_1<
        crate::app::rangedata::RangeData_Offset,
    >,
    pub m_target_x: i32,
    pub m_target_z: i32,
    pub m_index: i32,
    pub m_current: crate::app::aienum::AIEnum_SkillRangeEnemyData,
    pub m_enumerated: crate::system::collections::generic::list_1::List_1<i32>,
}

impl ::unity2::ClassIdentity for AIEnum_SkillRangeEnemyEnumerator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AIEnum.SkillRangeEnemyEnumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AIEnum_SkillRangeEnemyEnumerator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-aienum")]
#[::unity2::methods(value)]
impl AIEnum_SkillRangeEnemyEnumerator {
    #[method(name = "Setup", args = 6)]
    pub fn setup(
        self,
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        x: i32,
        z: i32,
        target_x: i32,
        target_z: i32,
    ) -> crate::app::aienum::AIEnum_SkillRangeEnemyEnumerator;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::app::aienum::AIEnum_SkillRangeEnemyData;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::app::aienum::AIEnum_SkillRangeEnemyEnumerator;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aienum/AIEnum.md")))]
#[::unity2::class(namespace = "App", name = "AIEnum")]
#[parent(crate::system::object::Object)]
pub struct AIEnum {
    #[static_field]
    #[rename(name = "s_SkillRangeEnemyEnumerator")]
    pub s_skill_range_enemy_enumerator: crate::app::aienum::AIEnum_SkillRangeEnemyEnumerator,
}

#[cfg(feature = "app-aienum")]
#[::unity2::methods]
impl AIEnum {
    #[method(name = "GetSkillRangeEnemy", args = 6)]
    pub fn get_skill_range_enemy(
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        x: i32,
        z: i32,
        target_x: i32,
        target_z: i32,
    ) -> crate::app::aienum::AIEnum_SkillRangeEnemyEnumerator;
}
