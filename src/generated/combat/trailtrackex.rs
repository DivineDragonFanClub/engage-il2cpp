
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/trailtrackex/TrailTrackEx.md")))]
#[::unity2::class(namespace = "Combat", name = "TrailTrackEx")]
#[parent(crate::system::object::Object)]
pub struct TrailTrackEx {}

#[cfg(feature = "combat-trailtrackex")]
#[::unity2::methods]
impl TrailTrackEx {
    #[method(name = "IsNull", args = 1)]
    pub fn is_null(a: crate::combat::trailtrack::TrailTrack) -> bool;

    #[method(name = "IsNotNull", args = 1)]
    pub fn is_not_null(a: crate::combat::trailtrack::TrailTrack) -> bool;
}
