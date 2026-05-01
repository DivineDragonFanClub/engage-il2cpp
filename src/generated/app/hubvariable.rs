
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubvariable/HubVariable.md")))]
#[::unity2::class(namespace = "App", name = "HubVariable")]
#[parent(crate::system::object::Object)]
pub struct HubVariable {
    #[static_field]
    #[rename(name = "MaxAnimalNum")]
    pub max_animal_num: i32,
}

#[cfg(feature = "app-hubvariable")]
#[::unity2::methods]
impl HubVariable {
    #[method(name = "get_Variable", args = 0)]
    pub fn get_variable() -> crate::app::gamevariable::GameVariable;

    #[method(name = "get_RandomKeyName", args = 0)]
    pub fn get_random_key_name() -> ::unity2::Il2CppString;

    #[method(name = "get_RandomItemKeyName", args = 0)]
    pub fn get_random_item_key_name() -> ::unity2::Il2CppString;

    #[method(name = "get_TimezoneKeyName", args = 0)]
    pub fn get_timezone_key_name() -> ::unity2::Il2CppString;

    #[method(name = "get_SceneKey", args = 0)]
    pub fn get_scene_key() -> ::unity2::Il2CppString;

    #[method(name = "get_StartKey", args = 0)]
    pub fn get_start_key() -> ::unity2::Il2CppString;

    #[method(name = "get_EnterCountKeyName", args = 0)]
    pub fn get_enter_count_key_name() -> ::unity2::Il2CppString;

    #[method(name = "get_EnterHubKeyName", args = 0)]
    pub fn get_enter_hub_key_name() -> ::unity2::Il2CppString;

    #[method(name = "get_MascotPointName", args = 0)]
    pub fn get_mascot_point_name() -> ::unity2::Il2CppString;

    #[method(name = "get_MascotLimitPointName", args = 0)]
    pub fn get_mascot_limit_point_name() -> ::unity2::Il2CppString;

    #[method(name = "get_MascotEquipHeadAcc", args = 0)]
    pub fn get_mascot_equip_head_acc() -> ::unity2::Il2CppString;

    #[method(name = "get_MascotEquipTailAcc", args = 0)]
    pub fn get_mascot_equip_tail_acc() -> ::unity2::Il2CppString;

    #[method(name = "get_MascotColor", args = 0)]
    pub fn get_mascot_color() -> ::unity2::Il2CppString;

    #[method(name = "get_MascotFirstFlagName", args = 0)]
    pub fn get_mascot_first_flag_name() -> ::unity2::Il2CppString;

    #[method(name = "get_MascotDoneStrok", args = 0)]
    pub fn get_mascot_done_strok() -> ::unity2::Il2CppString;

    #[method(name = "get_MascotDoneEatFood", args = 0)]
    pub fn get_mascot_done_eat_food() -> ::unity2::Il2CppString;

    #[method(name = "get_MascotDoneChangeEquip", args = 0)]
    pub fn get_mascot_done_change_equip() -> ::unity2::Il2CppString;

    #[method(name = "get_DragonKingClassChanged", args = 0)]
    pub fn get_dragon_king_class_changed() -> ::unity2::Il2CppString;

    #[method(name = "get_PlacedRareAnimalFlagName", args = 0)]
    pub fn get_placed_rare_animal_flag_name() -> ::unity2::Il2CppString;

    #[method(name = "get_StatueConditionFlagName", args = 0)]
    pub fn get_statue_condition_flag_name() -> ::unity2::Il2CppString;

    #[method(name = "get_PromiseRingFlagName", args = 0)]
    pub fn get_promise_ring_flag_name() -> ::unity2::Il2CppString;

    #[method(name = "get_HasPromiseRing", args = 0)]
    pub fn get_has_promise_ring() -> bool;

    #[method(name = "set_HasPromiseRing", args = 1)]
    pub fn set_has_promise_ring(value: bool) -> ();

    #[method(name = "get_EnterCount", args = 0)]
    pub fn get_enter_count() -> i32;

    #[method(name = "set_EnterCount", args = 1)]
    pub fn set_enter_count(value: i32) -> ();

    #[method(name = "get_EnterHub", args = 0)]
    pub fn get_enter_hub() -> bool;

    #[method(name = "set_EnterHub", args = 1)]
    pub fn set_enter_hub(value: bool) -> ();

    #[method(name = "GetAnimalFlagName", args = 1)]
    pub fn get_animal_flag_name(index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetAnimalItemFlagName", args = 1)]
    pub fn get_animal_item_flag_name(index: i32) -> ::unity2::Il2CppString;

    #[method(name = "CreateGlobalFlags", args = 0)]
    pub fn create_global_flags() -> ();

    #[method(name = "CompletedChapter", args = 0)]
    pub fn completed_chapter() -> ();

    #[method(name = "SetupScene", args = 0)]
    pub fn setup_scene() -> ();

    #[method(name = "SetTimezone", args = 1)]
    pub fn set_timezone(timezone_type: crate::app::hubutil::HubUtil_TimezoneType) -> ();

    #[method(name = "SetCurrentScene", args = 2)]
    pub fn set_current_scene(
        scene_name: ::unity2::Il2CppString,
        start: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetCurrentStartName", args = 1)]
    pub fn set_current_start_name(start: ::unity2::Il2CppString) -> ();

    #[method(name = "ClearCurrentStartName", args = 0)]
    pub fn clear_current_start_name() -> ();

    #[method(name = "SetRandomSeed", args = 1)]
    pub fn set_random_seed(seed: i32) -> ();

    #[method(name = "GetRandomSeed", args = 0)]
    pub fn get_random_seed() -> i32;

    #[method(name = "get_StatueCondition", args = 0)]
    pub fn get_statue_condition() -> i32;

    #[method(name = "set_StatueCondition", args = 1)]
    pub fn set_statue_condition(value: i32) -> ();

    #[method(name = "GetTimezoneType", args = 0)]
    pub fn get_timezone_type() -> crate::app::hubutil::HubUtil_TimezoneType;

    #[method(name = "GetCurrentSceneName", args = 0)]
    pub fn get_current_scene_name() -> ::unity2::Il2CppString;

    #[method(name = "GetCurrentStartName", args = 0)]
    pub fn get_current_start_name() -> ::unity2::Il2CppString;

    #[method(name = "GetHubSolanelBgm", args = 0)]
    pub fn get_hub_solanel_bgm() -> ::unity2::Il2CppString;

    #[method(name = "SetHubSolanelBgm", args = 1)]
    pub fn set_hub_solanel_bgm(event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "IsAppearedRareAnimal", args = 1)]
    pub fn is_appeared_rare_animal(prefixless_cid: ::unity2::Il2CppString) -> bool;

    #[method(name = "SetChapterForDebug", args = 0)]
    pub fn set_chapter_for_debug() -> ();

    #[method(name = "SetupDebug_Option", args = 1)]
    pub fn setup_debug_option(is_done_first_event: bool) -> ();

    #[method(name = "SetupDebug_MiniGame", args = 0)]
    pub fn setup_debug_mini_game() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hubvariable")]
impl HubVariable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubVariable),
                ::core::stringify!(new),
            )
        });
        <Self as IHubVariableMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubvariable/HubVariable_Mascot.md")))]
#[::unity2::class(namespace = "App", name = "HubVariable.Mascot")]
#[parent(crate::system::object::Object)]
pub struct HubVariable_Mascot {}

#[cfg(feature = "app-hubvariable")]
#[::unity2::methods]
impl HubVariable_Mascot {
    #[method(name = "get_CanFollow", args = 0)]
    pub fn get_can_follow() -> bool;

    #[method(name = "IsFound", args = 0)]
    pub fn is_found() -> bool;

    #[method(name = "Found", args = 0)]
    pub fn found() -> ();

    #[method(name = "GetPoint", args = 0)]
    pub fn get_point() -> i32;

    #[method(name = "SetPoint", args = 1)]
    pub fn set_point(point: i32) -> ();

    #[method(name = "get_IgnorePoint", args = 0)]
    pub fn get_ignore_point() -> i32;

    #[method(name = "get_TurnLimitPoint", args = 0)]
    pub fn get_turn_limit_point() -> i32;

    #[method(name = "GetLimitPoint", args = 0)]
    pub fn get_limit_point() -> i32;

    #[method(name = "SetLimitPoint", args = 1)]
    pub fn set_limit_point(point: i32) -> ();

    #[method(name = "AddPoint", args = 1)]
    pub fn add_point(point: i32) -> ();

    #[method(name = "DecPoint", args = 1)]
    pub fn dec_point(point: i32) -> ();

    #[method(name = "SetHeadAccName", args = 1)]
    pub fn set_head_acc_name(acc_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetHeadAccName", args = 0)]
    pub fn get_head_acc_name() -> ::unity2::Il2CppString;

    #[method(name = "SetTailAccName", args = 1)]
    pub fn set_tail_acc_name(acc_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetTailAccName", args = 0)]
    pub fn get_tail_acc_name() -> ::unity2::Il2CppString;

    #[method(name = "SetColorIndex", args = 1)]
    pub fn set_color_index(color_index: i32) -> ();

    #[method(name = "GetColorIndex", args = 0)]
    pub fn get_color_index() -> i32;

    #[method(name = "DoneStrok", args = 0)]
    pub fn done_strok() -> ();

    #[method(name = "IsDoneStrok", args = 0)]
    pub fn is_done_strok() -> bool;

    #[method(name = "DoneEatFood", args = 0)]
    pub fn done_eat_food() -> ();

    #[method(name = "IsDoneEatFood", args = 0)]
    pub fn is_done_eat_food() -> bool;

    #[method(name = "DoneChangeEquip", args = 0)]
    pub fn done_change_equip() -> ();

    #[method(name = "IsDoneChangeEquip", args = 0)]
    pub fn is_done_change_equip() -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubvariable")]
impl HubVariable_Mascot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubVariable_Mascot),
                ::core::stringify!(new),
            )
        });
        <Self as IHubVariable_MascotMethods>::ctor(this);
        this
    }
}
