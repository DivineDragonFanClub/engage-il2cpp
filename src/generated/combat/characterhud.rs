
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterhud/CharacterHUD.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterHUD")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterHUD {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
}

#[cfg(feature = "combat-characterhud")]
#[::unity2::methods]
impl CharacterHUD {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "In", args = 0)]
    pub fn r#in(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "TryUpdateValues", args = 3)]
    pub fn try_update_values(self, hp: i32, maxhp: i32, engage_count: i32) -> ();

    #[method(name = "PushShowSkill", args = 1)]
    pub fn push_show_skill(self, skill_data: crate::app::skilldata::SkillData) -> ();

    #[method(name = "Out", args = 1)]
    pub fn out(self, ntime: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characterhud")]
impl CharacterHUD {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterHUD),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterHUDMethods>::ctor(this);
        this
    }
}
