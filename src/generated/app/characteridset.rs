
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/characteridset/CharacterIdSet.md")))]
#[::unity2::class(namespace = "App", name = "CharacterIdSet")]
#[parent(crate::system::object::Object)]
pub struct CharacterIdSet {
    #[rename(name = "Pid")]
    pub pid: ::unity2::Il2CppString,
    #[rename(name = "Jid")]
    pub jid: ::unity2::Il2CppString,
    #[rename(name = "Sid")]
    pub sid: ::unity2::Il2CppString,
    #[rename(name = "Iid")]
    pub iid: ::unity2::Il2CppString,
    #[rename(name = "Position")]
    pub position: ::unity2::Array<i32>,
    #[rename(name = "Lv")]
    pub lv: i32,
    #[rename(name = "Hp")]
    pub hp: f32,
    #[rename(name = "Exp")]
    pub exp: i32,
    #[rename(name = "IsEngage")]
    pub is_engage: bool,
    #[rename(name = "IsMale")]
    pub is_male: bool,
}

#[cfg(feature = "app-characteridset")]
#[::unity2::methods]
impl CharacterIdSet {
    #[method(name = "get_JobNameList", args = 0)]
    pub fn get_job_name_list(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_JobNameList", args = 1)]
    pub fn set_job_name_list(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_WeaponNameList", args = 0)]
    pub fn get_weapon_name_list(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_WeaponNameList", args = 1)]
    pub fn set_weapon_name_list(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_SkillNameList", args = 0)]
    pub fn get_skill_name_list(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_SkillNameList", args = 1)]
    pub fn set_skill_name_list(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "GetHash", args = 0)]
    pub fn get_hash(self) -> i32;

    #[method(name = "Validate", args = 0)]
    pub fn validate(self) -> ();

    #[method(name = "SetupJobAndWeaponList", args = 1)]
    pub fn setup_job_and_weapon_list(self, set_random: bool) -> ();

    #[method(name = "SetupWeaponList", args = 1)]
    pub fn setup_weapon_list(self, set_random: bool) -> ();

    #[method(name = "SetupSkillList", args = 1)]
    pub fn setup_skill_list(self, set_random: bool) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-characteridset")]
impl CharacterIdSet {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterIdSet),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterIdSetMethods>::ctor(this);
        this
    }
}
