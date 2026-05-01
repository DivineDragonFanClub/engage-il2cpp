
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/physicmaterial/PhysicMaterial.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "PhysicMaterial")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct PhysicMaterial {}

#[cfg(feature = "unity_engine-physicmaterial")]
#[::unity2::methods]
impl PhysicMaterial {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Internal_CreateDynamicsMaterial", args = 2)]
    pub fn internal_create_dynamics_material(
        mat: crate::unity_engine::physicmaterial::PhysicMaterial,
        name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "get_bounciness", args = 0)]
    pub fn get_bounciness(self) -> f32;

    #[method(name = "set_bounciness", args = 1)]
    pub fn set_bounciness(self, value: f32) -> ();

    #[method(name = "get_dynamicFriction", args = 0)]
    pub fn get_dynamic_friction(self) -> f32;

    #[method(name = "set_dynamicFriction", args = 1)]
    pub fn set_dynamic_friction(self, value: f32) -> ();

    #[method(name = "get_staticFriction", args = 0)]
    pub fn get_static_friction(self) -> f32;

    #[method(name = "set_staticFriction", args = 1)]
    pub fn set_static_friction(self, value: f32) -> ();

    #[method(name = "get_frictionCombine", args = 0)]
    pub fn get_friction_combine(
        self,
    ) -> crate::unity_engine::physicmaterialcombine::PhysicMaterialCombine;

    #[method(name = "set_frictionCombine", args = 1)]
    pub fn set_friction_combine(
        self,
        value: crate::unity_engine::physicmaterialcombine::PhysicMaterialCombine,
    ) -> ();

    #[method(name = "get_bounceCombine", args = 0)]
    pub fn get_bounce_combine(
        self,
    ) -> crate::unity_engine::physicmaterialcombine::PhysicMaterialCombine;

    #[method(name = "set_bounceCombine", args = 1)]
    pub fn set_bounce_combine(
        self,
        value: crate::unity_engine::physicmaterialcombine::PhysicMaterialCombine,
    ) -> ();

    #[method(name = "get_bouncyness", args = 0)]
    pub fn get_bouncyness(self) -> f32;

    #[method(name = "set_bouncyness", args = 1)]
    pub fn set_bouncyness(self, value: f32) -> ();

    #[method(name = "get_frictionDirection2", args = 0)]
    pub fn get_friction_direction2(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_frictionDirection2", args = 1)]
    pub fn set_friction_direction2(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_dynamicFriction2", args = 0)]
    pub fn get_dynamic_friction2(self) -> f32;

    #[method(name = "set_dynamicFriction2", args = 1)]
    pub fn set_dynamic_friction2(self, value: f32) -> ();

    #[method(name = "get_staticFriction2", args = 0)]
    pub fn get_static_friction2(self) -> f32;

    #[method(name = "set_staticFriction2", args = 1)]
    pub fn set_static_friction2(self, value: f32) -> ();

    #[method(name = "get_frictionDirection", args = 0)]
    pub fn get_friction_direction(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_frictionDirection", args = 1)]
    pub fn set_friction_direction(self, value: crate::unity_engine::vector3::Vector3) -> ();
}

#[cfg(feature = "unity_engine-physicmaterial")]
impl PhysicMaterial {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhysicMaterial),
                ::core::stringify!(new),
            )
        });
        <Self as IPhysicMaterialMethods>::ctor(this);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhysicMaterial),
                ::core::stringify!(new_2),
            )
        });
        <Self as IPhysicMaterialMethods>::ctor_2(this, name);
        this
    }
}
