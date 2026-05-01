
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/animname/AnimName_Race.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AnimName_Race {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AnimName_Race {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "AnimName.Race";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AnimName_Race {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AnimName_Race {
    pub fn unknown() -> Self {
        Self { value: 0 }
    }

    pub fn human() -> Self {
        Self { value: 1 }
    }

    pub fn horse() -> Self {
        Self { value: 2 }
    }

    pub fn wolf() -> Self {
        Self { value: 3 }
    }

    pub fn dragon() -> Self {
        Self { value: 4 }
    }

    pub fn pegasus() -> Self {
        Self { value: 5 }
    }

    pub fn griffon() -> Self {
        Self { value: 6 }
    }

    pub fn dragonic() -> Self {
        Self { value: 7 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/animname/AnimName.md")))]
#[::unity2::class(namespace = "Combat", name = "AnimName")]
#[parent(crate::system::object::Object)]
pub struct AnimName {}

#[cfg(feature = "combat-animname")]
#[::unity2::methods]
impl AnimName {
    #[method(name = "GetRace", args = 1)]
    pub fn get_race(
        c: crate::combat::character::Character,
    ) -> crate::combat::animname::AnimName_Race;

    #[method(name = "GetRace", args = 1)]
    pub fn get_race_2(
        gs: crate::combat::charactergamestatus::CharacterGameStatus,
    ) -> crate::combat::animname::AnimName_Race;

    #[method(name = "GetRace", args = 1)]
    pub fn get_race_3(name: ::unity2::Il2CppString) -> crate::combat::animname::AnimName_Race;

    #[method(name = "IsHeavy", args = 1)]
    pub fn is_heavy(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsRidingHuman", args = 1)]
    pub fn is_riding_human(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsVehicle", args = 1)]
    pub fn is_vehicle(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "ToRideName", args = 1)]
    pub fn to_ride_name(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "IsUnknown", args = 1)]
    pub fn is_unknown(race: crate::combat::animname::AnimName_Race) -> bool;

    #[method(name = "IsUnknown", args = 1)]
    pub fn is_unknown_2(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsHuman", args = 1)]
    pub fn is_human(race: crate::combat::animname::AnimName_Race) -> bool;

    #[method(name = "IsHuman", args = 1)]
    pub fn is_human_2(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsRide", args = 1)]
    pub fn is_ride(race: crate::combat::animname::AnimName_Race) -> bool;

    #[method(name = "IsRide", args = 1)]
    pub fn is_ride_2(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "Is4Legged", args = 1)]
    pub fn is4_legged(race: crate::combat::animname::AnimName_Race) -> bool;

    #[method(name = "Is4Legged", args = 1)]
    pub fn is4_legged_2(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsFlying", args = 1)]
    pub fn is_flying(race: crate::combat::animname::AnimName_Race) -> bool;

    #[method(name = "IsFlying", args = 1)]
    pub fn is_flying_2(name: ::unity2::Il2CppString) -> bool;
}
