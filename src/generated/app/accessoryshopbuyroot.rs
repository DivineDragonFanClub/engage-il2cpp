
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuyroot/AccessoryShopBuyRoot_ReturnEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopBuyRoot.ReturnEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AccessoryShopBuyRoot_ReturnEventHandler {}

#[cfg(feature = "app-accessoryshopbuyroot")]
#[::unity2::methods]
impl AccessoryShopBuyRoot_ReturnEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-accessoryshopbuyroot")]
impl AccessoryShopBuyRoot_ReturnEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyRoot_ReturnEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyRoot_ReturnEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuyroot/AccessoryShopBuyRoot.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopBuyRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct AccessoryShopBuyRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_MenuObject")]
    pub m_menu_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UnitNameObject")]
    pub m_unit_name_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UnitName")]
    pub m_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_EquipmentInfoWindowObject")]
    pub m_equipment_info_window_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DetailInfoWindowObject")]
    pub m_detail_info_window_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_KeyHelpAllObject")]
    pub m_key_help_all_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_KeyHelpAllAnimator")]
    pub m_key_help_all_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_WatchingModeKeyHelpController")]
    pub m_watching_mode_key_help_controller: crate::app::keyhelpcontroller::KeyHelpController,
    #[rename(name = "m_AccessoryShopBuyRootProc")]
    pub m_accessory_shop_buy_root_proc:
        crate::app::accessoryshopbuyrootproc::AccessoryShopBuyRootProc,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_AccessoryShopBuyMenu")]
    pub m_accessory_shop_buy_menu: crate::app::accessoryshopbuymenu::AccessoryShopBuyMenu,
    #[rename(name = "m_AccessoryEquipmentInfoWindow")]
    pub m_accessory_equipment_info_window:
        crate::app::accessoryequipmentinfo::AccessoryEquipmentInfo,
    #[rename(name = "m_AccessoryDetailInfoWindow")]
    pub m_accessory_detail_info_window:
        crate::app::accessorydetailinfowindow::AccessoryDetailInfoWindow,
    #[rename(name = "m_ReturnEventHandler")]
    pub m_return_event_handler:
        crate::app::accessoryshopbuyroot::AccessoryShopBuyRoot_ReturnEventHandler,
    #[rename(name = "m_AccessoryData")]
    pub m_accessory_data: crate::app::accessorydata::AccessoryData,
}

#[cfg(feature = "app-accessoryshopbuyroot")]
#[::unity2::methods]
impl AccessoryShopBuyRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        return_event_handler : crate :: app :: accessoryshopbuyroot :: AccessoryShopBuyRoot_ReturnEventHandler,
    ) -> crate::app::accessoryshopbuyroot::AccessoryShopBuyRoot;

    #[method(name = "Create", args = 3)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        return_event_handler : crate :: app :: accessoryshopbuyroot :: AccessoryShopBuyRoot_ReturnEventHandler,
    ) -> ();

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(root: crate::app::accessoryshopbuyroot::AccessoryShopBuyRoot) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnSelectMenuItem", args = 1)]
    pub fn on_select_menu_item(
        self,
        accessory_data: crate::app::accessorydata::AccessoryData,
    ) -> ();

    #[method(name = "OnDecideMenuItem", args = 1)]
    pub fn on_decide_menu_item(
        self,
        accessory_data: crate::app::accessorydata::AccessoryData,
    ) -> ();

    #[method(name = "OnDisposeConfirmDialog", args = 0)]
    pub fn on_dispose_confirm_dialog(self) -> ();

    #[method(name = "OnBuy", args = 0)]
    pub fn on_buy(self) -> ();

    #[method(name = "OnChangeUnitToPrev", args = 0)]
    pub fn on_change_unit_to_prev(self) -> ();

    #[method(name = "OnChangeUnitToNext", args = 0)]
    pub fn on_change_unit_to_next(self) -> ();

    #[method(name = "OnChangeKind", args = 1)]
    pub fn on_change_kind(self, accessory_data: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = "OnStartWatching", args = 0)]
    pub fn on_start_watching(self) -> bool;

    #[method(name = "OnEndWatching", args = 0)]
    pub fn on_end_watching(self) -> ();

    #[method(name = "OnShowUI", args = 0)]
    pub fn on_show_ui(self) -> ();

    #[method(name = "OnHideUI", args = 0)]
    pub fn on_hide_ui(self) -> ();

    #[method(name = "OnRequestCloseMenu", args = 0)]
    pub fn on_request_close_menu(self) -> ();
}

#[cfg(feature = "app-accessoryshopbuyroot")]
impl AccessoryShopBuyRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyRootMethods>::ctor(this);
        this
    }
}
