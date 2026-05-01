
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventoryroot/InventoryRoot.md")))]
#[::unity2::class(namespace = "App", name = "InventoryRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct InventoryRoot {
    #[rename(name = "m_UnitItemList")]
    pub m_unit_item_list: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PoolItemList")]
    pub m_pool_item_list: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ItemInfo")]
    pub m_item_info: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-inventoryroot")]
#[::unity2::methods]
impl InventoryRoot {
    #[method(name = "SetItemInfo", args = 1)]
    pub fn set_item_info(self, item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "SetSelectItem", args = 2)]
    pub fn set_select_item(self, unit: crate::app::unit::Unit, item_no: i32) -> ();

    #[method(name = "GetGameObjectUnit", args = 0)]
    pub fn get_game_object_unit(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetGameObjectPool", args = 0)]
    pub fn get_game_object_pool(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetItemInfo", args = 0)]
    pub fn get_item_info(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "MenuOpen", args = 0)]
    pub fn menu_open(self) -> ();

    #[method(name = "MenuClose", args = 0)]
    pub fn menu_close(self) -> ();

    #[method(name = "IsMenuClose", args = 0)]
    pub fn is_menu_close(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inventoryroot")]
impl InventoryRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventoryRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IInventoryRootMethods>::ctor(this);
        this
    }
}
