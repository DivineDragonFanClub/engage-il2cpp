
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/magicsub/MagicSub.md")))]
#[::unity2::class(namespace = "Combat", name = "MagicSub")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MagicSub {
    #[rename(name = "BulletSettings")]
    pub bullet_settings: crate::combat::magicbulletsettings::MagicBulletSettings,
    #[rename(name = "Trackヒット時処理")]
    pub track______: crate::combat::magicsignaltrack::MagicSignalTrack,
    #[rename(name = "Trackミス時処理")]
    pub track_____: crate::combat::magicsignaltrack::MagicSignalTrack,
}

#[cfg(feature = "combat-magicsub")]
#[::unity2::methods]
impl MagicSub {
    #[method(name = "Track", args = 1)]
    pub fn track(self, i: i32) -> crate::combat::magicsignaltrack::MagicSignalTrack;

    #[method(name = "Setup", args = 3)]
    pub fn setup(
        self,
        chr: crate::combat::character::Character,
        initial_start_pos: crate::unity_engine::vector3::Vector3,
        initial_end_pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "ManualUpdate", args = 0)]
    pub fn manual_update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-magicsub")]
impl MagicSub {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MagicSub),
                ::core::stringify!(new),
            )
        });
        <Self as IMagicSubMethods>::ctor(this);
        this
    }
}
