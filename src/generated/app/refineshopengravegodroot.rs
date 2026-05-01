
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodroot/RefineShopEngraveGodRoot_ConfirmDialog.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopEngraveGodRoot.ConfirmDialog")]
#[parent(crate::system::object::Object)]
pub struct RefineShopEngraveGodRoot_ConfirmDialog {}

#[cfg(feature = "app-refineshopengravegodroot")]
#[::unity2::methods]
impl RefineShopEngraveGodRoot_ConfirmDialog {
    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        base_unit_item: crate::app::unititem::UnitItem,
        god_data: crate::app::goddata::GodData,
        yes_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ConfirmDialog_YesEventHandler,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refineshopengravegodroot")]
impl RefineShopEngraveGodRoot_ConfirmDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodRoot_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodRoot_ConfirmDialogMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodroot/RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopEngraveGodRoot.ConfirmToReplaceDialog.YesEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandler {}

#[cfg(feature = "app-refineshopengravegodroot")]
#[::unity2::methods]
impl RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refineshopengravegodroot")]
impl RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodroot/RefineShopEngraveGodRoot_ConfirmToReplaceDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopEngraveGodRoot.ConfirmToReplaceDialog"
)]
#[parent(crate::system::object::Object)]
pub struct RefineShopEngraveGodRoot_ConfirmToReplaceDialog {}

#[cfg(feature = "app-refineshopengravegodroot")]
#[::unity2::methods]
impl RefineShopEngraveGodRoot_ConfirmToReplaceDialog {
    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        engraved_unit_item: crate::app::unititem::UnitItem,
        god_data: crate::app::goddata::GodData,
        yes_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandler,
    ) -> crate::app::yesnodialog::YesNoDialog;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refineshopengravegodroot")]
impl RefineShopEngraveGodRoot_ConfirmToReplaceDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodRoot_ConfirmToReplaceDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodRoot_ConfirmToReplaceDialogMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodroot/RefineShopEngraveGodRoot_ConfirmDialog_YesEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopEngraveGodRoot.ConfirmDialog.YesEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopEngraveGodRoot_ConfirmDialog_YesEventHandler {}

#[cfg(feature = "app-refineshopengravegodroot")]
#[::unity2::methods]
impl RefineShopEngraveGodRoot_ConfirmDialog_YesEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refineshopengravegodroot")]
impl RefineShopEngraveGodRoot_ConfirmDialog_YesEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodRoot_ConfirmDialog_YesEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodRoot_ConfirmDialog_YesEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodroot/RefineShopEngraveGodRoot.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopEngraveGodRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RefineShopEngraveGodRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_RefineShopEngraveGodMenuObject")]
    pub m_refine_shop_engrave_god_menu_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ItemDetailWindowBase")]
    pub m_item_detail_window_base: crate::app::refineitemdetailwindow::RefineItemDetailWindow,
    #[rename(name = "m_ItemDetailWindowTarget")]
    pub m_item_detail_window_target: crate::app::refineitemdetailwindow::RefineItemDetailWindow,
    #[rename(name = "m_RefineShopEngraveGodMenu")]
    pub m_refine_shop_engrave_god_menu:
        crate::app::refineshopengravegodmenu::RefineShopEngraveGodMenu,
    #[rename(name = "m_ReturnEventHandler")]
    pub m_return_event_handler:
        crate::app::refineshopengravegodroot::RefineShopEngraveGodRoot_ReturnEventHandler,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_OwnerItemIndex")]
    pub m_owner_item_index: i32,
    #[rename(name = "m_GodData")]
    pub m_god_data: crate::app::goddata::GodData,
}

#[cfg(feature = "app-refineshopengravegodroot")]
#[::unity2::methods]
impl RefineShopEngraveGodRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        shop_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
        unit: crate::app::unit::Unit,
        item_index: i32,
        return_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ReturnEventHandler,
    ) -> crate::app::refineshopengravegodroot::RefineShopEngraveGodRoot;

    #[method(name = "Create", args = 5)]
    pub fn create(
        self,
        super_: crate::app::procinst::ProcInst,
        shop_weapon_model_renderer: crate::app::shopweaponmodelrenderer::ShopWeaponModelRenderer,
        unit: crate::app::unit::Unit,
        item_index: i32,
        return_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ReturnEventHandler,
    ) -> ();

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(root: crate::app::refineshopengravegodroot::RefineShopEngraveGodRoot) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "UpdateItemDetail", args = 2)]
    pub fn update_item_detail(
        self,
        item_detail_window: crate::app::refineitemdetailwindow::RefineItemDetailWindow,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "UpdateItemDetail", args = 3)]
    pub fn update_item_detail_2(
        self,
        item_detail_window: crate::app::refineitemdetailwindow::RefineItemDetailWindow,
        unit_item_base: crate::app::unititem::UnitItem,
        unit_item_target: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "OnSelectMenuItem", args = 2)]
    pub fn on_select_menu_item(
        self,
        god_data: crate::app::goddata::GodData,
        unit_item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "OnDecideMenuItem", args = 1)]
    pub fn on_decide_menu_item(self, god_data: crate::app::goddata::GodData) -> ();

    #[method(name = "OnConfirmYes", args = 0)]
    pub fn on_confirm_yes(self) -> ();

    #[method(name = "Engrave", args = 1)]
    pub fn engrave(self, reset_unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "OnRequestCloseMenu", args = 0)]
    pub fn on_request_close_menu(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();
}

#[cfg(feature = "app-refineshopengravegodroot")]
impl RefineShopEngraveGodRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodRootMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodroot/RefineShopEngraveGodRoot_ConfirmDialog_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopEngraveGodRoot.ConfirmDialog.YesMenuItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct RefineShopEngraveGodRoot_ConfirmDialog_YesMenuItem {
# [rename (name = "m_YesEventHandler")] pub m_yes_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ConfirmDialog_YesEventHandler ,
}

#[cfg(feature = "app-refineshopengravegodroot")]
#[::unity2::methods]
impl RefineShopEngraveGodRoot_ConfirmDialog_YesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        yes_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ConfirmDialog_YesEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshopengravegodroot")]
impl RefineShopEngraveGodRoot_ConfirmDialog_YesMenuItem {
    pub fn new(
        yes_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ConfirmDialog_YesEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodRoot_ConfirmDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodRoot_ConfirmDialog_YesMenuItemMethods>::ctor(
            this,
            yes_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodroot/RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopEngraveGodRoot.ConfirmToReplaceDialog.YesMenuItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesMenuItem {
# [rename (name = "m_YesEventHandler")] pub m_yes_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandler ,
}

#[cfg(feature = "app-refineshopengravegodroot")]
#[::unity2::methods]
impl RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        yes_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshopengravegodroot")]
impl RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesMenuItem {
    pub fn new(
        yes_event_handler : crate :: app :: refineshopengravegodroot :: RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodRoot_ConfirmToReplaceDialog_YesMenuItemMethods>::ctor(
            this,
            yes_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodroot/RefineShopEngraveGodRoot_ReturnEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopEngraveGodRoot.ReturnEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopEngraveGodRoot_ReturnEventHandler {}

#[cfg(feature = "app-refineshopengravegodroot")]
#[::unity2::methods]
impl RefineShopEngraveGodRoot_ReturnEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        result: crate::app::basicmenu::BasicMenu_Result,
        god_data: crate::app::goddata::GodData,
        after_unit_item: crate::app::unititem::UnitItem,
    ) -> ();
}

#[cfg(feature = "app-refineshopengravegodroot")]
impl RefineShopEngraveGodRoot_ReturnEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodRoot_ReturnEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodRoot_ReturnEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
