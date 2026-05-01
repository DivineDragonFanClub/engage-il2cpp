
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishinggamesequence/FishingGameSequence.md")))]
#[::unity2::class(namespace = "App", name = "FishingGameSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: hubsequence :: HubSequence >)]
pub struct FishingGameSequence {
    #[static_field]
    #[rename(name = "FirstAngle")]
    pub first_angle: f32,
    #[static_field]
    #[rename(name = "FirstDistance")]
    pub first_distance: f32,
    #[static_field]
    #[rename(name = "cAnime_SelectRod")]
    pub c_anime_select_rod: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAnime_ThrowIn")]
    pub c_anime_throw_in: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAnime_Wait")]
    pub c_anime_wait: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAnime_Resist")]
    pub c_anime_resist: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAnime_ResistL")]
    pub c_anime_resist_l: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAnime_ResistR")]
    pub c_anime_resist_r: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAnime_LethalResist")]
    pub c_anime_lethal_resist: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAnime_Catch")]
    pub c_anime_catch: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAnime_Stand")]
    pub c_anime_stand: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAnime_Joy")]
    pub c_anime_joy: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cCharaImageRenderPath")]
    pub c_chara_image_render_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIRootPath")]
    pub c_ui_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIHitButtonPath")]
    pub c_ui_hit_button_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIBattleRadarPath")]
    pub c_ui_battle_radar_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIBattleStickPath")]
    pub c_ui_battle_stick_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUIBattleButtonPath")]
    pub c_ui_battle_button_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cUILethalButtonPath")]
    pub c_ui_lethal_button_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "UIMovecircleHelpPath")]
    pub ui_movecircle_help_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cResultUIPath")]
    pub c_result_ui_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectRootPath")]
    pub c_effect_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectSplashNormal")]
    pub c_effect_splash_normal: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectCounterSplashSmall")]
    pub c_effect_counter_splash_small: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectCounterSplashMiddle")]
    pub c_effect_counter_splash_middle: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectCounterSplashLarge")]
    pub c_effect_counter_splash_large: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectLethalSplashSmall")]
    pub c_effect_lethal_splash_small: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectLethalSplashMiddle")]
    pub c_effect_lethal_splash_middle: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectLethalSplashLarge")]
    pub c_effect_lethal_splash_large: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectGuideCircle")]
    pub c_effect_guide_circle: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectCastSplash")]
    pub c_effect_cast_splash: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectRippleSmall")]
    pub c_effect_ripple_small: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectRippleMiddle")]
    pub c_effect_ripple_middle: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectRippleLarge")]
    pub c_effect_ripple_large: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectResultGlitter_Small")]
    pub c_effect_result_glitter_small: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectResultGlitter_Middle")]
    pub c_effect_result_glitter_middle: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectResultGlitter_Large")]
    pub c_effect_result_glitter_large: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectMissSplash")]
    pub c_effect_miss_splash: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cEffectRaderBubble")]
    pub c_effect_rader_bubble: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTelopRootPath")]
    pub c_telop_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTelopHit")]
    pub c_telop_hit: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTelopPerfect")]
    pub c_telop_perfect: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTelopSuccess")]
    pub c_telop_success: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAssistPopupPath")]
    pub c_assist_popup_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAssistGlitterPath")]
    pub c_assist_glitter_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTextureRootPath")]
    pub c_texture_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cAtlasPath")]
    pub c_atlas_path: ::unity2::Il2CppString,
    #[rename(name = "cLoadObjectPath")]
    pub c_load_object_path: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "cLoadTexturePath")]
    pub c_load_texture_path: ::unity2::Il2CppString,
    #[rename(name = "m_ConfigBase")]
    pub m_config_base: crate::app::fishingconfig_base::FishingConfig_Base,
    #[rename(name = "m_ConfigMoveCircle")]
    pub m_config_move_circle: crate::app::fishingconfig_movecircle::FishingConfig_MoveCircle,
    #[rename(name = "m_ConfigThrowIn")]
    pub m_config_throw_in: crate::app::fishingconfig_throwin::FishingConfig_ThrowIn,
    #[rename(name = "m_ConfigWaitCatch")]
    pub m_config_wait_catch: crate::app::fishingconfig_waitcatch::FishingConfig_WaitCatch,
    #[rename(name = "m_ConfigWaitCancel")]
    pub m_config_wait_cancel: crate::app::fishingconfig_waitcancel::FishingConfig_WaitCancel,
    #[rename(name = "m_ConfigBattle")]
    pub m_config_battle: crate::app::fishingconfig_battle::FishingConfig_Battle,
    #[rename(name = "m_ConfigDefeat")]
    pub m_config_defeat: crate::app::fishingconfig_defeat::FishingConfig_Defeat,
    #[rename(name = "m_ConfigResult")]
    pub m_config_result: crate::app::fishingconfig_result::FishingConfig_Result,
    #[rename(name = "m_PlayerRoot")]
    pub m_player_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PlayerController")]
    pub m_player_controller: crate::app::hubunitcontroller::HubUnitController,
    #[rename(name = "m_HubLookComponent")]
    pub m_hub_look_component: crate::app::hublookatcontroller::HubLookAtController,
    #[rename(name = "m_RodAnimator")]
    pub m_rod_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_Sola")]
    pub m_sola: crate::combat::character::Character,
    #[rename(name = "m_HubSolaLct")]
    pub m_hub_sola_lct: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IsSetSolaInvisible")]
    pub m_is_set_sola_invisible: bool,
    #[rename(name = "RodModelIDs")]
    pub rod_model_i_ds: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "LureModelIDs")]
    pub lure_model_i_ds: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_Talker")]
    pub m_talker: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TalkerChara")]
    pub m_talker_chara: crate::combat::character::Character,
    #[rename(name = "m_TalkerRotTestDir")]
    pub m_talker_rot_test_dir: f32,
    #[rename(name = "m_TalkerResetPos")]
    pub m_talker_reset_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_TalkerResetRot")]
    pub m_talker_reset_rot: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_TalkerResetDir")]
    pub m_talker_reset_dir: f32,
    #[rename(name = "m_Timer")]
    pub m_timer: f32,
    #[rename(name = "m_ResetPos")]
    pub m_reset_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_ResetRot")]
    pub m_reset_rot: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "m_ResetDir")]
    pub m_reset_dir: f32,
    #[rename(name = "m_CameraObj")]
    pub m_camera_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CameraComponent")]
    pub m_camera_component: crate::unity_engine::camera::Camera,
    #[rename(name = "m_DefeatCamera")]
    pub m_defeat_camera: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DefeatCameraAnim")]
    pub m_defeat_camera_anim: crate::unity_engine::animation::Animation,
    #[rename(name = "m_GuideCircleObj")]
    pub m_guide_circle_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_GuideEffect")]
    pub m_guide_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LureObj")]
    pub m_lure_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LureTrans")]
    pub m_lure_trans: crate::unity_engine::transform::Transform,
    #[rename(name = "m_LureScript")]
    pub m_lure_script: crate::app::fishinglure::FishingLure,
    #[rename(name = "m_LureModel")]
    pub m_lure_model: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LureAnime")]
    pub m_lure_anime: crate::unity_engine::animator::Animator,
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Angle")]
    pub m_angle: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_ThrowDistance")]
    pub m_throw_distance: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_IsPlayThrowSE")]
    pub m_is_play_throw_se: bool,
    #[rename(name = "m_IsPlaySinkSE")]
    pub m_is_play_sink_se: bool,
    #[rename(name = "m_IsThrowInCameraChange")]
    pub m_is_throw_in_camera_change: bool,
    #[rename(name = "m_ResetDistance")]
    pub m_reset_distance: f32,
    #[rename(name = "m_ResetAngle")]
    pub m_reset_angle: f32,
    #[rename(name = "m_IsStopLure")]
    pub m_is_stop_lure: bool,
    #[rename(name = "m_PastStickPower")]
    pub m_past_stick_power: f32,
    #[rename(name = "m_lureBasePos")]
    pub m_lure_base_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_StickWaitTimer")]
    pub m_stick_wait_timer: f32,
    #[rename(name = "m_AttackWaitTimer")]
    pub m_attack_wait_timer: f32,
    #[rename(name = "m_VoiceIntervalTimer")]
    pub m_voice_interval_timer: f32,
    #[rename(name = "m_BattleRadarObj")]
    pub m_battle_radar_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RadarScript")]
    pub m_radar_script: crate::app::fishingbattlerader::FishingBattleRader,
    #[rename(name = "m_HitButtonObj")]
    pub m_hit_button_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_StickObj")]
    pub m_stick_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ButtonObj")]
    pub m_button_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LethalButtonObj")]
    pub m_lethal_button_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_LethalButtonAnime")]
    pub m_lethal_button_anime: crate::unity_engine::animator::Animator,
    #[rename(name = "m_LethalAnimePlayWait")]
    pub m_lethal_anime_play_wait: bool,
    #[rename(name = "m_LethalSuccessAnimeWait")]
    pub m_lethal_success_anime_wait: bool,
    #[rename(name = "m_RadicalParam")]
    pub m_radical_param:
        ::unity2::Array<crate::app::fishingradicalparamdata::FishingRadicalParamData_RadicalParam>,
    #[rename(name = "m_UseRadicalIndex")]
    pub m_use_radical_index: i32,
    #[rename(name = "m_RadicalTimer")]
    pub m_radical_timer: f32,
    #[rename(name = "m_AlartSETimer")]
    pub m_alart_se_timer: f32,
    #[static_field]
    #[rename(name = "cAlartSec")]
    pub c_alart_sec: f32,
    #[rename(name = "m_PlayerAnimeTimer")]
    pub m_player_anime_timer: f32,
    #[rename(name = "m_PlayerState")]
    pub m_player_state: crate::app::fishinggamesequence::FishingGameSequence_FishingAngleState,
    #[rename(name = "m_BattleCameraState")]
    pub m_battle_camera_state:
        crate::app::fishinggamesequence::FishingGameSequence_FishingAngleState,
    #[rename(name = "m_BaseCameraAngle")]
    pub m_base_camera_angle: f32,
    #[rename(name = "m_isRunningReverseLerp")]
    pub m_is_running_reverse_lerp: bool,
    #[static_field]
    #[rename(name = "m_RodMenuResult")]
    pub m_rod_menu_result: crate::app::fishing::sticktype::StickType,
    #[rename(name = "m_FishObj")]
    pub m_fish_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FishScript")]
    pub m_fish_script: crate::app::fishingfish::FishingFish,
    #[rename(name = "m_CanvasRoot")]
    pub m_canvas_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Fader")]
    pub m_fader: crate::app::fishingeventfader::FishingEventFader,
    #[rename(name = "m_IsPullStick")]
    pub m_is_pull_stick: bool,
    #[rename(name = "m_IsCancelFadeOut")]
    pub m_is_cancel_fade_out: bool,
    #[rename(name = "m_IsCancelFadeIn")]
    pub m_is_cancel_fade_in: bool,
    #[rename(name = "m_IsSelectRodWait")]
    pub m_is_select_rod_wait: bool,
    #[rename(name = "m_FailType")]
    pub m_fail_type: crate::app::fishinggamesequence::FishingGameSequence_AnnounceType,
    #[rename(name = "m_ThrowinHeight")]
    pub m_throwin_height: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_MoveCircleHelp")]
    pub m_move_circle_help: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RipplesRoot")]
    pub m_ripples_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Ripples")]
    pub m_ripples: ::unity2::Array<crate::app::fishinggamesequence::FishingGameSequence_Ripple>,
    #[rename(name = "m_EnableRipples")]
    pub m_enable_ripples: bool,
    #[rename(name = "m_ImageRenderObj")]
    pub m_image_render_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ImageRender")]
    pub m_image_render: crate::app::fishingcharaimagerender::FishingCharaImageRender,
    #[rename(name = "m_LureHeight")]
    pub m_lure_height: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_IsAssitBattleStart")]
    pub m_is_assit_battle_start: bool,
    #[rename(name = "m_AssistPopUp")]
    pub m_assist_pop_up: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AssistRect")]
    pub m_assist_rect: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_AssistGlitter")]
    pub m_assist_glitter: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AssistDisplayRateX")]
    pub m_assist_display_rate_x: f32,
    #[rename(name = "m_AssistDisplayRateY")]
    pub m_assist_display_rate_y: f32,
    #[rename(name = "m_AssistDisplayPos")]
    pub m_assist_display_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_DefeatFrame")]
    pub m_defeat_frame: f32,
    #[rename(name = "m_DefeatFadeLength")]
    pub m_defeat_fade_length: f32,
    #[rename(name = "m_IsShowSucessTelop")]
    pub m_is_show_sucess_telop: bool,
    #[rename(name = "m_IsStartDefeatFade")]
    pub m_is_start_defeat_fade: bool,
    #[rename(name = "m_ResultUI")]
    pub m_result_ui: crate::app::fishingresultui::FishingResultUI,
    #[rename(name = "m_ResultGlitter")]
    pub m_result_glitter: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_IsPlayResultVoice")]
    pub m_is_play_result_voice: bool,
    #[rename(name = "m_IsGetBonus")]
    pub m_is_get_bonus: bool,
    #[static_field]
    #[rename(name = "m_ContinueResult")]
    pub m_continue_result: bool,
    #[rename(name = "m_AssistLevel")]
    pub m_assist_level: i32,
    #[rename(name = "m_AssistDamage")]
    pub m_assist_damage: f32,
}

#[cfg(feature = "app-fishinggamesequence")]
#[::unity2::methods]
impl FishingGameSequence {
    #[method(name = "get_IsPlayDrawSE", args = 0)]
    pub fn get_is_play_draw_se(self) -> bool;

    #[method(name = "set_IsPlayDrawSE", args = 1)]
    pub fn set_is_play_draw_se(self, value: bool) -> ();

    #[method(name = "get_IsAssist", args = 0)]
    pub fn get_is_assist(self) -> bool;

    #[method(name = "EnableFishPrefab", args = 0)]
    pub fn enable_fish_prefab(self) -> ();

    #[method(name = "PauseHubBGM", args = 0)]
    pub fn pause_hub_bgm(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "SetGuideActive", args = 1)]
    pub fn set_guide_active(self, set: bool) -> ();

    #[method(name = "InitImageRender", args = 0)]
    pub fn init_image_render(self) -> ();

    #[method(name = "InitBattleRadar", args = 0)]
    pub fn init_battle_radar(self) -> ();

    #[method(name = "CreateRodAndLureModel", args = 0)]
    pub fn create_rod_and_lure_model(self) -> ();

    #[method(name = "TryDestoyRodAndLureModel", args = 0)]
    pub fn try_destoy_rod_and_lure_model(self) -> ();

    #[method(name = "DestroyImageRender", args = 0)]
    pub fn destroy_image_render(self) -> ();

    #[method(name = "DestroyRadar", args = 0)]
    pub fn destroy_radar(self) -> ();

    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource(self) -> ();

    #[method(name = "IsLoadingResource", args = 0)]
    pub fn is_loading_resource(self) -> bool;

    #[method(name = "UnloadResource", args = 0)]
    pub fn unload_resource(self) -> ();

    #[method(name = "UpdateRipples", args = 0)]
    pub fn update_ripples(self) -> ();

    #[method(name = "SetEnablePopRipples", args = 1)]
    pub fn set_enable_pop_ripples(self, set: bool) -> ();

    #[method(name = "DestroyAllRippeles", args = 0)]
    pub fn destroy_all_rippeles(self) -> ();

    #[method(name = "ReadyFirstRod", args = 0)]
    pub fn ready_first_rod(self) -> ();

    #[method(name = "TickSelectRodWait", args = 0)]
    pub fn tick_select_rod_wait(self) -> ();

    #[method(name = "CreateTitleBar", args = 0)]
    pub fn create_title_bar(self) -> ();

    #[method(name = "CloseTitleBar", args = 0)]
    pub fn close_title_bar(self) -> ();

    #[method(name = "CreateRodSelectMenu", args = 0)]
    pub fn create_rod_select_menu(self) -> ();

    #[method(name = "TickRodSelectCamera", args = 0)]
    pub fn tick_rod_select_camera(self) -> ();

    #[method(name = "CreateAssistMenu", args = 0)]
    pub fn create_assist_menu(self) -> ();

    #[method(name = "CallAssistEnhance", args = 0)]
    pub fn call_assist_enhance(self) -> ();

    #[method(name = "TickMoveCircle", args = 0)]
    pub fn tick_move_circle(self) -> ();

    #[method(name = "TickThrowIn", args = 0)]
    pub fn tick_throw_in(self) -> ();

    #[method(name = "TickWaitCatch", args = 0)]
    pub fn tick_wait_catch(self) -> ();

    #[method(name = "TickWaitCancel", args = 0)]
    pub fn tick_wait_cancel(self) -> ();

    #[method(name = "ProcessAnnounceFailed", args = 0)]
    pub fn process_announce_failed(self) -> ();

    #[method(name = "TickHitPopup", args = 0)]
    pub fn tick_hit_popup(self) -> ();

    #[method(name = "TickAssistAttack", args = 0)]
    pub fn tick_assist_attack(self) -> ();

    #[method(name = "UpdatePlayerAnime", args = 0)]
    pub fn update_player_anime(self) -> ();

    #[method(name = "SetPlayerAnimeState", args = 1)]
    pub fn set_player_anime_state(
        self,
        set_state: crate::app::fishinggamesequence::FishingGameSequence_FishingAngleState,
    ) -> ();

    #[method(name = "ResetRadicalParam", args = 0)]
    pub fn reset_radical_param(self) -> ();

    #[method(name = "TickBattle", args = 0)]
    pub fn tick_battle(self) -> ();

    #[method(name = "TickBattleLethal", args = 0)]
    pub fn tick_battle_lethal(self) -> ();

    #[method(name = "TickWaitDisableUI", args = 0)]
    pub fn tick_wait_disable_ui(self) -> bool;

    #[method(name = "PhaseBattleFailed", args = 0)]
    pub fn phase_battle_failed(self) -> ();

    #[method(name = "TickDefeatMovie", args = 0)]
    pub fn tick_defeat_movie(self) -> ();

    #[method(name = "TickResult", args = 0)]
    pub fn tick_result(self) -> ();

    #[method(name = "CloseResult", args = 0)]
    pub fn close_result(self) -> ();

    #[method(name = "IsClosedResult", args = 0)]
    pub fn is_closed_result(self) -> bool;

    #[method(name = "ExitResult", args = 0)]
    pub fn exit_result(self) -> ();

    #[method(name = "GetPrizeBond", args = 0)]
    pub fn get_prize_bond(self) -> ();

    #[method(name = "GetPrizeItem", args = 0)]
    pub fn get_prize_item(self) -> ();

    #[method(name = "DecreasePlayCount", args = 0)]
    pub fn decrease_play_count(self) -> ();

    #[method(name = "IncreasePlayCounter", args = 0)]
    pub fn increase_play_counter(self) -> ();

    #[method(name = "CreateContinueDialog", args = 0)]
    pub fn create_continue_dialog(self) -> ();

    #[method(name = "InitContinue", args = 0)]
    pub fn init_continue(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "PlayDrawSE", args = 0)]
    pub fn play_draw_se(self) -> ();

    #[method(name = "StopDrawSE", args = 0)]
    pub fn stop_draw_se(self) -> ();

    #[method(name = "StopAllContinueSE", args = 0)]
    pub fn stop_all_continue_se(self) -> ();

    #[method(name = "JumpThrowIn", args = 0)]
    pub fn jump_throw_in(self) -> ();

    #[method(name = "JumpWaitCatch", args = 0)]
    pub fn jump_wait_catch(self) -> ();

    #[method(name = "JumpHitPopup", args = 0)]
    pub fn jump_hit_popup(self) -> ();

    #[method(name = "JumpAssistAttack", args = 0)]
    pub fn jump_assist_attack(self) -> ();

    #[method(name = "JumpBattle", args = 0)]
    pub fn jump_battle(self) -> ();

    #[method(name = "JumpBattleLethal", args = 0)]
    pub fn jump_battle_lethal(self) -> ();

    #[method(name = "JumpDefeatMovie", args = 0)]
    pub fn jump_defeat_movie(self) -> ();

    #[method(name = "JumpResult", args = 0)]
    pub fn jump_result(self) -> ();

    #[method(name = "JumpMoveCircle", args = 0)]
    pub fn jump_move_circle(self) -> ();

    #[method(name = "JumpWaitCancel", args = 0)]
    pub fn jump_wait_cancel(self) -> ();

    #[method(name = "JumpAnnounceFailed", args = 0)]
    pub fn jump_announce_failed(self) -> ();

    #[method(name = "JumpSelectRod", args = 0)]
    pub fn jump_select_rod(self) -> ();

    #[method(name = "JumpGetPrize", args = 0)]
    pub fn jump_get_prize(self) -> ();

    #[method(name = "JumpCheckContinue", args = 0)]
    pub fn jump_check_continue(self) -> ();

    #[method(name = "JumpBattleFailed", args = 0)]
    pub fn jump_battle_failed(self) -> ();

    #[method(name = "VoiceRodSelect", args = 0)]
    pub fn voice_rod_select(self) -> ();

    #[method(name = "VoiceMoveCircle", args = 0)]
    pub fn voice_move_circle(self) -> ();

    #[method(name = "VoiceHit", args = 0)]
    pub fn voice_hit(self) -> ();

    #[method(name = "VoiceAdviceLeft", args = 0)]
    pub fn voice_advice_left(self) -> ();

    #[method(name = "VoiceAdviceRight", args = 0)]
    pub fn voice_advice_right(self) -> ();

    #[method(name = "VoiceEscape", args = 0)]
    pub fn voice_escape(self) -> ();

    #[method(name = "VoiceSuccessGiant", args = 0)]
    pub fn voice_success_giant(self) -> ();

    #[method(name = "VoiceSuccessMiddle", args = 0)]
    pub fn voice_success_middle(self) -> ();

    #[method(name = "VoiceSuccessSmall", args = 0)]
    pub fn voice_success_small(self) -> ();

    #[method(name = "VoiceCheckContinue", args = 0)]
    pub fn voice_check_continue(self) -> ();

    #[method(name = "VoiceEndGame", args = 0)]
    pub fn voice_end_game(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-fishinggamesequence")]
impl FishingGameSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingGameSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingGameSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishinggamesequence/FishingGameSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FishingGameSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FishingGameSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "FishingGameSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FishingGameSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FishingGameSequence_Label {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn init() -> Self {
        Self { value: 1 }
    }

    pub fn select_rod() -> Self {
        Self { value: 2 }
    }

    pub fn confirm_assist() -> Self {
        Self { value: 3 }
    }

    pub fn move_circle() -> Self {
        Self { value: 4 }
    }

    pub fn throw_in() -> Self {
        Self { value: 5 }
    }

    pub fn wait_catch() -> Self {
        Self { value: 6 }
    }

    pub fn wait_cancel() -> Self {
        Self { value: 7 }
    }

    pub fn announce_failed() -> Self {
        Self { value: 8 }
    }

    pub fn hit_popup() -> Self {
        Self { value: 9 }
    }

    pub fn assist_atack() -> Self {
        Self { value: 10 }
    }

    pub fn battle() -> Self {
        Self { value: 11 }
    }

    pub fn battle_lethal() -> Self {
        Self { value: 12 }
    }

    pub fn battle_failed() -> Self {
        Self { value: 13 }
    }

    pub fn defeat_movie() -> Self {
        Self { value: 14 }
    }

    pub fn result() -> Self {
        Self { value: 15 }
    }

    pub fn mascot_bond() -> Self {
        Self { value: 16 }
    }

    pub fn get_prize() -> Self {
        Self { value: 17 }
    }

    pub fn check_continue() -> Self {
        Self { value: 18 }
    }

    pub fn init_continue() -> Self {
        Self { value: 19 }
    }

    pub fn exit() -> Self {
        Self { value: 20 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishinggamesequence/FishingGameSequence_Ripple.md")))]
#[::unity2::class(namespace = "App", name = "FishingGameSequence.Ripple")]
#[parent(crate::system::object::Object)]
pub struct FishingGameSequence_Ripple {
    #[rename(name = "m_obj")]
    pub m_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BasePos")]
    pub m_base_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_SizeList")]
    pub m_size_list: crate::system::collections::generic::list_1::List_1<i32>,
    #[rename(name = "m_BaseHeight")]
    pub m_base_height: f32,
    #[rename(name = "m_PopIntervalBaseTime")]
    pub m_pop_interval_base_time: f32,
    #[rename(name = "m_PopRandomMax")]
    pub m_pop_random_max: f32,
    #[rename(name = "m_Timer")]
    pub m_timer: f32,
    #[rename(name = "m_parentNode")]
    pub m_parent_node: crate::unity_engine::transform::Transform,
}

#[cfg(feature = "app-fishinggamesequence")]
#[::unity2::methods]
impl FishingGameSequence_Ripple {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Init", args = 5)]
    pub fn init(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        size: crate::system::collections::generic::list_1::List_1<i32>,
        base_height: f32,
        base_interval: f32,
        add_range: f32,
    ) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "DestroyObjSoon", args = 0)]
    pub fn destroy_obj_soon(self) -> ();
}

#[cfg(feature = "app-fishinggamesequence")]
impl FishingGameSequence_Ripple {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingGameSequence_Ripple),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingGameSequence_RippleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishinggamesequence/FishingGameSequence_FishingAngleState.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FishingGameSequence_FishingAngleState {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FishingGameSequence_FishingAngleState {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "FishingGameSequence.FishingAngleState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FishingGameSequence_FishingAngleState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FishingGameSequence_FishingAngleState {
    pub fn center() -> Self {
        Self { value: 0 }
    }

    pub fn right() -> Self {
        Self { value: 1 }
    }

    pub fn left() -> Self {
        Self { value: 2 }
    }

    pub fn lethal() -> Self {
        Self { value: 3 }
    }

    pub fn angle_state_count() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishinggamesequence/FishingGameSequence_AnnounceType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FishingGameSequence_AnnounceType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FishingGameSequence_AnnounceType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "FishingGameSequence.AnnounceType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FishingGameSequence_AnnounceType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FishingGameSequence_AnnounceType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn fast() -> Self {
        Self { value: 1 }
    }

    pub fn slow() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishinggamesequence/FishingGameSequence_LureRoot.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct FishingGameSequence_LureRoot {
    pub x: f32,
    pub y: f32,
    pub frame: f32,
}

impl ::unity2::ClassIdentity for FishingGameSequence_LureRoot {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "FishingGameSequence.LureRoot";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FishingGameSequence_LureRoot {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-fishinggamesequence")]
#[::unity2::methods(value)]
impl FishingGameSequence_LureRoot {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, set_x: f32, set_y: f32, set_frame: f32) -> ();
}
