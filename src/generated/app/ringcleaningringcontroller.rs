
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningringcontroller/RingCleaningRingController.md")))]
#[::unity2::class(namespace = "App", name = "RingCleaningRingController")]
#[parent(crate::system::object::Object)]
pub struct RingCleaningRingController {
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
    #[rename(name = "m_Ring")]
    pub m_ring: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_RingMaterials")]
    pub m_ring_materials: ::unity2::Array<crate::unity_engine::material::Material>,
    #[rename(name = "m_RingCollider")]
    pub m_ring_collider: crate::app::ringcollider::RingCollider,
    #[rename(name = "InitHeight")]
    pub init_height: f32,
    #[rename(name = "InitRotate")]
    pub init_rotate: crate::unity_engine::vector3::Vector3,
    #[rename(name = "RotateSpeed")]
    pub rotate_speed: f32,
    #[rename(name = "m_InitRotation")]
    pub m_init_rotation: crate::app::interpolatorvector3::InterpolatorVector3,
    #[rename(name = "m_InterDirty")]
    pub m_inter_dirty: crate::app::interpolatorint::InterpolatorInt,
    #[rename(name = "m_InfoRoot")]
    pub m_info_root: crate::app::ringcleaningroot::RingCleaningRoot,
}

#[cfg(feature = "app-ringcleaningringcontroller")]
#[::unity2::methods]
impl RingCleaningRingController {
    #[method(name = "get_CanResetRotate", args = 0)]
    pub fn get_can_reset_rotate(self) -> bool;

    #[method(name = "set_CanResetRotate", args = 1)]
    pub fn set_can_reset_rotate(self, value: bool) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "get_RingCollider", args = 0)]
    pub fn get_ring_collider(self) -> crate::app::ringcollider::RingCollider;

    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, info_root: crate::app::ringcleaningroot::RingCleaningRoot) -> ();

    #[method(name = "UpdateRotate", args = 2)]
    pub fn update_rotate(self, rdx: f32, rdy: f32) -> ();

    #[method(name = "UpdateDirtyTick", args = 0)]
    pub fn update_dirty_tick(self) -> ();

    #[method(name = "SetMaterialDirty", args = 1)]
    pub fn set_material_dirty(self, dirty: i32) -> ();

    #[method(name = "SetMaterialDirtyImmediately", args = 1)]
    pub fn set_material_dirty_immediately(self, dirty: i32) -> ();

    #[method(name = "SetResetRotation", args = 0)]
    pub fn set_reset_rotation(self) -> ();

    #[method(name = "UpdateResetRotation", args = 0)]
    pub fn update_reset_rotation(self) -> bool;
}

#[cfg(feature = "app-ringcleaningringcontroller")]
impl RingCleaningRingController {
    pub fn new(god: crate::app::godunit::GodUnit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingCleaningRingController),
                ::core::stringify!(new),
            )
        });
        <Self as IRingCleaningRingControllerMethods>::ctor(this, god);
        this
    }
}
