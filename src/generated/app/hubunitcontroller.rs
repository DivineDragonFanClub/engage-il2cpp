
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubunitcontroller/HubUnitController.md")))]
#[::unity2::class(namespace = "App", name = "HubUnitController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubUnitController {
    #[rename(name = "m_HomeDir")]
    pub m_home_dir: f32,
    #[rename(name = "m_HomePosition")]
    pub m_home_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_TalkHeadPosition")]
    pub m_talk_head_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_CharacterHeight")]
    pub m_character_height: f32,
    #[rename(name = "m_Dir")]
    pub m_dir: crate::app::interpolatorrotationcurve::InterpolatorRotationCurve,
    #[rename(name = "m_Move")]
    pub m_move: crate::app::interpolatorvector3::InterpolatorVector3,
    #[rename(name = "m_PlayerController")]
    pub m_player_controller: crate::app::hubplayercontroller::HubPlayerController,
    #[rename(name = "m_TalkCharacterController")]
    pub m_talk_character_controller:
        crate::app::talk3_d::talkcharactercontroller::TalkCharacterController,
    #[rename(name = "m_Access")]
    pub m_access: crate::app::hubaccess::HubAccess,
    #[rename(name = "m_Character")]
    pub m_character: crate::combat::character::Character,
    #[rename(name = "m_SoundAction")]
    pub m_sound_action: crate::app::hubrangeaction::HubRangeAction,
    #[rename(name = "m_LookAction")]
    pub m_look_action: crate::app::hubrangeaction::HubRangeAction,
    #[rename(name = "m_MoveController")]
    pub m_move_controller: crate::app::hubmovecontroller::HubMoveController,
    #[rename(name = "m_LookAt")]
    pub m_look_at: crate::app::hublookatcontroller::HubLookAtController,
    #[rename(name = "m_EnableFadeDelay")]
    pub m_enable_fade_delay: crate::unity_engine::coroutine::Coroutine,
    #[rename(name = "m_ReserveReset")]
    pub m_reserve_reset: bool,
    #[rename(name = "m_ReserveResetTime")]
    pub m_reserve_reset_time: f32,
    #[rename(name = "m_OutRangeTime")]
    pub m_out_range_time: f32,
    #[rename(name = "m_WaitAnimName")]
    pub m_wait_anim_name: ::unity2::Il2CppString,
    #[rename(name = "m_TalkDefaultAnimName")]
    pub m_talk_default_anim_name: ::unity2::Il2CppString,
    #[rename(name = "m_OldHeadPosition")]
    pub m_old_head_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_CenterPosition")]
    pub m_center_position: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "app-hubunitcontroller")]
#[::unity2::methods]
impl HubUnitController {
    #[method(name = "get_TurnBlendTime", args = 0)]
    pub fn get_turn_blend_time(self) -> f32;

    #[method(name = "get_IsStop", args = 0)]
    pub fn get_is_stop(self) -> bool;

    #[method(name = "set_IsStop", args = 1)]
    pub fn set_is_stop(self, value: bool) -> ();

    #[method(name = "get_IsPlayer", args = 0)]
    pub fn get_is_player(self) -> bool;

    #[method(name = "set_IsPlayer", args = 1)]
    pub fn set_is_player(self, value: bool) -> ();

    #[method(name = "get_IsMoveType", args = 0)]
    pub fn get_is_move_type(self) -> bool;

    #[method(name = "get_Character", args = 0)]
    pub fn get_character(self) -> crate::combat::character::Character;

    #[method(name = "get_TalkCharacter", args = 0)]
    pub fn get_talk_character(
        self,
    ) -> crate::app::talk3_d::talkcharactercontroller::TalkCharacterController;

    #[method(name = "get_Access", args = 0)]
    pub fn get_access(self) -> crate::app::hubaccess::HubAccess;

    #[method(name = "get_PID", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "get_VoiceID", args = 0)]
    pub fn get_voice_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Dir", args = 0)]
    pub fn get_dir(self) -> crate::app::interpolatorrotationcurve::InterpolatorRotationCurve;

    #[method(name = "get_LookAt", args = 0)]
    pub fn get_look_at(self) -> crate::app::hublookatcontroller::HubLookAtController;

    #[method(name = "get_LookAction", args = 0)]
    pub fn get_look_action(self) -> crate::app::hubrangeaction::HubRangeAction;

    #[method(name = "get_BindObject", args = 0)]
    pub fn get_bind_object(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >;

    #[method(name = "set_BindObject", args = 1)]
    pub fn set_bind_object(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::gameobject::GameObject,
        >,
    ) -> ();

    #[method(name = "get_IsLockDir", args = 0)]
    pub fn get_is_lock_dir(self) -> bool;

    #[method(name = "set_IsLockDir", args = 1)]
    pub fn set_is_lock_dir(self, value: bool) -> ();

    #[method(name = "IsTurnEnd", args = 1)]
    pub fn is_turn_end(self, margin_angle: f32) -> bool;

    #[method(name = "InPlayerRange", args = 1)]
    pub fn in_player_range(self, range: f32) -> bool;

    #[method(name = "SetPlayerController", args = 1)]
    pub fn set_player_controller(
        self,
        player_controller: crate::app::hubplayercontroller::HubPlayerController,
    ) -> ();

    #[method(name = "SetTalkCharaController", args = 1)]
    pub fn set_talk_chara_controller(
        self,
        talk_chara_controller : crate :: app :: talk3_d :: talkcharactercontroller :: TalkCharacterController,
    ) -> ();

    #[method(name = "SetCharacter", args = 1)]
    pub fn set_character(self, chara: crate::combat::character::Character) -> ();

    #[method(name = "SetAccess", args = 1)]
    pub fn set_access(self, access: crate::app::hubaccess::HubAccess) -> ();

    #[method(name = "SetDir", args = 1)]
    pub fn set_dir(self, dir: f32) -> ();

    #[method(name = "GetNowDir", args = 0)]
    pub fn get_now_dir(self) -> f32;

    #[method(name = "SetHomeDir", args = 1)]
    pub fn set_home_dir(self, home_dir: f32) -> ();

    #[method(name = "GetHomeDir", args = 0)]
    pub fn get_home_dir(self) -> f32;

    #[method(name = "SetHomePosition", args = 1)]
    pub fn set_home_position(self, home_position: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "GetHomePosition", args = 0)]
    pub fn get_home_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Stop", args = 0)]
    pub fn stop(self) -> ();

    #[method(name = "Restart", args = 0)]
    pub fn restart(self) -> ();

    #[method(name = "SetActiveBind", args = 1)]
    pub fn set_active_bind(self, active: bool) -> ();

    #[method(name = "LookTo", args = 3)]
    pub fn look_to(
        self,
        target: crate::unity_engine::vector3::Vector3,
        time: f32,
        with_turn_anim: bool,
    ) -> ();

    #[method(name = "LookTo", args = 3)]
    pub fn look_to_2(self, dir: f32, time: f32, with_turn_anim: bool) -> bool;

    #[method(name = "ResetLook", args = 1)]
    pub fn reset_look(self, time: f32) -> ();

    #[method(name = "ReserveReset", args = 1)]
    pub fn reserve_reset(self, time: f32) -> ();

    #[method(name = "CancelReserveReset", args = 0)]
    pub fn cancel_reserve_reset(self) -> ();

    #[method(name = "PlayTurn", args = 0)]
    pub fn play_turn(self) -> bool;

    #[method(name = "MoveTo", args = 2)]
    pub fn move_to(self, pos: crate::unity_engine::vector3::Vector3, time: f32) -> ();

    #[method(name = "ResetAnimatoin", args = 0)]
    pub fn reset_animatoin(self) -> ();

    #[method(name = "GetWaitAnimationName", args = 0)]
    pub fn get_wait_animation_name(self) -> ::unity2::Il2CppString;

    #[method(name = "SetWaitAnimationName", args = 1)]
    pub fn set_wait_animation_name(self, anim_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetWaitFaceAnimationName", args = 0)]
    pub fn get_wait_face_animation_name(self) -> ::unity2::Il2CppString;

    #[method(name = "WaitAnimation", args = 1)]
    pub fn wait_animation(self, transition_duration: f32) -> ();

    #[method(name = "GetHeadTransform", args = 0)]
    pub fn get_head_transform(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "GetTalkHeadPosition", args = 0)]
    pub fn get_talk_head_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "PlayVoice", args = 1)]
    pub fn play_voice(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetPlayerDistance", args = 0)]
    pub fn get_player_distance(self) -> f32;

    #[method(name = "GetPlayerDirection", args = 0)]
    pub fn get_player_direction(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "StartTalk", args = 1)]
    pub fn start_talk(self, target: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "EndTalk", args = 0)]
    pub fn end_talk(self) -> ();

    #[method(name = "DisableFade", args = 0)]
    pub fn disable_fade(self) -> ();

    #[method(name = "EnableFade", args = 0)]
    pub fn enable_fade(self) -> ();

    #[method(name = "EnableFadeDelay", args = 0)]
    pub fn enable_fade_delay(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "PlayBody", args = 4)]
    pub fn play_body(
        self,
        name: ::unity2::Il2CppString,
        transition_duration: f32,
        soon: bool,
        force: bool,
    ) -> ();

    #[method(name = "PlayFace", args = 2)]
    pub fn play_face(self, name: ::unity2::Il2CppString, is_fast_forward: bool) -> ();

    #[method(name = "ResetBody", args = 0)]
    pub fn reset_body(self) -> ();

    #[method(name = "IsEnableVoice", args = 0)]
    pub fn is_enable_voice(self) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "OnDrawGizmos", args = 0)]
    pub fn on_draw_gizmos(self) -> ();

    #[method(name = "ResetLookAt", args = 0)]
    pub fn reset_look_at(self) -> ();

    #[method(name = "SetupIdle", args = 0)]
    pub fn setup_idle(self) -> ();

    #[method(name = "UpdateRange", args = 0)]
    pub fn update_range(self) -> ();

    #[method(name = "UpdateMove", args = 1)]
    pub fn update_move(self, force: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubunitcontroller")]
impl HubUnitController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubUnitController),
                ::core::stringify!(new),
            )
        });
        <Self as IHubUnitControllerMethods>::ctor(this);
        this
    }
}
