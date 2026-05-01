
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fsmbuilder/FSMBuilder.md")))]
#[::unity2::class(namespace = "Combat", name = "FSMBuilder")]
#[parent(crate::system::object::Object)]
pub struct FSMBuilder {}

#[cfg(feature = "combat-fsmbuilder")]
#[::unity2::methods]
impl FSMBuilder {
    #[method(name = "get_world", args = 0)]
    pub fn get_world() -> crate::combat::combatworld::CombatWorld;

    #[method(name = "get_record", args = 0)]
    pub fn get_record() -> crate::combat::combatrecord::CombatRecord;

    #[method(name = "get_chrs", args = 0)]
    pub fn get_chrs() -> ::unity2::Array<crate::combat::character::Character>;

    #[method(name = "get_cam", args = 0)]
    pub fn get_cam() -> crate::combat::cameramanager::CameraManager;

    #[method(name = "get_style", args = 0)]
    pub fn get_style() -> crate::combat::combatstyle::CombatStyle;

    #[method(name = "get_anyone", args = 0)]
    pub fn get_anyone() -> crate::combat::character::Character;

    #[method(name = "get_FirstAttackPhase", args = 0)]
    pub fn get_first_attack_phase() -> crate::combat::phase::Phase;

    #[method(name = "get_FirstAttacker", args = 0)]
    pub fn get_first_attacker() -> crate::combat::character::Character;

    #[method(name = "PreLoad", args = 0)]
    pub fn pre_load() -> ();

    #[method(name = "CreateAndRun", args = 0)]
    pub fn create_and_run() -> crate::combat::fsmbuilder::FSMBuilder;

    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "BuildStart", args = 0)]
    pub fn build_start(self) -> ();

    #[method(name = "BuildMain", args = 0)]
    pub fn build_main(self) -> ();

    #[method(name = "BuildEnd", args = 0)]
    pub fn build_end(self) -> ();

    #[method(name = "BuildSkipover", args = 0)]
    pub fn build_skipover(self) -> ();

    #[method(name = "RunEternalCombatAppendObserver", args = 0)]
    pub fn run_eternal_combat_append_observer() -> ();

    #[method(name = "FSMAdd_WaitForTransitionCamera", args = 1)]
    pub fn fsm_add_wait_for_transition_camera(rate: f32) -> ();

    #[method(name = "ActionWairForTransitionCamera", args = 1)]
    pub fn action_wair_for_transition_camera(rate: f32) -> crate::combat::actionbase::ActionBase;

    #[method(name = "FSMAdd_SyncEveryone", args = 1)]
    pub fn fsm_add_sync_everyone(include_grandew: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-fsmbuilder")]
impl FSMBuilder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FSMBuilder),
                ::core::stringify!(new),
            )
        });
        <Self as IFSMBuilderMethods>::ctor(this);
        this
    }
}
