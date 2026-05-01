
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicdialogitemyes/BasicDialogItemYes.md")))]
#[::unity2::class(namespace = "App", name = "BasicDialogItemYes")]
#[parent(crate::app::basicdialogitem::BasicDialogItem)]
pub struct BasicDialogItemYes {}

#[cfg(feature = "app-basicdialogitemyes")]
#[::unity2::methods]
impl BasicDialogItemYes {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-basicdialogitemyes")]
impl BasicDialogItemYes {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicDialogItemYesMethods>::ctor(this);
        this
    }

    pub fn new_2(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicDialogItemYes),
                ::core::stringify!(new_2),
            )
        });
        <Self as IBasicDialogItemYesMethods>::ctor_2(this, text);
        this
    }
}
