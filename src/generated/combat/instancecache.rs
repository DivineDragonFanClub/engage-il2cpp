
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/instancecache/InstanceCache.md")))]
#[::unity2::class(namespace = "Combat", name = "InstanceCache")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct InstanceCache {
    #[static_field]
    #[rename(name = "s_this")]
    pub s_this: crate::combat::instancecache::InstanceCache,
    #[static_field]
    #[rename(name = "FreeTreeName")]
    pub free_tree_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "UsedTreeName")]
    pub used_tree_name: ::unity2::Il2CppString,
    #[rename(name = "usedDic")]
    pub used_dic: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::unity_engine::gameobject::GameObject,
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "freeDic")]
    pub free_dic: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::unity_engine::gameobject::GameObject,
        crate::system::collections::generic::stack_1::Stack_1<
            crate::unity_engine::gameobject::GameObject,
        >,
    >,
    #[rename(name = "usedNode")]
    pub used_node: crate::unity_engine::transform::Transform,
    #[rename(name = "freeNode")]
    pub free_node: crate::unity_engine::transform::Transform,
}

#[cfg(feature = "combat-instancecache")]
#[::unity2::methods]
impl InstanceCache {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> crate::combat::instancecache::InstanceCache;

    #[method(name = "get_UsedEffectRoot", args = 0)]
    pub fn get_used_effect_root(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "Create", args = 2)]
    pub fn create(
        self,
        prefab: crate::unity_engine::gameobject::GameObject,
        parent: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Delete", args = 1)]
    pub fn delete(self, go: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "ReplayAwakeSound", args = 1)]
    pub fn replay_awake_sound(self, go: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-instancecache")]
impl InstanceCache {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InstanceCache),
                ::core::stringify!(new),
            )
        });
        <Self as IInstanceCacheMethods>::ctor(this);
        this
    }
}
