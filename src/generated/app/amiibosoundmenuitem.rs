
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibosoundmenuitem/AmiiboSoundMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboSoundMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct AmiiboSoundMenuItem {
    #[rename(name = "m_musicData")]
    pub m_music_data: crate::app::musicdata::MusicData,
}

#[cfg(feature = "app-amiibosoundmenuitem")]
#[::unity2::methods]
impl AmiiboSoundMenuItem {
    #[method(name = "get_IsEmpty", args = 0)]
    pub fn get_is_empty(self) -> bool;

    #[method(name = "set_IsEmpty", args = 1)]
    pub fn set_is_empty(self, value: bool) -> ();

    #[method(name = "get_IsNew", args = 0)]
    pub fn get_is_new(self) -> bool;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, music_data: crate::app::musicdata::MusicData, empty: bool) -> ();

    #[method(name = "SawNewMusic", args = 0)]
    pub fn saw_new_music(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-amiibosoundmenuitem")]
impl AmiiboSoundMenuItem {
    pub fn new(music_data: crate::app::musicdata::MusicData, empty: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboSoundMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboSoundMenuItemMethods>::ctor(this, music_data, empty);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibosoundmenuitem/AmiiboSoundMenuItem_YesItem.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboSoundMenuItem.YesItem")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct AmiiboSoundMenuItem_YesItem {}

#[cfg(feature = "app-amiibosoundmenuitem")]
#[::unity2::methods]
impl AmiiboSoundMenuItem_YesItem {
    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-amiibosoundmenuitem")]
impl AmiiboSoundMenuItem_YesItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboSoundMenuItem_YesItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboSoundMenuItem_YesItemMethods>::ctor(this);
        this
    }
}
