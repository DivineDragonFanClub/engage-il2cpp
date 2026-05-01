
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fleamarketbuyroot/FleaMarketBuyRoot.md")))]
#[::unity2::class(namespace = "App", name = "FleaMarketBuyRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FleaMarketBuyRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_MenuObject")]
    pub m_menu_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_HoldingInfoWindowObject")]
    pub m_holding_info_window_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_FleaMarketBuyMenu")]
    pub m_flea_market_buy_menu: crate::app::itemshopbuymenu::ItemShopBuyMenu,
    #[rename(name = "m_ItemHoldingInfoWindow")]
    pub m_item_holding_info_window: crate::app::itemholdinginfowindow::ItemHoldingInfoWindow,
    #[rename(name = "m_ReturnEventHandler")]
    pub m_return_event_handler: crate::app::fleamarketbuyroot::FleaMarketBuyRoot_ReturnEventHandler,
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_ItemData")]
    pub m_item_data: crate::app::itemdata::ItemData,
    #[rename(name = "m_IsSendingItemToTransporter")]
    pub m_is_sending_item_to_transporter: bool,
    #[rename(name = "m_IsSendingUnitItem")]
    pub m_is_sending_unit_item: bool,
    #[rename(name = "m_SendingUnitItemIndex")]
    pub m_sending_unit_item_index: i32,
    #[rename(name = "m_IsDiscardingTransporterItem")]
    pub m_is_discarding_transporter_item: bool,
    #[rename(name = "m_DiscardingTransporterItemIndex")]
    pub m_discarding_transporter_item_index: i32,
    #[rename(name = "m_IsEnabledVoice")]
    pub m_is_enabled_voice: bool,
}

#[cfg(feature = "app-fleamarketbuyroot")]
#[::unity2::methods]
impl FleaMarketBuyRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        return_event_handler: crate::app::fleamarketbuyroot::FleaMarketBuyRoot_ReturnEventHandler,
        is_enabled_voice: bool,
    ) -> crate::app::fleamarketbuyroot::FleaMarketBuyRoot;

    #[method(name = "Create", args = 4)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        return_event_handler: crate::app::fleamarketbuyroot::FleaMarketBuyRoot_ReturnEventHandler,
        is_enabled_voice: bool,
    ) -> ();

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(root: crate::app::fleamarketbuyroot::FleaMarketBuyRoot) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnSelectMenuItem", args = 1)]
    pub fn on_select_menu_item(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "OnDecideMenuItem", args = 1)]
    pub fn on_decide_menu_item(self, item_data: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "OnYesToBuy", args = 0)]
    pub fn on_yes_to_buy(self) -> ();

    #[method(name = "OnDecideToSendItem", args = 2)]
    pub fn on_decide_to_send_item(self, is_selecting_unit_item: bool, unit_item_index: i32) -> ();

    #[method(name = "OnCancelToSendItem", args = 0)]
    pub fn on_cancel_to_send_item(self) -> ();

    #[method(name = "OnDecidelToDiscardItem", args = 2)]
    pub fn on_decidel_to_discard_item(
        self,
        transporter_is_selected: bool,
        transporter_item_index: i32,
    ) -> ();

    #[method(name = "OnCancelToDiscardItem", args = 0)]
    pub fn on_cancel_to_discard_item(self) -> ();

    #[method(name = "OnBuy", args = 0)]
    pub fn on_buy(self) -> ();

    #[method(name = "OnRequestClose", args = 0)]
    pub fn on_request_close(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();
}

#[cfg(feature = "app-fleamarketbuyroot")]
impl FleaMarketBuyRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FleaMarketBuyRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IFleaMarketBuyRootMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fleamarketbuyroot/FleaMarketBuyRoot_ReturnEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "FleaMarketBuyRoot.ReturnEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct FleaMarketBuyRoot_ReturnEventHandler {}

#[cfg(feature = "app-fleamarketbuyroot")]
#[::unity2::methods]
impl FleaMarketBuyRoot_ReturnEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-fleamarketbuyroot")]
impl FleaMarketBuyRoot_ReturnEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FleaMarketBuyRoot_ReturnEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IFleaMarketBuyRoot_ReturnEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
