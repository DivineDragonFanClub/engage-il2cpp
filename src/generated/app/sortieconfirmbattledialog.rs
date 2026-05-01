
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::yesnodialog::IYesNoDialog;
use crate::app::yesnodialog::YesNoDialog;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieconfirmbattledialog/SortieConfirmBattleDialog_ConfirmYesDialogItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SortieConfirmBattleDialog.ConfirmYesDialogItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct SortieConfirmBattleDialog_ConfirmYesDialogItem {}

#[cfg(feature = "app-sortieconfirmbattledialog")]
#[::unity2::methods]
impl SortieConfirmBattleDialog_ConfirmYesDialogItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortieconfirmbattledialog")]
impl SortieConfirmBattleDialog_ConfirmYesDialogItem {
    pub fn new(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieConfirmBattleDialog_ConfirmYesDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieConfirmBattleDialog_ConfirmYesDialogItemMethods>::ctor(this, text);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieconfirmbattledialog/SortieConfirmBattleDialog_From.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortieConfirmBattleDialog_From {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortieConfirmBattleDialog_From {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SortieConfirmBattleDialog.From";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortieConfirmBattleDialog_From {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortieConfirmBattleDialog_From {
    pub fn top() -> Self {
        Self { value: 0 }
    }

    pub fn unit_select() -> Self {
        Self { value: 1 }
    }

    pub fn troop_list() -> Self {
        Self { value: 2 }
    }

    pub fn num() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieconfirmbattledialog/SortieConfirmBattleDialog.md")))]
#[::unity2::class(namespace = "App", name = "SortieConfirmBattleDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct SortieConfirmBattleDialog {
    #[static_field]
    #[rename(name = "m_from")]
    pub m_from: crate::app::sortieconfirmbattledialog::SortieConfirmBattleDialog_From,
}

#[cfg(feature = "app-sortieconfirmbattledialog")]
#[::unity2::methods]
impl SortieConfirmBattleDialog {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        from: crate::app::sortieconfirmbattledialog::SortieConfirmBattleDialog_From,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-sortieconfirmbattledialog")]
impl SortieConfirmBattleDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieConfirmBattleDialog),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieConfirmBattleDialogMethods>::ctor(this, menu_item_list);
        this
    }
}
