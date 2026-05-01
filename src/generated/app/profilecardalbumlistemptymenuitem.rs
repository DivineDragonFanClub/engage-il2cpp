
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardalbumlistemptymenuitem/ProfileCardAlbumListEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardAlbumListEmptyMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ProfileCardAlbumListEmptyMenuItem {}

#[cfg(feature = "app-profilecardalbumlistemptymenuitem")]
#[::unity2::methods]
impl ProfileCardAlbumListEmptyMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-profilecardalbumlistemptymenuitem")]
impl ProfileCardAlbumListEmptyMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardAlbumListEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardAlbumListEmptyMenuItemMethods>::ctor(this);
        this
    }
}
