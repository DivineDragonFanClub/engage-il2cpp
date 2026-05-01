
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::actiondisposerholder::ActionDisposerHolder;
use crate::combat::actiondisposerholder::IActionDisposerHolder;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actiongranbase/ActionGranBase_MoveAct.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ActionGranBase_MoveAct {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ActionGranBase_MoveAct {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "ActionGranBase.MoveAct";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ActionGranBase_MoveAct {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ActionGranBase_MoveAct {
    pub fn waiting() -> Self {
        Self { value: 0 }
    }

    pub fn running() -> Self {
        Self { value: 1 }
    }

    pub fn backstep() -> Self {
        Self { value: 2 }
    }

    pub fn warp() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actiongranbase/ActionGranBase.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionGranBase")]
#[parent(crate::combat::actiondisposerholder::ActionDisposerHolder)]
pub struct ActionGranBase {
    #[static_field]
    #[rename(name = "AttackLineBehind")]
    pub attack_line_behind: f32,
    #[static_field]
    #[rename(name = "DamageLineBehind")]
    pub damage_line_behind: f32,
    #[static_field]
    #[rename(name = "BackstepDistance")]
    pub backstep_distance: f32,
    #[rename(name = "warpedGoal")]
    pub warped_goal: crate::combat::fxz::FXZ,
}

#[cfg(feature = "combat-actiongranbase")]
#[::unity2::methods]
impl ActionGranBase {
    #[method(name = "get_IsSkyLandCombat", args = 0)]
    pub fn get_is_sky_land_combat(self) -> bool;

    #[method(name = "MasterIsFarAndGrandewIsNear", args = 1)]
    pub fn master_is_far_and_grandew_is_near(ghr: crate::combat::character::Character) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, ghr: crate::combat::character::Character) -> ();

    #[method(name = "isskyland", args = 0)]
    pub fn isskyland() -> bool;

    #[method(name = "get_moveAct", args = 0)]
    pub fn get_move_act(self) -> crate::combat::actiongranbase::ActionGranBase_MoveAct;

    #[method(name = "set_moveAct", args = 1)]
    pub fn set_move_act(self, value: crate::combat::actiongranbase::ActionGranBase_MoveAct) -> ();

    #[method(name = "MoveTo", args = 1)]
    pub fn move_to(self, goal: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "RunTo", args = 1)]
    pub fn run_to(self, goal: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "Warp", args = 1)]
    pub fn warp(self, goal: crate::combat::fxz::FXZ) -> f32;

    #[method(name = "MoveEnd", args = 0)]
    pub fn move_end(self) -> ();

    #[method(name = "GetEvasionHash", args = 0)]
    pub fn get_evasion_hash(self) -> i32;

    #[method(name = "CalcAttackTR", args = 1)]
    pub fn calc_attack_tr(ghr: crate::combat::character::Character) -> crate::combat::tr::TR;

    #[method(name = "CalcAttackNNTR", args = 1)]
    pub fn calc_attack_nntr(ghr: crate::combat::character::Character) -> crate::combat::tr::TR;

    #[method(name = "CalcAttackNFTR", args = 1)]
    pub fn calc_attack_nftr(ghr: crate::combat::character::Character) -> crate::combat::tr::TR;

    #[method(name = "CalcAttackFFTR", args = 1)]
    pub fn calc_attack_fftr(ghr: crate::combat::character::Character) -> crate::combat::tr::TR;

    #[method(name = "CalcDamageTR", args = 1)]
    pub fn calc_damage_tr(ghr: crate::combat::character::Character) -> crate::combat::tr::TR;

    #[method(name = "AdjustGroundLevel", args = 1)]
    pub fn adjust_ground_level(self, side: i32) -> ();
}

#[cfg(feature = "combat-actiongranbase")]
impl ActionGranBase {
    pub fn new(ghr: crate::combat::character::Character) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionGranBase),
                ::core::stringify!(new),
            )
        });
        <Self as IActionGranBaseMethods>::ctor(this, ghr);
        this
    }
}
