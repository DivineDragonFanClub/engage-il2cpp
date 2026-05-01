
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitreliancedata/UnitRelianceData.md")))]
#[::unity2::class(namespace = "App", name = "UnitRelianceData")]
#[parent(crate::system::object::Object)]
pub struct UnitRelianceData {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_RelianceExp")]
    pub m_reliance_exp: crate::app::relianceexpdata::RelianceExpData,
    #[rename(name = "m_Level")]
    pub m_level: crate::app::reliancedata::RelianceData_Level,
    #[rename(name = "m_Exp")]
    pub m_exp: i8,
    #[rename(name = "m_Score")]
    pub m_score: i8,
}

#[cfg(feature = "app-unitreliancedata")]
#[::unity2::methods]
impl UnitRelianceData {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, reliance_exp: crate::app::relianceexpdata::RelianceExpData) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "ResetMapBegin", args = 0)]
    pub fn reset_map_begin(self) -> ();

    #[method(name = "AddExp", args = 1)]
    pub fn add_exp(self, value: i32) -> ();

    #[method(name = "AddScore", args = 1)]
    pub fn add_score(self, value: i32) -> ();

    #[method(name = "CanLevelUp", args = 0)]
    pub fn can_level_up(self) -> bool;

    #[method(name = "LevelUp", args = 0)]
    pub fn level_up(self) -> ();

    #[method(name = "SetLevelAPlus", args = 0)]
    pub fn set_level_a_plus(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> crate::app::reliancedata::RelianceData_Level;

    #[method(name = "get_Exp", args = 0)]
    pub fn get_exp(self) -> i32;

    #[method(name = "set_Exp", args = 1)]
    pub fn set_exp(self, value: i32) -> ();

    #[method(name = "get_Score", args = 0)]
    pub fn get_score(self) -> i32;

    #[method(name = "set_Score", args = 1)]
    pub fn set_score(self, value: i32) -> ();

    #[method(name = "GetNextLevelExp", args = 1)]
    pub fn get_next_level_exp(
        self,
        current_level: crate::app::reliancedata::RelianceData_Level,
    ) -> i32;

    #[method(name = "GetClampExp", args = 0)]
    pub fn get_clamp_exp(self) -> i32;

    #[method(name = "ClampExp", args = 1)]
    pub fn clamp_exp(self, new_exp: i32) -> i8;

    #[method(name = "ClampScore", args = 1)]
    pub fn clamp_score(self, new_score: i32) -> i8;
}

#[cfg(feature = "app-unitreliancedata")]
impl UnitRelianceData {
    pub fn new(reliance_exp: crate::app::relianceexpdata::RelianceExpData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitRelianceData),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitRelianceDataMethods>::ctor(this, reliance_exp);
        this
    }
}
