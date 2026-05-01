
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalinsidemenu/AnimalInsideMenu.md")))]
#[::unity2::class(namespace = "App", name = "AnimalInsideMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct AnimalInsideMenu {}

#[cfg(feature = "app-animalinsidemenu")]
#[::unity2::methods]
impl AnimalInsideMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "Create", args = 1)]
    pub fn create(
        menu_content: crate::app::animalinsidemenucontent::AnimalInsideMenuContent,
    ) -> crate::app::animalinsidemenu::AnimalInsideMenu;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "PublicBuild", args = 0)]
    pub fn public_build(self) -> ();

    #[method(name = "PublicRebuild", args = 0)]
    pub fn public_rebuild(self) -> ();

    #[method(name = "PublicUpdate", args = 0)]
    pub fn public_update(self) -> ();

    #[method(name = "Suspend", args = 0)]
    pub fn suspend(self) -> ();

    #[method(name = "Resume", args = 0)]
    pub fn resume(self) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();
}

#[cfg(feature = "app-animalinsidemenu")]
impl AnimalInsideMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalInsideMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalInsideMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
