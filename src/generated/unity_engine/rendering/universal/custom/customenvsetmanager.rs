
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/custom/customenvsetmanager/CustomEnvSetManager.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal.Custom",
    name = "CustomEnvSetManager"
)]
#[parent(crate::system::object::Object)]
pub struct CustomEnvSetManager {
    #[rename(name = "m_EnvSets")]
    pub m_env_sets: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::universal::custom::customenvset::CustomEnvSet,
    >,
}

#[cfg(feature = "unity_engine-rendering-universal-custom-customenvsetmanager")]
#[::unity2::methods]
impl CustomEnvSetManager {
    #[method(name = "get_instance", args = 0)]
    pub fn get_instance(
    ) -> crate::unity_engine::rendering::universal::custom::customenvsetmanager::CustomEnvSetManager;

    #[method(name = "Register", args = 1)]
    pub fn register(
        self,
        set: crate::unity_engine::rendering::universal::custom::customenvset::CustomEnvSet,
    ) -> ();

    #[method(name = "Unregister", args = 1)]
    pub fn unregister(
        self,
        set: crate::unity_engine::rendering::universal::custom::customenvset::CustomEnvSet,
    ) -> ();

    #[method(name = "TryGetReflectionProbe", args = 2)]
    pub fn try_get_reflection_probe(
        self,
        index: i32,
        probe: crate::unity_engine::reflectionprobe::ReflectionProbe,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-custom-customenvsetmanager")]
impl CustomEnvSetManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CustomEnvSetManager),
                ::core::stringify!(new),
            )
        });
        <Self as ICustomEnvSetManagerMethods>::ctor(this);
        this
    }
}
