
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptmap/ScriptMap.md")))]
#[::unity2::class(namespace = "App", name = "ScriptMap")]
#[parent(crate::app::scriptutil::ScriptUtil)]
pub struct ScriptMap {}

#[cfg(feature = "app-scriptmap")]
#[::unity2::methods]
impl ScriptMap {
    #[method(name = "MindGetForce", args = 1)]
    pub fn mind_get_force(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MindGetUnit", args = 1)]
    pub fn mind_get_unit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MindGetTargetUnit", args = 1)]
    pub fn mind_get_target_unit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MindGetEventUnit", args = 1)]
    pub fn mind_get_event_unit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CursorGetX", args = 1)]
    pub fn cursor_get_x(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CursorGetZ", args = 1)]
    pub fn cursor_get_z(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CursorSetPos", args = 1)]
    pub fn cursor_set_pos(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "CursorSetVisible", args = 1)]
    pub fn cursor_set_visible(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "CursorGetDistanceMode", args = 1)]
    pub fn cursor_get_distance_mode(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CursorSetDistanceMode", args = 1)]
    pub fn cursor_set_distance_mode(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "CursorSetDistanceScale", args = 1)]
    pub fn cursor_set_distance_scale(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "TerrainGet", args = 1)]
    pub fn terrain_get(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "TerrainSetBegin", args = 1)]
    pub fn terrain_set_begin(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "TerrainSetEnd", args = 1)]
    pub fn terrain_set_end(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "TerrainSet", args = 1)]
    pub fn terrain_set(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "TerrainSetOne", args = 1)]
    pub fn terrain_set_one(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "TerrainSetImpl", args = 2)]
    pub fn terrain_set_impl(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        is_multi: bool,
    ) -> ();

    #[method(name = "TerrainFill", args = 1)]
    pub fn terrain_fill(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "TerrainGetMoveCost", args = 1)]
    pub fn terrain_get_move_cost(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MapOverlapSetBegin", args = 1)]
    pub fn map_overlap_set_begin(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapOverlapSetEnd", args = 1)]
    pub fn map_overlap_set_end(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapOverlapSet", args = 1)]
    pub fn map_overlap_set(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapOverlapSetOne", args = 1)]
    pub fn map_overlap_set_one(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapOverlapSetImpl", args = 2)]
    pub fn map_overlap_set_impl(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        is_multi: bool,
    ) -> ();

    #[method(name = "MapOverlapGet", args = 1)]
    pub fn map_overlap_get(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MapOverlapRemove", args = 1)]
    pub fn map_overlap_remove(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapOverlapRemoveImpl", args = 2)]
    pub fn map_overlap_remove_impl(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        is_multi: bool,
    ) -> ();

    #[method(name = "MapRangeAddBegin", args = 1)]
    pub fn map_range_add_begin(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapRangeAddEnd", args = 1)]
    pub fn map_range_add_end(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapRangeAdd", args = 1)]
    pub fn map_range_add(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapRangeClear", args = 1)]
    pub fn map_range_clear(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Dispos", args = 1)]
    pub fn dispos(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> ();

    #[method(name = "DisposGetGroupCount", args = 1)]
    pub fn dispos_get_group_count(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DisposGetUnitX", args = 1)]
    pub fn dispos_get_unit_x(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "DisposGetUnitZ", args = 1)]
    pub fn dispos_get_unit_z(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GetDisposData", args = 2)]
    pub fn get_dispos_data(
        group: ::unity2::Il2CppString,
        index: i32,
    ) -> crate::app::disposdata::DisposData;

    #[method(name = "MapDamageBegin", args = 1)]
    pub fn map_damage_begin(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapDamageAdd", args = 1)]
    pub fn map_damage_add(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapDamageEnd", args = 1)]
    pub fn map_damage_end(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Battle", args = 1)]
    pub fn battle(args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>) -> ();

    #[method(name = "BattleSetAttack", args = 1)]
    pub fn battle_set_attack(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "BattleAddTarget", args = 1)]
    pub fn battle_add_target(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "BattleStart", args = 1)]
    pub fn battle_start(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapGetTurn", args = 1)]
    pub fn map_get_turn(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MapGetPhase", args = 1)]
    pub fn map_get_phase(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MapGetAverageLevel", args = 1)]
    pub fn map_get_average_level(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MapGetPosition", args = 1)]
    pub fn map_get_position(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MapGetHeight", args = 1)]
    pub fn map_get_height(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MapIsSight", args = 1)]
    pub fn map_is_sight(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "MapSetSight", args = 1)]
    pub fn map_set_sight(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapIsRecollection", args = 1)]
    pub fn map_is_recollection(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "WinRuleSet", args = 2)]
    pub fn win_rule_set(enable: bool, status: crate::app::mapsituation::MapSituation_Status) -> ();

    #[method(name = "WinRuleSetBreakdown", args = 1)]
    pub fn win_rule_set_breakdown(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "WinRuleSetDestroyBoss", args = 1)]
    pub fn win_rule_set_destroy_boss(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "WinRuleSetEnemyNumberLessThanOrEqualTo", args = 1)]
    pub fn win_rule_set_enemy_number_less_than_or_equal_to(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "WinRuleSetLimitTurn", args = 1)]
    pub fn win_rule_set_limit_turn(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "WinRuleSetMID", args = 1)]
    pub fn win_rule_set_mid(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "LoseRuleSetMID", args = 1)]
    pub fn lose_rule_set_mid(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "TryGetEffectArg", args = 4)]
    pub fn try_get_effect_arg(
        name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
        rotation: crate::unity_engine::quaternion::Quaternion,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> bool;

    #[method(name = "EffectPlay", args = 1)]
    pub fn effect_play(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EffectIsPlaying", args = 1)]
    pub fn effect_is_playing(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "EffectCreate", args = 1)]
    pub fn effect_create(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EffectDelete", args = 1)]
    pub fn effect_delete(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryTurn", args = 1)]
    pub fn event_entry_turn(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryTurnAfter", args = 1)]
    pub fn event_entry_turn_after(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryTurnEnd", args = 1)]
    pub fn event_entry_turn_end(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryArea", args = 1)]
    pub fn event_entry_area(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryDie", args = 1)]
    pub fn event_entry_die(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryReviveBefore", args = 1)]
    pub fn event_entry_revive_before(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryReviveAfter", args = 1)]
    pub fn event_entry_revive_after(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryFixed", args = 1)]
    pub fn event_entry_fixed(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryTalk", args = 1)]
    pub fn event_entry_talk(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryBattleBefore", args = 1)]
    pub fn event_entry_battle_before(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryBattleTalk", args = 1)]
    pub fn event_entry_battle_talk(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryBattleAfter", args = 1)]
    pub fn event_entry_battle_after(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryEscape", args = 1)]
    pub fn event_entry_escape(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryBreakdown", args = 1)]
    pub fn event_entry_breakdown(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryBreakdownEnemy", args = 1)]
    pub fn event_entry_breakdown_enemy(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryWaypoint", args = 1)]
    pub fn event_entry_waypoint(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryCommand", args = 1)]
    pub fn event_entry_command(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryPickup", args = 1)]
    pub fn event_entry_pickup(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryTargetSelect", args = 1)]
    pub fn event_entry_target_select(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryUnitCommandPrepare", args = 1)]
    pub fn event_entry_unit_command_prepare(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryUnitCommandInterrupt", args = 1)]
    pub fn event_entry_unit_command_interrupt(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryTbox", args = 1)]
    pub fn event_entry_tbox(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryVisit", args = 1)]
    pub fn event_entry_visit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryDoor", args = 1)]
    pub fn event_entry_door(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryDestroy", args = 1)]
    pub fn event_entry_destroy(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventOpenObject", args = 1)]
    pub fn event_open_object(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventOpenDoor", args = 1)]
    pub fn event_open_door(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventBrokenObject", args = 1)]
    pub fn event_broken_object(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventActionObject", args = 1)]
    pub fn event_action_object(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventActionMoveObject", args = 1)]
    pub fn event_action_move_object(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventStateObject", args = 1)]
    pub fn event_state_object(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventIsPlayingObject", args = 1)]
    pub fn event_is_playing_object(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "EventIsPlayingSkyCastle", args = 1)]
    pub fn event_is_playing_sky_castle(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "EventEngageSummon", args = 1)]
    pub fn event_engage_summon(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryEngageBefore", args = 1)]
    pub fn event_entry_engage_before(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "EventEntryEngageAfter", args = 1)]
    pub fn event_entry_engage_after(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapCameraIsScroll", args = 1)]
    pub fn map_camera_is_scroll(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "TurnEnd", args = 1)]
    pub fn turn_end(
        agrs: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapHistoryRewindEnable", args = 1)]
    pub fn map_history_rewind_enable(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapHistoryRewindDisable", args = 1)]
    pub fn map_history_rewind_disable(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapHistoryRewindReset", args = 1)]
    pub fn map_history_rewind_reset(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapHistoryMindDone", args = 1)]
    pub fn map_history_mind_done(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapHistoryEngageBreak", args = 1)]
    pub fn map_history_engage_break(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapHistoryPositionListBegin", args = 1)]
    pub fn map_history_position_list_begin(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapHistoryPositionList", args = 1)]
    pub fn map_history_position_list(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapHistoryPositionListEnd", args = 1)]
    pub fn map_history_position_list_end(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapMaterialSetFloat", args = 1)]
    pub fn map_material_set_float(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "MapMaterialSetColor", args = 1)]
    pub fn map_material_set_color(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GodSaveEquip", args = 1)]
    pub fn god_save_equip(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GodLoadEquip", args = 1)]
    pub fn god_load_equip(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Regist", args = 1)]
    pub fn regist(script: crate::app::eventscript::EventScript) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-scriptmap")]
impl ScriptMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptMap),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptMapMethods>::ctor(this);
        this
    }
}
