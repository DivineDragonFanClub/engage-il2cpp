
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencemind/MapSequenceMind.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceMind")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapSequenceMind {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Target")]
    pub m_target: crate::app::unit::Unit,
    #[rename(name = "m_IsMoveOnly")]
    pub m_is_move_only: bool,
}

#[cfg(feature = "app-mapsequencemind")]
#[::unity2::methods]
impl MapSequenceMind {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, target: crate::app::unit::Unit) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetTarget", args = 0)]
    pub fn get_target(self) -> crate::app::unit::Unit;

    #[method(name = "UnitMove", args = 0)]
    pub fn unit_move(self) -> ();

    #[method(name = "UnitMoveWait", args = 0)]
    pub fn unit_move_wait(self) -> ();

    #[method(name = "SetMoveOnly", args = 0)]
    pub fn set_move_only(self) -> ();

    #[method(name = "IsMoveOnly", args = 0)]
    pub fn is_move_only(self) -> bool;

    #[method(name = "IsSkipAIEngageRewarp", args = 0)]
    pub fn is_skip_ai_engage_rewarp(self) -> bool;

    #[method(name = "AIEngageRewarp", args = 0)]
    pub fn ai_engage_rewarp(self) -> ();

    #[method(name = "MultiTarget", args = 0)]
    pub fn multi_target(self) -> ();

    #[method(name = "TryAddTargetOfFireCannon", args = 2)]
    pub fn try_add_target_of_fire_cannon(self, x: i32, z: i32) -> ();

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "Breakdown", args = 0)]
    pub fn breakdown(self) -> ();

    #[method(name = "BreakdownEnemy", args = 0)]
    pub fn breakdown_enemy(self) -> ();

    #[method(name = "DestroyVillage", args = 0)]
    pub fn destroy_village(self) -> ();

    #[method(name = "EscapeEvent", args = 0)]
    pub fn escape_event(self) -> ();

    #[method(name = "Escape", args = 0)]
    pub fn escape(self) -> ();

    #[method(name = "Visit", args = 0)]
    pub fn visit(self) -> ();

    #[method(name = "Poke", args = 1)]
    pub fn poke(self, kind: crate::app::mapinspector::MapInspector_Kind) -> ();

    #[method(name = "Unlock", args = 0)]
    pub fn unlock(self) -> ();

    #[method(name = "TreasureBox", args = 0)]
    pub fn treasure_box(self) -> ();

    #[method(name = "Door", args = 0)]
    pub fn door(self) -> ();

    #[method(name = "Torch", args = 0)]
    pub fn torch(self) -> ();

    #[method(name = "Guard", args = 0)]
    pub fn guard(self) -> ();

    #[method(name = "Lockon", args = 0)]
    pub fn lockon(self) -> ();

    #[method(name = "Talk", args = 0)]
    pub fn talk(self) -> ();

    #[method(name = "DoneAction", args = 0)]
    pub fn done_action(self) -> ();

    #[method(name = "OverlapSkill", args = 0)]
    pub fn overlap_skill(self) -> ();

    #[method(name = "CommandSkill", args = 0)]
    pub fn command_skill(self) -> ();

    #[method(name = "EngageCharge", args = 0)]
    pub fn engage_charge(self) -> ();

    #[method(name = "EngageWait", args = 0)]
    pub fn engage_wait(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "UpdateImage", args = 0)]
    pub fn update_image(self) -> ();

    #[method(name = "TryTransfer", args = 1)]
    pub fn try_transfer(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "Fixed", args = 0)]
    pub fn fixed(self) -> ();

    #[method(name = "GodExp", args = 0)]
    pub fn god_exp(self) -> ();

    #[method(name = "GetFixedSkillEnum", args = 1)]
    pub fn get_fixed_skill_enum(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::skillenum::SkillEnum;

    #[method(name = "get_CanWaitSkip", args = 0)]
    pub fn get_can_wait_skip(self) -> bool;

    #[method(name = "FixedSkill", args = 0)]
    pub fn fixed_skill(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "CanFixedBonus", args = 1)]
    pub fn can_fixed_bonus(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "EngageHeal", args = 0)]
    pub fn engage_heal(self) -> ();

    #[method(name = "KillBonus", args = 0)]
    pub fn kill_bonus(self) -> ();

    #[method(name = "IsMoveSkip", args = 0)]
    pub fn is_move_skip(self) -> bool;

    #[method(name = "IsAlive", args = 1)]
    pub fn is_alive(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsFixedEvent", args = 1)]
    pub fn is_fixed_event(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsFixedEvent", args = 2)]
    pub fn is_fixed_event_2(
        self,
        unit: crate::app::unit::Unit,
        force_type: crate::app::force::Force_Type,
    ) -> bool;

    #[method(name = "HelpEvent", args = 0)]
    pub fn help_event(self) -> ();

    #[method(name = "FixedEvent", args = 0)]
    pub fn fixed_event(self) -> ();

    #[method(name = "AreaEvent", args = 0)]
    pub fn area_event(self) -> ();

    #[method(name = "AddUnitImage", args = 0)]
    pub fn add_unit_image(self) -> ();

    #[method(name = "DeleteUnitImage", args = 0)]
    pub fn delete_unit_image(self) -> ();

    #[method(name = "UnitDeadEvent", args = 0)]
    pub fn unit_dead_event(self) -> ();

    #[method(name = "UnitDeadFade", args = 0)]
    pub fn unit_dead_fade(self) -> ();

    #[method(name = "UnitDead", args = 0)]
    pub fn unit_dead(self) -> ();

    #[method(name = "UnitDeadKillBonus", args = 0)]
    pub fn unit_dead_kill_bonus(self) -> ();

    #[method(name = "Revive", args = 0)]
    pub fn revive(self) -> ();

    #[method(name = "LooselyFocus", args = 0)]
    pub fn loosely_focus(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindTalk", args = 1)]
    pub fn create_bind_talk(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindDie", args = 2)]
    pub fn create_bind_die(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "CreateBindDieWithoutEvent", args = 2)]
    pub fn create_bind_die_without_event(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "CreateBindEventBattle", args = 1)]
    pub fn create_bind_event_battle(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindMoveOnly", args = 1)]
    pub fn create_bind_move_only(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindImpl", args = 3)]
    pub fn create_bind_impl(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
    ) -> crate::app::mapsequencemind::MapSequenceMind;
}

#[cfg(feature = "app-mapsequencemind")]
impl MapSequenceMind {
    pub fn new(unit: crate::app::unit::Unit, target: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceMind),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceMindMethods>::ctor(this, unit, target);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencemind/MapSequenceMind_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceMind_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceMind_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceMind.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceMind_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceMind_Label {
    pub fn move_skip() -> Self {
        Self { value: 0 }
    }

    pub fn after_ai_engage_rewarp() -> Self {
        Self { value: 1 }
    }

    pub fn attack() -> Self {
        Self { value: 2 }
    }

    pub fn rod() -> Self {
        Self { value: 3 }
    }

    pub fn item_use() -> Self {
        Self { value: 4 }
    }

    pub fn breakdown() -> Self {
        Self { value: 5 }
    }

    pub fn breakdown_enemy() -> Self {
        Self { value: 6 }
    }

    pub fn destroy() -> Self {
        Self { value: 7 }
    }

    pub fn escape() -> Self {
        Self { value: 8 }
    }

    pub fn visit() -> Self {
        Self { value: 9 }
    }

    pub fn destroy_village() -> Self {
        Self { value: 10 }
    }

    pub fn treasure_box() -> Self {
        Self { value: 11 }
    }

    pub fn door() -> Self {
        Self { value: 12 }
    }

    pub fn torch() -> Self {
        Self { value: 13 }
    }

    pub fn talk() -> Self {
        Self { value: 14 }
    }

    pub fn dance() -> Self {
        Self { value: 15 }
    }

    pub fn guard() -> Self {
        Self { value: 16 }
    }

    pub fn full_bullet() -> Self {
        Self { value: 17 }
    }

    pub fn enchant() -> Self {
        Self { value: 18 }
    }

    pub fn contract() -> Self {
        Self { value: 19 }
    }

    pub fn engage_charge() -> Self {
        Self { value: 20 }
    }

    pub fn engage_wait() -> Self {
        Self { value: 21 }
    }

    pub fn engage_summon() -> Self {
        Self { value: 22 }
    }

    pub fn overlap_skill() -> Self {
        Self { value: 23 }
    }

    pub fn command_skill() -> Self {
        Self { value: 24 }
    }

    pub fn event_battle() -> Self {
        Self { value: 25 }
    }

    pub fn fixed() -> Self {
        Self { value: 26 }
    }

    pub fn dead() -> Self {
        Self { value: 27 }
    }

    pub fn dead_without_event() -> Self {
        Self { value: 28 }
    }

    pub fn end() -> Self {
        Self { value: 29 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencemind/MapSequenceMind_ProcEscape.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceMind.ProcEscape")]
#[parent(crate::app::mapsequencemind::MapSequenceMind_ProcMindBase)]
pub struct MapSequenceMind_ProcEscape {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
}

#[cfg(feature = "app-mapsequencemind")]
#[::unity2::methods]
impl MapSequenceMind_ProcEscape {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "GetDir", args = 0)]
    pub fn get_dir(self) -> crate::app::dir_2::Dir_Type;
}

#[cfg(feature = "app-mapsequencemind")]
impl MapSequenceMind_ProcEscape {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceMind_ProcEscape),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceMind_ProcEscapeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencemind/MapSequenceMind_ProcMindBase.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceMind.ProcMindBase")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapSequenceMind_ProcMindBase {}

#[cfg(feature = "app-mapsequencemind")]
#[::unity2::methods]
impl MapSequenceMind_ProcMindBase {
    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequencemind")]
impl MapSequenceMind_ProcMindBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceMind_ProcMindBase),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceMind_ProcMindBaseMethods>::ctor(this);
        this
    }
}
