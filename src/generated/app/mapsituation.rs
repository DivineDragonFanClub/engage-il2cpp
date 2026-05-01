
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsituation/MapSituation_ForceCursor.md")))]
#[::unity2::class(namespace = "App", name = "MapSituation.ForceCursor")]
#[parent(crate::system::object::Object)]
pub struct MapSituation_ForceCursor {}

#[cfg(feature = "app-mapsituation")]
#[::unity2::methods]
impl MapSituation_ForceCursor {
    #[method(name = "Set", args = 2)]
    pub fn set(self, x: i32, z: i32) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_X", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "set_Z", args = 1)]
    pub fn set_z(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsituation")]
impl MapSituation_ForceCursor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSituation_ForceCursor),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSituation_ForceCursorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsituation/MapSituation_SubPhases.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSituation_SubPhases {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSituation_SubPhases {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSituation.SubPhases";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSituation_SubPhases {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSituation_SubPhases {
    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn charm_confusion() -> Self {
        Self { value: 1 }
    }

    pub fn num() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsituation/MapSituation_StatusField.md")))]
#[::unity2::class(namespace = "App", name = "MapSituation.StatusField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: mapsituation :: MapSituation_Status >)]
pub struct MapSituation_StatusField {}

#[cfg(feature = "app-mapsituation")]
#[::unity2::methods]
impl MapSituation_StatusField {
    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::mapsituation::MapSituation_Status) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsituation")]
impl MapSituation_StatusField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSituation_StatusField),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSituation_StatusFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsituation/MapSituation_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSituation_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSituation_Status {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSituation.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSituation_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSituation_Status {
    pub fn win_rule_breakdown() -> Self {
        Self { value: 1 }
    }

    pub fn win_rule_destroy_boss() -> Self {
        Self { value: 2 }
    }

    pub fn sequence_replay_cancel() -> Self {
        Self { value: 134217728 }
    }

    pub fn sequence_opening() -> Self {
        Self { value: 268435456 }
    }

    pub fn sequence_ai_entrust_cancel() -> Self {
        Self { value: 536870912 }
    }

    pub fn sequence_ai() -> Self {
        Self { value: 1073741824 }
    }

    pub fn sequence_mind() -> Self {
        Self { value: -2147483648 }
    }

    pub fn win_rule_mask() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsituation/MapSituation.md")))]
#[::unity2::class(namespace = "App", name = "MapSituation")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapsituation :: MapSituation >)]
pub struct MapSituation {
    #[static_field]
    #[rename(name = "TurnMax")]
    pub turn_max: i32,
    #[rename(name = "m_Status")]
    pub m_status: crate::app::mapsituation::MapSituation_StatusField,
    #[rename(name = "m_Players")]
    pub m_players: ::unity2::Array<crate::app::mapsituation::MapSituation_Player>,
    #[rename(name = "m_Groups")]
    pub m_groups: ::unity2::Array<crate::app::force::Force_Type>,
    #[rename(name = "m_CurrentForceType")]
    pub m_current_force_type: crate::app::force::Force_Type,
    #[rename(name = "m_HumanForceType")]
    pub m_human_force_type: crate::app::force::Force_Type,
    #[rename(name = "m_ForceCursors")]
    pub m_force_cursors: ::unity2::Array<crate::app::mapsituation::MapSituation_ForceCursor>,
    #[rename(name = "m_Turn")]
    pub m_turn: i32,
    #[rename(name = "m_SubPhase")]
    pub m_sub_phase: crate::app::mapsituation::MapSituation_SubPhases,
    #[rename(name = "m_WinRuleEnemyNumLessThanOrEqualTo")]
    pub m_win_rule_enemy_num_less_than_or_equal_to: i32,
    #[rename(name = "m_WinRuleLimitTurn")]
    pub m_win_rule_limit_turn: i32,
    #[rename(name = "m_WinLoseResult")]
    pub m_win_lose_result: crate::app::winloserule::WinLoseRule,
    #[rename(name = "m_Entrust")]
    pub m_entrust: crate::app::unitentrust::UnitEntrust_Type,
    #[rename(name = "m_WinRuleMID")]
    pub m_win_rule_mid: ::unity2::Il2CppString,
    #[rename(name = "m_WinRuleMIDArg")]
    pub m_win_rule_mid_arg: ::unity2::Il2CppString,
    #[rename(name = "m_LoseRuleMID")]
    pub m_lose_rule_mid: ::unity2::Il2CppString,
    #[rename(name = "m_LoseRuleMIDArg")]
    pub m_lose_rule_mid_arg: ::unity2::Il2CppString,
    #[rename(name = "m_AverageLevel")]
    pub m_average_level: i32,
}

#[cfg(feature = "app-mapsituation")]
#[::unity2::methods]
impl MapSituation {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetStatus", args = 0)]
    pub fn get_status(self) -> crate::app::mapsituation::MapSituation_StatusField;

    #[method(name = "SetStatus", args = 1)]
    pub fn set_status(self, status: crate::app::mapsituation::MapSituation_Status) -> ();

    #[method(name = "ClearStatus", args = 1)]
    pub fn clear_status(self, status: crate::app::mapsituation::MapSituation_Status) -> ();

    #[method(name = "CheckStatus", args = 1)]
    pub fn check_status(self, status: crate::app::mapsituation::MapSituation_Status) -> bool;

    #[method(name = "NotStatus", args = 1)]
    pub fn not_status(self, status: crate::app::mapsituation::MapSituation_Status) -> bool;

    #[method(name = "SetPlayer", args = 2)]
    pub fn set_player(
        self,
        force_type: crate::app::force::Force_Type,
        player: crate::app::mapsituation::MapSituation_Player,
    ) -> ();

    #[method(name = "GetPlayer", args = 1)]
    pub fn get_player(
        self,
        force_type: crate::app::force::Force_Type,
    ) -> crate::app::mapsituation::MapSituation_Player;

    #[method(name = "SetGroup", args = 2)]
    pub fn set_group(
        self,
        force_type: crate::app::force::Force_Type,
        group: crate::app::force::Force_Type,
    ) -> ();

    #[method(name = "GetGroup", args = 1)]
    pub fn get_group(
        self,
        force_type: crate::app::force::Force_Type,
    ) -> crate::app::force::Force_Type;

    #[method(name = "IsAllide", args = 2)]
    pub fn is_allide(
        self,
        force1: crate::app::force::Force_Type,
        force2: crate::app::force::Force_Type,
    ) -> bool;

    #[method(name = "GetCurrentPlayer", args = 0)]
    pub fn get_current_player(self) -> crate::app::mapsituation::MapSituation_Player;

    #[method(name = "GetCurrentForce", args = 0)]
    pub fn get_current_force(self) -> crate::app::force::Force;

    #[method(name = "IsHumanPlayer", args = 1)]
    pub fn is_human_player(self, force_type: crate::app::force::Force_Type) -> bool;

    #[method(name = "IsHumanPlayer", args = 0)]
    pub fn is_human_player_2(self) -> bool;

    #[method(name = "GetForceCursor", args = 1)]
    pub fn get_force_cursor(
        self,
        force_type: crate::app::force::Force_Type,
    ) -> crate::app::mapsituation::MapSituation_ForceCursor;

    #[method(name = "NextSubPhase", args = 0)]
    pub fn next_sub_phase(self) -> ();

    #[method(name = "ResetSubPhase", args = 0)]
    pub fn reset_sub_phase(self) -> ();

    #[method(name = "IsCasual", args = 0)]
    pub fn is_casual(self) -> bool;

    #[method(name = "IsPhoenix", args = 0)]
    pub fn is_phoenix(self) -> bool;

    #[method(name = "IsVersus", args = 0)]
    pub fn is_versus(self) -> bool;

    #[method(name = "IsRelay", args = 0)]
    pub fn is_relay(self) -> bool;

    #[method(name = "IsChallenge", args = 0)]
    pub fn is_challenge(self) -> bool;

    #[method(name = "IsTrial", args = 0)]
    pub fn is_trial(self) -> bool;

    #[method(name = "IsResurrect", args = 0)]
    pub fn is_resurrect(self) -> bool;

    #[method(name = "IsRecordKill", args = 1)]
    pub fn is_record_kill(self, force_type: crate::app::force::Force_Type) -> bool;

    #[method(name = "IsRecordDead", args = 1)]
    pub fn is_record_dead(self, force_type: crate::app::force::Force_Type) -> bool;

    #[method(name = "IsShowOrder", args = 0)]
    pub fn is_show_order(self) -> bool;

    #[method(name = "CanOrder", args = 0)]
    pub fn can_order(self) -> bool;

    #[method(name = "IsShowEscape", args = 0)]
    pub fn is_show_escape(self) -> bool;

    #[method(name = "CanEscape", args = 0)]
    pub fn can_escape(self) -> bool;

    #[method(name = "CanExistenceDie", args = 0)]
    pub fn can_existence_die(self) -> bool;

    #[method(name = "IsShowTurn", args = 0)]
    pub fn is_show_turn(self) -> bool;

    #[method(name = "IsComplete", args = 0)]
    pub fn is_complete(self) -> bool;

    #[method(name = "IsGameOver", args = 0)]
    pub fn is_game_over(self) -> bool;

    #[method(name = "IsRelayUnsettled", args = 0)]
    pub fn is_relay_unsettled(self) -> bool;

    #[method(name = "IsEntrustAI", args = 0)]
    pub fn is_entrust_ai(self) -> bool;

    #[method(name = "IsNotReturnMap", args = 0)]
    pub fn is_not_return_map(self) -> bool;

    #[method(name = "IsCancelOperation", args = 0)]
    pub fn is_cancel_operation(self) -> bool;

    #[method(name = "IsErrorOperation", args = 0)]
    pub fn is_error_operation(self) -> bool;

    #[method(name = "GetMessageWaitFrame", args = 0)]
    pub fn get_message_wait_frame(self) -> i32;

    #[method(name = "TurnEnd", args = 0)]
    pub fn turn_end(self) -> ();

    #[method(name = "TurnNext", args = 0)]
    pub fn turn_next(self) -> ();

    #[method(name = "GameEndCheck", args = 0)]
    pub fn game_end_check(self) -> ();

    #[method(name = "IsLoseUnitDead", args = 1)]
    pub fn is_lose_unit_dead(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "GameEndCheckUnitDead", args = 1)]
    pub fn game_end_check_unit_dead(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "SetComplete", args = 1)]
    pub fn set_complete(self, result: crate::app::winloserule::WinLoseRule) -> ();

    #[method(name = "SetGameOver", args = 1)]
    pub fn set_game_over(self, result: crate::app::winloserule::WinLoseRule) -> ();

    #[method(name = "SetRelayUnsettled", args = 0)]
    pub fn set_relay_unsettled(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "get_CurrentForceType", args = 0)]
    pub fn get_current_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "set_CurrentForceType", args = 1)]
    pub fn set_current_force_type(self, value: crate::app::force::Force_Type) -> ();

    #[method(name = "get_HumanForceType", args = 0)]
    pub fn get_human_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "set_HumanForceType", args = 1)]
    pub fn set_human_force_type(self, value: crate::app::force::Force_Type) -> ();

    #[method(name = "get_HeroForceType", args = 0)]
    pub fn get_hero_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "get_Turn", args = 0)]
    pub fn get_turn(self) -> i32;

    #[method(name = "set_Turn", args = 1)]
    pub fn set_turn(self, value: i32) -> ();

    #[method(name = "get_SubPhase", args = 0)]
    pub fn get_sub_phase(self) -> crate::app::mapsituation::MapSituation_SubPhases;

    #[method(name = "get_WinRuleEnemyNumLessThanOrEqualTo", args = 0)]
    pub fn get_win_rule_enemy_num_less_than_or_equal_to(self) -> i32;

    #[method(name = "set_WinRuleEnemyNumLessThanOrEqualTo", args = 1)]
    pub fn set_win_rule_enemy_num_less_than_or_equal_to(self, value: i32) -> ();

    #[method(name = "get_WinRuleLimitTurn", args = 0)]
    pub fn get_win_rule_limit_turn(self) -> i32;

    #[method(name = "set_WinRuleLimitTurn", args = 1)]
    pub fn set_win_rule_limit_turn(self, value: i32) -> ();

    #[method(name = "get_WinLoseResult", args = 0)]
    pub fn get_win_lose_result(self) -> crate::app::winloserule::WinLoseRule;

    #[method(name = "get_Entrust", args = 0)]
    pub fn get_entrust(self) -> crate::app::unitentrust::UnitEntrust_Type;

    #[method(name = "set_Entrust", args = 1)]
    pub fn set_entrust(self, value: crate::app::unitentrust::UnitEntrust_Type) -> ();

    #[method(name = "get_WinRuleMID", args = 0)]
    pub fn get_win_rule_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_WinRuleMID", args = 1)]
    pub fn set_win_rule_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_WinRuleMIDArg", args = 0)]
    pub fn get_win_rule_mid_arg(self) -> ::unity2::Il2CppString;

    #[method(name = "set_WinRuleMIDArg", args = 1)]
    pub fn set_win_rule_mid_arg(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LoseRuleMID", args = 0)]
    pub fn get_lose_rule_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LoseRuleMID", args = 1)]
    pub fn set_lose_rule_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LoseRuleMIDArg", args = 0)]
    pub fn get_lose_rule_mid_arg(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LoseRuleMIDArg", args = 1)]
    pub fn set_lose_rule_mid_arg(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AverageLevel", args = 0)]
    pub fn get_average_level(self) -> i32;

    #[method(name = "set_AverageLevel", args = 1)]
    pub fn set_average_level(self, value: i32) -> ();

    #[method(name = "GetWinRuleString", args = 0)]
    pub fn get_win_rule_string() -> ::unity2::Il2CppString;

    #[method(name = "GetLoseRuleString", args = 0)]
    pub fn get_lose_rule_string() -> ::unity2::Il2CppString;

    #[method(name = "IsHeroLose", args = 0)]
    pub fn is_hero_lose() -> bool;

    #[method(name = "GetHeroUnitForLoseRuleString", args = 0)]
    pub fn get_hero_unit_for_lose_rule_string() -> crate::app::unit::Unit;
}

#[cfg(feature = "app-mapsituation")]
impl MapSituation {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSituation),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSituationMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsituation/MapSituation_Player.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSituation_Player {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSituation_Player {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSituation.Player";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSituation_Player {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSituation_Player {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn human() -> Self {
        Self { value: 1 }
    }

    pub fn ai() -> Self {
        Self { value: 2 }
    }

    pub fn link() -> Self {
        Self { value: 3 }
    }

    pub fn replay() -> Self {
        Self { value: 4 }
    }

    pub fn num() -> Self {
        Self { value: 5 }
    }
}
