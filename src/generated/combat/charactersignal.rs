
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/charactersignal/CharacterSignal.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterSignal")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterSignal {
    #[rename(name = "cp_cached")]
    pub cp_cached: bool,
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "hitPassed")]
    pub hit_passed: bool,
}

#[cfg(feature = "combat-charactersignal")]
#[::unity2::methods]
impl CharacterSignal {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "InitOnAttackState", args = 0)]
    pub fn init_on_attack_state(self) -> ();

    #[method(name = "命中先行呼び出し", args = 1)]
    pub fn _unnamed(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "Call命中Common", args = 1)]
    pub fn call___common(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "なし", args = 0)]
    pub fn _unnamed_2(self) -> ();

    #[method(name = "汎用Object", args = 0)]
    pub fn __object(self) -> ();

    #[method(name = "Run速度", args = 0)]
    pub fn run__(self) -> ();

    #[method(name = "Vec3", args = 0)]
    pub fn vec3(self) -> ();

    #[method(name = "魔法動作1", args = 1)]
    pub fn ____1(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "魔法動作2", args = 1)]
    pub fn ____2(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "魔法動作3", args = 1)]
    pub fn ____3(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "FootIK", args = 1)]
    pub fn foot_ik(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-charactersignal")]
impl CharacterSignal {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterSignal),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterSignalMethods>::ctor(this);
        this
    }
}
