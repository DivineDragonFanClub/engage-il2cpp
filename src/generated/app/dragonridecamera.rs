
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridecamera/DragonRideCamera.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideCamera")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DragonRideCamera {
    #[static_field]
    #[rename(name = "cShotPrefabPath")]
    pub c_shot_prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "InstructionPrefabPath")]
    pub instruction_prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SolaRenderPath")]
    pub sola_render_path: ::unity2::Il2CppString,
    #[rename(name = "m_ShotIntervalTimer")]
    pub m_shot_interval_timer: f32,
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PlayerChara")]
    pub m_player_chara: crate::combat::character::Character,
    #[rename(name = "m_DragonReins")]
    pub m_dragon_reins: crate::combat::reinscontroller::ReinsController,
    #[rename(name = "m_AssistChara")]
    pub m_assist_chara: crate::combat::character::Character,
    #[rename(name = "m_ScreenDust")]
    pub m_screen_dust: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DustEffect")]
    pub m_dust_effect: crate::unity_engine::particlesystem::ParticleSystem,
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Cart")]
    pub m_cart: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_EventCart")]
    pub m_event_cart: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Timeline")]
    pub m_timeline: crate::unity_engine::playables::playabledirector::PlayableDirector,
    #[rename(name = "m_CursorObj")]
    pub m_cursor_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_StartTelop")]
    pub m_start_telop: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FinishTelop")]
    pub m_finish_telop: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IsPlayStartTelop")]
    pub m_is_play_start_telop: bool,
    #[rename(name = "m_IsPlayFinishTelop")]
    pub m_is_play_finish_telop: bool,
    #[rename(name = "m_IsDoneStartTelop")]
    pub m_is_done_start_telop: bool,
    #[rename(name = "m_IsDoneFinishTelop")]
    pub m_is_done_finish_telop: bool,
    #[rename(name = "m_ShotParent")]
    pub m_shot_parent: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PastPosition")]
    pub m_past_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_LevelStr")]
    pub m_level_str: ::unity2::Il2CppString,
    #[rename(name = "m_CourseID")]
    pub m_course_id: ::unity2::Il2CppString,
    #[rename(name = "m_ShowGroupList")]
    pub m_show_group_list: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "cAssistLevelMax")]
    pub c_assist_level_max: i32,
    #[rename(name = "m_AngleX")]
    pub m_angle_x: f32,
    #[rename(name = "m_AngleY")]
    pub m_angle_y: f32,
    #[rename(name = "m_AngleXSpeed")]
    pub m_angle_x_speed: f32,
    #[rename(name = "m_AngleYSpeed")]
    pub m_angle_y_speed: f32,
    #[rename(name = "m_AngleSpeedRate")]
    pub m_angle_speed_rate: f32,
    #[rename(name = "m_AngleSpeedLerpCounter")]
    pub m_angle_speed_lerp_counter: f32,
    #[static_field]
    #[rename(name = "cAngleSpeedRateMin")]
    pub c_angle_speed_rate_min: f32,
    #[static_field]
    #[rename(name = "cAngleSpeedRateMax")]
    pub c_angle_speed_rate_max: f32,
    #[rename(name = "m_SolaRenderObj")]
    pub m_sola_render_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SolaImagerRender")]
    pub m_sola_imager_render: crate::app::dragonridesolaimagerender::DragonRideSolaImageRender,
    #[rename(name = "m_Cutin")]
    pub m_cutin: crate::unity_engine::animator::Animator,
    #[rename(name = "m_Cutin2")]
    pub m_cutin2: crate::unity_engine::animator::Animator,
    #[rename(name = "m_CutinSate")]
    pub m_cutin_sate: crate::app::dragonridecamera::DragonRideCamera_CutinState,
    #[rename(name = "m_CutinTimer")]
    pub m_cutin_timer: f32,
    #[rename(name = "m_SpecialTimer")]
    pub m_special_timer: f32,
    #[rename(name = "m_InstructionObj")]
    pub m_instruction_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ShotNamingCounter")]
    pub m_shot_naming_counter: i32,
    #[rename(name = "m_AssistNamingCounter")]
    pub m_assist_naming_counter: i32,
    #[rename(name = "m_Config")]
    pub m_config: crate::app::dragonrideconfig::DragonRideConfig,
    #[rename(name = "m_FaderScript")]
    pub m_fader_script: crate::app::dragonrideeventfader::DragonRideEventFader,
    #[rename(name = "m_AssistIntervalTimer")]
    pub m_assist_interval_timer: f32,
    #[rename(name = "cAssistIntervalFrame")]
    pub c_assist_interval_frame: ::unity2::Array<i32>,
    #[rename(name = "cLevelString")]
    pub c_level_string: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_RotateSpeedXZAxis")]
    pub m_rotate_speed_xz_axis: f32,
    #[rename(name = "m_RotateSpeedYAxis")]
    pub m_rotate_speed_y_axis: f32,
    #[rename(name = "m_RotateHighSpeedMult")]
    pub m_rotate_high_speed_mult: f32,
    #[rename(name = "m_XZAxisRotateMax")]
    pub m_xz_axis_rotate_max: f32,
    #[rename(name = "m_XZAxisRotateMin")]
    pub m_xz_axis_rotate_min: f32,
    #[rename(name = "m_YAxisRotateMax")]
    pub m_y_axis_rotate_max: f32,
    #[rename(name = "m_YAxisRotateMin")]
    pub m_y_axis_rotate_min: f32,
    #[rename(name = "m_ShotAdjustX")]
    pub m_shot_adjust_x: f32,
    #[rename(name = "m_ShotAdjustY")]
    pub m_shot_adjust_y: f32,
    #[rename(name = "m_AssistShotAngle")]
    pub m_assist_shot_angle: f32,
    #[rename(name = "m_AssistCutInRate")]
    pub m_assist_cut_in_rate: f32,
    #[static_field]
    #[rename(name = "cAssistAngleMax")]
    pub c_assist_angle_max: f32,
    #[rename(name = "m_ShotSpeed")]
    pub m_shot_speed: f32,
    #[rename(name = "m_AssistShotSpeed")]
    pub m_assist_shot_speed: f32,
    #[rename(name = "m_ShotLifeSecond")]
    pub m_shot_life_second: f32,
    #[rename(name = "m_ShotInterpStraightSec")]
    pub m_shot_interp_straight_sec: f32,
    #[rename(name = "m_MuzzleOffsetX")]
    pub m_muzzle_offset_x: f32,
    #[rename(name = "m_MuzzleOffsetY")]
    pub m_muzzle_offset_y: f32,
    #[rename(name = "m_MuzzleOffsetZ")]
    pub m_muzzle_offset_z: f32,
    #[rename(name = "m_TargetDistanceMax")]
    pub m_target_distance_max: f32,
    #[rename(name = "m_CursorMoveHalfWidth")]
    pub m_cursor_move_half_width: f32,
    #[rename(name = "m_CursorMoveHalfHeight")]
    pub m_cursor_move_half_height: f32,
    #[rename(name = "m_CursorMaxSpeed")]
    pub m_cursor_max_speed: f32,
    #[rename(name = "m_BaseAngleX")]
    pub m_base_angle_x: f32,
    #[rename(name = "m_BaseAngleZ")]
    pub m_base_angle_z: f32,
    #[rename(name = "m_DeviceType")]
    pub m_device_type: crate::app::gyromnager::GyroMnager_DeviceType,
}

#[cfg(feature = "app-dragonridecamera")]
#[::unity2::methods]
impl DragonRideCamera {
    #[method(name = "get_IsReadyPlayerModel", args = 0)]
    pub fn get_is_ready_player_model(self) -> bool;

    #[method(name = "set_IsReadyPlayerModel", args = 1)]
    pub fn set_is_ready_player_model(self, value: bool) -> ();

    #[method(name = "get_IsReadyAssistModel", args = 0)]
    pub fn get_is_ready_assist_model(self) -> bool;

    #[method(name = "set_IsReadyAssistModel", args = 1)]
    pub fn set_is_ready_assist_model(self, value: bool) -> ();

    #[method(name = "get_CameraRoateteParamName", args = 0)]
    pub fn get_camera_roatete_param_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_IsSpecialTime", args = 0)]
    pub fn get_is_special_time(self) -> bool;

    #[method(name = "set_IsSpecialTime", args = 1)]
    pub fn set_is_special_time(self, value: bool) -> ();

    #[method(name = "get_IsRailFinished", args = 0)]
    pub fn get_is_rail_finished(self) -> bool;

    #[method(name = "set_IsRailFinished", args = 1)]
    pub fn set_is_rail_finished(self, value: bool) -> ();

    #[method(name = "get_IsFirstCameraEventFinish", args = 0)]
    pub fn get_is_first_camera_event_finish(self) -> bool;

    #[method(name = "set_IsFirstCameraEventFinish", args = 1)]
    pub fn set_is_first_camera_event_finish(self, value: bool) -> ();

    #[method(name = "get_IsEventCamera", args = 0)]
    pub fn get_is_event_camera(self) -> bool;

    #[method(name = "set_IsEventCamera", args = 1)]
    pub fn set_is_event_camera(self, value: bool) -> ();

    #[method(name = "get_IsReadyEventCamera", args = 0)]
    pub fn get_is_ready_event_camera(self) -> bool;

    #[method(name = "set_IsReadyEventCamera", args = 1)]
    pub fn set_is_ready_event_camera(self, value: bool) -> ();

    #[method(name = "get_LastEventTime", args = 0)]
    pub fn get_last_event_time(self) -> f64;

    #[method(name = "set_LastEventTime", args = 1)]
    pub fn set_last_event_time(self, value: f64) -> ();

    #[method(name = "get_LastControleTargetTime", args = 0)]
    pub fn get_last_controle_target_time(self) -> f64;

    #[method(name = "set_LastControleTargetTime", args = 1)]
    pub fn set_last_controle_target_time(self, value: f64) -> ();

    #[method(name = "get_LastFadeMarkerTime", args = 0)]
    pub fn get_last_fade_marker_time(self) -> f64;

    #[method(name = "set_LastFadeMarkerTime", args = 1)]
    pub fn set_last_fade_marker_time(self, value: f64) -> ();

    #[method(name = "get_IsNeedAssistRequest", args = 0)]
    pub fn get_is_need_assist_request(self) -> bool;

    #[method(name = "set_IsNeedAssistRequest", args = 1)]
    pub fn set_is_need_assist_request(self, value: bool) -> ();

    #[method(name = "get_AssistLevel", args = 0)]
    pub fn get_assist_level(self) -> i32;

    #[method(name = "set_AssistLevel", args = 1)]
    pub fn set_assist_level(self, value: i32) -> ();

    #[method(name = "get_IsAssist", args = 0)]
    pub fn get_is_assist(self) -> bool;

    #[method(name = "get_IsPlaySpecialSE", args = 0)]
    pub fn get_is_play_special_se(self) -> bool;

    #[method(name = "set_IsPlaySpecialSE", args = 1)]
    pub fn set_is_play_special_se(self, value: bool) -> ();

    #[method(name = "get_IsNeedTestPatternRequest", args = 0)]
    pub fn get_is_need_test_pattern_request(self) -> bool;

    #[method(name = "set_IsNeedTestPatternRequest", args = 1)]
    pub fn set_is_need_test_pattern_request(self, value: bool) -> ();

    #[method(name = "get_TestPatternRequest", args = 0)]
    pub fn get_test_pattern_request(self) -> ::unity2::Il2CppString;

    #[method(name = "set_TestPatternRequest", args = 1)]
    pub fn set_test_pattern_request(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_TestPatternCount", args = 0)]
    pub fn get_test_pattern_count(self) -> i32;

    #[method(name = "set_TestPatternCount", args = 1)]
    pub fn set_test_pattern_count(self, value: i32) -> ();

    #[method(name = "get_TargetShowHideFlag", args = 0)]
    pub fn get_target_show_hide_flag(
        self,
    ) -> crate::app::dragonridecamera::DragonRideCamera_TargetControleFlag;

    #[method(name = "set_TargetShowHideFlag", args = 1)]
    pub fn set_target_show_hide_flag(
        self,
        value: crate::app::dragonridecamera::DragonRideCamera_TargetControleFlag,
    ) -> ();

    #[method(name = "get_IsPauseStop", args = 0)]
    pub fn get_is_pause_stop(self) -> bool;

    #[method(name = "set_IsPauseStop", args = 1)]
    pub fn set_is_pause_stop(self, value: bool) -> ();

    #[method(name = "get_RetireGame", args = 0)]
    pub fn get_retire_game(self) -> bool;

    #[method(name = "set_RetireGame", args = 1)]
    pub fn set_retire_game(self, value: bool) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "GetParamTime", args = 1)]
    pub fn get_param_time(name: ::unity2::Il2CppString) -> f32;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "ResetCursol", args = 1)]
    pub fn reset_cursol(self, is_reset_angle: bool) -> ();

    #[method(name = "CreateDragonRideCharacter", args = 3)]
    pub fn create_dragon_ride_character(
        self,
        app: crate::combat::characterappearance::CharacterAppearance,
        parent: crate::unity_engine::transform::Transform,
        invisible: bool,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateDragonModel", args = 0)]
    pub fn create_dragon_model(self) -> ();

    #[method(name = "CreateAssistModel", args = 0)]
    pub fn create_assist_model(self) -> ();

    #[method(name = "InitializeCameraBase", args = 0)]
    pub fn initialize_camera_base(self) -> ();

    #[method(name = "CheckEnableCutin", args = 0)]
    pub fn check_enable_cutin(self) -> bool;

    #[method(name = "OpenCutin", args = 0)]
    pub fn open_cutin(self) -> ();

    #[method(name = "CloseCutin", args = 0)]
    pub fn close_cutin(self) -> ();

    #[method(name = "FinalizeCameraBase", args = 0)]
    pub fn finalize_camera_base(self) -> ();

    #[method(name = "InitializeCourseCamera", args = 2)]
    pub fn initialize_course_camera(self, course_id: ::unity2::Il2CppString, level: i32) -> ();

    #[method(name = "InitFake", args = 0)]
    pub fn init_fake(self) -> ();

    #[method(name = "TickFake", args = 0)]
    pub fn tick_fake(self) -> ();

    #[method(name = "StartTimeline", args = 0)]
    pub fn start_timeline(self) -> ();

    #[method(name = "UpdateCameraParam", args = 0)]
    pub fn update_camera_param(self) -> ();

    #[method(name = "UpdateCameraParam_Key", args = 0)]
    pub fn update_camera_param_key(self) -> ();

    #[method(name = "UpdateCameraParam_Gyro", args = 1)]
    pub fn update_camera_param_gyro(self, is_reset: bool) -> ();

    #[method(name = "CommitCamera", args = 0)]
    pub fn commit_camera(self) -> ();

    #[method(name = "CommitEventCamera", args = 0)]
    pub fn commit_event_camera(self) -> ();

    #[method(name = "UpdateShot", args = 0)]
    pub fn update_shot(self) -> ();

    #[method(name = "UpdateAssist", args = 0)]
    pub fn update_assist(self) -> ();

    #[method(name = "GetForwardVec", args = 0)]
    pub fn get_forward_vec(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetAssistShotDotRate", args = 0)]
    pub fn get_assist_shot_dot_rate(self) -> f32;

    #[method(name = "GetAssistCutInDotRate", args = 0)]
    pub fn get_assist_cut_in_dot_rate(self) -> f32;

    #[method(name = "AssistShot", args = 3)]
    pub fn assist_shot(
        self,
        is_aiming: bool,
        aim_pos: crate::unity_engine::vector3::Vector3,
        is_penetrate: bool,
    ) -> ();

    #[method(name = "SetTestPattern", args = 2)]
    pub fn set_test_pattern(self, pattern_id: ::unity2::Il2CppString, num: i32) -> ();

    #[method(name = "GetSelectedTestPatternNum", args = 0)]
    pub fn get_selected_test_pattern_num(self) -> i32;

    #[method(name = "FixedUpdate", args = 0)]
    pub fn fixed_update(self) -> ();

    #[method(name = "TickMain", args = 0)]
    pub fn tick_main(self) -> ();

    #[method(name = "TickResult", args = 0)]
    pub fn tick_result(self) -> ();

    #[method(name = "_RefreshObj", args = 1)]
    pub fn refresh_obj(self, target: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "FinalizeCourseCamera", args = 0)]
    pub fn finalize_course_camera(self) -> ();

    #[method(name = "SetEventMode", args = 3)]
    pub fn set_event_mode(
        self,
        flag: bool,
        time: f64,
        notify_obj: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "PlayOneShotSE", args = 1)]
    pub fn play_one_shot_se(self, label: ::unity2::Il2CppString) -> ();

    #[method(name = "FinishRail", args = 0)]
    pub fn finish_rail(self) -> ();

    #[method(name = "SetShowGroupList", args = 2)]
    pub fn set_show_group_list(
        self,
        group_list: ::unity2::Array<::unity2::Il2CppString>,
        time: f64,
    ) -> ();

    #[method(name = "SetHideGroupList", args = 2)]
    pub fn set_hide_group_list(
        self,
        group_list: ::unity2::Array<::unity2::Il2CppString>,
        time: f64,
    ) -> ();

    #[method(name = "GetControleGroupList", args = 1)]
    pub fn get_controle_group_list(self, list_ref: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "SetResultMode", args = 0)]
    pub fn set_result_mode(self) -> ();

    #[method(name = "GetTimelineMsec", args = 0)]
    pub fn get_timeline_msec(self) -> f64;

    #[method(name = "RunEventFadeIn", args = 1)]
    pub fn run_event_fade_in(self, r#type: crate::app::dragon_ride::fadetype_2::FadeType_2) -> ();

    #[method(name = "RunEventFadeOut", args = 1)]
    pub fn run_event_fade_out(self, r#type: crate::app::dragon_ride::fadetype_2::FadeType_2) -> ();

    #[method(name = "SetSpecialTime", args = 0)]
    pub fn set_special_time(self) -> ();

    #[method(name = "CreateStartTelop", args = 0)]
    pub fn create_start_telop(self) -> ();

    #[method(name = "CreateFinishTelop", args = 0)]
    pub fn create_finish_telop(self) -> ();

    #[method(name = "StopWindLineEffect", args = 0)]
    pub fn stop_wind_line_effect(self) -> ();

    #[method(name = "RestartWindLineEffect", args = 0)]
    pub fn restart_wind_line_effect(self) -> ();

    #[method(name = "PlaySpecialBurstSE", args = 0)]
    pub fn play_special_burst_se(self) -> ();

    #[method(name = "StopSpecialBurstSE", args = 0)]
    pub fn stop_special_burst_se(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dragonridecamera")]
impl DragonRideCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideCameraMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridecamera/DragonRideCamera_TargetControleFlag.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DragonRideCamera_TargetControleFlag {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DragonRideCamera_TargetControleFlag {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DragonRideCamera.TargetControleFlag";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DragonRideCamera_TargetControleFlag {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DragonRideCamera_TargetControleFlag {
    pub fn none() -> Self {
        Self { value: 1 }
    }

    pub fn show() -> Self {
        Self { value: 2 }
    }

    pub fn hide() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridecamera/DragonRideCamera_CutinState.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DragonRideCamera_CutinState {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DragonRideCamera_CutinState {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DragonRideCamera.CutinState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DragonRideCamera_CutinState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DragonRideCamera_CutinState {
    pub fn close_stay() -> Self {
        Self { value: 1 }
    }

    pub fn open() -> Self {
        Self { value: 2 }
    }

    pub fn closing() -> Self {
        Self { value: 4 }
    }
}
