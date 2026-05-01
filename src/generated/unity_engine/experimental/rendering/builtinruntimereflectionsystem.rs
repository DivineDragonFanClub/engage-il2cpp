
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/builtinruntimereflectionsystem/BuiltinRuntimeReflectionSystem.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering",
    name = "BuiltinRuntimeReflectionSystem"
)]
#[parent(crate::system::object::Object)]
pub struct BuiltinRuntimeReflectionSystem {}

#[cfg(feature = "unity_engine-experimental-rendering-builtinruntimereflectionsystem")]
#[::unity2::methods]
impl BuiltinRuntimeReflectionSystem {
    #[method(name = "TickRealtimeProbes", args = 0)]
    pub fn tick_realtime_probes(self) -> bool;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose_2(self, disposing: bool) -> ();

    #[method(name = "BuiltinUpdate", args = 0)]
    pub fn builtin_update() -> bool;

    #[method(name = "Internal_BuiltinRuntimeReflectionSystem_New", args = 0)]
    pub fn internal_builtin_runtime_reflection_system_new () -> crate :: unity_engine :: experimental :: rendering :: builtinruntimereflectionsystem :: BuiltinRuntimeReflectionSystem ;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-builtinruntimereflectionsystem")]
impl BuiltinRuntimeReflectionSystem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BuiltinRuntimeReflectionSystem),
                ::core::stringify!(new),
            )
        });
        <Self as IBuiltinRuntimeReflectionSystemMethods>::ctor(this);
        this
    }
}
