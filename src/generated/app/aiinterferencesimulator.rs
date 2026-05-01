
use crate::app::aisimulatorbase::AISimulatorBase;
use crate::app::aisimulatorbase::IAISimulatorBase;
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aiinterferencesimulator/AIInterferenceSimulator.md")))]
#[::unity2::class(namespace = "App", name = "AIInterferenceSimulator")]
#[parent(crate::app::aisimulatorbase::AISimulatorBase)]
pub struct AIInterferenceSimulator {
    #[rename(name = "m_IsNotSuitable")]
    pub m_is_not_suitable: bool,
    #[rename(name = "m_flag")]
    pub m_flag: i32,
}

#[cfg(feature = "app-aiinterferencesimulator")]
#[::unity2::methods]
impl AIInterferenceSimulator {
    #[method(name = "get_IsNotSuitable", args = 0)]
    pub fn get_is_not_suitable(self) -> bool;

    #[method(name = "get_Hit", args = 0)]
    pub fn get_hit(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Prepare", args = 1)]
    pub fn prepare(self, flag: i32) -> ();

    #[method(name = "Calculate", args = 8)]
    pub fn calculate(
        self,
        offense: crate::app::unit::Unit,
        defense: crate::app::unit::Unit,
        x: i32,
        z: i32,
        range: i32,
        item_index: i32,
        defense_x: i32,
        defense_z: i32,
    ) -> ();

    #[method(name = "CalculateBattleInfo", args = 8)]
    pub fn calculate_battle_info(
        self,
        offense: crate::app::unit::Unit,
        defense: crate::app::unit::Unit,
        x: i32,
        z: i32,
        range: i32,
        item_index: i32,
        defense_x: i32,
        defense_z: i32,
    ) -> ();

    #[method(name = "CalculateScore", args = 0)]
    pub fn calculate_score(self) -> ();
}

#[cfg(feature = "app-aiinterferencesimulator")]
impl AIInterferenceSimulator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIInterferenceSimulator),
                ::core::stringify!(new),
            )
        });
        <Self as IAIInterferenceSimulatorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aiinterferencesimulator/AIInterferenceSimulator_Flag.md")))]
#[::unity2::class(namespace = "App", name = "AIInterferenceSimulator.Flag")]
#[parent(crate::app::bitfield32::BitField32)]
pub struct AIInterferenceSimulator_Flag {
    #[static_field]
    #[rename(name = "HighMagic")]
    pub high_magic: i32,
    #[static_field]
    #[rename(name = "LowMagic")]
    pub low_magic: i32,
}

#[cfg(feature = "app-aiinterferencesimulator")]
#[::unity2::methods]
impl AIInterferenceSimulator_Flag {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-aiinterferencesimulator")]
impl AIInterferenceSimulator_Flag {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIInterferenceSimulator_Flag),
                ::core::stringify!(new),
            )
        });
        <Self as IAIInterferenceSimulator_FlagMethods>::ctor(this);
        this
    }
}
