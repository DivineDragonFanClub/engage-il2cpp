
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/encountmob/EncountMob.md")))]
#[::unity2::class(namespace = "App", name = "EncountMob")]
#[parent(crate::system::object::Object)]
pub struct EncountMob {
    #[rename(name = "m_EJid")]
    pub m_e_jid: ::unity2::Il2CppString,
    #[rename(name = "m_Type")]
    pub m_type: crate::app::gmapspot::GmapSpot_EncountPersonType,
    #[rename(name = "m_Rank")]
    pub m_rank: u8,
    #[rename(name = "m_Cycle")]
    pub m_cycle: u8,
    #[rename(name = "m_PlayerDispos")]
    pub m_player_dispos: u8,
    #[rename(name = "m_EnemyDispos")]
    pub m_enemy_dispos: u8,
    #[rename(name = "m_Seed")]
    pub m_seed: crate::app::randomseed::RandomSeed,
    #[rename(name = "m_RareType")]
    pub m_rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    #[rename(name = "m_ExpRareCount")]
    pub m_exp_rare_count: u8,
    #[rename(name = "m_GoldRareCount")]
    pub m_gold_rare_count: u8,
    #[rename(name = "m_LeaderJid")]
    pub m_leader_jid: ::unity2::Il2CppString,
    #[rename(name = "m_LeaderGender")]
    pub m_leader_gender: u8,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
}

#[cfg(feature = "app-encountmob")]
#[::unity2::methods]
impl EncountMob {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ::unity2::Il2CppString;

    #[method(name = "Appear", args = 4)]
    pub fn appear(
        self,
        r#type: crate::app::gmapspot::GmapSpot_EncountPersonType,
        rank: i32,
        ejid: ::unity2::Il2CppString,
        spot_id: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "DecideEncountRare", args = 3)]
    pub fn decide_encount_rare(
        self,
        spot_id: ::unity2::Il2CppString,
        rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
        rare_num: i32,
    ) -> bool;

    #[method(name = "DecideEnemyType", args = 1)]
    pub fn decide_enemy_type(
        self,
        nation_name: ::unity2::Il2CppString,
    ) -> crate::app::encountunitdata::EncountUnitData_RareType;

    #[method(name = "GetEnemyRate", args = 2)]
    pub fn get_enemy_rate(
        self,
        nation_name: ::unity2::Il2CppString,
        rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    ) -> i32;

    #[method(name = "GetEnemyRateAvarage", args = 1)]
    pub fn get_enemy_rate_avarage(
        self,
        rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    ) -> i32;

    #[method(name = "DecideEnemyNum", args = 1)]
    pub fn decide_enemy_num(self, nation_level: i32) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SetRandomEncountJob", args = 1)]
    pub fn set_random_encount_job(self, ejid: ::unity2::Il2CppString) -> ();

    #[method(name = "GetEncountJob", args = 4)]
    pub fn get_encount_job(
        self,
        random: crate::app::random_2::Random_2,
        level: i32,
        is_leader: bool,
        is_ignore_mage_cannon: bool,
    ) -> crate::app::jobdata::JobData;

    #[method(name = "GetLeaderJob", args = 0)]
    pub fn get_leader_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "GetEncountPerson", args = 3)]
    pub fn get_encount_person(
        self,
        random: crate::app::random_2::Random_2,
        job: crate::app::jobdata::JobData,
        unit_rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    ) -> crate::app::persondata::PersonData;

    #[method(name = "GetLeaderPerson", args = 1)]
    pub fn get_leader_person(
        self,
        unit_rare_type: crate::app::encountunitdata::EncountUnitData_RareType,
    ) -> crate::app::persondata::PersonData;

    #[method(name = "GetIconPerson", args = 0)]
    pub fn get_icon_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "GetRareCount", args = 0)]
    pub fn get_rare_count(self) -> i32;

    #[method(name = "TryGetHighJobPerson", args = 2)]
    pub fn try_get_high_job_person(
        person: crate::app::persondata::PersonData,
        job: crate::app::jobdata::JobData,
    ) -> crate::app::persondata::PersonData;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Correct", args = 0)]
    pub fn correct(self) -> ();

    #[method(name = "get_Rank", args = 0)]
    pub fn get_rank(self) -> i32;

    #[method(name = "set_Rank", args = 1)]
    pub fn set_rank(self, value: i32) -> ();

    #[method(name = "get_LeaderRank", args = 0)]
    pub fn get_leader_rank(self) -> i32;

    #[method(name = "get_Cycle", args = 0)]
    pub fn get_cycle(self) -> i32;

    #[method(name = "set_Cycle", args = 1)]
    pub fn set_cycle(self, value: i32) -> ();

    #[method(name = "get_PlayerDispos", args = 0)]
    pub fn get_player_dispos(self) -> i32;

    #[method(name = "set_PlayerDispos", args = 1)]
    pub fn set_player_dispos(self, value: i32) -> ();

    #[method(name = "get_EnemyDispos", args = 0)]
    pub fn get_enemy_dispos(self) -> i32;

    #[method(name = "set_EnemyDispos", args = 1)]
    pub fn set_enemy_dispos(self, value: i32) -> ();

    #[method(name = "get_Seed", args = 0)]
    pub fn get_seed(self) -> crate::app::randomseed::RandomSeed;

    #[method(name = "get_RareType", args = 0)]
    pub fn get_rare_type(self) -> crate::app::encountunitdata::EncountUnitData_RareType;

    #[method(name = "set_ExpRareCount", args = 1)]
    pub fn set_exp_rare_count(self, value: i32) -> ();

    #[method(name = "set_GoldRareCount", args = 1)]
    pub fn set_gold_rare_count(self, value: i32) -> ();

    #[method(name = "get_LeaderGender", args = 0)]
    pub fn get_leader_gender(self) -> crate::app::gender::Gender;

    #[method(name = "set_LeaderGender", args = 1)]
    pub fn set_leader_gender(self, value: crate::app::gender::Gender) -> ();
}

#[cfg(feature = "app-encountmob")]
impl EncountMob {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EncountMob),
                ::core::stringify!(new),
            )
        });
        <Self as IEncountMobMethods>::ctor(this);
        this
    }
}
