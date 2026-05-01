
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characteranimset/CharacterAnimset.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterAnimset")]
#[parent(crate::system::object::Object)]
pub struct CharacterAnimset {
    #[rename(name = "BodyDAOC")]
    pub body_daoc: crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
    #[rename(name = "RideDAOC")]
    pub ride_daoc: crate::unity_engine::animatoroverridecontroller::AnimatorOverrideController,
    #[rename(name = "Attack1")]
    pub attack1: crate::combat::animasset::AnimAsset,
    #[rename(name = "Attack1R")]
    pub attack1_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Attack2")]
    pub attack2: crate::combat::animasset::AnimAsset,
    #[rename(name = "Attack2R")]
    pub attack2_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Attack3")]
    pub attack3: crate::combat::animasset::AnimAsset,
    #[rename(name = "Attack3R")]
    pub attack3_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Attack4")]
    pub attack4: crate::combat::animasset::AnimAsset,
    #[rename(name = "Attack4R")]
    pub attack4_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Attack5")]
    pub attack5: crate::combat::animasset::AnimAsset,
    #[rename(name = "Attack5R")]
    pub attack5_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "AttackC")]
    pub attack_c: crate::combat::animasset::AnimAsset,
    #[rename(name = "AttackCR")]
    pub attack_cr: crate::combat::animasset::AnimAsset,
    #[rename(name = "AttackT")]
    pub attack_t: crate::combat::animasset::AnimAsset,
    #[rename(name = "AttackTR")]
    pub attack_tr: crate::combat::animasset::AnimAsset,
    #[rename(name = "DamageHigh")]
    pub damage_high: crate::combat::animasset::AnimAsset,
    #[rename(name = "DamageHighR")]
    pub damage_high_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "DamageMidB")]
    pub damage_mid_b: crate::combat::animasset::AnimAsset,
    #[rename(name = "DamageMidBR")]
    pub damage_mid_br: crate::combat::animasset::AnimAsset,
    #[rename(name = "DamageMidDU")]
    pub damage_mid_du: crate::combat::animasset::AnimAsset,
    #[rename(name = "DamageMidDUR")]
    pub damage_mid_dur: crate::combat::animasset::AnimAsset,
    #[rename(name = "DamageMidUD")]
    pub damage_mid_ud: crate::combat::animasset::AnimAsset,
    #[rename(name = "DamageMidUDR")]
    pub damage_mid_udr: crate::combat::animasset::AnimAsset,
    #[rename(name = "DieB")]
    pub die_b: crate::combat::animasset::AnimAsset,
    #[rename(name = "DieBR")]
    pub die_br: crate::combat::animasset::AnimAsset,
    #[rename(name = "DieL")]
    pub die_l: crate::combat::animasset::AnimAsset,
    #[rename(name = "DieLR")]
    pub die_lr: crate::combat::animasset::AnimAsset,
    #[rename(name = "DieR")]
    pub die_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "DieRR")]
    pub die_rr: crate::combat::animasset::AnimAsset,
    #[rename(name = "Dive")]
    pub dive: crate::combat::animasset::AnimAsset,
    #[rename(name = "DiveR")]
    pub dive_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Engage1")]
    pub engage1: crate::combat::animasset::AnimAsset,
    #[rename(name = "Engage1R")]
    pub engage1_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Engage2")]
    pub engage2: crate::combat::animasset::AnimAsset,
    #[rename(name = "Engage2R")]
    pub engage2_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Engage3")]
    pub engage3: crate::combat::animasset::AnimAsset,
    #[rename(name = "Engage3R")]
    pub engage3_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "EvasionB")]
    pub evasion_b: crate::combat::animasset::AnimAsset,
    #[rename(name = "EvasionBR")]
    pub evasion_br: crate::combat::animasset::AnimAsset,
    #[rename(name = "EvasionL")]
    pub evasion_l: crate::combat::animasset::AnimAsset,
    #[rename(name = "EvasionLR")]
    pub evasion_lr: crate::combat::animasset::AnimAsset,
    #[rename(name = "EvasionR")]
    pub evasion_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "EvasionRR")]
    pub evasion_rr: crate::combat::animasset::AnimAsset,
    #[rename(name = "Guard")]
    pub guard: crate::combat::animasset::AnimAsset,
    #[rename(name = "GuardR")]
    pub guard_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "HoveringLoop")]
    pub hovering_loop: crate::combat::animasset::AnimAsset,
    #[rename(name = "HoveringLoopR")]
    pub hovering_loop_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "IdleDying")]
    pub idle_dying: crate::combat::animasset::AnimAsset,
    #[rename(name = "IdleDyingR")]
    pub idle_dying_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "IdleNormal")]
    pub idle_normal: crate::combat::animasset::AnimAsset,
    #[rename(name = "IdleNormalR")]
    pub idle_normal_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "ParryL")]
    pub parry_l: crate::combat::animasset::AnimAsset,
    #[rename(name = "ParryLR")]
    pub parry_lr: crate::combat::animasset::AnimAsset,
    #[rename(name = "ParryR")]
    pub parry_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "ParryRR")]
    pub parry_rr: crate::combat::animasset::AnimAsset,
    #[rename(name = "Ready")]
    pub ready: crate::combat::animasset::AnimAsset,
    #[rename(name = "ReadyR")]
    pub ready_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "RelaxLoop")]
    pub relax_loop: crate::combat::animasset::AnimAsset,
    #[rename(name = "RelaxLoopR")]
    pub relax_loop_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Repelled")]
    pub repelled: crate::combat::animasset::AnimAsset,
    #[rename(name = "RepelledR")]
    pub repelled_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "RunLoop")]
    pub run_loop: crate::combat::animasset::AnimAsset,
    #[rename(name = "RunLoopR")]
    pub run_loop_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "RunStart")]
    pub run_start: crate::combat::animasset::AnimAsset,
    #[rename(name = "RunStartR")]
    pub run_start_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Special1")]
    pub special1: crate::combat::animasset::AnimAsset,
    #[rename(name = "Special1R")]
    pub special1_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Start")]
    pub start: crate::combat::animasset::AnimAsset,
    #[rename(name = "StartR")]
    pub start_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "Win")]
    pub win: crate::combat::animasset::AnimAsset,
    #[rename(name = "WinR")]
    pub win_r: crate::combat::animasset::AnimAsset,
    #[rename(name = "WinLoop")]
    pub win_loop: crate::combat::animasset::AnimAsset,
    #[rename(name = "WinLoopR")]
    pub win_loop_r: crate::combat::animasset::AnimAsset,
}

#[cfg(feature = "combat-characteranimset")]
#[::unity2::methods]
impl CharacterAnimset {
    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(a: crate::combat::characteranimset::CharacterAnimset) -> bool;

    #[method(name = "get_PreloadAnims", args = 0)]
    pub fn get_preload_anims(self) -> crate::combat::preloadanims::PreloadAnims;

    #[method(name = "set_PreloadAnims", args = 1)]
    pub fn set_preload_anims(self, value: crate::combat::preloadanims::PreloadAnims) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        animset_names: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        weapon_style: crate::combat::weaponstyle::WeaponStyle,
    ) -> ();

    #[method(name = "IsInDB", args = 1)]
    pub fn is_in_db(animset_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "Override", args = 1)]
    pub fn r#override(self, animset_name: ::unity2::Il2CppString) -> ();

    #[method(name = "MaskUnusedAnims", args = 1)]
    pub fn mask_unused_anims(self, f: crate::combat::preloadanims::PreloadAnims) -> ();

    #[method(name = "GetKeyword", args = 1)]
    pub fn get_keyword(s: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "MakeDAOC", args = 1)]
    pub fn make_daoc(self, ap: crate::combat::characterappearance::CharacterAppearance) -> ();

    #[method(name = "UseEngage2", args = 0)]
    pub fn use_engage2(self) -> ();

    #[method(name = "Has", args = 1)]
    pub fn has(self, hash: i32) -> bool;

    #[method(name = "HasDie", args = 0)]
    pub fn has_die(self) -> bool;

    #[method(name = "GetAnimAssetsNameList", args = 0)]
    pub fn get_anim_assets_name_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "InsertNML", args = 2)]
    pub fn insert_nml(
        aset: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        my_nml: u16,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "get_HasWinAsStandingDie", args = 0)]
    pub fn get_has_win_as_standing_die(self) -> bool;

    #[method(name = "UpdateStatistics", args = 1)]
    pub fn update_statistics(chr: crate::combat::character::Character) -> ();

    #[method(name = "get_CanBeSmashed", args = 0)]
    pub fn get_can_be_smashed(self) -> bool;

    #[method(name = "SelectImpl", args = 3)]
    pub fn select_impl(
        self,
        shuffle: bool,
        last_hash: i32,
        candidates: ::unity2::Array<i32>,
    ) -> i32;

    #[method(name = "SelectRandom", args = 2)]
    pub fn select_random(self, lash_hash: i32, candidates: ::unity2::Array<i32>) -> i32;

    #[method(name = "SelectRandomAttack12345", args = 1)]
    pub fn select_random_attack12345(self, last_hash: i32) -> i32;

    #[method(name = "SelectInOrder", args = 1)]
    pub fn select_in_order(self, candidates: ::unity2::Array<i32>) -> i32;
}

#[cfg(feature = "combat-characteranimset")]
impl CharacterAnimset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterAnimset),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterAnimsetMethods>::ctor(this);
        this
    }

    pub fn new_2(
        animset_names: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        weapon_style: crate::combat::weaponstyle::WeaponStyle,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterAnimset),
                ::core::stringify!(new_2),
            )
        });
        <Self as ICharacterAnimsetMethods>::ctor_2(this, animset_names, weapon_style);
        this
    }
}
