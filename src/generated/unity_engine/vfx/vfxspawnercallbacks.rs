
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/vfx/vfxspawnercallbacks/VFXSpawnerCallbacks.md")))]
#[::unity2::class(namespace = "UnityEngine.VFX", name = "VFXSpawnerCallbacks")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct VFXSpawnerCallbacks {}

#[cfg(feature = "unity_engine-vfx-vfxspawnercallbacks")]
#[::unity2::methods]
impl VFXSpawnerCallbacks {
    #[method(name = "OnPlay", args = 3)]
    pub fn on_play(
        self,
        state: crate::unity_engine::vfx::vfxspawnerstate::VFXSpawnerState,
        vfx_values: crate::unity_engine::vfx::vfxexpressionvalues::VFXExpressionValues,
        vfx_component: crate::unity_engine::vfx::visualeffect::VisualEffect,
    ) -> ();

    #[method(name = "OnUpdate", args = 3)]
    pub fn on_update(
        self,
        state: crate::unity_engine::vfx::vfxspawnerstate::VFXSpawnerState,
        vfx_values: crate::unity_engine::vfx::vfxexpressionvalues::VFXExpressionValues,
        vfx_component: crate::unity_engine::vfx::visualeffect::VisualEffect,
    ) -> ();

    #[method(name = "OnStop", args = 3)]
    pub fn on_stop(
        self,
        state: crate::unity_engine::vfx::vfxspawnerstate::VFXSpawnerState,
        vfx_values: crate::unity_engine::vfx::vfxexpressionvalues::VFXExpressionValues,
        vfx_component: crate::unity_engine::vfx::visualeffect::VisualEffect,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-vfx-vfxspawnercallbacks")]
impl VFXSpawnerCallbacks {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VFXSpawnerCallbacks),
                ::core::stringify!(new),
            )
        });
        <Self as IVFXSpawnerCallbacksMethods>::ctor(this);
        this
    }
}
