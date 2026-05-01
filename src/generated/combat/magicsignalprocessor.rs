
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/magicsignalprocessor/MagicSignalProcessor.md")))]
#[::unity2::class(namespace = "Combat", name = "MagicSignalProcessor")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MagicSignalProcessor {
    #[rename(name = "CP")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "m_FlyingObject")]
    pub m_flying_object: crate::combat::magicflying::MagicFlying,
    #[rename(name = "m_CachedNodeConstraint")]
    pub m_cached_node_constraint: crate::combat::characternodeconstraint::CharacterNodeConstraint,
    #[rename(name = "m_Phase")]
    pub m_phase: crate::combat::phase::Phase,
    #[rename(name = "m_DecayTime")]
    pub m_decay_time: f32,
    #[rename(name = "m_IsFocusEnemyPassed")]
    pub m_is_focus_enemy_passed: bool,
    #[rename(name = "m_bReached")]
    pub m_b_reached: bool,
    #[rename(name = "TL開始時処理")]
    pub tl_____: crate::combat::magicsignaltimeline::MagicSignalTimeline,
    #[rename(name = "TL魔法動作1処理")]
    pub tl____1__: crate::combat::magicsignaltimeline::MagicSignalTimeline,
    #[rename(name = "TL魔法動作2処理")]
    pub tl____2__: crate::combat::magicsignaltimeline::MagicSignalTimeline,
    #[rename(name = "TL魔法動作3処理")]
    pub tl____3__: crate::combat::magicsignaltimeline::MagicSignalTimeline,
    #[rename(name = "TLヒット時処理")]
    pub tl______: crate::combat::magicsignaltimeline::MagicSignalTimeline,
    #[rename(name = "m_ManagedObjects")]
    pub m_managed_objects: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_DeletedObjects")]
    pub m_deleted_objects: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_KnockoffAttackHash")]
    pub m_knockoff_attack_hash: i32,
}

#[cfg(feature = "combat-magicsignalprocessor")]
#[::unity2::methods]
impl MagicSignalProcessor {
    #[method(name = "get_Magic", args = 0)]
    pub fn get_magic(self) -> crate::combat::magic::Magic;

    #[method(name = "set_Magic", args = 1)]
    pub fn set_magic(self, value: crate::combat::magic::Magic) -> ();

    #[method(name = "OnEnterAttack", args = 3)]
    pub fn on_enter_attack(
        self,
        owner: crate::combat::character::Character,
        catalog: crate::combat::magic::Magic,
        world_hit_time: f32,
    ) -> ();

    #[method(name = "RegisterObservers", args = 1)]
    pub fn register_observers(self, world_hit_time: f32) -> ();

    #[method(name = "HitProcess", args = 0)]
    pub fn hit_process(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "OnEnterMagicSwordHit", args = 2)]
    pub fn on_enter_magic_sword_hit(
        self,
        owner: crate::combat::character::Character,
        catalog: crate::combat::magic::Magic,
    ) -> ();

    #[method(name = "OnHitMagicSwordHit", args = 0)]
    pub fn on_hit_magic_sword_hit(self) -> ();

    #[method(name = "GetIceRockSkippedPrefab仕方なく", args = 0)]
    pub fn get_ice_rock_skipped_prefab____(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "UpdateTimeline", args = 1)]
    pub fn update_timeline(
        self,
        timeline: crate::combat::magicsignaltimeline::MagicSignalTimeline,
    ) -> ();

    #[method(name = "CmdCreate", args = 2)]
    pub fn cmd_create(self, ev: crate::combat::magicsignal::MagicSignal, uncached: bool) -> ();

    #[method(name = "CmdCreateImpl", args = 2)]
    pub fn cmd_create_impl(self, ev: crate::combat::magicsignal::MagicSignal, uncached: bool)
        -> ();

    #[method(name = "CmdCreateGO", args = 2)]
    pub fn cmd_create_go(
        self,
        ev: crate::combat::magicsignal::MagicSignal,
        uncached: bool,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "CmdDelete", args = 1)]
    pub fn cmd_delete(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CmdShoot", args = 1)]
    pub fn cmd_shoot(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CmdCollideFlying", args = 1)]
    pub fn cmd_collide_flying(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CmdStopFlying", args = 1)]
    pub fn cmd_stop_flying(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CmdChangeTarget", args = 1)]
    pub fn cmd_change_target(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CmdSound", args = 1)]
    pub fn cmd_sound(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CmdCutChange", args = 1)]
    pub fn cmd_cut_change(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CallOnFocusEnemy", args = 0)]
    pub fn call_on_focus_enemy(self) -> ();

    #[method(name = "CmdCamera", args = 1)]
    pub fn cmd_camera(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CmdCameraResume", args = 1)]
    pub fn cmd_camera_resume(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CmdRadialBlur", args = 1)]
    pub fn cmd_radial_blur(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = "CmdBGBrightness", args = 1)]
    pub fn cmd_bg_brightness(self, ev: crate::combat::magicsignal::MagicSignal) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-magicsignalprocessor")]
impl MagicSignalProcessor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MagicSignalProcessor),
                ::core::stringify!(new),
            )
        });
        <Self as IMagicSignalProcessorMethods>::ctor(this);
        this
    }
}
