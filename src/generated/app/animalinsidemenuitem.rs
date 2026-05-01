
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalinsidemenuitem/AnimalInsideMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "AnimalInsideMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct AnimalInsideMenuItem {}

#[cfg(feature = "app-animalinsidemenuitem")]
#[::unity2::methods]
impl AnimalInsideMenuItem {
    #[method(name = "get_Animal", args = 0)]
    pub fn get_animal(self) -> crate::app::animaldata::AnimalData;

    #[method(name = "get_IsActive", args = 0)]
    pub fn get_is_active(self) -> bool;

    #[method(name = "set_IsActive", args = 1)]
    pub fn set_is_active(self, value: bool) -> ();

    #[method(name = "get_IsActiveSelect", args = 0)]
    pub fn get_is_active_select(self) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, animal: crate::app::animaldata::AnimalData) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCountText", args = 0)]
    pub fn get_count_text(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-animalinsidemenuitem")]
impl AnimalInsideMenuItem {
    pub fn new(animal: crate::app::animaldata::AnimalData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalInsideMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalInsideMenuItemMethods>::ctor(this, animal);
        this
    }
}
