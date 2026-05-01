
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/character/Character.md")))]
#[::unity2::class(namespace = "Combat", name = "Character")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct Character {
    #[rename(name = "_observable")]
    pub observable: crate::combat::characterobservable::CharacterObservable,
    #[rename(name = "FSM")]
    pub fsm: crate::combat::fsm::FSM,
    #[rename(name = "m_Brain")]
    pub m_brain: crate::combat::grandewbrain::GrandewBrain,
    #[rename(name = "_gs")]
    pub gs: crate::combat::charactergamestatus::CharacterGameStatus,
    #[rename(name = "m_EnemySide")]
    pub m_enemy_side: i32,
    #[rename(name = "rushDir")]
    pub rush_dir: crate::combat::fxz::FXZ,
    #[rename(name = "_bodyAnimator")]
    pub body_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "_rideAnimator")]
    pub ride_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "_faceAnimator")]
    pub face_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "Play直後Idle誤判断防止フレーム数")]
    pub play___idle__________: i32,
    #[rename(name = "同じアニメ再生時の前回の再生時刻")]
    pub _unnamed: f32,
    #[rename(name = "m_PlayEndWorldPos")]
    pub m_play_end_world_pos: crate::combat::fxz::FXZ,
    #[rename(name = "cached_DitherFade")]
    pub cached_dither_fade: bool,
    #[rename(name = "cached_MaterialEngage")]
    pub cached_material_engage: bool,
    #[rename(name = "_Signal")]
    pub signal: crate::combat::charactersignal::CharacterSignal,
    #[rename(name = "_Lying")]
    pub lying: crate::combat::characterlying::CharacterLying,
    #[rename(name = "_Joint")]
    pub joint: crate::combat::characterjoint::CharacterJoint,
    #[rename(name = "cached_Joint")]
    pub cached_joint: bool,
    #[rename(name = "_Proportion")]
    pub proportion: crate::combat::characterproportion::CharacterProportion,
    #[rename(name = "cached_Proportion")]
    pub cached_proportion: bool,
    #[rename(name = "_Config")]
    pub config: crate::combat::characterconfig::CharacterConfig,
    #[rename(name = "cached_Config")]
    pub cached_config: bool,
    #[rename(name = "_Move")]
    pub r#move: crate::combat::charactermove::CharacterMove,
    #[rename(name = "cached_Move")]
    pub cached_move: bool,
    #[rename(name = "_Turn")]
    pub turn: crate::combat::characterturn::CharacterTurn,
    #[rename(name = "cached_Turn")]
    pub cached_turn: bool,
    #[rename(name = "_Timespace")]
    pub timespace: crate::combat::charactertimespace::CharacterTimespace,
    #[rename(name = "cached_Timespace")]
    pub cached_timespace: bool,
    #[rename(name = "_HUD")]
    pub hud: crate::combat::characterhud::CharacterHUD,
    #[rename(name = "cached_HUD")]
    pub cached_hud: bool,
    #[rename(name = "_Sound")]
    pub sound: crate::combat::charactersound::CharacterSound,
    #[rename(name = "cached_Sound")]
    pub cached_sound: bool,
    #[rename(name = "_Builder")]
    pub builder: crate::combat::characterbuilder::CharacterBuilder,
    #[rename(name = "cached_Builder")]
    pub cached_builder: bool,
    #[rename(name = "_Aura")]
    pub aura: crate::combat::characteraura::CharacterAura,
    #[rename(name = "cached_Aura")]
    pub cached_aura: bool,
    #[rename(name = "_Illusion")]
    pub illusion: crate::combat::characterillusion::CharacterIllusion,
    #[rename(name = "cached_Illusion")]
    pub cached_illusion: bool,
    #[rename(name = "_IKFoot")]
    pub ik_foot: crate::combat::characterikfoot::CharacterIKFoot,
    #[rename(name = "cached_IKFoot")]
    pub cached_ik_foot: bool,
    #[rename(name = "_IKLookAt")]
    pub ik_look_at: crate::combat::characteriklookat::CharacterIKLookAt,
    #[rename(name = "cached_IKLookAt")]
    pub cached_ik_look_at: bool,
    #[rename(name = "_IKAim")]
    pub ik_aim: crate::combat::characterikaim::CharacterIKAim,
    #[rename(name = "cached_IKAim")]
    pub cached_ik_aim: bool,
    #[rename(name = "_Weapon")]
    pub weapon: crate::combat::characterweapon::CharacterWeapon,
    #[rename(name = "cached_Weapon")]
    pub cached_weapon: bool,
    #[rename(name = "_SignalObserver")]
    pub signal_observer: crate::combat::charactersignalobserver::CharacterSignalObserver,
    #[rename(name = "cached_SignalObserver")]
    pub cached_signal_observer: bool,
}

#[cfg(feature = "combat-character")]
#[::unity2::methods]
impl Character {
    #[method(name = "get_Side", args = 0)]
    pub fn get_side(self) -> i32;

    #[method(name = "set_Side", args = 1)]
    pub fn set_side(self, value: i32) -> ();

    #[method(name = "get_ChainId", args = 0)]
    pub fn get_chain_id(self) -> i32;

    #[method(name = "set_ChainId", args = 1)]
    pub fn set_chain_id(self, value: i32) -> ();

    #[method(name = "get_Prefetched", args = 0)]
    pub fn get_prefetched(self) -> crate::combat::prefetchedsignalstore::PrefetchedSignalStore;

    #[method(name = "set_Prefetched", args = 1)]
    pub fn set_prefetched(
        self,
        value: crate::combat::prefetchedsignalstore::PrefetchedSignalStore,
    ) -> ();

    #[method(name = "get_Launcher", args = 0)]
    pub fn get_launcher(self) -> crate::combat::launchbehaviour::LaunchBehaviour;

    #[method(name = "get_Magic", args = 0)]
    pub fn get_magic(self) -> crate::combat::magic::Magic;

    #[method(name = "get_Throwing", args = 0)]
    pub fn get_throwing(self) -> crate::combat::throwing::Throwing;

    #[method(name = "get_Effect", args = 0)]
    pub fn get_effect(self) -> crate::combat::charactereffect::CharacterEffect;

    #[method(name = "set_Effect", args = 1)]
    pub fn set_effect(self, value: crate::combat::charactereffect::CharacterEffect) -> ();

    #[method(name = "get_Observable", args = 0)]
    pub fn get_observable(self) -> crate::combat::characterobservable::CharacterObservable;

    #[method(name = "get_IdleSMB", args = 0)]
    pub fn get_idle_smb(self) -> crate::combat::characteridlesmb::CharacterIdleSMB;

    #[method(name = "set_IdleSMB", args = 1)]
    pub fn set_idle_smb(self, value: crate::combat::characteridlesmb::CharacterIdleSMB) -> ();

    #[method(name = "StartGrandewBrain", args = 0)]
    pub fn start_grandew_brain(self) -> ();

    #[method(name = "DisposeGrandewBrain", args = 0)]
    pub fn dispose_grandew_brain(self) -> ();

    #[method(name = "get_World", args = 0)]
    pub fn get_world(self) -> crate::combat::combatworld::CombatWorld;

    #[method(name = "get_Record", args = 0)]
    pub fn get_record(self) -> crate::combat::combatrecord::CombatRecord;

    #[method(name = "get_Phase", args = 0)]
    pub fn get_phase(self) -> crate::combat::phase::Phase;

    #[method(name = "get_GameStatus", args = 0)]
    pub fn get_game_status(self) -> crate::combat::charactergamestatus::CharacterGameStatus;

    #[method(name = "set_GameStatus", args = 1)]
    pub fn set_game_status(
        self,
        value: crate::combat::charactergamestatus::CharacterGameStatus,
    ) -> ();

    #[method(name = "get_IsSetupDone", args = 0)]
    pub fn get_is_setup_done(self) -> bool;

    #[method(name = "set_IsSetupDone", args = 1)]
    pub fn set_is_setup_done(self, value: bool) -> ();

    #[method(name = "get_IsVisible", args = 0)]
    pub fn get_is_visible(self) -> bool;

    #[method(name = "set_IsVisible", args = 1)]
    pub fn set_is_visible(self, value: bool) -> ();

    #[method(name = "get_Grandew", args = 0)]
    pub fn get_grandew(self) -> crate::combat::character::Character;

    #[method(name = "get_Master", args = 0)]
    pub fn get_master(self) -> crate::combat::character::Character;

    #[method(name = "get_EnemyGrandew", args = 0)]
    pub fn get_enemy_grandew(self) -> crate::combat::character::Character;

    #[method(name = "get_Enemy", args = 0)]
    pub fn get_enemy(self) -> crate::combat::character::Character;

    #[method(name = "SetEnemyForChainGuard", args = 1)]
    pub fn set_enemy_for_chain_guard(self, c: crate::combat::character::Character) -> ();

    #[method(name = "SetEnemyToDefault", args = 0)]
    pub fn set_enemy_to_default(self) -> ();

    #[method(name = "get_GroundLevel", args = 0)]
    pub fn get_ground_level(self) -> f32;

    #[method(name = "set_GroundLevel", args = 1)]
    pub fn set_ground_level(self, value: f32) -> ();

    #[method(name = "AdjustGroundLevelTo", args = 1)]
    pub fn adjust_ground_level_to(self, target_side: i32) -> ();

    #[method(name = "get_Pos2D", args = 0)]
    pub fn get_pos2_d(self) -> crate::combat::fxz::FXZ;

    #[method(name = "get_Pos3D", args = 0)]
    pub fn get_pos3_d(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ToXHZ", args = 1)]
    pub fn to_xhz(
        self,
        pos: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ToABCD", args = 1)]
    pub fn to_abcd(
        self,
        pos: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "TransformToWorld", args = 1)]
    pub fn transform_to_world(
        self,
        local_offset: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_Dir2D", args = 0)]
    pub fn get_dir2_d(self) -> crate::combat::fxz::FXZ;

    #[method(name = "get_DirToEnemy2D", args = 0)]
    pub fn get_dir_to_enemy2_d(self) -> crate::combat::fxz::FXZ;

    #[method(name = "get_SqDistToEnemy2D", args = 0)]
    pub fn get_sq_dist_to_enemy2_d(self) -> f32;

    #[method(name = "get_SqDistToPartner2D", args = 0)]
    pub fn get_sq_dist_to_partner2_d(self) -> f32;

    #[method(name = "get_DistToEnemy2D", args = 0)]
    pub fn get_dist_to_enemy2_d(self) -> f32;

    #[method(name = "get_WorldHitDir", args = 0)]
    pub fn get_world_hit_dir(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_WorldHitDir", args = 1)]
    pub fn set_world_hit_dir(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_AttackBeatenDir", args = 0)]
    pub fn get_attack_beaten_dir(self) -> crate::combat::fxz::FXZ;

    #[method(name = "set_AttackBeatenDir", args = 1)]
    pub fn set_attack_beaten_dir(self, value: crate::combat::fxz::FXZ) -> ();

    #[method(name = "GetTall", args = 0)]
    pub fn get_tall(self) -> f32;

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "MyUpdate", args = 0)]
    pub fn my_update(self) -> ();

    #[method(name = "IsAppearedFromBeginning", args = 0)]
    pub fn is_appeared_from_beginning(self) -> bool;

    #[method(name = "ManualAwakeForCombat", args = 2)]
    pub fn manual_awake_for_combat(self, my_side: i32, chain_id: i32) -> ();

    #[method(name = "SetupForCombat", args = 1)]
    pub fn setup_for_combat(self, my_side: i32) -> ();

    #[method(name = "SetupByRace", args = 0)]
    pub fn setup_by_race(self) -> ();

    #[method(name = "SetCollisionOffset", args = 3)]
    pub fn set_collision_offset(
        self,
        name: ::unity2::Il2CppString,
        move_offset: f32,
        size_offset: f32,
    ) -> ();

    #[method(name = "SetCollisionOffset", args = 3)]
    pub fn set_collision_offset_2(self, scale: f32, move_offset: f32, size_offset: f32) -> ();

    #[method(name = "AddAndGetRBAndCollider", args = 1)]
    pub fn add_and_get_rb_and_collider(self, t: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "SetupForGrandew", args = 0)]
    pub fn setup_for_grandew(self) -> ();

    #[method(name = "HideWholeShadow", args = 0)]
    pub fn hide_whole_shadow(self) -> ();

    #[method(name = "TeardownForCombat", args = 0)]
    pub fn teardown_for_combat(self) -> ();

    #[method(name = "SetupForTalk", args = 0)]
    pub fn setup_for_talk(self) -> ();

    #[method(name = "SetupForHub", args = 0)]
    pub fn setup_for_hub(self) -> ();

    #[method(name = "get_IsLeftInScreenSpace", args = 0)]
    pub fn get_is_left_in_screen_space(self) -> bool;

    #[method(name = "get_BodyAnimator", args = 0)]
    pub fn get_body_animator(self) -> crate::unity_engine::animator::Animator;

    #[method(name = "get_RideAnimator", args = 0)]
    pub fn get_ride_animator(self) -> crate::unity_engine::animator::Animator;

    #[method(name = "get_FaceAnimator", args = 0)]
    pub fn get_face_animator(self) -> crate::unity_engine::animator::Animator;

    #[method(name = "get_ConstantSpeedPlayback", args = 0)]
    pub fn get_constant_speed_playback(self) -> bool;

    #[method(name = "set_ConstantSpeedPlayback", args = 1)]
    pub fn set_constant_speed_playback(self, value: bool) -> ();

    #[method(name = "get_PlayingHash", args = 0)]
    pub fn get_playing_hash(self) -> i32;

    #[method(name = "set_PlayingHash", args = 1)]
    pub fn set_playing_hash(self, value: i32) -> ();

    #[method(name = "get_PlayingStore", args = 0)]
    pub fn get_playing_store(self) -> crate::combat::prefetchedsignal::PrefetchedSignal;

    #[method(name = "set_PlayingStore", args = 1)]
    pub fn set_playing_store(self, value: crate::combat::prefetchedsignal::PrefetchedSignal) -> ();

    #[method(name = "SetPlayingHashAndStore", args = 1)]
    pub fn set_playing_hash_and_store(self, hash: i32) -> ();

    #[method(name = "get_PlayingEvent", args = 0)]
    pub fn get_playing_event(self) -> crate::unity_engine::animationevent::AnimationEvent;

    #[method(name = "set_PlayingEvent", args = 1)]
    pub fn set_playing_event(
        self,
        value: crate::unity_engine::animationevent::AnimationEvent,
    ) -> ();

    #[method(name = "get_PlayEndWorldPos", args = 0)]
    pub fn get_play_end_world_pos(self) -> crate::combat::fxz::FXZ;

    #[method(name = "SetAnimatorParameter", args = 2)]
    pub fn set_animator_parameter(self, parameter_hash: i32, value: f32) -> ();

    #[method(name = "SetAnimatorParameter", args = 2)]
    pub fn set_animator_parameter_2(self, parameter_hash: i32, value: bool) -> ();

    #[method(name = "SetPlayingAnimationTime", args = 1)]
    pub fn set_playing_animation_time(self, fixed_time: f32) -> ();

    #[method(name = "GetAnimatorStateInfo", args = 0)]
    pub fn get_animator_state_info(
        self,
    ) -> crate::unity_engine::animatorstateinfo::AnimatorStateInfo;

    #[method(name = "GetPlayingTime", args = 0)]
    pub fn get_playing_time(self) -> f32;

    #[method(name = "GetPlaybackRate", args = 0)]
    pub fn get_playback_rate(self) -> f32;

    #[method(name = "SetPlaybackRate", args = 1)]
    pub fn set_playback_rate(self, value: f32) -> ();

    #[method(name = "get_WeaponStyle", args = 0)]
    pub fn get_weapon_style(self) -> crate::combat::weaponstyle::WeaponStyle;

    #[method(name = "WillPlay", args = 1)]
    pub fn will_play(self, hash: i32) -> ();

    #[method(name = "WillNotPlay", args = 1)]
    pub fn will_not_play(self, hash: i32) -> ();

    #[method(name = "PlayDyingInstant", args = 0)]
    pub fn play_dying_instant(self) -> ();

    #[method(name = "PlayIdleNormalInstant", args = 0)]
    pub fn play_idle_normal_instant(self) -> ();

    #[method(name = "Play", args = 4)]
    pub fn play(
        self,
        target_name: ::unity2::Il2CppString,
        flags: crate::combat::playflags::PlayFlags,
        transition_duration: f32,
        start_time: f32,
    ) -> f32;

    #[method(name = "PlayInstant", args = 1)]
    pub fn play_instant(self, hash: i32) -> ();

    #[method(name = "Play", args = 4)]
    pub fn play_2(
        self,
        hash: i32,
        flags: crate::combat::playflags::PlayFlags,
        transition_duration: f32,
        start_time: f32,
    ) -> f32;

    #[method(name = "get_IsIdle", args = 0)]
    pub fn get_is_idle(self) -> bool;

    #[method(name = "get_IsPhysicalAttacking", args = 0)]
    pub fn get_is_physical_attacking(self) -> bool;

    #[method(name = "PlayFacial", args = 1)]
    pub fn play_facial(self, state_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PlayFacial", args = 2)]
    pub fn play_facial_2(self, state_hash: i32, fixed_transition_duration: f32) -> ();

    #[method(name = "SkipCombat", args = 0)]
    pub fn skip_combat(self) -> bool;

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "get_Signal", args = 0)]
    pub fn get_signal(self) -> crate::combat::charactersignal::CharacterSignal;

    #[method(name = "get_Lying", args = 0)]
    pub fn get_lying(self) -> crate::combat::characterlying::CharacterLying;

    #[method(name = "get_Joint", args = 0)]
    pub fn get_joint(self) -> crate::combat::characterjoint::CharacterJoint;

    #[method(name = "get_Proportion", args = 0)]
    pub fn get_proportion(self) -> crate::combat::characterproportion::CharacterProportion;

    #[method(name = "get_Config", args = 0)]
    pub fn get_config(self) -> crate::combat::characterconfig::CharacterConfig;

    #[method(name = "get_Move", args = 0)]
    pub fn get_move(self) -> crate::combat::charactermove::CharacterMove;

    #[method(name = "get_Turn", args = 0)]
    pub fn get_turn(self) -> crate::combat::characterturn::CharacterTurn;

    #[method(name = "get_Timespace", args = 0)]
    pub fn get_timespace(self) -> crate::combat::charactertimespace::CharacterTimespace;

    #[method(name = "get_HUD", args = 0)]
    pub fn get_hud(self) -> crate::combat::characterhud::CharacterHUD;

    #[method(name = "get_Sound", args = 0)]
    pub fn get_sound(self) -> crate::combat::charactersound::CharacterSound;

    #[method(name = "get_Builder", args = 0)]
    pub fn get_builder(self) -> crate::combat::characterbuilder::CharacterBuilder;

    #[method(name = "get_Aura", args = 0)]
    pub fn get_aura(self) -> crate::combat::characteraura::CharacterAura;

    #[method(name = "get_Illusion", args = 0)]
    pub fn get_illusion(self) -> crate::combat::characterillusion::CharacterIllusion;

    #[method(name = "get_IKFoot", args = 0)]
    pub fn get_ik_foot(self) -> crate::combat::characterikfoot::CharacterIKFoot;

    #[method(name = "get_IKLookAt", args = 0)]
    pub fn get_ik_look_at(self) -> crate::combat::characteriklookat::CharacterIKLookAt;

    #[method(name = "get_IKAim", args = 0)]
    pub fn get_ik_aim(self) -> crate::combat::characterikaim::CharacterIKAim;

    #[method(name = "get_Weapon", args = 0)]
    pub fn get_weapon(self) -> crate::combat::characterweapon::CharacterWeapon;

    #[method(name = "get_SignalObserver", args = 0)]
    pub fn get_signal_observer(
        self,
    ) -> crate::combat::charactersignalobserver::CharacterSignalObserver;

    #[method(name = "CallOnSetupDone", args = 1)]
    pub fn call_on_setup_done(self, func: crate::system::action::Action) -> ();

    #[method(name = "CallOnSetupDone", args = 4)]
    pub fn call_on_setup_done_2(
        self,
        component: crate::unity_engine::monobehaviour::MonoBehaviour,
        my_start: crate::system::action::Action,
        my_update: crate::system::action::Action,
        my_late_update: crate::system::action::Action,
    ) -> ();

    #[method(name = "registerRegularHanderOnSetupDone", args = 4)]
    pub fn register_regular_hander_on_setup_done(
        component: crate::unity_engine::monobehaviour::MonoBehaviour,
        my_start: crate::system::action::Action,
        my_update: crate::system::action::Action,
        my_late_update: crate::system::action::Action,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-character")]
impl Character {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Character),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterMethods>::ctor(this);
        this
    }
}
