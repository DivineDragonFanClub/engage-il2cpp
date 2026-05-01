
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponmodelmanager/WeaponModelManager.md")))]
#[::unity2::class(namespace = "App", name = "WeaponModelManager")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct WeaponModelManager {
    #[static_field]
    #[rename(name = "s_AssetTableResult")]
    pub s_asset_table_result: crate::app::assettable::AssetTable_Result,
    #[rename(name = "m_Resources")]
    pub m_resources: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::app::resourcegameobject::ResourceGameObject,
    >,
    #[rename(name = "m_GmaeObjects")]
    pub m_gmae_objects: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "app-weaponmodelmanager")]
#[::unity2::methods]
impl WeaponModelManager {
    #[method(name = "TryCreateGameObject", args = 3)]
    pub fn try_create_game_object(
        self,
        key: i32,
        name: ::unity2::Il2CppString,
        child: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Show", args = 1)]
    pub fn show(self, iid: ::unity2::Il2CppString) -> ();

    #[method(name = "Show", args = 1)]
    pub fn show_2(self, root: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Show", args = 1)]
    pub fn show_3(self, item: crate::app::itemdata::ItemData) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-weaponmodelmanager")]
impl WeaponModelManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponModelManager),
                ::core::stringify!(new),
            )
        });
        <Self as IWeaponModelManagerMethods>::ctor(this);
        this
    }
}
