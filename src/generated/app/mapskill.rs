
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_UnitFunc.md")))]
#[::unity2::class(namespace = "App", name = "MapSkill.UnitFunc")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MapSkill_UnitFunc {}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods]
impl MapSkill_UnitFunc {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(self, x: i32, z: i32, target: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-mapskill")]
impl MapSkill_UnitFunc {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSkill_UnitFunc),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSkill_UnitFuncMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_HistoryScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapSkill_HistoryScope {
    pub m_unit: crate::app::unit::Unit,
    pub m_prev_hp: i32,
    pub m_prev_count: i32,
}

impl ::unity2::ClassIdentity for MapSkill_HistoryScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSkill.HistoryScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSkill_HistoryScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods(value)]
impl MapSkill_HistoryScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_AroundCalculator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapSkill_AroundCalculator {
    pub m_side: crate::app::battleinfoside::BattleInfoSide,
    pub m_results: crate::system::collections::generic::list_1::List_1<
        crate::app::mapskill::MapSkill_AroundCalculator_Result,
    >,
}

impl ::unity2::ClassIdentity for MapSkill_AroundCalculator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSkill.AroundCalculator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSkill_AroundCalculator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods(value)]
impl MapSkill_AroundCalculator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, side: crate::app::battleinfoside::BattleInfoSide) -> ();

    #[method(name = "Enumerate", args = 1)]
    pub fn enumerate(self, frequency: crate::app::skilldata::SkillData_Frequencies) -> bool;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "get_Results", args = 0)]
    pub fn get_results(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::mapskill::MapSkill_AroundCalculator_Result,
    >;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_SkillCalculator_UnitList.md")))]
#[::unity2::class(namespace = "App", name = "MapSkill.SkillCalculator.UnitList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: unit :: Unit >)]
pub struct MapSkill_SkillCalculator_UnitList {}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods]
impl MapSkill_SkillCalculator_UnitList {
    #[method(name = "TryAdd", args = 1)]
    pub fn try_add(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapskill")]
impl MapSkill_SkillCalculator_UnitList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSkill_SkillCalculator_UnitList),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSkill_SkillCalculator_UnitListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_Result.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapSkill_Result {
    pub moved: bool,
    pub unit: crate::app::unit::Unit,
    pub x: i32,
    pub z: i32,
}

impl ::unity2::ClassIdentity for MapSkill_Result {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSkill.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSkill_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods(value)]
impl MapSkill_Result {
    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_FixedCalculator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapSkill_FixedCalculator {
    pub m_unit: crate::app::unit::Unit,
    pub m_skill: crate::app::skilldata::SkillData,
    pub m_targets: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
}

impl ::unity2::ClassIdentity for MapSkill_FixedCalculator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSkill.FixedCalculator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSkill_FixedCalculator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods(value)]
impl MapSkill_FixedCalculator {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, skill: crate::app::skilldata::SkillData) -> ();

    #[method(name = "IsIgnore", args = 1)]
    pub fn is_ignore(self, skill: crate::app::skilldata::SkillData) -> bool;

    #[method(name = "Enumerate", args = 0)]
    pub fn enumerate(self) -> bool;

    #[method(name = "ExecuteAct", args = 1)]
    pub fn execute_act(self, target: crate::app::unit::Unit) -> bool;

    #[method(name = "Commit", args = 1)]
    pub fn commit(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_Results.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapSkill_Results {
    pub skill: crate::app::skilldata::SkillData,
    pub current: crate::app::mapskill::MapSkill_Result,
    pub reverse: crate::app::mapskill::MapSkill_Result,
}

impl ::unity2::ClassIdentity for MapSkill_Results {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSkill.Results";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSkill_Results {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods(value)]
impl MapSkill_Results {
    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_SkillCalculator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapSkill_SkillCalculator {
    pub m_unit: crate::app::unit::Unit,
    pub m_target: crate::app::unit::Unit,
    pub m_skill: crate::app::skilldata::SkillData,
    pub m_arounds: crate::app::mapskill::MapSkill_SkillCalculator_UnitList,
}

impl ::unity2::ClassIdentity for MapSkill_SkillCalculator {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSkill.SkillCalculator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSkill_SkillCalculator {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods(value)]
impl MapSkill_SkillCalculator {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        target: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "Enumerate", args = 1)]
    pub fn enumerate(self, frequency: crate::app::skilldata::SkillData_Frequencies) -> bool;

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_AroundCalculator_Result.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapSkill_AroundCalculator_Result {
    pub target: crate::app::unit::Unit,
    pub skill: crate::app::skilldata::SkillData,
}

impl ::unity2::ClassIdentity for MapSkill_AroundCalculator_Result {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSkill.AroundCalculator.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSkill_AroundCalculator_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods(value)]
impl MapSkill_AroundCalculator_Result {
    #[method(name = "Commit", args = 1)]
    pub fn commit(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill_TerrainFunc.md")))]
#[::unity2::class(namespace = "App", name = "MapSkill.TerrainFunc")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct MapSkill_TerrainFunc {}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods]
impl MapSkill_TerrainFunc {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(self, x: i32, z: i32, terrain: crate::app::terraindata_2::TerrainData_2) -> ();
}

#[cfg(feature = "app-mapskill")]
impl MapSkill_TerrainFunc {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSkill_TerrainFunc),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSkill_TerrainFuncMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapskill/MapSkill.md")))]
#[::unity2::class(namespace = "App", name = "MapSkill")]
#[parent(crate::system::object::Object)]
pub struct MapSkill {
    #[static_field]
    #[rename(name = "WaitTime")]
    pub wait_time: f32,
    #[static_field]
    #[rename(name = "BeginWaitTime")]
    pub begin_wait_time: f32,
    #[static_field]
    #[rename(name = "EndWaitTime")]
    pub end_wait_time: f32,
}

#[cfg(feature = "app-mapskill")]
#[::unity2::methods]
impl MapSkill {
    #[method(name = "Prediction", args = 3)]
    pub fn prediction(
        current: crate::app::unit::Unit,
        reverse: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "IsSightOut", args = 3)]
    pub fn is_sight_out(unit: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "CanEnter", args = 4)]
    pub fn can_enter(
        current: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        x: i32,
        z: i32,
    ) -> bool;

    #[method(name = "TryGetX", args = 1)]
    pub fn try_get_x(unit: crate::app::unit::Unit) -> i32;

    #[method(name = "TryGetZ", args = 1)]
    pub fn try_get_z(unit: crate::app::unit::Unit) -> i32;

    #[method(name = "Prediction", args = 4)]
    pub fn prediction_2(
        current: crate::app::unit::Unit,
        reverse: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        results: crate::app::mapskill::MapSkill_Results,
    ) -> bool;

    #[method(name = "CalcPierce", args = 4)]
    pub fn calc_pierce(
        current: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        func: crate::app::mapskill::MapSkill_UnitFunc,
    ) -> bool;

    #[method(name = "CalcStepPos", args = 6)]
    pub fn calc_step_pos(
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
        skill: crate::app::skilldata::SkillData,
        is_reverse: bool,
    ) -> bool;

    #[method(name = "GetRangeDir", args = 2)]
    pub fn get_range_dir(
        attack: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
    ) -> crate::app::dir_2::Dir_Type;

    #[method(name = "TryGetRangeCenter", args = 5)]
    pub fn try_get_range_center(
        current: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        x: i32,
        z: i32,
    ) -> bool;

    #[method(name = "TryAddAttackRange", args = 5)]
    pub fn try_add_attack_range(
        current: crate::app::unit::Unit,
        x: i32,
        z: i32,
        is_unit_only: bool,
        func: crate::app::mapskill::MapSkill_UnitFunc,
    ) -> ();

    #[method(name = "ForeachAttackRange", args = 5)]
    pub fn foreach_attack_range(
        current: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        is_unit_only: bool,
        func: crate::app::mapskill::MapSkill_UnitFunc,
    ) -> ();

    #[method(name = "ForeachAttackRange", args = 6)]
    pub fn foreach_attack_range_2(
        current: crate::app::unit::Unit,
        target_x: i32,
        target_z: i32,
        skill: crate::app::skilldata::SkillData,
        is_unit_only: bool,
        func: crate::app::mapskill::MapSkill_UnitFunc,
    ) -> ();

    #[method(name = "ForeachOverlapRange", args = 3)]
    pub fn foreach_overlap_range(
        current: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        func: crate::app::mapskill::MapSkill_TerrainFunc,
    ) -> ();

    #[method(name = "ForeachOverlapRange", args = 7)]
    pub fn foreach_overlap_range_2(
        current: crate::app::unit::Unit,
        x: i32,
        z: i32,
        target_x: i32,
        target_z: i32,
        skill: crate::app::skilldata::SkillData,
        func: crate::app::mapskill::MapSkill_TerrainFunc,
    ) -> ();

    #[method(name = "ForeachOverlapRange", args = 4)]
    pub fn foreach_overlap_range_3(
        current: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        skill: crate::app::skilldata::SkillData,
        func: crate::app::mapskill::MapSkill_TerrainFunc,
    ) -> ();

    #[method(name = "ForeachOverlapRange", args = 6)]
    pub fn foreach_overlap_range_4(
        current: crate::app::unit::Unit,
        x: i32,
        z: i32,
        dir: crate::app::dir_2::Dir_Type,
        skill: crate::app::skilldata::SkillData,
        func: crate::app::mapskill::MapSkill_TerrainFunc,
    ) -> ();

    #[method(name = "GetAroundCenter", args = 3)]
    pub fn get_around_center(
        skill: crate::app::skilldata::SkillData,
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
    ) -> crate::app::unit::Unit;

    #[method(name = "HasAddGiveSkill", args = 3)]
    pub fn has_add_give_skill(
        skill: crate::app::skilldata::SkillData,
        target: crate::app::skilldata::SkillData_GiveTargets,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "TryAddGiveSkill", args = 3)]
    pub fn try_add_give_skill(
        skill: crate::app::skilldata::SkillData,
        target: crate::app::skilldata::SkillData_GiveTargets,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "TryHitSkill", args = 2)]
    pub fn try_hit_skill(
        skill: crate::app::skilldata::SkillData,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "ExecuteAct", args = 2)]
    pub fn execute_act(skill: crate::app::skilldata::SkillData, unit: crate::app::unit::Unit)
        -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapskill")]
impl MapSkill {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSkill),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSkillMethods>::ctor(this);
        this
    }
}
