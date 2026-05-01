
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomwakeupsequence/MyRoomWakeupSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomWakeupSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomWakeupSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomWakeupSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomWakeupSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomWakeupSequence_Label {
    pub fn wakeup_main() -> Self {
        Self { value: 0 }
    }

    pub fn wakeup_exit() -> Self {
        Self { value: 1 }
    }

    pub fn skip_end_fade() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomwakeupsequence/MyRoomWakeupSequence.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomWakeupSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: myroomwakeupsequence :: MyRoomWakeupSequence >)]
pub struct MyRoomWakeupSequence {
    #[rename(name = "IsRecallSelect")]
    pub is_recall_select: bool,
    #[rename(name = "RecallPID")]
    pub recall_pid: ::unity2::Il2CppString,
    #[rename(name = "RecallLevel")]
    pub recall_level: crate::app::reliancedata::RelianceData_Level,
    #[rename(name = "RecallPattern")]
    pub recall_pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    #[rename(name = "m_wakeupUnit")]
    pub m_wakeup_unit: crate::app::unit::Unit,
    #[rename(name = "m_level")]
    pub m_level: crate::app::reliancedata::RelianceData_Level,
    #[rename(name = "m_pattern")]
    pub m_pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    #[rename(name = "m_pid")]
    pub m_pid: ::unity2::Il2CppString,
    #[rename(name = "m_eventRoot")]
    pub m_event_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_cameraRoot")]
    pub m_camera_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_cameraRootParent")]
    pub m_camera_root_parent: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_cameraData")]
    pub m_camera_data: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_cameraData2")]
    pub m_camera_data2: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_character")]
    pub m_character: crate::combat::character::Character,
    #[rename(name = "m_lookAt")]
    pub m_look_at: crate::app::hublookatcontroller::HubLookAtController,
    #[rename(name = "m_CharacterPosition")]
    pub m_character_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_CharacterRotation")]
    pub m_character_rotation: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_timelineObject")]
    pub m_timeline_object: crate::unity_engine::playables::playableasset::PlayableAsset,
    #[rename(name = "m_playableDirector")]
    pub m_playable_director: crate::unity_engine::playables::playabledirector::PlayableDirector,
    #[rename(name = "m_playableAssetHandle")]
    pub m_playable_asset_handle: crate::app::resourcehandle_2::ResourceHandle_2,
    #[rename(name = "m_aocHandle")]
    pub m_aoc_handle: ::unity2::Array<crate::app::resourcehandle_2::ResourceHandle_2>,
    #[rename(name = "m_effectHandle")]
    pub m_effect_handle: crate::system::collections::generic::list_1::List_1<
        crate::app::resourcehandle_2::ResourceHandle_2,
    >,
    #[rename(name = "m_aocList")]
    pub m_aoc_list: ::unity2::Array<
        crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
    >,
    #[rename(name = "m_currentAnimName")]
    pub m_current_anim_name: ::unity2::Il2CppString,
    #[rename(name = "m_eventVoiceList")]
    pub m_event_voice_list:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_eventCanvas")]
    pub m_event_canvas: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_eventCanvasForeground")]
    pub m_event_canvas_foreground: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_eventCanvasBackground")]
    pub m_event_canvas_background: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_caption")]
    pub m_caption: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_env")]
    pub m_env: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_isFastWakeup")]
    pub m_is_fast_wakeup: bool,
    #[rename(name = "m_isLoading")]
    pub m_is_loading: bool,
    #[rename(name = "m_captionText")]
    pub m_caption_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "FlagName")]
    pub flag_name: ::unity2::Il2CppString,
    #[rename(name = "eventWalkin")]
    pub event_walkin: ::unity2::Il2CppString,
    #[rename(name = "message")]
    pub message: crate::app::gamemessage::GameMessage,
}

#[cfg(feature = "app-myroomwakeupsequence")]
#[::unity2::methods]
impl MyRoomWakeupSequence {
    #[method(name = "get_VoiceID", args = 0)]
    pub fn get_voice_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Character", args = 0)]
    pub fn get_character(self) -> crate::combat::character::Character;

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> crate::app::reliancedata::RelianceData_Level;

    #[method(name = "get_Pattern", args = 0)]
    pub fn get_pattern(self) -> crate::app::gamesound::GameSound_WakeupVoicePattern;

    #[method(name = "get_FootstepID", args = 0)]
    pub fn get_footstep_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_FadeRender", args = 0)]
    pub fn get_fade_render(self) -> crate::app::myroomfaderender::MyRoomFadeRender;

    #[method(name = "get_EventCanvas", args = 0)]
    pub fn get_event_canvas(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_EventCanvasForeground", args = 0)]
    pub fn get_event_canvas_foreground(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_EventCanvasBackground", args = 0)]
    pub fn get_event_canvas_background(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_CameraRoot", args = 0)]
    pub fn get_camera_root(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_CameraRootParent", args = 0)]
    pub fn get_camera_root_parent(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_CameraData", args = 0)]
    pub fn get_camera_data(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_CameraData2", args = 0)]
    pub fn get_camera_data2(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "get_CharacterPosition", args = 0)]
    pub fn get_character_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_CharacterRotation", args = 0)]
    pub fn get_character_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "get_Time", args = 0)]
    pub fn get_time(self) -> f64;

    #[method(name = "get_PlayableAssetPath", args = 0)]
    pub fn get_playable_asset_path(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PlayableAssetPath", args = 1)]
    pub fn set_playable_asset_path(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EnterStartTime", args = 0)]
    pub fn get_enter_start_time(self) -> f64;

    #[method(name = "set_EnterStartTime", args = 1)]
    pub fn set_enter_start_time(self, value: f64) -> ();

    #[method(name = "get_EnterEndTime", args = 0)]
    pub fn get_enter_end_time(self) -> f64;

    #[method(name = "set_EnterEndTime", args = 1)]
    pub fn set_enter_end_time(self, value: f64) -> ();

    #[method(name = "get_WakeupStartTime", args = 0)]
    pub fn get_wakeup_start_time(self) -> f64;

    #[method(name = "set_WakeupStartTime", args = 1)]
    pub fn set_wakeup_start_time(self, value: f64) -> ();

    #[method(name = "get_WakeupEndTime", args = 0)]
    pub fn get_wakeup_end_time(self) -> f64;

    #[method(name = "set_WakeupEndTime", args = 1)]
    pub fn set_wakeup_end_time(self, value: f64) -> ();

    #[method(name = "get_BeforeStartTime", args = 0)]
    pub fn get_before_start_time(self) -> f64;

    #[method(name = "set_BeforeStartTime", args = 1)]
    pub fn set_before_start_time(self, value: f64) -> ();

    #[method(name = "get_BeforeEndTime", args = 0)]
    pub fn get_before_end_time(self) -> f64;

    #[method(name = "set_BeforeEndTime", args = 1)]
    pub fn set_before_end_time(self, value: f64) -> ();

    #[method(name = "get_AfterStartTime", args = 0)]
    pub fn get_after_start_time(self) -> f64;

    #[method(name = "set_AfterStartTime", args = 1)]
    pub fn set_after_start_time(self, value: f64) -> ();

    #[method(name = "get_AfterEndTime", args = 0)]
    pub fn get_after_end_time(self) -> f64;

    #[method(name = "set_AfterEndTime", args = 1)]
    pub fn set_after_end_time(self, value: f64) -> ();

    #[method(name = "AddEventVoice", args = 1)]
    pub fn add_event_voice(self, event_voice: ::unity2::Il2CppString) -> ();

    #[method(name = "RemoveEventVoice", args = 1)]
    pub fn remove_event_voice(self, event_voice: ::unity2::Il2CppString) -> ();

    #[method(name = "ContainsEventVoice", args = 1)]
    pub fn contains_event_voice(self, event_voice: ::unity2::Il2CppString) -> bool;

    #[method(name = "SwitchLookAtIK", args = 1)]
    pub fn switch_look_at_ik(self, looking_camera: bool) -> ();

    #[method(name = "SetCaption", args = 1)]
    pub fn set_caption(self, mid: ::unity2::Il2CppString) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindRecall", args = 4)]
    pub fn create_bind_recall(
        super_: crate::app::procinst::ProcInst,
        pid: ::unity2::Il2CppString,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> ();

    #[method(name = "GetBodyClip", args = 1)]
    pub fn get_body_clip(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "GetFaceClip", args = 1)]
    pub fn get_face_clip(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::animationclip::AnimationClip;

    #[method(name = "get_Player", args = 0)]
    pub fn get_player(self) -> crate::app::hubplayercontroller::HubPlayerController;

    #[method(name = "get_SaveZoomParam", args = 0)]
    pub fn get_save_zoom_param(self) -> f32;

    #[method(name = "set_SaveZoomParam", args = 1)]
    pub fn set_save_zoom_param(self, value: f32) -> ();

    #[method(name = "get_SaveTimeZone", args = 0)]
    pub fn get_save_time_zone(self) -> i32;

    #[method(name = "set_SaveTimeZone", args = 1)]
    pub fn set_save_time_zone(self, value: i32) -> ();

    #[method(name = "get_IsReturnEnv", args = 0)]
    pub fn get_is_return_env(self) -> bool;

    #[method(name = "set_IsReturnEnv", args = 1)]
    pub fn set_is_return_env(self, value: bool) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "SwitchEnv", args = 0)]
    pub fn switch_env(self) -> ();

    #[method(name = "ResetEnv", args = 0)]
    pub fn reset_env(self) -> ();

    #[method(name = "IsSelected", args = 0)]
    pub fn is_selected(self) -> bool;

    #[method(name = "ChangeEnv", args = 0)]
    pub fn change_env(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "IsWakeupIgnorePattern", args = 3)]
    pub fn is_wakeup_ignore_pattern(
        self,
        pid: ::unity2::Il2CppString,
        level: crate::app::reliancedata::RelianceData_Level,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    ) -> bool;

    #[method(name = "GetWakeupUnit", args = 1)]
    pub fn get_wakeup_unit(self, pid: ::unity2::Il2CppString) -> crate::app::unit::Unit;

    #[method(name = "SetAchiveDone", args = 3)]
    pub fn set_achive_done(
        self,
        pid: ::unity2::Il2CppString,
        level: crate::app::reliancedata::RelianceData_Level,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    ) -> ();

    #[method(name = "SelectRelianceWakeup", args = 0)]
    pub fn select_reliance_wakeup(self) -> bool;

    #[method(name = "InitWakeup", args = 0)]
    pub fn init_wakeup(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "GetCharacterAppearanceRemoveAcc", args = 1)]
    pub fn get_character_appearance_remove_acc(
        self,
        pid: ::unity2::Il2CppString,
    ) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "ProcessFrameFaceAnimation", args = 0)]
    pub fn process_frame_face_animation(self) -> ();

    #[method(name = "BeforeWakeupAction", args = 0)]
    pub fn before_wakeup_action(self) -> ();

    #[method(name = "WaitBeforeWakeupAction", args = 0)]
    pub fn wait_before_wakeup_action(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "StartWakeupTimeline", args = 0)]
    pub fn start_wakeup_timeline(self) -> ();

    #[method(name = "MainWakeupEnter", args = 0)]
    pub fn main_wakeup_enter(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "MainWakeup", args = 0)]
    pub fn main_wakeup(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "MainWakeupBeforeAfter", args = 0)]
    pub fn main_wakeup_before_after(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ExitWakeup", args = 0)]
    pub fn exit_wakeup(self) -> ();

    #[method(name = "ExitEnv", args = 0)]
    pub fn exit_env(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ExitWakeupAfter", args = 0)]
    pub fn exit_wakeup_after(self) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomwakeupsequence")]
impl MyRoomWakeupSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomWakeupSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomWakeupSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomwakeupsequence/MyRoomWakeupSequence_RelianceWakeup.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomWakeupSequence.RelianceWakeup")]
#[parent(crate::system::object::Object)]
pub struct MyRoomWakeupSequence_RelianceWakeup {
    #[rename(name = "m_pid")]
    pub m_pid: ::unity2::Il2CppString,
    #[rename(name = "m_level")]
    pub m_level: crate::app::reliancedata::RelianceData_Level,
    #[rename(name = "m_pattern")]
    pub m_pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
}

#[cfg(feature = "app-myroomwakeupsequence")]
#[::unity2::methods]
impl MyRoomWakeupSequence_RelianceWakeup {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        pid: ::unity2::Il2CppString,
        level: crate::app::reliancedata::RelianceData_Level,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-myroomwakeupsequence")]
impl MyRoomWakeupSequence_RelianceWakeup {
    pub fn new(
        pid: ::unity2::Il2CppString,
        level: crate::app::reliancedata::RelianceData_Level,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomWakeupSequence_RelianceWakeup),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomWakeupSequence_RelianceWakeupMethods>::ctor(this, pid, level, pattern);
        this
    }
}
