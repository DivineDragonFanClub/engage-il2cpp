
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/friendlistselectmenu/FriendListSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "FriendListSelectMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct FriendListSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-friendlistselectmenu")]
#[::unity2::methods]
impl FriendListSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, pid: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-friendlistselectmenu")]
impl FriendListSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FriendListSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IFriendListSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/friendlistselectmenu/FriendListSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "FriendListSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct FriendListSelectMenu {
    #[static_field]
    #[rename(name = "s_ScrollIndex")]
    pub s_scroll_index: i32,
}

#[cfg(feature = "app-friendlistselectmenu")]
#[::unity2::methods]
impl FriendListSelectMenu {
    #[method(name = "ResetScrollRecord", args = 0)]
    pub fn reset_scroll_record() -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        parent: crate::app::procinst::ProcInst,
        friend_list_data_array: crate::system::collections::generic::list_1::List_1<
            crate::app::friendlistdata::FriendListData,
        >,
        selected_friend_data: crate::app::friendlistdata::FriendListData,
        decide_event_handler : crate :: app :: friendlistselectmenu :: FriendListSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::friendlistselectmenucontent::FriendListSelectMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-friendlistselectmenu")]
impl FriendListSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::friendlistselectmenucontent::FriendListSelectMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FriendListSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IFriendListSelectMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/friendlistselectmenu/FriendListSelectMenu_FriendListSelectMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "FriendListSelectMenu.FriendListSelectMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct FriendListSelectMenu_FriendListSelectMenuItem {
    #[rename(name = "m_Pid")]
    pub m_pid: ::unity2::Il2CppString,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::friendlistselectmenu::FriendListSelectMenu_DecideEventHandler,
}

#[cfg(feature = "app-friendlistselectmenu")]
#[::unity2::methods]
impl FriendListSelectMenu_FriendListSelectMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        pid: ::unity2::Il2CppString,
        decide_event_handler : crate :: app :: friendlistselectmenu :: FriendListSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetPID", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-friendlistselectmenu")]
impl FriendListSelectMenu_FriendListSelectMenuItem {
    pub fn new(
        pid: ::unity2::Il2CppString,
        decide_event_handler : crate :: app :: friendlistselectmenu :: FriendListSelectMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FriendListSelectMenu_FriendListSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IFriendListSelectMenu_FriendListSelectMenuItemMethods>::ctor(
            this,
            pid,
            decide_event_handler,
        );
        this
    }
}
