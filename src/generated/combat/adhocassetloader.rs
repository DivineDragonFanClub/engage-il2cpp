
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/adhocassetloader/AdhocAssetLoader_Asset.md")))]
#[::unity2::class(namespace = "Combat", name = "AdhocAssetLoader.Asset")]
#[parent(crate::system::object::Object)]
pub struct AdhocAssetLoader_Asset {
    #[rename(name = "m_FileHandle")]
    pub m_file_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "combat-adhocassetloader")]
#[::unity2::methods]
impl AdhocAssetLoader_Asset {
    #[method(name = "get_NameHash", args = 0)]
    pub fn get_name_hash(self) -> i32;

    #[method(name = "set_NameHash", args = 1)]
    pub fn set_name_hash(self, value: i32) -> ();

    #[method(name = "get_AddressablePath", args = 0)]
    pub fn get_addressable_path(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AddressablePath", args = 1)]
    pub fn set_addressable_path(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Prefab", args = 0)]
    pub fn get_prefab(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_Prefab", args = 1)]
    pub fn set_prefab(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Is", args = 1)]
    pub fn is(self, hash: i32) -> bool;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, hash: i32) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load(self) -> ();

    #[method(name = "IsReady", args = 0)]
    pub fn is_ready(self) -> bool;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "combat-adhocassetloader")]
impl AdhocAssetLoader_Asset {
    pub fn new(name: ::unity2::Il2CppString, hash: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AdhocAssetLoader_Asset),
                ::core::stringify!(new),
            )
        });
        <Self as IAdhocAssetLoader_AssetMethods>::ctor(this, name, hash);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/adhocassetloader/AdhocAssetLoader.md")))]
#[::unity2::class(namespace = "Combat", name = "AdhocAssetLoader")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: combat :: adhocassetloader :: AdhocAssetLoader >)]
pub struct AdhocAssetLoader {
    #[rename(name = "assets")]
    pub assets: crate::system::collections::generic::list_1::List_1<
        crate::combat::adhocassetloader::AdhocAssetLoader_Asset,
    >,
}

#[cfg(feature = "combat-adhocassetloader")]
#[::unity2::methods]
impl AdhocAssetLoader {
    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(
        self,
        path: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Add", args = 1)]
    pub fn add(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Preload", args = 0)]
    pub fn preload(self) -> ();

    #[method(name = "IsReady", args = 0)]
    pub fn is_ready(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-adhocassetloader")]
impl AdhocAssetLoader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AdhocAssetLoader),
                ::core::stringify!(new),
            )
        });
        <Self as IAdhocAssetLoaderMethods>::ctor(this);
        this
    }
}
