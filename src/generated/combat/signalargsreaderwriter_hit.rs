
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/signalargsreaderwriter_hit/SignalArgsReaderWriter_Hit.md")))]
#[::unity2::class(namespace = "Combat", name = "SignalArgsReaderWriter_Hit")]
#[parent(crate::system::object::Object)]
pub struct SignalArgsReaderWriter_Hit {}

#[cfg(feature = "combat-signalargsreaderwriter_hit")]
#[::unity2::methods]
impl SignalArgsReaderWriter_Hit {
    #[method(name = "HitIsDummy", args = 1)]
    pub fn hit_is_dummy(ev: crate::unity_engine::animationevent::AnimationEvent) -> bool;

    #[method(name = "HitIsDummy", args = 2)]
    pub fn hit_is_dummy_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        value: bool,
    ) -> ();

    #[method(name = "HitHandType", args = 1)]
    pub fn hit_hand_type(ev: crate::unity_engine::animationevent::AnimationEvent) -> i32;

    #[method(name = "HitHandType", args = 2)]
    pub fn hit_hand_type_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        value: i32,
    ) -> ();

    #[method(name = "HitSlashType", args = 2)]
    pub fn hit_slash_type(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        cp: crate::combat::character::Character,
    ) -> crate::combat::slashtype::SlashType;

    #[method(name = "HitSlashType", args = 2)]
    pub fn hit_slash_type_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        value: crate::combat::slashtype::SlashType,
    ) -> ();

    #[method(name = "HitSlashDir", args = 1)]
    pub fn hit_slash_dir(
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "HitSlashDir", args = 2)]
    pub fn hit_slash_dir_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        hit_dir: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "HitstopScale", args = 1)]
    pub fn hitstop_scale(
        ev: crate::unity_engine::animationevent::AnimationEvent,
    ) -> crate::combat::hitstopscale::HitstopScale;

    #[method(name = "HitstopScaleAsFloat", args = 1)]
    pub fn hitstop_scale_as_float(ev: crate::unity_engine::animationevent::AnimationEvent) -> f32;

    #[method(name = "HitstopScale", args = 2)]
    pub fn hitstop_scale_2(
        ev: crate::unity_engine::animationevent::AnimationEvent,
        value: i32,
    ) -> ();
}
