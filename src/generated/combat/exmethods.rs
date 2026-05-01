
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/exmethods/ExMethods.md")))]
#[::unity2::class(namespace = "Combat", name = "ExMethods")]
#[parent(crate::system::object::Object)]
pub struct ExMethods {}

#[cfg(feature = "combat-exmethods")]
#[::unity2::methods]
impl ExMethods {
    #[method(name = "Any", args = 2)]
    pub fn any(
        lhs: crate::combat::phase::Phase_HitType,
        rhs: crate::combat::phase::Phase_HitType,
    ) -> bool;

    #[method(name = "NoneOf", args = 2)]
    pub fn none_of(
        lhs: crate::combat::phase::Phase_HitType,
        rhs: crate::combat::phase::Phase_HitType,
    ) -> bool;

    #[method(name = "All", args = 2)]
    pub fn all(
        lhs: crate::combat::phase::Phase_HitType,
        rhs: crate::combat::phase::Phase_HitType,
    ) -> bool;

    #[method(name = "Change", args = 3)]
    pub fn change(
        self_: crate::combat::phase::Phase_HitType,
        src: crate::combat::phase::Phase_HitType,
        dst: crate::combat::phase::Phase_HitType,
    ) -> ();

    #[method(name = "Any", args = 2)]
    pub fn any_2(
        lhs: crate::combat::phase::Phase_Detail,
        rhs: crate::combat::phase::Phase_Detail,
    ) -> bool;

    #[method(name = "NoneOf", args = 2)]
    pub fn none_of_2(
        lhs: crate::combat::phase::Phase_Detail,
        rhs: crate::combat::phase::Phase_Detail,
    ) -> bool;

    #[method(name = "All", args = 2)]
    pub fn all_2(
        lhs: crate::combat::phase::Phase_Detail,
        rhs: crate::combat::phase::Phase_Detail,
    ) -> bool;

    #[method(name = "Change", args = 3)]
    pub fn change_2(
        self_: crate::combat::phase::Phase_Detail,
        src: crate::combat::phase::Phase_Detail,
        dst: crate::combat::phase::Phase_Detail,
    ) -> ();
}
