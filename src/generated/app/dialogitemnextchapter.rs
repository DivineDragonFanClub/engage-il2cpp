
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dialogitemnextchapter/DialogItemNextChapter.md")))]
#[::unity2::class(namespace = "App", name = "DialogItemNextChapter")]
#[parent(crate::app::basicdialogitem::BasicDialogItem)]
pub struct DialogItemNextChapter {
    #[rename(name = "m_func")]
    pub m_func: crate::system::action::Action,
}

#[cfg(feature = "app-dialogitemnextchapter")]
#[::unity2::methods]
impl DialogItemNextChapter {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, func: crate::system::action::Action) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-dialogitemnextchapter")]
impl DialogItemNextChapter {
    pub fn new(func: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DialogItemNextChapter),
                ::core::stringify!(new),
            )
        });
        <Self as IDialogItemNextChapterMethods>::ctor(this, func);
        this
    }
}
