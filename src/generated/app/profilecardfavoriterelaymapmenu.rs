
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardfavoriterelaymapmenu/ProfileCardFavoriteRelayMapMenu_DisposeEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardFavoriteRelayMapMenu.DisposeEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardFavoriteRelayMapMenu_DisposeEventHandler {}

#[cfg(feature = "app-profilecardfavoriterelaymapmenu")]
#[::unity2::methods]
impl ProfileCardFavoriteRelayMapMenu_DisposeEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardfavoriterelaymapmenu")]
impl ProfileCardFavoriteRelayMapMenu_DisposeEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardFavoriteRelayMapMenu_DisposeEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardFavoriteRelayMapMenu_DisposeEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardfavoriterelaymapmenu/ProfileCardFavoriteRelayMapMenu.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardFavoriteRelayMapMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ProfileCardFavoriteRelayMapMenu {
# [static_field] # [rename (name = "m_MenuItemIndexNone")] pub m_menu_item_index_none : i32 ,
# [static_field] # [rename (name = "m_MenuItemIndexEmpty")] pub m_menu_item_index_empty : i32 ,
# [rename (name = "m_DisposeEventHandler")] pub m_dispose_event_handler : crate :: app :: profilecardfavoriterelaymapmenu :: ProfileCardFavoriteRelayMapMenu_DisposeEventHandler ,
# [rename (name = "m_ProfileCardRoot")] pub m_profile_card_root : crate :: app :: profilecardroot :: ProfileCardRoot ,
# [rename (name = "m_MyProfileCardTemp")] pub m_my_profile_card_temp : crate :: app :: profilecard :: ProfileCard ,
# [rename (name = "m_DecidedMenuItemIndex")] pub m_decided_menu_item_index : i32 ,
# [rename (name = "m_Sorted")] pub m_sorted : bool ,
}

#[cfg(feature = "app-profilecardfavoriterelaymapmenu")]
#[::unity2::methods]
impl ProfileCardFavoriteRelayMapMenu {
    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::profilecardtextlistmenucontent::ProfileCardTextListMenuContent,
        profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        dispose_event_handler : crate :: app :: profilecardfavoriterelaymapmenu :: ProfileCardFavoriteRelayMapMenu_DisposeEventHandler,
    ) -> crate::app::profilecardfavoriterelaymapmenu::ProfileCardFavoriteRelayMapMenu;

    #[method(name = "CreateMenuItem", args = 3)]
    pub fn create_menu_item(
        initial_favorite_map_data : crate :: app :: profilecardfavoritemapdata :: ProfileCardFavoriteMapData,
        sorting: bool,
        initial_decided_index: i32,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        initial_decided_index: i32,
        dispose_event_handler : crate :: app :: profilecardfavoriterelaymapmenu :: ProfileCardFavoriteRelayMapMenu_DisposeEventHandler,
    ) -> ();

    #[method(name = "OnBuild", args = 1)]
    pub fn on_build(self, is_first_build: bool) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "UpdateCardRoot", args = 1)]
    pub fn update_card_root(
        self,
        favorite_map_data: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
    ) -> ();

    #[method(name = "UpdateDecided", args = 1)]
    pub fn update_decided(self, menu_item_index: i32) -> bool;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-profilecardfavoriterelaymapmenu")]
impl ProfileCardFavoriteRelayMapMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        initial_decided_index: i32,
        dispose_event_handler : crate :: app :: profilecardfavoriterelaymapmenu :: ProfileCardFavoriteRelayMapMenu_DisposeEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardFavoriteRelayMapMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardFavoriteRelayMapMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            profile_card_root,
            my_profile_card_temp,
            initial_decided_index,
            dispose_event_handler,
        );
        this
    }
}
