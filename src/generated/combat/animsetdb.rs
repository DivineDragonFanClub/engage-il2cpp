
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/animsetdb/AnimSetDB.md")))]
#[::unity2::class(namespace = "Combat", name = "AnimSetDB")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: combat :: animsetdb :: AnimSetDB >)]
pub struct AnimSetDB {}

#[cfg(feature = "combat-animsetdb")]
#[::unity2::methods]
impl AnimSetDB {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Attack1", args = 0)]
    pub fn get_attack1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Attack1", args = 1)]
    pub fn set_attack1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Attack2", args = 0)]
    pub fn get_attack2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Attack2", args = 1)]
    pub fn set_attack2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Attack3", args = 0)]
    pub fn get_attack3(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Attack3", args = 1)]
    pub fn set_attack3(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Attack4", args = 0)]
    pub fn get_attack4(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Attack4", args = 1)]
    pub fn set_attack4(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Attack5", args = 0)]
    pub fn get_attack5(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Attack5", args = 1)]
    pub fn set_attack5(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AttackC", args = 0)]
    pub fn get_attack_c(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AttackC", args = 1)]
    pub fn set_attack_c(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AttackT", args = 0)]
    pub fn get_attack_t(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AttackT", args = 1)]
    pub fn set_attack_t(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DamageHigh", args = 0)]
    pub fn get_damage_high(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DamageHigh", args = 1)]
    pub fn set_damage_high(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DamageMidB", args = 0)]
    pub fn get_damage_mid_b(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DamageMidB", args = 1)]
    pub fn set_damage_mid_b(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DamageMidDU", args = 0)]
    pub fn get_damage_mid_du(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DamageMidDU", args = 1)]
    pub fn set_damage_mid_du(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DamageMidUD", args = 0)]
    pub fn get_damage_mid_ud(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DamageMidUD", args = 1)]
    pub fn set_damage_mid_ud(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DieB", args = 0)]
    pub fn get_die_b(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DieB", args = 1)]
    pub fn set_die_b(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DieL", args = 0)]
    pub fn get_die_l(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DieL", args = 1)]
    pub fn set_die_l(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DieR", args = 0)]
    pub fn get_die_r(self) -> ::unity2::Il2CppString;

    #[method(name = "set_DieR", args = 1)]
    pub fn set_die_r(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Dive", args = 0)]
    pub fn get_dive(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Dive", args = 1)]
    pub fn set_dive(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Engage1", args = 0)]
    pub fn get_engage1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Engage1", args = 1)]
    pub fn set_engage1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Engage2", args = 0)]
    pub fn get_engage2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Engage2", args = 1)]
    pub fn set_engage2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Engage3", args = 0)]
    pub fn get_engage3(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Engage3", args = 1)]
    pub fn set_engage3(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EvasionB", args = 0)]
    pub fn get_evasion_b(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EvasionB", args = 1)]
    pub fn set_evasion_b(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EvasionL", args = 0)]
    pub fn get_evasion_l(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EvasionL", args = 1)]
    pub fn set_evasion_l(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EvasionR", args = 0)]
    pub fn get_evasion_r(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EvasionR", args = 1)]
    pub fn set_evasion_r(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Guard", args = 0)]
    pub fn get_guard(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Guard", args = 1)]
    pub fn set_guard(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HoveringLoop", args = 0)]
    pub fn get_hovering_loop(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HoveringLoop", args = 1)]
    pub fn set_hovering_loop(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IdleDying", args = 0)]
    pub fn get_idle_dying(self) -> ::unity2::Il2CppString;

    #[method(name = "set_IdleDying", args = 1)]
    pub fn set_idle_dying(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IdleNormal", args = 0)]
    pub fn get_idle_normal(self) -> ::unity2::Il2CppString;

    #[method(name = "set_IdleNormal", args = 1)]
    pub fn set_idle_normal(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ParryL", args = 0)]
    pub fn get_parry_l(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ParryL", args = 1)]
    pub fn set_parry_l(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ParryR", args = 0)]
    pub fn get_parry_r(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ParryR", args = 1)]
    pub fn set_parry_r(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Ready", args = 0)]
    pub fn get_ready(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Ready", args = 1)]
    pub fn set_ready(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RelaxLoop", args = 0)]
    pub fn get_relax_loop(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RelaxLoop", args = 1)]
    pub fn set_relax_loop(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Repelled", args = 0)]
    pub fn get_repelled(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Repelled", args = 1)]
    pub fn set_repelled(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RunLoop", args = 0)]
    pub fn get_run_loop(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RunLoop", args = 1)]
    pub fn set_run_loop(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RunStart", args = 0)]
    pub fn get_run_start(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RunStart", args = 1)]
    pub fn set_run_start(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Special1", args = 0)]
    pub fn get_special1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Special1", args = 1)]
    pub fn set_special1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Start", args = 0)]
    pub fn get_start(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Start", args = 1)]
    pub fn set_start(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Win", args = 0)]
    pub fn get_win(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Win", args = 1)]
    pub fn set_win(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_WinLoop", args = 0)]
    pub fn get_win_loop(self) -> ::unity2::Il2CppString;

    #[method(name = "set_WinLoop", args = 1)]
    pub fn set_win_loop(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-animsetdb")]
impl AnimSetDB {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimSetDB),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimSetDBMethods>::ctor(this);
        this
    }
}
