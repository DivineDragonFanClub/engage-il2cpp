
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/stringitem/StringItem.md")))]
#[::unity2::class(namespace = "App", name = "StringItem")]
#[parent(crate::app::menuitem::MenuItem)]
pub struct StringItem {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_English")]
    pub m_english: ::unity2::Il2CppString,
}

#[cfg(feature = "app-stringitem")]
#[::unity2::methods]
impl StringItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString, english: ::unity2::Il2CppString) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetNameEnglish", args = 0)]
    pub fn get_name_english(self) -> ::unity2::Il2CppString;

    #[method(name = "SetName", args = 2)]
    pub fn set_name(self, name: ::unity2::Il2CppString, english: ::unity2::Il2CppString) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-stringitem")]
impl StringItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StringItem),
                ::core::stringify!(new),
            )
        });
        <Self as IStringItemMethods>::ctor(this);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString, english: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StringItem),
                ::core::stringify!(new_2),
            )
        });
        <Self as IStringItemMethods>::ctor_2(this, name, english);
        this
    }
}
