
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenaordersequence/ArenaOrderSequence_GodInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ArenaOrderSequence_GodInfo {
    pub god: crate::app::godunit::GodUnit,
    pub r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
}

impl ::unity2::ClassIdentity for ArenaOrderSequence_GodInfo {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ArenaOrderSequence.GodInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ArenaOrderSequence_GodInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenaordersequence/ArenaOrderSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ArenaOrderSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ArenaOrderSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ArenaOrderSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ArenaOrderSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ArenaOrderSequence_Label {
    pub fn top() -> Self {
        Self { value: 0 }
    }

    pub fn exp_select_unit() -> Self {
        Self { value: 1 }
    }

    pub fn bond_select_unit() -> Self {
        Self { value: 2 }
    }

    pub fn bond_select_emblem() -> Self {
        Self { value: 3 }
    }

    pub fn bond_select_level() -> Self {
        Self { value: 4 }
    }

    pub fn training() -> Self {
        Self { value: 5 }
    }

    pub fn result() -> Self {
        Self { value: 6 }
    }

    pub fn end() -> Self {
        Self { value: 7 }
    }

    pub fn skill_inheritance() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenaordersequence/ArenaOrderSequence_TrainType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ArenaOrderSequence_TrainType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ArenaOrderSequence_TrainType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ArenaOrderSequence.TrainType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ArenaOrderSequence_TrainType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ArenaOrderSequence_TrainType {
    pub fn random() -> Self {
        Self { value: 0 }
    }

    pub fn emblem() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenaordersequence/ArenaOrderSequence.md")))]
#[::unity2::class(namespace = "App", name = "ArenaOrderSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: arenaordersequence :: ArenaOrderSequence >)]
pub struct ArenaOrderSequence {
    #[rename(name = "m_ExpUnitSelectRoot")]
    pub m_exp_unit_select_root: crate::app::arenaexpunitselectroot::ArenaExpUnitSelectRoot,
    #[rename(name = "m_BondUnitSelectRoot")]
    pub m_bond_unit_select_root: crate::app::arenabondunitselectroot::ArenaBondUnitSelectRoot,
    #[rename(name = "m_BondEmblemSelectRoot")]
    pub m_bond_emblem_select_root: crate::app::arenabondgodselectroot::ArenaBondGodSelectRoot,
    #[rename(name = "m_BondLevelSelectRoot")]
    pub m_bond_level_select_root: crate::app::arenabondlevelselectroot::ArenaBondLevelSelectRoot,
    #[rename(name = "m_NextLabel")]
    pub m_next_label: crate::app::arenaordersequence::ArenaOrderSequence_Label,
    #[rename(name = "m_IsBackBondSelectEmblem")]
    pub m_is_back_bond_select_emblem: bool,
    #[rename(name = "m_ArenaObjects")]
    pub m_arena_objects: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
    #[rename(name = "m_Ring")]
    pub m_ring: crate::app::unitring::UnitRing,
}

#[cfg(feature = "app-arenaordersequence")]
#[::unity2::methods]
impl ArenaOrderSequence {
    #[method(name = "get_SelectableUnit", args = 0)]
    pub fn get_selectable_unit(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>;

    #[method(name = "set_SelectableUnit", args = 1)]
    pub fn set_selectable_unit(
        self,
        value: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    ) -> ();

    #[method(name = "get_SelectableGod", args = 0)]
    pub fn get_selectable_god(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::arenaordersequence::ArenaOrderSequence_GodInfo,
    >;

    #[method(name = "set_SelectableGod", args = 1)]
    pub fn set_selectable_god(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::arenaordersequence::ArenaOrderSequence_GodInfo,
        >,
    ) -> ();

    #[method(name = "SetupSelectableGodList", args = 1)]
    pub fn setup_selectable_god_list(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "IsSelectableGod", args = 2)]
    pub fn is_selectable_god(
        god: crate::app::godunit::GodUnit,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "IsSelectableGodImpl", args = 1)]
    pub fn is_selectable_god_impl(god: crate::app::godunit::GodUnit) -> bool;

    #[method(name = "GetNextLevelCap", args = 4)]
    pub fn get_next_level_cap(
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        is_cap_over: bool,
        is_level_cap_count: bool,
    ) -> i32;

    #[method(name = "IsLevelCapTalk", args = 2)]
    pub fn is_level_cap_talk(
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
    ) -> bool;

    #[method(name = "get_IsEmblemBattle", args = 0)]
    pub fn get_is_emblem_battle(self) -> bool;

    #[method(name = "set_IsEmblemBattle", args = 1)]
    pub fn set_is_emblem_battle(self, value: bool) -> ();

    #[method(name = "get_IsSpecialBattle", args = 0)]
    pub fn get_is_special_battle(self) -> bool;

    #[method(name = "set_IsSpecialBattle", args = 1)]
    pub fn set_is_special_battle(self, value: bool) -> ();

    #[method(name = "get_TrainingType", args = 0)]
    pub fn get_training_type(self) -> crate::app::arenaordersequence::ArenaOrderSequence_TrainType;

    #[method(name = "set_TrainingType", args = 1)]
    pub fn set_training_type(
        self,
        value: crate::app::arenaordersequence::ArenaOrderSequence_TrainType,
    ) -> ();

    #[method(name = "get_TrainingUnit", args = 0)]
    pub fn get_training_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_TrainingUnit", args = 1)]
    pub fn set_training_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_BattleUnit", args = 0)]
    pub fn get_battle_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_BattleUnit", args = 1)]
    pub fn set_battle_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_BattleEmblem", args = 0)]
    pub fn get_battle_emblem(self) -> crate::app::godunit::GodUnit;

    #[method(name = "set_BattleEmblem", args = 1)]
    pub fn set_battle_emblem(self, value: crate::app::godunit::GodUnit) -> ();

    #[method(name = "get_EmblemType", args = 0)]
    pub fn get_emblem_type(self) -> crate::app::ringcleaningsequence::RingCleaningSequence_GodType;

    #[method(name = "set_EmblemType", args = 1)]
    pub fn set_emblem_type(
        self,
        value: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "get_BondExp", args = 0)]
    pub fn get_bond_exp(self) -> i32;

    #[method(name = "set_BondExp", args = 1)]
    pub fn set_bond_exp(self, value: i32) -> ();

    #[method(name = "get_Calculator", args = 0)]
    pub fn get_calculator(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "set_Calculator", args = 1)]
    pub fn set_calculator(self, value: crate::app::battlecalculator::BattleCalculator) -> ();

    #[method(name = "get_SimCalculator", args = 0)]
    pub fn get_sim_calculator(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "set_SimCalculator", args = 1)]
    pub fn set_sim_calculator(self, value: crate::app::battlecalculator::BattleCalculator) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "BackgroundIn", args = 0)]
    pub fn background_in(self) -> ();

    #[method(name = "BackgroundOut", args = 0)]
    pub fn background_out(self) -> ();

    #[method(name = "DispTitleBar", args = 0)]
    pub fn disp_title_bar(self) -> ();

    #[method(name = "CreateSkillInheritance", args = 0)]
    pub fn create_skill_inheritance(self) -> ();

    #[method(name = "CreateTopMenu", args = 0)]
    pub fn create_top_menu(self) -> ();

    #[method(name = "CreateExpUnitSelectMenu", args = 0)]
    pub fn create_exp_unit_select_menu(self) -> ();

    #[method(name = "DestroyExpUnitSelectMenu", args = 0)]
    pub fn destroy_exp_unit_select_menu(self) -> ();

    #[method(name = "CreateBondUnitSelectMenu", args = 0)]
    pub fn create_bond_unit_select_menu(self) -> ();

    #[method(name = "DestroyBondUnitSelectMenu", args = 0)]
    pub fn destroy_bond_unit_select_menu(self) -> ();

    #[method(name = "CreateBondEmblemSelectMenu", args = 0)]
    pub fn create_bond_emblem_select_menu(self) -> ();

    #[method(name = "DestroyBondEmblemSelectMenu", args = 0)]
    pub fn destroy_bond_emblem_select_menu(self) -> ();

    #[method(name = "CreateBondLevelSelectMenu", args = 0)]
    pub fn create_bond_level_select_menu(self) -> ();

    #[method(name = "DestroyBondLevelSelectMenu", args = 0)]
    pub fn destroy_bond_level_select_menu(self) -> ();

    #[method(name = "SetupRandomUnit", args = 0)]
    pub fn setup_random_unit(self) -> ();

    #[method(name = "SetupEmblem", args = 2)]
    pub fn setup_emblem(
        self,
        emblem: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "Culculate", args = 0)]
    pub fn culculate(self) -> ();

    #[method(name = "SetupTraining", args = 0)]
    pub fn setup_training(self) -> ();

    #[method(name = "StartTraining", args = 0)]
    pub fn start_training(self) -> ();

    #[method(name = "FinishTraining", args = 0)]
    pub fn finish_training(self) -> ();

    #[method(name = "SetBattleUnitWeapon", args = 1)]
    pub fn set_battle_unit_weapon(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetEmblemWeapon", args = 3)]
    pub fn set_emblem_weapon(
        self,
        unit: crate::app::unit::Unit,
        emblem: crate::app::godunit::GodUnit,
        bond_level: i32,
    ) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-arenaordersequence")]
impl ArenaOrderSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaOrderSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaOrderSequenceMethods>::ctor(this);
        this
    }
}
