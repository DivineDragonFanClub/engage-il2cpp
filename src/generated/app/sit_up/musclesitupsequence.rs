
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sit_up/musclesitupsequence/MuscleSitupSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MuscleSitupSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MuscleSitupSequence_Label {
    const NAMESPACE: &'static str = "App.SitUp";

    const NAME: &'static str = "MuscleSitupSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MuscleSitupSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MuscleSitupSequence_Label {
    pub fn init() -> Self {
        Self { value: 0 }
    }

    pub fn ready_count() -> Self {
        Self { value: 1 }
    }

    pub fn tick() -> Self {
        Self { value: 2 }
    }

    pub fn ready_finish() -> Self {
        Self { value: 3 }
    }

    pub fn finish() -> Self {
        Self { value: 4 }
    }

    pub fn result() -> Self {
        Self { value: 5 }
    }

    pub fn prize() -> Self {
        Self { value: 6 }
    }

    pub fn mascot_bond() -> Self {
        Self { value: 7 }
    }

    pub fn r#final() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sit_up/musclesitupsequence/MuscleSitupSequence.md")))]
#[::unity2::class(namespace = "App.SitUp", name = "MuscleSitupSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MuscleSitupSequence {
    #[static_field]
    #[rename(name = "ObjectRootPath")]
    pub object_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIRootPath")]
    pub c_ui_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIWindow")]
    pub c_ui_window: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIResult")]
    pub c_ui_result: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIPopUpGood")]
    pub c_ui_pop_up_good: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUICirclePerfect")]
    pub c_ui_circle_perfect: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUICircleGood")]
    pub c_ui_circle_good: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIPopupAssist")]
    pub c_ui_popup_assist: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTelopRootPath")]
    pub c_telop_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTelopCountDown")]
    pub c_telop_count_down: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTelopFinish")]
    pub c_telop_finish: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTelopStop")]
    pub c_telop_stop: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectRootPath")]
    pub c_effect_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectAssist")]
    pub c_effect_assist: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectEraseBad")]
    pub c_effect_erase_bad: ::unity2::Il2CppString,
    #[rename(name = "cLoadObjectList")]
    pub c_load_object_list: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "cBarDownBaseSpeed")]
    pub c_bar_down_base_speed: f32,
    #[static_field]
    #[rename(name = "cBarTopHeight")]
    pub c_bar_top_height: f32,
    #[static_field]
    #[rename(name = "cBarBottomHeight")]
    pub c_bar_bottom_height: f32,
    #[static_field]
    #[rename(name = "cBarLength")]
    pub c_bar_length: f32,
    #[static_field]
    #[rename(name = "cResetReachRate")]
    pub c_reset_reach_rate: f32,
    #[static_field]
    #[rename(name = "cDangerLineRate")]
    pub c_danger_line_rate: f32,
    #[static_field]
    #[rename(name = "cCountDownFrame")]
    pub c_count_down_frame: f32,
    #[static_field]
    #[rename(name = "cFinishFrame")]
    pub c_finish_frame: f32,
    #[static_field]
    #[rename(name = "cResultMinimumSec")]
    pub c_result_minimum_sec: f32,
    #[static_field]
    #[rename(name = "cAnimeRoopFrame")]
    pub c_anime_roop_frame: f32,
    #[rename(name = "cPlayerPos")]
    pub c_player_pos: crate::unity_engine::vector3::Vector3,
    #[static_field]
    #[rename(name = "cPlayerRotY")]
    pub c_player_rot_y: f32,
    #[rename(name = "cAssistPos")]
    pub c_assist_pos: crate::unity_engine::vector3::Vector3,
    #[static_field]
    #[rename(name = "cAssistRotY")]
    pub c_assist_rot_y: f32,
    #[rename(name = "PushRateSEs")]
    pub push_rate_s_es: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_Window")]
    pub m_window: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BarObj")]
    pub m_bar_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BarTrans")]
    pub m_bar_trans: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_BarImage")]
    pub m_bar_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_AButton")]
    pub m_a_button: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AButtonAnime")]
    pub m_a_button_anime: crate::unity_engine::animator::Animator,
    #[rename(name = "m_DangerArea")]
    pub m_danger_area: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DangerAreaAnime")]
    pub m_danger_area_anime: crate::unity_engine::animator::Animator,
    #[rename(name = "m_PlayerRoot")]
    pub m_player_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PlayerController")]
    pub m_player_controller: crate::app::hubunitcontroller::HubUnitController,
    #[rename(name = "m_ExerciseChara")]
    pub m_exercise_chara: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PlayerAnimator")]
    pub m_player_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_PlayerResetPos")]
    pub m_player_reset_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_PlayerResetRot")]
    pub m_player_reset_rot: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_PlayerResetDir")]
    pub m_player_reset_dir: f32,
    #[rename(name = "m_AnimeFrame")]
    pub m_anime_frame: f32,
    #[rename(name = "m_AnimeFrameInterp")]
    pub m_anime_frame_interp: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_IsReadyPlayerModel")]
    pub m_is_ready_player_model: bool,
    #[rename(name = "m_IsClear")]
    pub m_is_clear: bool,
    #[static_field]
    #[rename(name = "CameraAnimeCountMax")]
    pub camera_anime_count_max: i32,
    #[rename(name = "m_UseCamera")]
    pub m_use_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_CameraAnime")]
    pub m_camera_anime: crate::unity_engine::animation::Animation,
    #[rename(name = "m_UseCameraAnimeIndex")]
    pub m_use_camera_anime_index: i32,
    #[rename(name = "m_CameraAnimeResouce")]
    pub m_camera_anime_resouce: ::unity2::Array<crate::app::resourcehandle_2::ResourceHandle_2>,
    #[rename(name = "m_CameraResourceObjList")]
    pub m_camera_resource_obj_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_Talker")]
    pub m_talker: crate::unity_engine::transform::Transform,
    #[rename(name = "m_TalkChara")]
    pub m_talk_chara: crate::combat::character::Character,
    #[rename(name = "m_VoiceHandle")]
    pub m_voice_handle: crate::app::gamesound::GameSound_Handle,
    #[rename(name = "m_VoiceIntervalTimer")]
    pub m_voice_interval_timer: f32,
    #[static_field]
    #[rename(name = "VoiceIntervalSec")]
    pub voice_interval_sec: f32,
    #[rename(name = "m_Sola")]
    pub m_sola: crate::combat::character::Character,
    #[rename(name = "m_SolaAnime")]
    pub m_sola_anime: crate::unity_engine::animator::Animator,
    #[rename(name = "m_SolaReaction")]
    pub m_sola_reaction: bool,
    #[rename(name = "m_AssistMax")]
    pub m_assist_max: i32,
    #[rename(name = "m_AssistCount")]
    pub m_assist_count: i32,
    #[rename(name = "m_IsReadyAssistModel")]
    pub m_is_ready_assist_model: bool,
    #[rename(name = "m_HubSolaLct")]
    pub m_hub_sola_lct: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IsSetSolaInvisible")]
    pub m_is_set_sola_invisible: bool,
    #[rename(name = "m_AssistSpeed")]
    pub m_assist_speed: f32,
    #[rename(name = "m_IsDoingAssist")]
    pub m_is_doing_assist: bool,
    #[rename(name = "m_CountTimer")]
    pub m_count_timer: f32,
    #[rename(name = "m_CountTelop")]
    pub m_count_telop: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_StopTelop")]
    pub m_stop_telop: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CountText")]
    pub m_count_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_PastCount")]
    pub m_past_count: i32,
    #[rename(name = "m_PerfectText")]
    pub m_perfect_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_GoodText")]
    pub m_good_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BadBase")]
    pub m_bad_base: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BadAnime")]
    pub m_bad_anime: crate::unity_engine::animator::Animator,
    #[rename(name = "m_BadCount")]
    pub m_bad_count: i32,
    #[rename(name = "m_PerfectCount")]
    pub m_perfect_count: i32,
    #[rename(name = "m_GoodCount")]
    pub m_good_count: i32,
    #[rename(name = "m_BarRate")]
    pub m_bar_rate: f32,
    #[rename(name = "m_FallSpeed")]
    pub m_fall_speed: f32,
    #[rename(name = "m_FallSpeedHigh")]
    pub m_fall_speed_high: f32,
    #[rename(name = "m_GainHeight")]
    pub m_gain_height: f32,
    #[rename(name = "m_RankTimer")]
    pub m_rank_timer: f32,
    #[rename(name = "m_DangerTimer")]
    pub m_danger_timer: f32,
    #[rename(name = "m_ResultWindow")]
    pub m_result_window: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PopUpPerfect")]
    pub m_pop_up_perfect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PopUpGood")]
    pub m_pop_up_good: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PopUpAssist")]
    pub m_pop_up_assist: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PopUpAssistRectTrans")]
    pub m_pop_up_assist_rect_trans: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_AssistGlitter")]
    pub m_assist_glitter: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CurrentPopUp")]
    pub m_current_pop_up: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PopupTimer")]
    pub m_popup_timer: f32,
    #[rename(name = "m_PopupAssistTimer")]
    pub m_popup_assist_timer: f32,
    #[rename(name = "m_SpeedList")]
    pub m_speed_list: crate::system::collections::generic::list_1::List_1<
        crate::app::musclesitupfalldata::MuscleSitUpFallData,
    >,
    #[rename(name = "m_UseSpeedNum")]
    pub m_use_speed_num: i32,
    #[rename(name = "m_PerfectLimit")]
    pub m_perfect_limit: f32,
    #[rename(name = "m_GoodScore")]
    pub m_good_score: i32,
    #[rename(name = "m_PerfectScore")]
    pub m_perfect_score: i32,
    #[rename(name = "m_TargetScore")]
    pub m_target_score: i32,
    #[rename(name = "m_EndlessLimitCount")]
    pub m_endless_limit_count: i32,
    #[rename(name = "m_RankStr")]
    pub m_rank_str: ::unity2::Il2CppString,
    #[rename(name = "m_RankBonus")]
    pub m_rank_bonus: ::unity2::Il2CppString,
    #[rename(name = "m_RankBond")]
    pub m_rank_bond: i32,
    #[rename(name = "m_ResultTimer")]
    pub m_result_timer: f32,
}

#[cfg(feature = "app-sit_up-musclesitupsequence")]
#[::unity2::methods]
impl MuscleSitupSequence {
    #[method(name = "get_SelectLevel", args = 0)]
    pub fn get_select_level(self) -> i32;

    #[method(name = "set_SelectLevel", args = 1)]
    pub fn set_select_level(self, value: i32) -> ();

    #[method(name = "get_AssistLevel", args = 0)]
    pub fn get_assist_level(self) -> i32;

    #[method(name = "set_AssistLevel", args = 1)]
    pub fn set_assist_level(self, value: i32) -> ();

    #[method(name = "get_IsAssist", args = 0)]
    pub fn get_is_assist(self) -> bool;

    #[method(name = "get_IsReachTop", args = 0)]
    pub fn get_is_reach_top(self) -> bool;

    #[method(name = "set_IsReachTop", args = 1)]
    pub fn set_is_reach_top(self, value: bool) -> ();

    #[method(name = "get_IsDangerArea", args = 0)]
    pub fn get_is_danger_area(self) -> bool;

    #[method(name = "set_IsDangerArea", args = 1)]
    pub fn set_is_danger_area(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "IsReadyModel", args = 0)]
    pub fn is_ready_model(self) -> bool;

    #[method(name = "SetupCamera", args = 0)]
    pub fn setup_camera(self) -> ();

    #[method(name = "IsLoadingCamera", args = 0)]
    pub fn is_loading_camera(self) -> bool;

    #[method(name = "FinalizeCamera", args = 0)]
    pub fn finalize_camera(self) -> ();

    #[method(name = "GetAnimeRate", args = 0)]
    pub fn get_anime_rate(self) -> f32;

    #[method(name = "InitCountDown", args = 0)]
    pub fn init_count_down(self) -> ();

    #[method(name = "TickCount", args = 0)]
    pub fn tick_count(self) -> ();

    #[method(name = "JumpMain", args = 0)]
    pub fn jump_main(self) -> ();

    #[method(name = "CheckClear", args = 0)]
    pub fn check_clear(self) -> bool;

    #[method(name = "CheckStop", args = 0)]
    pub fn check_stop(self) -> bool;

    #[method(name = "JumpReadyFinish", args = 1)]
    pub fn jump_ready_finish(self, is_clear: bool) -> ();

    #[method(name = "JumpFinish", args = 0)]
    pub fn jump_finish(self) -> ();

    #[method(name = "StartPopUp", args = 1)]
    pub fn start_pop_up(
        self,
        set_rank: crate::app::sit_up::musclesitupsequence::MuscleSitupSequence_judgeRank,
    ) -> ();

    #[method(name = "TickPopup", args = 0)]
    pub fn tick_popup(self) -> ();

    #[method(name = "PopUpPerfect", args = 0)]
    pub fn pop_up_perfect(self) -> ();

    #[method(name = "PopUpGood", args = 0)]
    pub fn pop_up_good(self) -> ();

    #[method(name = "PopUpAssist", args = 0)]
    pub fn pop_up_assist(self) -> ();

    #[method(name = "AddCirclePerfect", args = 0)]
    pub fn add_circle_perfect(self) -> ();

    #[method(name = "AddCircleGood", args = 0)]
    pub fn add_circle_good(self) -> ();

    #[method(name = "AddPerfectCount", args = 0)]
    pub fn add_perfect_count(self) -> ();

    #[method(name = "AddGoodCount", args = 0)]
    pub fn add_good_count(self) -> ();

    #[method(name = "PlaySEPerfect", args = 0)]
    pub fn play_se_perfect(self) -> ();

    #[method(name = "PlaySEGood", args = 0)]
    pub fn play_se_good(self) -> ();

    #[method(name = "PlaySEBad", args = 0)]
    pub fn play_se_bad(self) -> ();

    #[method(name = "PlaySEStart", args = 0)]
    pub fn play_se_start(self) -> ();

    #[method(name = "PlaySEFinish", args = 0)]
    pub fn play_se_finish(self) -> ();

    #[method(name = "SetReachTop", args = 0)]
    pub fn set_reach_top(self) -> ();

    #[method(name = "ResetReachTop", args = 0)]
    pub fn reset_reach_top(self) -> ();

    #[method(name = "TickCamera", args = 0)]
    pub fn tick_camera(self) -> ();

    #[method(name = "PlayButtonPushSE", args = 1)]
    pub fn play_button_push_se(self, rate: f32) -> ();

    #[method(name = "TickMain", args = 0)]
    pub fn tick_main(self) -> ();

    #[method(name = "TickReadyFinish", args = 0)]
    pub fn tick_ready_finish(self) -> ();

    #[method(name = "TickFinish", args = 0)]
    pub fn tick_finish(self) -> ();

    #[method(name = "CalcRank", args = 0)]
    pub fn calc_rank(self) -> ();

    #[method(name = "ReadyResultCameraAndAnime", args = 0)]
    pub fn ready_result_camera_and_anime(self) -> ();

    #[method(name = "InitResult", args = 0)]
    pub fn init_result(self) -> ();

    #[method(name = "PlayResultVoice", args = 0)]
    pub fn play_result_voice(self) -> ();

    #[method(name = "TickResult", args = 0)]
    pub fn tick_result(self) -> ();

    #[method(name = "CloseResult", args = 0)]
    pub fn close_result(self) -> ();

    #[method(name = "IsClosedResult", args = 0)]
    pub fn is_closed_result(self) -> bool;

    #[method(name = "ExitResult", args = 0)]
    pub fn exit_result(self) -> ();

    #[method(name = "SetMascotBond", args = 0)]
    pub fn set_mascot_bond(self) -> ();

    #[method(name = "CheckGetablePrize", args = 0)]
    pub fn check_getable_prize(self) -> bool;

    #[method(name = "SetPrizeFlag", args = 0)]
    pub fn set_prize_flag(self) -> ();

    #[method(name = "GetPrizeBonus", args = 0)]
    pub fn get_prize_bonus(self) -> ();

    #[method(name = "GetPrizeBond", args = 0)]
    pub fn get_prize_bond(self) -> ();

    #[method(name = "UnloadResource", args = 0)]
    pub fn unload_resource(self) -> ();

    #[method(name = "Final", args = 0)]
    pub fn r#final(self) -> ();

    #[method(name = "IsEnableVoice", args = 0)]
    pub fn is_enable_voice(self) -> bool;

    #[method(name = "IsPlayingVoice", args = 0)]
    pub fn is_playing_voice(self) -> bool;

    #[method(name = "TryPlayVoice", args = 1)]
    pub fn try_play_voice(self, id: ::unity2::Il2CppString) -> ();

    #[method(name = "VoicePerfect", args = 0)]
    pub fn voice_perfect(self) -> ();

    #[method(name = "VoiceGood", args = 0)]
    pub fn voice_good(self) -> ();

    #[method(name = "VoiceBad", args = 0)]
    pub fn voice_bad(self) -> ();

    #[method(name = "VoiceEnhance1", args = 0)]
    pub fn voice_enhance1(self) -> ();

    #[method(name = "VoiceEnhance2", args = 0)]
    pub fn voice_enhance2(self) -> ();

    #[method(name = "VoiceEnhance3", args = 0)]
    pub fn voice_enhance3(self) -> ();

    #[method(name = "VoiceResultPerfect", args = 0)]
    pub fn voice_result_perfect(self) -> ();

    #[method(name = "VoiceResultGood", args = 0)]
    pub fn voice_result_good(self) -> ();

    #[method(name = "VoiceResultBad", args = 0)]
    pub fn voice_result_bad(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, level: i32, assist: bool) -> ();
}

#[cfg(feature = "app-sit_up-musclesitupsequence")]
impl MuscleSitupSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleSitupSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleSitupSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sit_up/musclesitupsequence/MuscleSitupSequence_judgeRank.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MuscleSitupSequence_judgeRank {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MuscleSitupSequence_judgeRank {
    const NAMESPACE: &'static str = "App.SitUp";

    const NAME: &'static str = "MuscleSitupSequence.judgeRank";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MuscleSitupSequence_judgeRank {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MuscleSitupSequence_judgeRank {
    pub fn perfect() -> Self {
        Self { value: 0 }
    }

    pub fn good() -> Self {
        Self { value: 1 }
    }

    pub fn assist() -> Self {
        Self { value: 2 }
    }
}
