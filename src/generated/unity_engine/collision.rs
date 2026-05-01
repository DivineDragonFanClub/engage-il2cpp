
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/collision/Collision.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Collision")]
#[parent(crate::system::object::Object)]
pub struct Collision {
    #[rename(name = "m_Impulse")]
    pub m_impulse: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_RelativeVelocity")]
    pub m_relative_velocity: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Body")]
    pub m_body: crate::unity_engine::component::Component,
    #[rename(name = "m_Collider")]
    pub m_collider: crate::unity_engine::collider::Collider,
    #[rename(name = "m_ContactCount")]
    pub m_contact_count: i32,
    #[rename(name = "m_ReusedContacts")]
    pub m_reused_contacts: ::unity2::Array<crate::unity_engine::contactpoint::ContactPoint>,
    #[rename(name = "m_LegacyContacts")]
    pub m_legacy_contacts: ::unity2::Array<crate::unity_engine::contactpoint::ContactPoint>,
}

#[cfg(feature = "unity_engine-collision")]
#[::unity2::methods]
impl Collision {
    #[method(name = "get_relativeVelocity", args = 0)]
    pub fn get_relative_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_collider", args = 0)]
    pub fn get_collider(self) -> crate::unity_engine::collider::Collider;

    #[method(name = "get_gameObject", args = 0)]
    pub fn get_game_object(self) -> crate::unity_engine::gameobject::GameObject;
}
