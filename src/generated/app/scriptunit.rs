
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptunit/ScriptUnit.md")))]
#[::unity2::class(namespace = "App", name = "ScriptUnit")]
#[parent(crate::app::scriptutil::ScriptUtil)]
pub struct ScriptUnit {}

#[cfg(feature = "app-scriptunit")]
#[::unity2::methods]
impl ScriptUnit {
    #[method(name = "ForceUnitGetFirst", args = 1)]
    pub fn force_unit_get_first(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ForceUnitGetNext", args = 1)]
    pub fn force_unit_get_next(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "ForceUnitDelete", args = 1)]
    pub fn force_unit_delete(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitIsExist", args = 1)]
    pub fn unit_is_exist(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetByPID", args = 1)]
    pub fn unit_get_by_pid(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetByPos", args = 1)]
    pub fn unit_get_by_pos(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetPID", args = 1)]
    pub fn unit_get_pid(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetJID", args = 1)]
    pub fn unit_get_jid(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetMPID", args = 1)]
    pub fn unit_get_mpid(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitIsStatus", args = 1)]
    pub fn unit_is_status(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitSetStatus", args = 1)]
    pub fn unit_set_status(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitClearStatus", args = 1)]
    pub fn unit_clear_status(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitSetShow", args = 1)]
    pub fn unit_set_show(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitGetForce", args = 1)]
    pub fn unit_get_force(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetLevel", args = 1)]
    pub fn unit_get_level(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetHp", args = 1)]
    pub fn unit_get_hp(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetMoveCost", args = 1)]
    pub fn unit_get_move_cost(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitSetHp", args = 1)]
    pub fn unit_set_hp(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitResetParam", args = 1)]
    pub fn unit_reset_param(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitUpdate", args = 1)]
    pub fn unit_update(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitGetCapability", args = 1)]
    pub fn unit_get_capability(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitDie", args = 1)]
    pub fn unit_die(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitDieWithoutEvent", args = 1)]
    pub fn unit_die_without_event(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitTransferImpl", args = 2)]
    pub fn unit_transfer_impl(
        unit: crate::app::unit::Unit,
        force: crate::app::force::Force_Type,
    ) -> ();

    #[method(name = "UnitTransfer", args = 1)]
    pub fn unit_transfer(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitDelete", args = 1)]
    pub fn unit_delete(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitJoinImpl", args = 2)]
    pub fn unit_join_impl(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> crate::app::persondata::PersonData;

    #[method(name = "UnitJoin", args = 1)]
    pub fn unit_join(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitJoinSilent", args = 1)]
    pub fn unit_join_silent(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitSetPrivateSkill", args = 1)]
    pub fn unit_set_private_skill(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitClearPrivateSkill", args = 1)]
    pub fn unit_clear_private_skill(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitHasPrivateSkill", args = 1)]
    pub fn unit_has_private_skill(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitHasWholeSkill", args = 1)]
    pub fn unit_has_whole_skill(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetX", args = 1)]
    pub fn unit_get_x(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetZ", args = 1)]
    pub fn unit_get_z(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitCanEnter", args = 1)]
    pub fn unit_can_enter(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetItemCount", args = 1)]
    pub fn unit_get_item_count(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetItemRangeI", args = 1)]
    pub fn unit_get_item_range_i(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetItemRangeO", args = 1)]
    pub fn unit_get_item_range_o(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitSetItemEquip", args = 1)]
    pub fn unit_set_item_equip(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitPutOffItem", args = 1)]
    pub fn unit_put_off_item(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "AiSetActive", args = 1)]
    pub fn ai_set_active(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "AiGetActive", args = 1)]
    pub fn ai_get_active(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "AiGetBandNo", args = 1)]
    pub fn ai_get_band_no(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "AiSetBandNo", args = 1)]
    pub fn ai_set_band_no(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "AiSetPriority", args = 1)]
    pub fn ai_set_priority(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "AiSetSequence", args = 1)]
    pub fn ai_set_sequence(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "AiGetSequence", args = 1)]
    pub fn ai_get_sequence(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "AiSetRerewarp", args = 1)]
    pub fn ai_set_rerewarp(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "AiGetRerewarpPosition", args = 1)]
    pub fn ai_get_rerewarp_position(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "AiSetRejectPower0Attack", args = 1)]
    pub fn ai_set_reject_power0_attack(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "AiClearMoveLimit", args = 1)]
    pub fn ai_clear_move_limit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "CanEnter", args = 3)]
    pub fn can_enter(unit: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "TryGetMovePos", args = 5)]
    pub fn try_get_move_pos(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        flag: crate::app::mapmoveflag::MapMoveFlag,
    ) -> bool;

    #[method(name = "UnitSetPos", args = 1)]
    pub fn unit_set_pos(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitMovePos", args = 1)]
    pub fn unit_move_pos(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitSyncSkyCastle", args = 1)]
    pub fn unit_sync_sky_castle(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitJumpPos", args = 1)]
    pub fn unit_jump_pos(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitRotation", args = 1)]
    pub fn unit_rotation(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitWarpIn", args = 1)]
    pub fn unit_warp_in(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitWarpOut", args = 1)]
    pub fn unit_warp_out(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitTranslation", args = 1)]
    pub fn unit_translation(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitIsAction", args = 1)]
    pub fn unit_is_action(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitPlayAnim", args = 1)]
    pub fn unit_play_anim(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitGetEngageCount", args = 1)]
    pub fn unit_get_engage_count(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitSetEngageCount", args = 1)]
    pub fn unit_set_engage_count(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitGetEngaging", args = 1)]
    pub fn unit_get_engaging(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UpdateImage", args = 0)]
    pub fn update_image() -> ();

    #[method(name = "UnitSetEngaging", args = 1)]
    pub fn unit_set_engaging(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitSetGodUnit", args = 1)]
    pub fn unit_set_god_unit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitGetGodUnit", args = 1)]
    pub fn unit_get_god_unit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GodUnitCreate", args = 1)]
    pub fn god_unit_create(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GodUnitDelete", args = 1)]
    pub fn god_unit_delete(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GodUnitSetDarkness", args = 1)]
    pub fn god_unit_set_darkness(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GodUnitSetEscape", args = 1)]
    pub fn god_unit_set_escape(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GodUnitExists", args = 1)]
    pub fn god_unit_exists(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "GodDataGetMGID", args = 1)]
    pub fn god_data_get_mgid(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetGodStateCount", args = 1)]
    pub fn unit_get_god_state_count(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetGodState", args = 1)]
    pub fn unit_get_god_state(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitSetGodState", args = 1)]
    pub fn unit_set_god_state(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitReliancePermitAPlus", args = 1)]
    pub fn unit_reliance_permit_a_plus(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitShine", args = 1)]
    pub fn unit_shine(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitSetHpStock", args = 1)]
    pub fn unit_set_hp_stock(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "UnitGetHpStock", args = 1)]
    pub fn unit_get_hp_stock(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "UnitGetHpStockMax", args = 1)]
    pub fn unit_get_hp_stock_max(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Regist", args = 1)]
    pub fn regist(script: crate::app::eventscript::EventScript) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-scriptunit")]
impl ScriptUnit {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptUnit),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptUnitMethods>::ctor(this);
        this
    }
}
