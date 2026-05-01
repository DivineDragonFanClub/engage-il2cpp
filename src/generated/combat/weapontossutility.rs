
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/weapontossutility/WeaponTossUtility.md")))]
#[::unity2::class(namespace = "Combat", name = "WeaponTossUtility")]
#[parent(crate::system::object::Object)]
pub struct WeaponTossUtility {}

#[cfg(feature = "combat-weapontossutility")]
#[::unity2::methods]
impl WeaponTossUtility {
    #[method(name = "Toss", args = 3)]
    pub fn toss(cp: crate::combat::character::Character, power: f32, skipped: bool) -> ();

    #[method(name = "TossMain", args = 4)]
    pub fn toss_main(
        cp: crate::combat::character::Character,
        wep: crate::unity_engine::gameobject::GameObject,
        power: f32,
        skipped: bool,
    ) -> ();

    #[method(name = "AddJustFitBoxCollider", args = 3)]
    pub fn add_just_fit_box_collider(
        go: crate::unity_engine::gameobject::GameObject,
        mesh_go: crate::unity_engine::gameobject::GameObject,
        scale: f32,
    ) -> crate::unity_engine::boxcollider::BoxCollider;

    #[method(name = "GetWeaponTipOffset", args = 1)]
    pub fn get_weapon_tip_offset(mesh_go: crate::unity_engine::gameobject::GameObject) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-weapontossutility")]
impl WeaponTossUtility {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponTossUtility),
                ::core::stringify!(new),
            )
        });
        <Self as IWeaponTossUtilityMethods>::ctor(this);
        this
    }
}
