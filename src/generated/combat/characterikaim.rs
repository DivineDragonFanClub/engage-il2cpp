
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/characterikaim/CharacterIKAim.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterIKAim")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterIKAim {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "AnimAimDir")]
    pub anim_aim_dir: crate::unity_engine::vector3::Vector3,
    #[rename(name = "CurrentAimDir")]
    pub current_aim_dir: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "combat-characterikaim")]
#[::unity2::methods]
impl CharacterIKAim {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "get_Parabola", args = 0)]
    pub fn get_parabola(self) -> crate::combat::parabola::Parabola;

    #[method(name = "set_Parabola", args = 1)]
    pub fn set_parabola(self, value: crate::combat::parabola::Parabola) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "MyStart", args = 0)]
    pub fn my_start(self) -> ();

    #[method(name = "MyLateUpdate", args = 0)]
    pub fn my_late_update(self) -> ();

    #[method(name = "AimOn", args = 0)]
    pub fn aim_on(self) -> ();

    #[method(name = "AimOff", args = 0)]
    pub fn aim_off(self) -> ();

    #[method(name = "Skip", args = 0)]
    pub fn skip(self) -> ();

    #[method(name = "Aim", args = 2)]
    pub fn aim(self, end_weight: f32, end_time: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-characterikaim")]
impl CharacterIKAim {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterIKAim),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterIKAimMethods>::ctor(this);
        this
    }
}
