
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatstylefuncs/CombatStyleFuncs.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatStyleFuncs")]
#[parent(crate::system::object::Object)]
pub struct CombatStyleFuncs {}

#[cfg(feature = "combat-combatstylefuncs")]
#[::unity2::methods]
impl CombatStyleFuncs {
    #[method(name = "MakeStyle1st", args = 1)]
    pub fn make_style1st(
        record: crate::combat::combatrecord::CombatRecord,
    ) -> crate::combat::combatstyle::CombatStyle;

    #[method(name = "MakeStyle2nd", args = 1)]
    pub fn make_style2nd(
        record: crate::combat::combatrecord::CombatRecord,
    ) -> crate::combat::combatstyle::CombatStyle;

    #[method(name = "GetShootStyle", args = 1)]
    pub fn get_shoot_style(
        record: crate::combat::combatrecord::CombatRecord,
    ) -> crate::combat::combatstyle::CombatStyle;
}
