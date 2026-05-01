
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/privateeffectcatalog/PrivateEffectCatalog.md")))]
#[::unity2::class(namespace = "Combat", name = "PrivateEffectCatalog")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct PrivateEffectCatalog {
    #[rename(name = "TrailMeshPrefab")]
    pub trail_mesh_prefab: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "AppearEffect")]
    pub appear_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "DisappearEffect")]
    pub disappear_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "SyncHitEffect")]
    pub sync_hit_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "GuardEffectPrefab")]
    pub guard_effect_prefab: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "GuardEffectNodeName")]
    pub guard_effect_node_name: ::unity2::Il2CppString,
}

#[cfg(feature = "combat-privateeffectcatalog")]
#[::unity2::methods]
impl PrivateEffectCatalog {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-privateeffectcatalog")]
impl PrivateEffectCatalog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PrivateEffectCatalog),
                ::core::stringify!(new),
            )
        });
        <Self as IPrivateEffectCatalogMethods>::ctor(this);
        this
    }
}
