
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/confirmdialogitemno/ConfirmDialogItemNo.md")))]
#[::unity2::class(namespace = "App", name = "ConfirmDialogItemNo")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct ConfirmDialogItemNo {}

#[cfg(feature = "app-confirmdialogitemno")]
#[::unity2::methods]
impl ConfirmDialogItemNo {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, label: ::unity2::Il2CppString) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-confirmdialogitemno")]
impl ConfirmDialogItemNo {
    pub fn new(label: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConfirmDialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IConfirmDialogItemNoMethods>::ctor(this, label);
        this
    }
}
