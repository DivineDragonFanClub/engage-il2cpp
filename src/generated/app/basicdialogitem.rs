
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/basicdialogitem/BasicDialogItem.md")))]
#[::unity2::class(namespace = "App", name = "BasicDialogItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct BasicDialogItem {
    #[rename(name = "m_Text")]
    pub m_text: ::unity2::Il2CppString,
}

#[cfg(feature = "app-basicdialogitem")]
#[::unity2::methods]
impl BasicDialogItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-basicdialogitem")]
impl BasicDialogItem {
    pub fn new(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasicDialogItem),
                ::core::stringify!(new),
            )
        });
        <Self as IBasicDialogItemMethods>::ctor(this, text);
        this
    }
}
