
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaytakeovermenu/RelayTakeOverMenu_MenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RelayTakeOverMenu.MenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RelayTakeOverMenu_MenuItem {
    #[rename(name = "m_TitleText")]
    pub m_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CommentText")]
    pub m_comment_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_NameLabel")]
    pub m_name_label: ::unity2::Il2CppString,
    #[rename(name = "m_CommentLabel")]
    pub m_comment_label: ::unity2::Il2CppString,
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::relay::Relay_TakeOverModes,
}

#[cfg(feature = "app-relaytakeovermenu")]
#[::unity2::methods]
impl RelayTakeOverMenu_MenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_obj: crate::unity_engine::gameobject::GameObject,
        result: crate::app::relaytakeovermenu::RelayTakeOverMenu_Result2,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-relaytakeovermenu")]
impl RelayTakeOverMenu_MenuItem {
    pub fn new(
        menu_obj: crate::unity_engine::gameobject::GameObject,
        result: crate::app::relaytakeovermenu::RelayTakeOverMenu_Result2,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayTakeOverMenu_MenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayTakeOverMenu_MenuItemMethods>::ctor(this, menu_obj, result);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaytakeovermenu/RelayTakeOverMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelayTakeOverMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelayTakeOverMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelayTakeOverMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelayTakeOverMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelayTakeOverMenu_Result2 {
    pub fn random() -> Self {
        Self { value: 0 }
    }

    pub fn data_code() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaytakeovermenu/RelayTakeOverMenu.md")))]
#[::unity2::class(namespace = "App", name = "RelayTakeOverMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RelayTakeOverMenu {}

#[cfg(feature = "app-relaytakeovermenu")]
#[::unity2::methods]
impl RelayTakeOverMenu {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected: crate::app::relaytakeovermenu::RelayTakeOverMenu_Result2,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::relaymodemenucontent::RelayModeMenuContent,
        initial_selected: crate::app::relaytakeovermenu::RelayTakeOverMenu_Result2,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTutorial", args = 0)]
    pub fn get_tutorial(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-relaytakeovermenu")]
impl RelayTakeOverMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::relaymodemenucontent::RelayModeMenuContent,
        initial_selected: crate::app::relaytakeovermenu::RelayTakeOverMenu_Result2,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayTakeOverMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayTakeOverMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_selected,
        );
        this
    }
}
