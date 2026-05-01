
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dialogitemconfirmcancel/DialogItemConfirmCancel.md")))]
#[::unity2::class(namespace = "App", name = "DialogItemConfirmCancel")]
#[parent(crate::app::basicdialogitem::BasicDialogItem)]
pub struct DialogItemConfirmCancel {}

#[cfg(feature = "app-dialogitemconfirmcancel")]
#[::unity2::methods]
impl DialogItemConfirmCancel {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dialogitemconfirmcancel")]
impl DialogItemConfirmCancel {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DialogItemConfirmCancel),
                ::core::stringify!(new),
            )
        });
        <Self as IDialogItemConfirmCancelMethods>::ctor(this);
        this
    }
}
