
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotfoodselectmenu/MascotFoodSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "MascotFoodSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MascotFoodSelectMenu {}

#[cfg(feature = "app-mascotfoodselectmenu")]
#[::unity2::methods]
impl MascotFoodSelectMenu {
    #[method(name = "get_CurrentMenuSelect", args = 0)]
    pub fn get_current_menu_select() -> crate::app::basicmenuselect::BasicMenuSelect;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "SetHelpText", args = 1)]
    pub fn set_help_text(self, help_text: ::unity2::Il2CppString) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mascotfoodselectmenu")]
impl MascotFoodSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotFoodSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotFoodSelectMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
