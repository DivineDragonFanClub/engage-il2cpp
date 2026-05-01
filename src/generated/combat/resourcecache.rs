
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/resourcecache/ResourceCache.md")))]
#[::unity2::class(namespace = "Combat", name = "ResourceCache")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: combat :: resourcecache :: ResourceCache >)]
pub struct ResourceCache {
    #[rename(name = "dic")]
    pub dic: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::resourcehandle_2::ResourceHandle_2,
    >,
}

#[cfg(feature = "combat-resourcecache")]
#[::unity2::methods]
impl ResourceCache {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Create", args = 1)]
    pub fn create(
        self,
        asset_path: ::unity2::Il2CppString,
    ) -> crate::app::resourcehandle_2::ResourceHandle_2;

    #[method(name = "LoadAsync", args = 2)]
    pub fn load_async(
        asset_path: ::unity2::Il2CppString,
        done_callback: crate::system::action_1::Action_1<crate::unity_engine::object_2::Object_2>,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-resourcecache")]
impl ResourceCache {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourceCache),
                ::core::stringify!(new),
            )
        });
        <Self as IResourceCacheMethods>::ctor(this);
        this
    }
}
