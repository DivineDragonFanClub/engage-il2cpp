
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rigidbody2d/Rigidbody2D.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Rigidbody2D")]
#[parent(crate::unity_engine::component::Component)]
pub struct Rigidbody2D {}

#[cfg(feature = "unity_engine-rigidbody2d")]
#[::unity2::methods]
impl Rigidbody2D {
    #[method(name = "get_velocity", args = 0)]
    pub fn get_velocity(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_mass", args = 0)]
    pub fn get_mass(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_velocity_Injected", args = 1)]
    pub fn get_velocity_injected(self, ret: crate::unity_engine::vector2::Vector2) -> ();
}

#[cfg(feature = "unity_engine-rigidbody2d")]
impl Rigidbody2D {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Rigidbody2D),
                ::core::stringify!(new),
            )
        });
        <Self as IRigidbody2DMethods>::ctor(this);
        this
    }
}
