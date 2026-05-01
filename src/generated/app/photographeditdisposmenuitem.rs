
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographeditdisposmenuitem/PhotographEditDisposMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "PhotographEditDisposMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct PhotographEditDisposMenuItem {}

#[cfg(feature = "app-photographeditdisposmenuitem")]
#[::unity2::methods]
impl PhotographEditDisposMenuItem {
    #[method(name = "SetCharacterId", args = 1)]
    pub fn set_character_id(self, character_id: ::unity2::Il2CppString) -> ();

    #[method(name = "SetBodyAccData", args = 2)]
    pub fn set_body_acc_data(
        self,
        body_acc: crate::app::accessorydata::AccessoryData,
        gender: crate::app::accessoryshoputility::AccessoryShopUtility_Female,
    ) -> ();

    #[method(name = "SetFaceAccData", args = 1)]
    pub fn set_face_acc_data(self, face_acc: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = "SetPauseData", args = 1)]
    pub fn set_pause_data(self, pause: crate::app::photographpausedata::PhotographPauseData) -> ();

    #[method(name = "SetWeaponData", args = 1)]
    pub fn set_weapon_data(self, weapon_data: crate::app::itemdata::ItemData) -> ();

    #[method(name = "SetMascotColor", args = 1)]
    pub fn set_mascot_color(self, color_idx: i32) -> ();

    #[method(name = "UpdateColor", args = 0)]
    pub fn update_color(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-photographeditdisposmenuitem")]
impl PhotographEditDisposMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographEditDisposMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographEditDisposMenuItemMethods>::ctor(this);
        this
    }
}
