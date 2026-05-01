
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentlistmenu/ProfileCardCommentListMenu_CancelEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardCommentListMenu.CancelEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardCommentListMenu_CancelEventHandler {}

#[cfg(feature = "app-profilecardcommentlistmenu")]
#[::unity2::methods]
impl ProfileCardCommentListMenu_CancelEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardcommentlistmenu")]
impl ProfileCardCommentListMenu_CancelEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentListMenu_CancelEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentListMenu_CancelEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentlistmenu/ProfileCardCommentListMenu_DisposeEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardCommentListMenu.DisposeEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardCommentListMenu_DisposeEventHandler {}

#[cfg(feature = "app-profilecardcommentlistmenu")]
#[::unity2::methods]
impl ProfileCardCommentListMenu_DisposeEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardcommentlistmenu")]
impl ProfileCardCommentListMenu_DisposeEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentListMenu_DisposeEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentListMenu_DisposeEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentlistmenu/ProfileCardCommentListMenu_SelectEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardCommentListMenu.SelectEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardCommentListMenu_SelectEventHandler {}

#[cfg(feature = "app-profilecardcommentlistmenu")]
#[::unity2::methods]
impl ProfileCardCommentListMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        comment_data: crate::app::profilecardcommentdata::ProfileCardCommentData,
    ) -> ();
}

#[cfg(feature = "app-profilecardcommentlistmenu")]
impl ProfileCardCommentListMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentListMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentListMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentlistmenu/ProfileCardCommentListMenu.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardCommentListMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ProfileCardCommentListMenu {
    #[static_field]
    #[rename(name = "m_MenuItemIndexNone")]
    pub m_menu_item_index_none: i32,
    #[static_field]
    #[rename(name = "m_MenuItemIndexEmpty")]
    pub m_menu_item_index_empty: i32,
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::profilecardcommentlistmenu::ProfileCardCommentListMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardcommentlistmenu::ProfileCardCommentListMenu_DecideEventHandler,
    #[rename(name = "m_CancelEventHandler")]
    pub m_cancel_event_handler:
        crate::app::profilecardcommentlistmenu::ProfileCardCommentListMenu_CancelEventHandler,
    #[rename(name = "m_DisposeEventHandler")]
    pub m_dispose_event_handler:
        crate::app::profilecardcommentlistmenu::ProfileCardCommentListMenu_DisposeEventHandler,
    #[rename(name = "m_MyProfileCardTemp")]
    pub m_my_profile_card_temp: crate::app::profilecard::ProfileCard,
    #[rename(name = "m_Category")]
    pub m_category: crate::app::profilecardcommentdata::ProfileCardCommentData_Categories,
    #[rename(name = "m_Selects")]
    pub m_selects: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_DecidedMenuItemIndex")]
    pub m_decided_menu_item_index: i32,
    #[rename(name = "m_DecidedCategory")]
    pub m_decided_category: crate::app::profilecardcommentdata::ProfileCardCommentData_Categories,
    #[rename(name = "m_DecidedCommentData")]
    pub m_decided_comment_data: crate::app::profilecardcommentdata::ProfileCardCommentData,
    #[rename(name = "m_Sorted")]
    pub m_sorted: bool,
}

#[cfg(feature = "app-profilecardcommentlistmenu")]
#[::unity2::methods]
impl ProfileCardCommentListMenu {
    #[method(name = "CreateBind", args = 8)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: profilecardmessagelistmenucontent :: ProfileCardMessageListMenuContent,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        initial_comment_data: crate::app::profilecardcommentdata::ProfileCardCommentData,
        select_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_DecideEventHandler,
        cancel_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_CancelEventHandler,
        dispose_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_DisposeEventHandler,
    ) -> crate::app::profilecardcommentlistmenu::ProfileCardCommentListMenu;

    #[method(name = "CreateMenuItem", args = 5)]
    pub fn create_menu_item(
        category: crate::app::profilecardcommentdata::ProfileCardCommentData_Categories,
        initial_comment_data: crate::app::profilecardcommentdata::ProfileCardCommentData,
        sorting: bool,
        select_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_SelectEventHandler,
        initial_decided_index: i32,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 10)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        initial_category: crate::app::profilecardcommentdata::ProfileCardCommentData_Categories,
        initial_decided_index: i32,
        initial_comment_data: crate::app::profilecardcommentdata::ProfileCardCommentData,
        select_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_DecideEventHandler,
        cancel_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_CancelEventHandler,
        dispose_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_DisposeEventHandler,
    ) -> ();

    #[method(name = "OnBuild", args = 1)]
    pub fn on_build(self, is_first_build: bool) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetSelect", args = 0)]
    pub fn get_select(self) -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = "UpdateDecided", args = 2)]
    pub fn update_decided(self, menu_item_index: i32, initial: bool) -> bool;

    #[method(name = "UpdateContent", args = 0)]
    pub fn update_content(self) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-profilecardcommentlistmenu")]
impl ProfileCardCommentListMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        initial_category: crate::app::profilecardcommentdata::ProfileCardCommentData_Categories,
        initial_decided_index: i32,
        initial_comment_data: crate::app::profilecardcommentdata::ProfileCardCommentData,
        select_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_DecideEventHandler,
        cancel_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_CancelEventHandler,
        dispose_event_handler : crate :: app :: profilecardcommentlistmenu :: ProfileCardCommentListMenu_DisposeEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentListMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentListMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            my_profile_card_temp,
            initial_category,
            initial_decided_index,
            initial_comment_data,
            select_event_handler,
            decide_event_handler,
            cancel_event_handler,
            dispose_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentlistmenu/ProfileCardCommentListMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardCommentListMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardCommentListMenu_DecideEventHandler {}

#[cfg(feature = "app-profilecardcommentlistmenu")]
#[::unity2::methods]
impl ProfileCardCommentListMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        comment_data: crate::app::profilecardcommentdata::ProfileCardCommentData,
    ) -> ();
}

#[cfg(feature = "app-profilecardcommentlistmenu")]
impl ProfileCardCommentListMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentListMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentListMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
