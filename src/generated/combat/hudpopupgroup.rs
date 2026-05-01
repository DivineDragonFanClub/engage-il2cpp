
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/hudpopupgroup/HUDPopupGroup.md")))]
#[::unity2::class(namespace = "Combat", name = "HUDPopupGroup")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HUDPopupGroup {
    #[rename(name = "m_RectTransform")]
    pub m_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_WorldPos")]
    pub m_world_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "spos")]
    pub spos: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "combat-hudpopupgroup")]
#[::unity2::methods]
impl HUDPopupGroup {
    #[method(name = "get_IsAlive", args = 0)]
    pub fn get_is_alive(self) -> bool;

    #[method(name = "DamagePopup", args = 1)]
    pub fn damage_popup(phase: crate::combat::phase::Phase) -> ();

    #[method(name = "SetAsUnscaled", args = 1)]
    pub fn set_as_unscaled(root: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "InitialUpdate", args = 0)]
    pub fn initial_update(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "DoesNeedForAttacker", args = 1)]
    pub fn does_need_for_attacker(phase: crate::combat::phase::Phase) -> bool;

    #[method(name = "SetupAsAttacker", args = 2)]
    pub fn setup_as_attacker(
        self,
        phase: crate::combat::phase::Phase,
        world_pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetupAsDamager", args = 2)]
    pub fn setup_as_damager(
        self,
        phase: crate::combat::phase::Phase,
        world_pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetDamage", args = 1)]
    pub fn get_damage(phase: crate::combat::phase::Phase) -> i32;

    #[method(name = "init", args = 1)]
    pub fn init(self, world_pos: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "GetNumberPrefabName", args = 2)]
    pub fn get_number_prefab_name(
        phase: crate::combat::phase::Phase,
        value: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetPopupPrefabName", args = 2)]
    pub fn get_popup_prefab_name(
        phase: crate::combat::phase::Phase,
        style: crate::combat::combatstyle::CombatStyle,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CreateHUD", args = 2)]
    pub fn create_hud(
        self,
        name: ::unity2::Il2CppString,
        y: f32,
    ) -> crate::combat::hudpopup::HUDPopup;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-hudpopupgroup")]
impl HUDPopupGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HUDPopupGroup),
                ::core::stringify!(new),
            )
        });
        <Self as IHUDPopupGroupMethods>::ctor(this);
        this
    }
}
