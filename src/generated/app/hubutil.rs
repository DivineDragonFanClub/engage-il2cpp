
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubutil/HubUtil.md")))]
#[::unity2::class(namespace = "App", name = "HubUtil")]
#[parent(crate::system::object::Object)]
pub struct HubUtil {
    #[static_field]
    #[rename(name = "s_HubParams")]
    pub s_hub_params: crate::app::hubparams::HubParams,
}

#[cfg(feature = "app-hubutil")]
#[::unity2::methods]
impl HubUtil {
    #[method(name = "get_Params", args = 0)]
    pub fn get_params() -> crate::app::hubparams::HubParams;

    #[method(name = "IsParamsValid", args = 0)]
    pub fn is_params_valid() -> bool;

    #[method(name = "GetUnitDisplayNum", args = 0)]
    pub fn get_unit_display_num() -> i32;

    #[method(name = "get_PadThreshold", args = 0)]
    pub fn get_pad_threshold() -> f32;

    #[method(name = "get_PlayerMaxSpeed", args = 0)]
    pub fn get_player_max_speed() -> f32;

    #[method(name = "get_PlayerAccel", args = 0)]
    pub fn get_player_accel() -> f32;

    #[method(name = "get_PlayerDecel", args = 0)]
    pub fn get_player_decel() -> f32;

    #[method(name = "get_PlayerRotateSpeedRate", args = 0)]
    pub fn get_player_rotate_speed_rate() -> f32;

    #[method(name = "get_PlayerDashStopTime", args = 0)]
    pub fn get_player_dash_stop_time() -> f32;

    #[method(name = "get_PlayerDashSpeedIntensity", args = 0)]
    pub fn get_player_dash_speed_intensity() -> f32;

    #[method(name = "get_SpringGravityY", args = 0)]
    pub fn get_spring_gravity_y() -> f32;

    #[method(name = "GetPlayerSpeedCurve", args = 1)]
    pub fn get_player_speed_curve(magnitude: f32) -> f32;

    #[method(name = "GetTurnCurve", args = 0)]
    pub fn get_turn_curve() -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "get_MinLookAtDist", args = 0)]
    pub fn get_min_look_at_dist() -> f32;

    #[method(name = "get_MaxLookAtDist", args = 0)]
    pub fn get_max_look_at_dist() -> f32;

    #[method(name = "get_PadAllowance", args = 0)]
    pub fn get_pad_allowance() -> f32;

    #[method(name = "get_OthersBodyWeight", args = 0)]
    pub fn get_others_body_weight() -> f32;

    #[method(name = "get_OthersHeadWeight", args = 0)]
    pub fn get_others_head_weight() -> f32;

    #[method(name = "get_EmptyWord", args = 0)]
    pub fn get_empty_word() -> ::unity2::Il2CppString;

    #[method(name = "GetCharacterAppearance", args = 2)]
    pub fn get_character_appearance(
        pid: ::unity2::Il2CppString,
        accessory: ::unity2::Il2CppString,
    ) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "GetPlayerAppearance", args = 1)]
    pub fn get_player_appearance(
        pid: ::unity2::Il2CppString,
    ) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "CreateLookAt", args = 2)]
    pub fn create_look_at(
        game_object: crate::unity_engine::gameobject::GameObject,
        disabled_param: bool,
    ) -> crate::app::hublookatcontroller::HubLookAtController;

    #[method(name = "IsMain", args = 0)]
    pub fn is_main() -> bool;

    #[method(name = "IsHubSequence", args = 0)]
    pub fn is_hub_sequence() -> bool;

    #[method(name = "IsPlayerFemale", args = 0)]
    pub fn is_player_female() -> bool;

    #[method(name = "IsComplete", args = 1)]
    pub fn is_complete(cid: ::unity2::Il2CppString) -> bool;

    #[method(name = "PIDToGID", args = 1)]
    pub fn pid_to_gid(pid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GIDToPID", args = 1)]
    pub fn gid_to_pid(gid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "TryGetSortieUnit", args = 1)]
    pub fn try_get_sortie_unit(pid: ::unity2::Il2CppString) -> crate::app::unit::Unit;

    #[method(name = "IsSortieUnit", args = 1)]
    pub fn is_sortie_unit(pid: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsBestReliance", args = 1)]
    pub fn is_best_reliance(pid: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetNowYear", args = 0)]
    pub fn get_now_year() -> i32;

    #[method(name = "IsBirthday", args = 1)]
    pub fn is_birthday(pid: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsHeroBirthday", args = 0)]
    pub fn is_hero_birthday() -> bool;

    #[method(name = "IsBirthdayPresentGot", args = 1)]
    pub fn is_birthday_present_got(pid: ::unity2::Il2CppString) -> bool;

    #[method(name = "CanPromiseEngage", args = 1)]
    pub fn can_promise_engage(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsAvaliableDispos", args = 2)]
    pub fn is_avaliable_dispos(
        data: crate::app::hubdisposdata::HubDisposData,
        timezone_type: crate::app::hubutil::HubUtil_TimezoneType,
    ) -> bool;

    #[method(name = "LevelUpReliance", args = 3)]
    pub fn level_up_reliance(
        a: crate::app::unit::Unit,
        b: crate::app::unit::Unit,
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> ();

    #[method(name = "GetRelianceSuffixLetter", args = 1)]
    pub fn get_reliance_suffix_letter(
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CanAnyRelianceLevelUp", args = 0)]
    pub fn can_any_reliance_level_up() -> bool;

    #[method(name = "CanAnyGodLevelUp", args = 0)]
    pub fn can_any_god_level_up() -> bool;

    #[method(name = "CanAnyRelianceLevelUp", args = 2)]
    pub fn can_any_reliance_level_up_2(pid: ::unity2::Il2CppString, without_a_plus: bool) -> bool;

    #[method(name = "CanAnyRelianceLevelUpPlayer", args = 2)]
    pub fn can_any_reliance_level_up_player(
        pid: ::unity2::Il2CppString,
        without_a_plus: bool,
    ) -> bool;

    #[method(name = "GetAPlusAsciiName", args = 0)]
    pub fn get_a_plus_ascii_name() -> ::unity2::Il2CppString;

    #[method(name = "ContinuousSortieCount", args = 1)]
    pub fn continuous_sortie_count(pid: ::unity2::Il2CppString) -> i32;

    #[method(name = "IsExistsMID", args = 1)]
    pub fn is_exists_mid(label: ::unity2::Il2CppString) -> bool;

    #[method(name = "HasStoryTalk", args = 1)]
    pub fn has_story_talk(pid: ::unity2::Il2CppString) -> bool;

    #[method(name = "HasSwimsuit", args = 1)]
    pub fn has_swimsuit(pid: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetSwimsuit", args = 0)]
    pub fn get_swimsuit() -> crate::app::accessorydata::AccessoryData;

    #[method(name = "GetAnimal", args = 1)]
    pub fn get_animal(locator: ::unity2::Il2CppString) -> crate::app::animaldata::AnimalData;

    #[method(name = "IsCaptureAnimal", args = 1)]
    pub fn is_capture_animal(animal: crate::app::animaldata::AnimalData) -> bool;

    #[method(name = "GetAnimalCaptureNum", args = 1)]
    pub fn get_animal_capture_num(animal: crate::app::animaldata::AnimalData) -> i32;

    #[method(name = "SetAnimalCaptureNum", args = 2)]
    pub fn set_animal_capture_num(animal: crate::app::animaldata::AnimalData, num: i32) -> ();

    #[method(name = "IncAnimalCaptureNum", args = 2)]
    pub fn inc_animal_capture_num(animal: crate::app::animaldata::AnimalData, num: i32) -> ();

    #[method(name = "get_EncountMaterialBase", args = 0)]
    pub fn get_encount_material_base() -> i32;

    #[method(name = "GetEncountMaterialItemCount", args = 0)]
    pub fn get_encount_material_item_count() -> i32;

    #[method(name = "GetItemCountWithBonus", args = 2)]
    pub fn get_item_count_with_bonus(item: crate::app::itemdata::ItemData, base_count: i32) -> i32;

    #[method(name = "UpdateLocate", args = 1)]
    pub fn update_locate(pid: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsFirstEntry", args = 0)]
    pub fn get_is_first_entry() -> bool;

    #[method(name = "set_IsFirstEntry", args = 1)]
    pub fn set_is_first_entry(value: bool) -> ();

    #[method(name = "get_LastScenarioChapter", args = 0)]
    pub fn get_last_scenario_chapter() -> ::unity2::Il2CppString;

    #[method(name = "set_LastScenarioChapter", args = 1)]
    pub fn set_last_scenario_chapter(value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsConditionMorning", args = 0)]
    pub fn get_is_condition_morning() -> bool;

    #[method(name = "set_IsConditionMorning", args = 1)]
    pub fn set_is_condition_morning(value: bool) -> ();

    #[method(name = "get_IsConditionDay", args = 0)]
    pub fn get_is_condition_day() -> bool;

    #[method(name = "set_IsConditionDay", args = 1)]
    pub fn set_is_condition_day(value: bool) -> ();

    #[method(name = "get_IsConditionEvening", args = 0)]
    pub fn get_is_condition_evening() -> bool;

    #[method(name = "set_IsConditionEvening", args = 1)]
    pub fn set_is_condition_evening(value: bool) -> ();

    #[method(name = "get_IsConditionNight", args = 0)]
    pub fn get_is_condition_night() -> bool;

    #[method(name = "set_IsConditionNight", args = 1)]
    pub fn set_is_condition_night(value: bool) -> ();

    #[method(name = "get_IsConditionWeaponOpened", args = 0)]
    pub fn get_is_condition_weapon_opened() -> bool;

    #[method(name = "set_IsConditionWeaponOpened", args = 1)]
    pub fn set_is_condition_weapon_opened(value: bool) -> ();

    #[method(name = "get_IsConditionItemOpened", args = 0)]
    pub fn get_is_condition_item_opened() -> bool;

    #[method(name = "set_IsConditionItemOpened", args = 1)]
    pub fn set_is_condition_item_opened(value: bool) -> ();

    #[method(name = "get_IsConditionRefinementOpened", args = 0)]
    pub fn get_is_condition_refinement_opened() -> bool;

    #[method(name = "set_IsConditionRefinementOpened", args = 1)]
    pub fn set_is_condition_refinement_opened(value: bool) -> ();

    #[method(name = "get_IsConditionAccessoryOpened", args = 0)]
    pub fn get_is_condition_accessory_opened() -> bool;

    #[method(name = "set_IsConditionAccessoryOpened", args = 1)]
    pub fn set_is_condition_accessory_opened(value: bool) -> ();

    #[method(name = "get_IsConditionTent1", args = 0)]
    pub fn get_is_condition_tent1() -> bool;

    #[method(name = "set_IsConditionTent1", args = 1)]
    pub fn set_is_condition_tent1(value: bool) -> ();

    #[method(name = "get_IsConditionTent2", args = 0)]
    pub fn get_is_condition_tent2() -> bool;

    #[method(name = "set_IsConditionTent2", args = 1)]
    pub fn set_is_condition_tent2(value: bool) -> ();

    #[method(name = "get_IsConditionTent3", args = 0)]
    pub fn get_is_condition_tent3() -> bool;

    #[method(name = "set_IsConditionTent3", args = 1)]
    pub fn set_is_condition_tent3(value: bool) -> ();

    #[method(name = "get_IsConditionFire", args = 0)]
    pub fn get_is_condition_fire() -> bool;

    #[method(name = "set_IsConditionFire", args = 1)]
    pub fn set_is_condition_fire(value: bool) -> ();

    #[method(name = "get_IsConditionFortuneHutOpened", args = 0)]
    pub fn get_is_condition_fortune_hut_opened() -> bool;

    #[method(name = "set_IsConditionFortuneHutOpened", args = 1)]
    pub fn set_is_condition_fortune_hut_opened(value: bool) -> ();

    #[method(name = "get_IsConditionFleaMarket", args = 0)]
    pub fn get_is_condition_flea_market() -> bool;

    #[method(name = "set_IsConditionFleaMarket", args = 1)]
    pub fn set_is_condition_flea_market(value: bool) -> ();

    #[method(name = "get_IsConditionStatue", args = 0)]
    pub fn get_is_condition_statue() -> bool;

    #[method(name = "set_IsConditionStatue", args = 1)]
    pub fn set_is_condition_statue(value: bool) -> ();

    #[method(name = "get_IsConditionRefineGodWeaponOpened", args = 0)]
    pub fn get_is_condition_refine_god_weapon_opened() -> bool;

    #[method(name = "set_IsConditionRefineGodWeaponOpened", args = 1)]
    pub fn set_is_condition_refine_god_weapon_opened(value: bool) -> ();

    #[method(name = "get_IsConditionMuscleHardOpened", args = 0)]
    pub fn get_is_condition_muscle_hard_opened() -> bool;

    #[method(name = "set_IsConditionMuscleHardOpened", args = 1)]
    pub fn set_is_condition_muscle_hard_opened(value: bool) -> ();

    #[method(name = "get_IsConditionMuscleMasterAndEndlessOpened", args = 0)]
    pub fn get_is_condition_muscle_master_and_endless_opened() -> bool;

    #[method(name = "set_IsConditionMuscleMasterAndEndlessOpened", args = 1)]
    pub fn set_is_condition_muscle_master_and_endless_opened(value: bool) -> ();

    #[method(name = "get_IsConditionDragonRideHardOpened", args = 0)]
    pub fn get_is_condition_dragon_ride_hard_opened() -> bool;

    #[method(name = "set_IsConditionDragonRideHardOpened", args = 1)]
    pub fn set_is_condition_dragon_ride_hard_opened(value: bool) -> ();

    #[method(name = "get_IsConditionDragonRideExpertOpened", args = 0)]
    pub fn get_is_condition_dragon_ride_expert_opened() -> bool;

    #[method(name = "set_IsConditionDragonRideExpertOpened", args = 1)]
    pub fn set_is_condition_dragon_ride_expert_opened(value: bool) -> ();

    #[method(name = "get_IsConditionPoolCircleSwim", args = 0)]
    pub fn get_is_condition_pool_circle_swim() -> bool;

    #[method(name = "set_IsConditionPoolCircleSwim", args = 1)]
    pub fn set_is_condition_pool_circle_swim(value: bool) -> ();

    #[method(name = "IsFirstAccessRefineGodWeapon", args = 0)]
    pub fn is_first_access_refine_god_weapon() -> bool;

    #[method(name = "SetFirstAccessRefineGodWeapon", args = 0)]
    pub fn set_first_access_refine_god_weapon() -> ();

    #[method(name = "get_CurrentCookingPid", args = 0)]
    pub fn get_current_cooking_pid() -> ::unity2::Il2CppString;

    #[method(name = "set_CurrentCookingPid", args = 1)]
    pub fn set_current_cooking_pid(value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PersonLimitCount", args = 0)]
    pub fn get_person_limit_count() -> i32;

    #[method(name = "get_GodPersonLimitCount", args = 0)]
    pub fn get_god_person_limit_count() -> i32;

    #[method(name = "get_AnimalLimitCount", args = 0)]
    pub fn get_animal_limit_count() -> i32;

    #[method(name = "get_KizunaPersonLimitCount", args = 0)]
    pub fn get_kizuna_person_limit_count() -> i32;

    #[method(name = "get_DragonRideLevel", args = 0)]
    pub fn get_dragon_ride_level() -> i32;

    #[method(name = "set_DragonRideLevel", args = 1)]
    pub fn set_dragon_ride_level(value: i32) -> ();

    #[method(name = "get_IsDragonRideTimeTest", args = 0)]
    pub fn get_is_dragon_ride_time_test() -> bool;

    #[method(name = "set_IsDragonRideTimeTest", args = 1)]
    pub fn set_is_dragon_ride_time_test(value: bool) -> ();

    #[method(name = "get_IsDragonRideWalkThrough", args = 0)]
    pub fn get_is_dragon_ride_walk_through() -> bool;

    #[method(name = "set_IsDragonRideWalkThrough", args = 1)]
    pub fn set_is_dragon_ride_walk_through(value: bool) -> ();

    #[method(name = "get_DragonRideTotalScore", args = 0)]
    pub fn get_dragon_ride_total_score() -> i32;

    #[method(name = "set_DragonRideTotalScore", args = 1)]
    pub fn set_dragon_ride_total_score(value: i32) -> ();

    #[method(name = "get_DragonRideNormalTargetCount", args = 0)]
    pub fn get_dragon_ride_normal_target_count() -> i32;

    #[method(name = "set_DragonRideNormalTargetCount", args = 1)]
    pub fn set_dragon_ride_normal_target_count(value: i32) -> ();

    #[method(name = "get_DragonRideBigTargetCount", args = 0)]
    pub fn get_dragon_ride_big_target_count() -> i32;

    #[method(name = "set_DragonRideBigTargetCount", args = 1)]
    pub fn set_dragon_ride_big_target_count(value: i32) -> ();

    #[method(name = "get_DragonRideLinkTargetCount", args = 0)]
    pub fn get_dragon_ride_link_target_count() -> i32;

    #[method(name = "set_DragonRideLinkTargetCount", args = 1)]
    pub fn set_dragon_ride_link_target_count(value: i32) -> ();

    #[method(name = "get_DragonRideSpecialTargetCount", args = 0)]
    pub fn get_dragon_ride_special_target_count() -> i32;

    #[method(name = "set_DragonRideSpecialTargetCount", args = 1)]
    pub fn set_dragon_ride_special_target_count(value: i32) -> ();

    #[method(name = "get_DragonRideRouletteTargetCount", args = 0)]
    pub fn get_dragon_ride_roulette_target_count() -> i32;

    #[method(name = "set_DragonRideRouletteTargetCount", args = 1)]
    pub fn set_dragon_ride_roulette_target_count(value: i32) -> ();

    #[method(name = "get_DragonRideAssistScore", args = 0)]
    pub fn get_dragon_ride_assist_score() -> i32;

    #[method(name = "set_DragonRideAssistScore", args = 1)]
    pub fn set_dragon_ride_assist_score(value: i32) -> ();

    #[method(name = "get_DragonRidePlayRankNum", args = 0)]
    pub fn get_dragon_ride_play_rank_num() -> i32;

    #[method(name = "set_DragonRidePlayRankNum", args = 1)]
    pub fn set_dragon_ride_play_rank_num(value: i32) -> ();

    #[method(name = "get_DragonRideRetireFlag", args = 0)]
    pub fn get_dragon_ride_retire_flag() -> bool;

    #[method(name = "set_DragonRideRetireFlag", args = 1)]
    pub fn set_dragon_ride_retire_flag(value: bool) -> ();

    #[method(name = "get_DragonRideWalkThroughList", args = 0)]
    pub fn get_dragon_ride_walk_through_list(
    ) -> ::unity2::Array<crate::app::dragonridepresetparamdata::DragonRidePresetParamData_CourseData>;

    #[method(name = "set_DragonRideWalkThroughList", args = 1)]
    pub fn set_dragon_ride_walk_through_list(
        value: ::unity2::Array<
            crate::app::dragonridepresetparamdata::DragonRidePresetParamData_CourseData,
        >,
    ) -> ();

    #[method(name = "get_DragonRideUsingPresetID", args = 0)]
    pub fn get_dragon_ride_using_preset_id() -> ::unity2::Il2CppString;

    #[method(name = "set_DragonRideUsingPresetID", args = 1)]
    pub fn set_dragon_ride_using_preset_id(value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DragonRideTargetSuicideSec", args = 0)]
    pub fn get_dragon_ride_target_suicide_sec() -> f32;

    #[method(name = "set_DragonRideTargetSuicideSec", args = 1)]
    pub fn set_dragon_ride_target_suicide_sec(value: f32) -> ();

    #[method(name = "get_DragonRideTargetSuicideSecRandomRange", args = 0)]
    pub fn get_dragon_ride_target_suicide_sec_random_range() -> f32;

    #[method(name = "set_DragonRideTargetSuicideSecRandomRange", args = 1)]
    pub fn set_dragon_ride_target_suicide_sec_random_range(value: f32) -> ();

    #[method(name = "get_DragonRideAssistLevel", args = 0)]
    pub fn get_dragon_ride_assist_level() -> i32;

    #[method(name = "set_DragonRideAssistLevel", args = 1)]
    pub fn set_dragon_ride_assist_level(value: i32) -> ();

    #[method(name = "get_DebugDragonRideAssistSet", args = 0)]
    pub fn get_debug_dragon_ride_assist_set() -> bool;

    #[method(name = "set_DebugDragonRideAssistSet", args = 1)]
    pub fn set_debug_dragon_ride_assist_set(value: bool) -> ();

    #[method(name = "get_IsDragonRideResultCheck", args = 0)]
    pub fn get_is_dragon_ride_result_check() -> bool;

    #[method(name = "set_IsDragonRideResultCheck", args = 1)]
    pub fn set_is_dragon_ride_result_check(value: bool) -> ();

    #[method(name = "get_IsDragonRideResultCheck_Assist", args = 0)]
    pub fn get_is_dragon_ride_result_check_assist() -> bool;

    #[method(name = "set_IsDragonRideResultCheck_Assist", args = 1)]
    pub fn set_is_dragon_ride_result_check_assist(value: bool) -> ();

    #[method(name = "get_IsDragonRideResultCheck_HighScore", args = 0)]
    pub fn get_is_dragon_ride_result_check_high_score() -> bool;

    #[method(name = "set_IsDragonRideResultCheck_HighScore", args = 1)]
    pub fn set_is_dragon_ride_result_check_high_score(value: bool) -> ();

    #[method(name = "get_FishingLureType", args = 0)]
    pub fn get_fishing_lure_type() -> i32;

    #[method(name = "set_FishingLureType", args = 1)]
    pub fn set_fishing_lure_type(value: i32) -> ();

    #[method(name = "get_FishingGetFishID", args = 0)]
    pub fn get_fishing_get_fish_id() -> ::unity2::Il2CppString;

    #[method(name = "set_FishingGetFishID", args = 1)]
    pub fn set_fishing_get_fish_id(value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FishingGetFishSize", args = 0)]
    pub fn get_fishing_get_fish_size() -> f32;

    #[method(name = "set_FishingGetFishSize", args = 1)]
    pub fn set_fishing_get_fish_size(value: f32) -> ();

    #[method(name = "get_IsPlayMuscleExercise", args = 0)]
    pub fn get_is_play_muscle_exercise() -> bool;

    #[method(name = "set_IsPlayMuscleExercise", args = 1)]
    pub fn set_is_play_muscle_exercise(value: bool) -> ();

    #[method(name = "InitializeCondition", args = 0)]
    pub fn initialize_condition() -> ();

    #[method(name = "FinalizeCondition", args = 0)]
    pub fn finalize_condition() -> ();

    #[method(name = "IsCondition", args = 1)]
    pub fn is_condition(condition_type: crate::app::hubutil::HubUtil_ConditionType) -> bool;

    #[method(name = "InitCookingPerson", args = 0)]
    pub fn init_cooking_person() -> ();

    #[method(name = "SetupCharacterDisposType", args = 2)]
    pub fn setup_character_dispos_type(
        chara: crate::combat::character::Character,
        dispos_type: crate::app::hubdisposdata::HubDisposData_DisposTypes,
    ) -> ();

    #[method(name = "SetupCharacterGroundHeight", args = 3)]
    pub fn setup_character_ground_height(
        chara: crate::combat::character::Character,
        ground_offset: f32,
        gravity: f32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hubutil")]
impl HubUtil {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubUtil),
                ::core::stringify!(new),
            )
        });
        <Self as IHubUtilMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubutil/HubUtil_TimezoneType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubUtil_TimezoneType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubUtil_TimezoneType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubUtil.TimezoneType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubUtil_TimezoneType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubUtil_TimezoneType {
    pub fn morning() -> Self {
        Self { value: 0 }
    }

    pub fn day() -> Self {
        Self { value: 1 }
    }

    pub fn evening() -> Self {
        Self { value: 2 }
    }

    pub fn night() -> Self {
        Self { value: 3 }
    }

    pub fn max() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubutil/HubUtil_ConditionType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubUtil_ConditionType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubUtil_ConditionType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubUtil.ConditionType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubUtil_ConditionType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubUtil_ConditionType {
    pub fn morning() -> Self {
        Self { value: 0 }
    }

    pub fn day() -> Self {
        Self { value: 1 }
    }

    pub fn evening() -> Self {
        Self { value: 2 }
    }

    pub fn night() -> Self {
        Self { value: 3 }
    }

    pub fn weapon_opened() -> Self {
        Self { value: 4 }
    }

    pub fn item_opened() -> Self {
        Self { value: 5 }
    }

    pub fn refinement_opened() -> Self {
        Self { value: 6 }
    }

    pub fn accessory_opened() -> Self {
        Self { value: 7 }
    }

    pub fn tent1() -> Self {
        Self { value: 8 }
    }

    pub fn tent2() -> Self {
        Self { value: 9 }
    }

    pub fn tent3() -> Self {
        Self { value: 10 }
    }

    pub fn fire() -> Self {
        Self { value: 11 }
    }

    pub fn fortune_hut_opened() -> Self {
        Self { value: 12 }
    }

    pub fn flea_market() -> Self {
        Self { value: 13 }
    }

    pub fn statue() -> Self {
        Self { value: 14 }
    }

    pub fn pool_circle_swim() -> Self {
        Self { value: 15 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubutil/HubUtil_BSpline.md")))]
#[::unity2::class(namespace = "App", name = "HubUtil.BSpline")]
#[parent(crate::system::object::Object)]
pub struct HubUtil_BSpline {}

#[cfg(feature = "app-hubutil")]
#[::unity2::methods]
impl HubUtil_BSpline {
    #[method(name = "Loop", args = 3)]
    pub fn r#loop(n: i32, min: i32, max: i32) -> i32;

    #[method(name = "Coefficient", args = 1)]
    pub fn coefficient(t: f32) -> f32;

    #[method(name = "Calc", args = 2)]
    pub fn calc(
        v: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        t: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CalcLoop", args = 2)]
    pub fn calc_loop(
        v: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
        t: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubutil")]
impl HubUtil_BSpline {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubUtil_BSpline),
                ::core::stringify!(new),
            )
        });
        <Self as IHubUtil_BSplineMethods>::ctor(this);
        this
    }
}
