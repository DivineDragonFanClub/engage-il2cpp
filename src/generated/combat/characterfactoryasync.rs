
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterfactoryasync/CharacterFactoryAsync.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterFactoryAsync")]
#[parent(crate::system::object::Object)]
pub struct CharacterFactoryAsync {}

#[cfg(feature = "combat-characterfactoryasync")]
#[::unity2::methods]
impl CharacterFactoryAsync {
    #[method(name = "Create", args = 3)]
    pub fn create(
        appearance: crate::combat::characterappearance::CharacterAppearance,
        parent: crate::unity_engine::transform::Transform,
        invisible: bool,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateForTalk", args = 3)]
    pub fn create_for_talk(
        appearance: crate::combat::characterappearance::CharacterAppearance,
        parent: crate::unity_engine::transform::Transform,
        invisible: bool,
    ) -> crate::combat::character::Character;

    #[method(name = "CreateImpl", args = 4)]
    pub fn create_impl(
        asset_path: ::unity2::Il2CppString,
        appearance: crate::combat::characterappearance::CharacterAppearance,
        parent: crate::unity_engine::transform::Transform,
        invisible: bool,
    ) -> crate::combat::character::Character;
}
