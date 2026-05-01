
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::gridmenu::GridMenu;
use crate::app::gridmenu::IGridMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstamplistmenu/ProfileCardStampListMenu_CloseEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardStampListMenu.CloseEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardStampListMenu_CloseEventHandler {}

#[cfg(feature = "app-profilecardstamplistmenu")]
#[::unity2::methods]
impl ProfileCardStampListMenu_CloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardstamplistmenu")]
impl ProfileCardStampListMenu_CloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampListMenu_CloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampListMenu_CloseEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstamplistmenu/ProfileCardStampListMenu.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardStampListMenu")]
#[parent(crate::app::gridmenu::GridMenu)]
pub struct ProfileCardStampListMenu {
    #[static_field]
    #[rename(name = "m_MenuItemIndexNone")]
    pub m_menu_item_index_none: i32,
    #[static_field]
    #[rename(name = "m_MenuItemIndexEmpty")]
    pub m_menu_item_index_empty: i32,
    #[rename(name = "m_Selects")]
    pub m_selects: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_Category")]
    pub m_category: crate::app::profilecardstampdata::ProfileCardStampData_Categories,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardstamplistmenu::ProfileCardStampListMenu_DecideEventHandler,
    #[rename(name = "m_CloseEventHandler")]
    pub m_close_event_handler:
        crate::app::profilecardstamplistmenu::ProfileCardStampListMenu_CloseEventHandler,
    #[rename(name = "m_DisposeEventHandler")]
    pub m_dispose_event_handler:
        crate::app::profilecardstamplistmenu::ProfileCardStampListMenu_DisposeEventHandler,
}

#[cfg(feature = "app-profilecardstamplistmenu")]
#[::unity2::methods]
impl ProfileCardStampListMenu {
    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::profilecardstamplistmenucontent::ProfileCardStampListMenuContent,
        initial_stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
        decide_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_DecideEventHandler,
        close_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_CloseEventHandler,
        dispose_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_DisposeEventHandler,
    ) -> crate::app::profilecardstamplistmenu::ProfileCardStampListMenu;

    #[method(name = "CreateMenuItem", args = 4)]
    pub fn create_menu_item(
        category: crate::app::profilecardstampdata::ProfileCardStampData_Categories,
        initial_stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
        decide_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_DecideEventHandler,
        initial_decided_index: i32,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::gridmenucontent::GridMenuContent,
        initial_category: crate::app::profilecardstampdata::ProfileCardStampData_Categories,
        initial_selected_index: i32,
        decide_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_DecideEventHandler,
        close_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_CloseEventHandler,
        dispose_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_DisposeEventHandler,
    ) -> ();

    #[method(name = "OnBuild", args = 1)]
    pub fn on_build(self, is_first_build: bool) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetSelect", args = 0)]
    pub fn get_select(self) -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = "UpdateContent", args = 0)]
    pub fn update_content(self) -> ();

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "LRepeatCall", args = 1)]
    pub fn l_repeat_call(self, is_trigger: bool) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "RRepeatCall", args = 1)]
    pub fn r_repeat_call(self, is_trigger: bool) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-profilecardstamplistmenu")]
impl ProfileCardStampListMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::gridmenucontent::GridMenuContent,
        initial_category: crate::app::profilecardstampdata::ProfileCardStampData_Categories,
        initial_selected_index: i32,
        decide_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_DecideEventHandler,
        close_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_CloseEventHandler,
        dispose_event_handler : crate :: app :: profilecardstamplistmenu :: ProfileCardStampListMenu_DisposeEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampListMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampListMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_category,
            initial_selected_index,
            decide_event_handler,
            close_event_handler,
            dispose_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstamplistmenu/ProfileCardStampListMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardStampListMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardStampListMenu_DecideEventHandler {}

#[cfg(feature = "app-profilecardstamplistmenu")]
#[::unity2::methods]
impl ProfileCardStampListMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, stamp_data: crate::app::profilecardstampdata::ProfileCardStampData) -> ();
}

#[cfg(feature = "app-profilecardstamplistmenu")]
impl ProfileCardStampListMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampListMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampListMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstamplistmenu/ProfileCardStampListMenu_DisposeEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardStampListMenu.DisposeEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardStampListMenu_DisposeEventHandler {}

#[cfg(feature = "app-profilecardstamplistmenu")]
#[::unity2::methods]
impl ProfileCardStampListMenu_DisposeEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardstamplistmenu")]
impl ProfileCardStampListMenu_DisposeEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampListMenu_DisposeEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampListMenu_DisposeEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
