
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
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtopmenu/ProfileCardTopMenu_StampVisibilitySettingMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "ProfileCardTopMenu.StampVisibilitySettingMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardTopMenu_StampVisibilitySettingMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-profilecardtopmenu")]
#[::unity2::methods]
impl ProfileCardTopMenu_StampVisibilitySettingMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardtopmenu")]
impl ProfileCardTopMenu_StampVisibilitySettingMenuItem {
    pub fn new(
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardTopMenu_StampVisibilitySettingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardTopMenu_StampVisibilitySettingMenuItemMethods>::ctor(
            this,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtopmenu/ProfileCardTopMenu_MyCardMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardTopMenu.MyCardMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardTopMenu_MyCardMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-profilecardtopmenu")]
#[::unity2::methods]
impl ProfileCardTopMenu_MyCardMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardtopmenu")]
impl ProfileCardTopMenu_MyCardMenuItem {
    pub fn new(
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardTopMenu_MyCardMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardTopMenu_MyCardMenuItemMethods>::ctor(this, decide_event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtopmenu/ProfileCardTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardTopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ProfileCardTopMenu {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    #[rename(name = "m_Root")]
    pub m_root: crate::app::profilecardtopmenuroot::ProfileCardTopMenuRoot,
}

#[cfg(feature = "app-profilecardtopmenu")]
#[::unity2::methods]
impl ProfileCardTopMenu {
    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected: crate::app::profilecardtopmenu::ProfileCardTopMenu_Result2,
        enabled_photo: bool,
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> crate::app::profilecardtopmenu::ProfileCardTopMenu;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        menu_root: crate::app::profilecardtopmenuroot::ProfileCardTopMenuRoot,
        initial_selected: crate::app::profilecardtopmenu::ProfileCardTopMenu_Result2,
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "UpdateInfoWindow", args = 3)]
    pub fn update_info_window(
        self,
        caption_mid: ::unity2::Il2CppString,
        description_mid: ::unity2::Il2CppString,
        warning_mid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-profilecardtopmenu")]
impl ProfileCardTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        menu_root: crate::app::profilecardtopmenuroot::ProfileCardTopMenuRoot,
        initial_selected: crate::app::profilecardtopmenu::ProfileCardTopMenu_Result2,
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardTopMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            menu_root,
            initial_selected,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtopmenu/ProfileCardTopMenu_PublicSettingMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardTopMenu.PublicSettingMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardTopMenu_PublicSettingMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-profilecardtopmenu")]
#[::unity2::methods]
impl ProfileCardTopMenu_PublicSettingMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardtopmenu")]
impl ProfileCardTopMenu_PublicSettingMenuItem {
    pub fn new(
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardTopMenu_PublicSettingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardTopMenu_PublicSettingMenuItemMethods>::ctor(
            this,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtopmenu/ProfileCardTopMenu_PhotoMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardTopMenu.PhotoMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardTopMenu_PhotoMenuItem {
    #[rename(name = "m_Enabled")]
    pub m_enabled: bool,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-profilecardtopmenu")]
#[::unity2::methods]
impl ProfileCardTopMenu_PhotoMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        enabled: bool,
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardtopmenu")]
impl ProfileCardTopMenu_PhotoMenuItem {
    pub fn new(
        enabled: bool,
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardTopMenu_PhotoMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardTopMenu_PhotoMenuItemMethods>::ctor(
            this,
            enabled,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtopmenu/ProfileCardTopMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardTopMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardTopMenu_DecideEventHandler {}

#[cfg(feature = "app-profilecardtopmenu")]
#[::unity2::methods]
impl ProfileCardTopMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::profilecardtopmenu::ProfileCardTopMenu_Result2) -> ();
}

#[cfg(feature = "app-profilecardtopmenu")]
impl ProfileCardTopMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardTopMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardTopMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtopmenu/ProfileCardTopMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ProfileCardTopMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ProfileCardTopMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProfileCardTopMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfileCardTopMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ProfileCardTopMenu_Result2 {
    pub fn my_card() -> Self {
        Self { value: 0 }
    }

    pub fn album() -> Self {
        Self { value: 1 }
    }

    pub fn photo() -> Self {
        Self { value: 2 }
    }

    pub fn public_setting() -> Self {
        Self { value: 3 }
    }

    pub fn stamp_visibility_setting() -> Self {
        Self { value: 4 }
    }

    pub fn end() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardtopmenu/ProfileCardTopMenu_AlbumMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardTopMenu.AlbumMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardTopMenu_AlbumMenuItem {
    #[rename(name = "m_Enabled")]
    pub m_enabled: bool,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-profilecardtopmenu")]
#[::unity2::methods]
impl ProfileCardTopMenu_AlbumMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-profilecardtopmenu")]
impl ProfileCardTopMenu_AlbumMenuItem {
    pub fn new(
        decide_event_handler: crate::app::profilecardtopmenu::ProfileCardTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardTopMenu_AlbumMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardTopMenu_AlbumMenuItemMethods>::ctor(this, decide_event_handler);
        this
    }
}
