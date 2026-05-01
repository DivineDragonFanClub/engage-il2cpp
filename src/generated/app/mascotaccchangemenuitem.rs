
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotaccchangemenuitem/MascotAccChangeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MascotAccChangeMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MascotAccChangeMenuItem {
    #[rename(name = "m_mascotData")]
    pub m_mascot_data: crate::app::mascotaccdata::MascotAccData,
    #[rename(name = "m_accData")]
    pub m_acc_data: crate::app::accessorydata::AccessoryData,
}

#[cfg(feature = "app-mascotaccchangemenuitem")]
#[::unity2::methods]
impl MascotAccChangeMenuItem {
    #[method(name = "get_IsEquip", args = 0)]
    pub fn get_is_equip(self) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, mascot_data: crate::app::mascotaccdata::MascotAccData) -> ();

    #[method(name = "GetPartsType", args = 0)]
    pub fn get_parts_type(self) -> crate::app::mascotaccdata::MascotAccData_PartsType;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "UpdateEquipState", args = 0)]
    pub fn update_equip_state(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mascotaccchangemenuitem")]
impl MascotAccChangeMenuItem {
    pub fn new(mascot_data: crate::app::mascotaccdata::MascotAccData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotAccChangeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotAccChangeMenuItemMethods>::ctor(this, mascot_data);
        this
    }
}
