
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aiinterruptattack/AIInterruptAttack.md")))]
#[::unity2::class(namespace = "App", name = "AIInterruptAttack")]
#[parent(crate::system::object::Object)]
pub struct AIInterruptAttack {
    #[static_field]
    #[rename(name = "MaxAttackCountLunatic")]
    pub max_attack_count_lunatic: i32,
    #[static_field]
    #[rename(name = "MaxAttackCountDefault")]
    pub max_attack_count_default: i32,
    #[static_field]
    #[rename(name = "MaxCandidate")]
    pub max_candidate: i32,
    #[rename(name = "m_Original")]
    pub m_original: crate::app::unit::Unit,
    #[rename(name = "m_OriginalMoveX")]
    pub m_original_move_x: i32,
    #[rename(name = "m_OriginalMoveZ")]
    pub m_original_move_z: i32,
    #[rename(name = "m_OriginalAttackX")]
    pub m_original_attack_x: i32,
    #[rename(name = "m_OriginalAttackZ")]
    pub m_original_attack_z: i32,
    #[rename(name = "m_OriginalItemIndex")]
    pub m_original_item_index: i32,
    #[rename(name = "m_OriginalBulletPattern")]
    pub m_original_bullet_pattern: i32,
    #[rename(name = "m_Target")]
    pub m_target: crate::app::unit::Unit,
    #[rename(name = "m_TargetBaseX")]
    pub m_target_base_x: i32,
    #[rename(name = "m_TargetBaseZ")]
    pub m_target_base_z: i32,
    #[rename(name = "m_TargetX")]
    pub m_target_x: i32,
    #[rename(name = "m_TargetZ")]
    pub m_target_z: i32,
    #[rename(name = "m_Candidates")]
    pub m_candidates: crate::app::aiinterruptattack::AIInterruptAttack_CandidateList,
    #[rename(name = "m_AttackCount")]
    pub m_attack_count: i32,
    #[rename(name = "m_MaxAttackCount")]
    pub m_max_attack_count: i32,
    #[rename(name = "m_IsDecidedAttacker")]
    pub m_is_decided_attacker: bool,
    #[rename(name = "m_BattleSimulator")]
    pub m_battle_simulator: crate::app::aibattlesimulator::AIBattleSimulator,
    #[rename(name = "m_ASResult")]
    pub m_as_result: crate::app::aiinterruptattack::AIInterruptAttack_AttackScoreResult,
}

#[cfg(feature = "app-aiinterruptattack")]
#[::unity2::methods]
impl AIInterruptAttack {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Think", args = 10)]
    pub fn think(
        self,
        actor: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        move_x: i32,
        move_z: i32,
        attack_x: i32,
        attack_z: i32,
        item_index: i32,
        target_x: i32,
        target_z: i32,
        bullet_pattern: i32,
    ) -> bool;

    #[method(name = "ThinkImpl", args = 10)]
    pub fn think_impl(
        self,
        actor: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        move_x: i32,
        move_z: i32,
        attack_x: i32,
        attack_z: i32,
        item_index: i32,
        target_x: i32,
        target_z: i32,
        bullet_pattern: i32,
    ) -> bool;

    #[method(name = "Attack", args = 0)]
    pub fn attack(self) -> bool;

    #[method(name = "IsReserved", args = 0)]
    pub fn is_reserved(self) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "EnumerateCandidates", args = 2)]
    pub fn enumerate_candidates(
        self,
        is_original_think_break: bool,
        is_original_think_chain: bool,
    ) -> ();

    #[method(name = "DecideAttacker", args = 1)]
    pub fn decide_attacker(self, is_add_moved_original: bool) -> bool;

    #[method(name = "Deploy", args = 1)]
    pub fn deploy(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetAttackScore", args = 4)]
    pub fn get_attack_score(
        self,
        actor: crate::app::unit::Unit,
        flag: i32,
        bullet_pattern: i32,
        result: crate::app::aiinterruptattack::AIInterruptAttack_AttackScoreResult,
    ) -> bool;

    #[method(name = "GetRange", args = 1)]
    pub fn get_range(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetMaxAttackCount", args = 0)]
    pub fn get_max_attack_count(self) -> i32;

    #[method(name = "AddMovedOriginalToImage", args = 0)]
    pub fn add_moved_original_to_image(self) -> ();

    #[method(name = "DeleteMovedOriginalFromImage", args = 0)]
    pub fn delete_moved_original_from_image(self) -> ();

    #[method(name = "Mind", args = 7)]
    pub fn mind(
        self,
        attacker: crate::app::unit::Unit,
        move_x: i32,
        move_z: i32,
        attack_x: i32,
        attack_z: i32,
        item_index: i32,
        bullet_pattern: i32,
    ) -> ();

    #[method(name = "DbgLog", args = 1)]
    pub fn dbg_log(self, msg: ::unity2::Il2CppString) -> ();

    #[method(name = "DbgLogOrignalAndTarget", args = 0)]
    pub fn dbg_log_orignal_and_target(self) -> ();

    #[method(name = "DbgLogCandidates", args = 0)]
    pub fn dbg_log_candidates(self) -> ();

    #[method(name = "DbgLogAttacker", args = 8)]
    pub fn dbg_log_attacker(
        self,
        msg: ::unity2::Il2CppString,
        unit: crate::app::unit::Unit,
        move_x: i32,
        move_z: i32,
        attack_x: i32,
        attack_z: i32,
        item_index: i32,
        bullet_pattern: i32,
    ) -> ();

    #[method(name = "DbgLogExist", args = 4)]
    pub fn dbg_log_exist(
        self,
        method_name: ::unity2::Il2CppString,
        attacker: crate::app::unit::Unit,
        move_x: i32,
        move_z: i32,
    ) -> ();
}

#[cfg(feature = "app-aiinterruptattack")]
impl AIInterruptAttack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIInterruptAttack),
                ::core::stringify!(new),
            )
        });
        <Self as IAIInterruptAttackMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aiinterruptattack/AIInterruptAttack_AttackScoreResult.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AIInterruptAttack_AttackScoreResult {}

impl ::unity2::ClassIdentity for AIInterruptAttack_AttackScoreResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AIInterruptAttack.AttackScoreResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AIInterruptAttack_AttackScoreResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-aiinterruptattack")]
#[::unity2::methods(value)]
impl AIInterruptAttack_AttackScoreResult {
    #[method(name = "get_Score", args = 0)]
    pub fn get_score(self) -> u32;

    #[method(name = "set_Score", args = 1)]
    pub fn set_score(self, value: u32) -> ();

    #[method(name = "get_MoveX", args = 0)]
    pub fn get_move_x(self) -> i32;

    #[method(name = "set_MoveX", args = 1)]
    pub fn set_move_x(self, value: i32) -> ();

    #[method(name = "get_MoveZ", args = 0)]
    pub fn get_move_z(self) -> i32;

    #[method(name = "set_MoveZ", args = 1)]
    pub fn set_move_z(self, value: i32) -> ();

    #[method(name = "get_AttackX", args = 0)]
    pub fn get_attack_x(self) -> i32;

    #[method(name = "set_AttackX", args = 1)]
    pub fn set_attack_x(self, value: i32) -> ();

    #[method(name = "get_AttackZ", args = 0)]
    pub fn get_attack_z(self) -> i32;

    #[method(name = "set_AttackZ", args = 1)]
    pub fn set_attack_z(self, value: i32) -> ();

    #[method(name = "get_ItemIndex", args = 0)]
    pub fn get_item_index(self) -> i32;

    #[method(name = "set_ItemIndex", args = 1)]
    pub fn set_item_index(self, value: i32) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aiinterruptattack/AIInterruptAttack_Candidate.md")))]
#[::unity2::class(namespace = "App", name = "AIInterruptAttack.Candidate")]
#[parent(crate::system::object::Object)]
pub struct AIInterruptAttack_Candidate {}

#[cfg(feature = "app-aiinterruptattack")]
#[::unity2::methods]
impl AIInterruptAttack_Candidate {
    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_Unit", args = 1)]
    pub fn set_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_IsThinkBreak", args = 0)]
    pub fn get_is_think_break(self) -> bool;

    #[method(name = "set_IsThinkBreak", args = 1)]
    pub fn set_is_think_break(self, value: bool) -> ();

    #[method(name = "get_IsThinkChain", args = 0)]
    pub fn get_is_think_chain(self) -> bool;

    #[method(name = "set_IsThinkChain", args = 1)]
    pub fn set_is_think_chain(self, value: bool) -> ();

    #[method(name = "get_Dist", args = 0)]
    pub fn get_dist(self) -> i32;

    #[method(name = "set_Dist", args = 1)]
    pub fn set_dist(self, value: i32) -> ();

    #[method(name = "get_BulletPattern", args = 0)]
    pub fn get_bullet_pattern(self) -> i32;

    #[method(name = "set_BulletPattern", args = 1)]
    pub fn set_bullet_pattern(self, value: i32) -> ();

    #[method(name = "get_MoveX", args = 0)]
    pub fn get_move_x(self) -> i32;

    #[method(name = "set_MoveX", args = 1)]
    pub fn set_move_x(self, value: i32) -> ();

    #[method(name = "get_MoveZ", args = 0)]
    pub fn get_move_z(self) -> i32;

    #[method(name = "set_MoveZ", args = 1)]
    pub fn set_move_z(self, value: i32) -> ();

    #[method(name = "get_AttackX", args = 0)]
    pub fn get_attack_x(self) -> i32;

    #[method(name = "set_AttackX", args = 1)]
    pub fn set_attack_x(self, value: i32) -> ();

    #[method(name = "get_AttackZ", args = 0)]
    pub fn get_attack_z(self) -> i32;

    #[method(name = "set_AttackZ", args = 1)]
    pub fn set_attack_z(self, value: i32) -> ();

    #[method(name = "get_ItemIndex", args = 0)]
    pub fn get_item_index(self) -> i32;

    #[method(name = "set_ItemIndex", args = 1)]
    pub fn set_item_index(self, value: i32) -> ();

    #[method(name = "get_Score", args = 0)]
    pub fn get_score(self) -> u32;

    #[method(name = "set_Score", args = 1)]
    pub fn set_score(self, value: u32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-aiinterruptattack")]
impl AIInterruptAttack_Candidate {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIInterruptAttack_Candidate),
                ::core::stringify!(new),
            )
        });
        <Self as IAIInterruptAttack_CandidateMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aiinterruptattack/AIInterruptAttack_CandidateList.md")))]
#[::unity2::class(namespace = "App", name = "AIInterruptAttack.CandidateList")]
#[parent(crate::system::object::Object)]
pub struct AIInterruptAttack_CandidateList {
    #[rename(name = "m_Pool")]
    pub m_pool: crate::system::collections::generic::list_1::List_1<
        crate::app::aiinterruptattack::AIInterruptAttack_Candidate,
    >,
    #[rename(name = "m_List")]
    pub m_list: crate::system::collections::generic::list_1::List_1<
        crate::app::aiinterruptattack::AIInterruptAttack_Candidate,
    >,
}

#[cfg(feature = "app-aiinterruptattack")]
#[::unity2::methods]
impl AIInterruptAttack_CandidateList {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Add", args = 4)]
    pub fn add(
        self,
        unit: crate::app::unit::Unit,
        is_think_break: bool,
        is_think_chain: bool,
        dist: i32,
    ) -> ();

    #[method(name = "Sort", args = 0)]
    pub fn sort(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "HasAttacker", args = 0)]
    pub fn has_attacker(self) -> bool;

    #[method(name = "GetAttacker", args = 0)]
    pub fn get_attacker(self) -> crate::app::aiinterruptattack::AIInterruptAttack_Candidate;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32)
        -> crate::app::aiinterruptattack::AIInterruptAttack_Candidate;
}

#[cfg(feature = "app-aiinterruptattack")]
impl AIInterruptAttack_CandidateList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIInterruptAttack_CandidateList),
                ::core::stringify!(new),
            )
        });
        <Self as IAIInterruptAttack_CandidateListMethods>::ctor(this);
        this
    }
}
