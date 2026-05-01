
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatstyleexmethods/CombatStyleExMethods.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatStyleExMethods")]
#[parent(crate::system::object::Object)]
pub struct CombatStyleExMethods {}

#[cfg(feature = "combat-combatstyleexmethods")]
#[::unity2::methods]
impl CombatStyleExMethods {
    #[method(name = "Any", args = 2)]
    pub fn any(
        lhs: crate::combat::combatstyle::CombatStyle,
        rhs: crate::combat::combatstyle::CombatStyle,
    ) -> bool;

    #[method(name = "NoneOf", args = 2)]
    pub fn none_of(
        lhs: crate::combat::combatstyle::CombatStyle,
        rhs: crate::combat::combatstyle::CombatStyle,
    ) -> bool;

    #[method(name = "All", args = 2)]
    pub fn all(
        lhs: crate::combat::combatstyle::CombatStyle,
        rhs: crate::combat::combatstyle::CombatStyle,
    ) -> bool;
}
