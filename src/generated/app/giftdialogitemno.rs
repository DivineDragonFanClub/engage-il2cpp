
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/giftdialogitemno/GiftDialogItemNo.md")))]
#[::unity2::class(namespace = "App", name = "GiftDialogItemNo")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct GiftDialogItemNo {}

#[cfg(feature = "app-giftdialogitemno")]
#[::unity2::methods]
impl GiftDialogItemNo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-giftdialogitemno")]
impl GiftDialogItemNo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GiftDialogItemNo),
                ::core::stringify!(new),
            )
        });
        <Self as IGiftDialogItemNoMethods>::ctor(this);
        this
    }
}
