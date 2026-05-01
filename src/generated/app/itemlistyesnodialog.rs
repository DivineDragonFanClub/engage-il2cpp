
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::yesnodialog::IYesNoDialog;
use crate::app::yesnodialog::YesNoDialog;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemlistyesnodialog/ItemListYesNoDialog.md")))]
#[::unity2::class(namespace = "App", name = "ItemListYesNoDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct ItemListYesNoDialog {}

#[cfg(feature = "app-itemlistyesnodialog")]
#[::unity2::methods]
impl ItemListYesNoDialog {
    #[method(name = "get_m_ItemParamList", args = 0)]
    pub fn get_m_item_param_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::itemlistyesnodialog::ItemListYesNoDialog_ItemParam,
    >;

    #[method(name = "set_m_ItemParamList", args = 1)]
    pub fn set_m_item_param_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::itemlistyesnodialog::ItemListYesNoDialog_ItemParam,
        >,
    ) -> ();

    #[method(name = "get_m_MoneyParam", args = 0)]
    pub fn get_m_money_param(
        self,
    ) -> crate::app::itemlistyesnodialog::ItemListYesNoDialog_MoneyParam;

    #[method(name = "set_m_MoneyParam", args = 1)]
    pub fn set_m_money_param(
        self,
        value: crate::app::itemlistyesnodialog::ItemListYesNoDialog_MoneyParam,
    ) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        message: ::unity2::Il2CppString,
        item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemlistyesnodialog::ItemListYesNoDialog_ItemParam,
        >,
        money_param: crate::app::itemlistyesnodialog::ItemListYesNoDialog_MoneyParam,
    ) -> crate::app::itemlistyesnodialog::ItemListYesNoDialog;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::itemlistdialogcontent::ItemListDialogContent,
        message: ::unity2::Il2CppString,
        item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemlistyesnodialog::ItemListYesNoDialog_ItemParam,
        >,
        money_param: crate::app::itemlistyesnodialog::ItemListYesNoDialog_MoneyParam,
    ) -> ();
}

#[cfg(feature = "app-itemlistyesnodialog")]
impl ItemListYesNoDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::itemlistdialogcontent::ItemListDialogContent,
        message: ::unity2::Il2CppString,
        item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::itemlistyesnodialog::ItemListYesNoDialog_ItemParam,
        >,
        money_param: crate::app::itemlistyesnodialog::ItemListYesNoDialog_MoneyParam,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemListYesNoDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IItemListYesNoDialogMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            message,
            item_param_list,
            money_param,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemlistyesnodialog/ItemListYesNoDialog_ItemParam.md")))]
#[::unity2::class(namespace = "App", name = "ItemListYesNoDialog.ItemParam")]
#[parent(crate::system::object::Object)]
pub struct ItemListYesNoDialog_ItemParam {
    #[rename(name = "sprite")]
    pub sprite: crate::unity_engine::sprite::Sprite,
    #[rename(name = "name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "num")]
    pub num: i32,
}

#[cfg(feature = "app-itemlistyesnodialog")]
#[::unity2::methods]
impl ItemListYesNoDialog_ItemParam {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-itemlistyesnodialog")]
impl ItemListYesNoDialog_ItemParam {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemListYesNoDialog_ItemParam),
                ::core::stringify!(new),
            )
        });
        <Self as IItemListYesNoDialog_ItemParamMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemlistyesnodialog/ItemListYesNoDialog_MoneyParam.md")))]
#[::unity2::class(namespace = "App", name = "ItemListYesNoDialog.MoneyParam")]
#[parent(crate::system::object::Object)]
pub struct ItemListYesNoDialog_MoneyParam {
    #[rename(name = "num")]
    pub num: i32,
}

#[cfg(feature = "app-itemlistyesnodialog")]
#[::unity2::methods]
impl ItemListYesNoDialog_MoneyParam {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-itemlistyesnodialog")]
impl ItemListYesNoDialog_MoneyParam {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemListYesNoDialog_MoneyParam),
                ::core::stringify!(new),
            )
        });
        <Self as IItemListYesNoDialog_MoneyParamMethods>::ctor(this);
        this
    }
}
