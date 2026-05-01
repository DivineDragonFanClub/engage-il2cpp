
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/traderoot/TradeRoot.md")))]
#[::unity2::class(namespace = "App", name = "TradeRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TradeRoot {
    #[rename(name = "m_ItemListLeft")]
    pub m_item_list_left: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ItemListRight")]
    pub m_item_list_right: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ItemInfo")]
    pub m_item_info: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-traderoot")]
#[::unity2::methods]
impl TradeRoot {
    #[method(name = "SetItemInfo", args = 1)]
    pub fn set_item_info(self, item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "Setup", args = 2)]
    pub fn setup(self, from_unit: crate::app::unit::Unit, to_unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetSelectItem", args = 1)]
    pub fn set_select_item(self, item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "SetSelectItem", args = 2)]
    pub fn set_select_item_2(self, unit: crate::app::unit::Unit, item_no: i32) -> ();

    #[method(name = "GetGameObjectLeft", args = 0)]
    pub fn get_game_object_left(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetGameObjectRight", args = 0)]
    pub fn get_game_object_right(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetItemInfo", args = 0)]
    pub fn get_item_info(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-traderoot")]
impl TradeRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TradeRoot),
                ::core::stringify!(new),
            )
        });
        <Self as ITradeRootMethods>::ctor(this);
        this
    }
}
