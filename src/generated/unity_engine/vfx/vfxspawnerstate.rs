
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/vfx/vfxspawnerstate/VFXSpawnerState.md")))]
#[::unity2::class(namespace = "UnityEngine.VFX", name = "VFXSpawnerState")]
#[parent(crate::system::object::Object)]
pub struct VFXSpawnerState {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_Owner")]
    pub m_owner: bool,
}

#[cfg(feature = "unity_engine-vfx-vfxspawnerstate")]
#[::unity2::methods]
impl VFXSpawnerState {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, ptr: ::unity2::IntPtr, owner: bool) -> ();

    #[method(name = "CreateSpawnerStateWrapper", args = 0)]
    pub fn create_spawner_state_wrapper(
    ) -> crate::unity_engine::vfx::vfxspawnerstate::VFXSpawnerState;

    #[method(name = "SetWrapValue", args = 1)]
    pub fn set_wrap_value(self, ptr: ::unity2::IntPtr) -> ();

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Internal_Destroy", args = 1)]
    pub fn internal_destroy(ptr: ::unity2::IntPtr) -> ();
}

#[cfg(feature = "unity_engine-vfx-vfxspawnerstate")]
impl VFXSpawnerState {
    pub fn new(ptr: ::unity2::IntPtr, owner: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VFXSpawnerState),
                ::core::stringify!(new),
            )
        });
        <Self as IVFXSpawnerStateMethods>::ctor(this, ptr, owner);
        this
    }
}
