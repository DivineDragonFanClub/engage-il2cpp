
use crate::combat::launchbehaviour::ILaunchBehaviour;
use crate::combat::launchbehaviour::LaunchBehaviour;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/magic/Magic.md")))]
#[::unity2::class(namespace = "Combat", name = "Magic")]
#[parent(crate::combat::launchbehaviour::LaunchBehaviour)]
pub struct Magic {
    #[rename(name = "BulletSettings")]
    pub bullet_settings: crate::combat::magicbulletsettings::MagicBulletSettings,
    #[rename(name = "m_SignalProcessor")]
    pub m_signal_processor: crate::combat::magicsignalprocessor::MagicSignalProcessor,
    #[rename(name = "Track開始時処理")]
    pub track_____: crate::combat::magicsignaltrack::MagicSignalTrack,
    #[rename(name = "Track魔法動作1処理")]
    pub track____1__: crate::combat::magicsignaltrack::MagicSignalTrack,
    #[rename(name = "Track魔法動作2処理")]
    pub track____2__: crate::combat::magicsignaltrack::MagicSignalTrack,
    #[rename(name = "Track魔法動作3処理")]
    pub track____3__: crate::combat::magicsignaltrack::MagicSignalTrack,
    #[rename(name = "Trackヒット時処理")]
    pub track______: crate::combat::magicsignaltrack::MagicSignalTrack,
    #[rename(name = "_homeNode")]
    pub home_node: crate::unity_engine::transform::Transform,
    #[rename(name = "_targetNode")]
    pub target_node: crate::unity_engine::transform::Transform,
}

#[cfg(feature = "combat-magic")]
#[::unity2::methods]
impl Magic {
    #[method(name = "get_InitialStartPos", args = 0)]
    pub fn get_initial_start_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_InitialStartPos", args = 1)]
    pub fn set_initial_start_pos(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_InitialEndPos", args = 0)]
    pub fn get_initial_end_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_InitialEndPos", args = 1)]
    pub fn set_initial_end_pos(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Track", args = 1)]
    pub fn track(self, i: i32) -> crate::combat::magicsignaltrack::MagicSignalTrack;

    #[method(name = "get_HomeNode", args = 0)]
    pub fn get_home_node(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_TargetNode", args = 0)]
    pub fn get_target_node(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "get_TargetPosition", args = 0)]
    pub fn get_target_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "RecalcFlyingTime", args = 0)]
    pub fn recalc_flying_time(self) -> ();

    #[method(name = "OnCharacterSetup", args = 1)]
    pub fn on_character_setup(self, owner: crate::combat::character::Character) -> ();

    #[method(name = "OnEnterAttack", args = 0)]
    pub fn on_enter_attack(self) -> ();

    #[method(name = "OnHitTimePredicted", args = 1)]
    pub fn on_hit_time_predicted(self, world_hit_time: f32) -> ();

    #[method(name = "OnHitMagicSwordHit", args = 0)]
    pub fn on_hit_magic_sword_hit(self) -> ();

    #[method(name = "GetIceRockSkippedPrefab仕方なく", args = 0)]
    pub fn get_ice_rock_skipped_prefab____(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-magic")]
impl Magic {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Magic),
                ::core::stringify!(new),
            )
        });
        <Self as IMagicMethods>::ctor(this);
        this
    }
}
