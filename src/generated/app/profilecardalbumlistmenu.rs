
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
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardalbumlistmenu/ProfileCardAlbumListMenu_DeleteEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardAlbumListMenu.DeleteEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardAlbumListMenu_DeleteEventHandler {}

#[cfg(feature = "app-profilecardalbumlistmenu")]
#[::unity2::methods]
impl ProfileCardAlbumListMenu_DeleteEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardalbumlistmenu")]
impl ProfileCardAlbumListMenu_DeleteEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardAlbumListMenu_DeleteEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardAlbumListMenu_DeleteEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardalbumlistmenu/ProfileCardAlbumListMenu.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardAlbumListMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ProfileCardAlbumListMenu {
    #[rename(name = "m_ProfileCardRoot")]
    pub m_profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardalbumlistmenu::ProfileCardAlbumListMenu_DecideEventHandler,
    #[rename(name = "m_DeleteEventHandler")]
    pub m_delete_event_handler:
        crate::app::profilecardalbumlistmenu::ProfileCardAlbumListMenu_DeleteEventHandler,
    #[rename(name = "m_DisposeEventHandler")]
    pub m_dispose_event_handler:
        crate::app::profilecardalbumlistmenu::ProfileCardAlbumListMenu_DisposeEventHandler,
    #[rename(name = "m_FirstTimeUpdateCardRoot")]
    pub m_first_time_update_card_root: bool,
}

#[cfg(feature = "app-profilecardalbumlistmenu")]
#[::unity2::methods]
impl ProfileCardAlbumListMenu {
    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::profilecardalbumlistmenucontent::ProfileCardAlbumListMenuContent,
        profiled_card_root: crate::app::profilecardroot::ProfileCardRoot,
        initial_index: i32,
        decide_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DecideEventHandler,
        delete_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DeleteEventHandler,
        dispose_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DisposeEventHandler,
    ) -> crate::app::profilecardalbumlistmenu::ProfileCardAlbumListMenu;

    #[method(name = "CreateMenuItem", args = 1)]
    pub fn create_menu_item(
        decide_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DecideEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
        initial_index: i32,
        decide_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DecideEventHandler,
        delete_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DeleteEventHandler,
        dispose_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DisposeEventHandler,
    ) -> ();

    #[method(name = "OnBuild", args = 1)]
    pub fn on_build(self, is_first_build: bool) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "RemoveProfileCard", args = 1)]
    pub fn remove_profile_card(self, index: i32) -> ();

    #[method(name = "UpdateCardRoot", args = 1)]
    pub fn update_card_root(self, profile_card: crate::app::profilecard::ProfileCard) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-profilecardalbumlistmenu")]
impl ProfileCardAlbumListMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
        initial_index: i32,
        decide_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DecideEventHandler,
        delete_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DeleteEventHandler,
        dispose_event_handler : crate :: app :: profilecardalbumlistmenu :: ProfileCardAlbumListMenu_DisposeEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardAlbumListMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardAlbumListMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            profile_card_root,
            initial_index,
            decide_event_handler,
            delete_event_handler,
            dispose_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardalbumlistmenu/ProfileCardAlbumListMenu_DisposeEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardAlbumListMenu.DisposeEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardAlbumListMenu_DisposeEventHandler {}

#[cfg(feature = "app-profilecardalbumlistmenu")]
#[::unity2::methods]
impl ProfileCardAlbumListMenu_DisposeEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardalbumlistmenu")]
impl ProfileCardAlbumListMenu_DisposeEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardAlbumListMenu_DisposeEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardAlbumListMenu_DisposeEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardalbumlistmenu/ProfileCardAlbumListMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardAlbumListMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardAlbumListMenu_DecideEventHandler {}

#[cfg(feature = "app-profilecardalbumlistmenu")]
#[::unity2::methods]
impl ProfileCardAlbumListMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        result: crate::app::profilecardalbumlistmenu::ProfileCardAlbumListMenu_Result2,
        index: i32,
    ) -> ();
}

#[cfg(feature = "app-profilecardalbumlistmenu")]
impl ProfileCardAlbumListMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardAlbumListMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardAlbumListMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardalbumlistmenu/ProfileCardAlbumListMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ProfileCardAlbumListMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ProfileCardAlbumListMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProfileCardAlbumListMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfileCardAlbumListMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ProfileCardAlbumListMenu_Result2 {
    pub fn decide() -> Self {
        Self { value: 0 }
    }

    pub fn cancel() -> Self {
        Self { value: 1 }
    }

    pub fn close_album() -> Self {
        Self { value: 2 }
    }
}
