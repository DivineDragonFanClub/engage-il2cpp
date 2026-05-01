
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatworld/CombatWorld.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatWorld")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CombatWorld {
    #[static_field]
    #[rename(name = "s_this")]
    pub s_this: crate::combat::combatworld::CombatWorld,
    #[rename(name = "m_bRunning")]
    pub m_b_running: bool,
    #[rename(name = "chrs")]
    pub chrs: ::unity2::Array<crate::combat::character::Character>,
    #[rename(name = "_Timespace")]
    pub timespace: crate::combat::combattimespace::CombatTimespace,
    #[rename(name = "_Staging")]
    pub staging: crate::combat::combatstaging::CombatStaging,
    #[rename(name = "_Observable")]
    pub observable: crate::combat::combatobservable::CombatObservable,
    #[rename(name = "_Config")]
    pub config: crate::combat::combatconfig::CombatConfig,
    #[rename(name = "_Skip")]
    pub skip: crate::combat::combatskip::CombatSkip,
    #[rename(name = "_Input")]
    pub input: crate::combat::combatinput::CombatInput,
}

#[cfg(feature = "combat-combatworld")]
#[::unity2::methods]
impl CombatWorld {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> crate::combat::combatworld::CombatWorld;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "get_IsRunning", args = 0)]
    pub fn get_is_running() -> bool;

    #[method(name = "get_Characters", args = 0)]
    pub fn get_characters(self) -> ::unity2::Array<crate::combat::character::Character>;

    #[method(name = "get_ChainAttackers", args = 0)]
    pub fn get_chain_attackers(self) -> ::unity2::Array<crate::combat::character::Character>;

    #[method(name = "set_ChainAttackers", args = 1)]
    pub fn set_chain_attackers(
        self,
        value: ::unity2::Array<crate::combat::character::Character>,
    ) -> ();

    #[method(name = "get_CharaDragonic", args = 0)]
    pub fn get_chara_dragonic(self) -> ::unity2::Array<crate::combat::character::Character>;

    #[method(name = "set_CharaDragonic", args = 1)]
    pub fn set_chara_dragonic(
        self,
        value: ::unity2::Array<crate::combat::character::Character>,
    ) -> ();

    #[method(name = "get_Record", args = 0)]
    pub fn get_record(self) -> crate::combat::combatrecord::CombatRecord;

    #[method(name = "set_Record", args = 1)]
    pub fn set_record(self, value: crate::combat::combatrecord::CombatRecord) -> ();

    #[method(name = "get_EffectCatalog", args = 0)]
    pub fn get_effect_catalog(self) -> crate::combat::effectcatalog::EffectCatalog;

    #[method(name = "set_EffectCatalog", args = 1)]
    pub fn set_effect_catalog(self, value: crate::combat::effectcatalog::EffectCatalog) -> ();

    #[method(name = "get_FSMBuilder", args = 0)]
    pub fn get_fsm_builder(self) -> crate::combat::fsmbuilder::FSMBuilder;

    #[method(name = "set_FSMBuilder", args = 1)]
    pub fn set_fsm_builder(self, value: crate::combat::fsmbuilder::FSMBuilder) -> ();

    #[method(name = "IsFromGame", args = 0)]
    pub fn is_from_game() -> bool;

    #[method(name = "get_CombatTime", args = 0)]
    pub fn get_combat_time(self) -> f32;

    #[method(name = "set_CombatTime", args = 1)]
    pub fn set_combat_time(self, value: f32) -> ();

    #[method(name = "get_Timespace", args = 0)]
    pub fn get_timespace(self) -> crate::combat::combattimespace::CombatTimespace;

    #[method(name = "get_Staging", args = 0)]
    pub fn get_staging(self) -> crate::combat::combatstaging::CombatStaging;

    #[method(name = "get_Observable", args = 0)]
    pub fn get_observable(self) -> crate::combat::combatobservable::CombatObservable;

    #[method(name = "get_Config", args = 0)]
    pub fn get_config(self) -> crate::combat::combatconfig::CombatConfig;

    #[method(name = "get_Skip", args = 0)]
    pub fn get_skip(self) -> crate::combat::combatskip::CombatSkip;

    #[method(name = "get_Input", args = 0)]
    pub fn get_input(self) -> crate::combat::combatinput::CombatInput;

    #[method(name = "Run", args = 1)]
    pub fn run(self, record: crate::combat::combatrecord::CombatRecord) -> ();

    #[method(name = "InitializeAfterCreateChara", args = 0)]
    pub fn initialize_after_create_chara(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "RegisterHitObserver", args = 0)]
    pub fn register_hit_observer(self) -> ();

    #[method(name = "SyncParams", args = 0)]
    pub fn sync_params(self) -> ();

    #[method(name = "CreateCharacters", args = 0)]
    pub fn create_characters(self) -> ();

    #[method(name = "CreateCharactersImpl", args = 3)]
    pub fn create_characters_impl(
        self,
        side: i32,
        gs: crate::combat::charactergamestatus::CharacterGameStatus,
        chain_id: i32,
    ) -> ();

    #[method(name = "IsCharacterCreated", args = 0)]
    pub fn is_character_created(self) -> bool;

    #[method(name = "SetupCharacters", args = 0)]
    pub fn setup_characters(self) -> ();

    #[method(name = "LoadAnimset", args = 0)]
    pub fn load_animset(self) -> ();

    #[method(name = "IsAnimSetLoading", args = 0)]
    pub fn is_anim_set_loading(self) -> bool;

    #[method(name = "FadeInHUD", args = 0)]
    pub fn fade_in_hud(self) -> ();

    #[method(name = "FadeOutHUD", args = 0)]
    pub fn fade_out_hud(self) -> ();

    #[method(name = "InstantOutHUD", args = 0)]
    pub fn instant_out_hud(self) -> ();

    #[method(name = "ForceEnd", args = 0)]
    pub fn force_end(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "FinalizeWhenTransCamera", args = 0)]
    pub fn finalize_when_trans_camera(self) -> ();

    #[method(name = "CreateInstance", args = 1)]
    pub fn create_instance(style: crate::combat::combatstyle::CombatStyle) -> ();

    #[method(name = "DeleteInstance", args = 1)]
    pub fn delete_instance(style: crate::combat::combatstyle::CombatStyle) -> ();

    #[method(name = "CreateCombatEffectInstance仮", args = 0)]
    pub fn create_combat_effect_instance_() -> ();

    #[method(name = "DeleteCombatEffectInstance仮", args = 0)]
    pub fn delete_combat_effect_instance_() -> ();

    #[method(name = "createInstance", args = 1)]
    pub fn create_instance_2(name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-combatworld")]
impl CombatWorld {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatWorld),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatWorldMethods>::ctor(this);
        this
    }
}
