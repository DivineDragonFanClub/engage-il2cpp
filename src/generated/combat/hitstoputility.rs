
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/hitstoputility/HitStopUtility.md")))]
#[::unity2::class(namespace = "Combat", name = "HitStopUtility")]
#[parent(crate::system::object::Object)]
pub struct HitStopUtility {}

#[cfg(feature = "combat-hitstoputility")]
#[::unity2::methods]
impl HitStopUtility {
    #[method(name = "CalcHitStopBaseTime", args = 1)]
    pub fn calc_hit_stop_base_time(phase: crate::combat::phase::Phase) -> f32;
}
