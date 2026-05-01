
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/materiallistmenu/MaterialListMenu.md")))]
#[::unity2::class(namespace = "App", name = "MaterialListMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MaterialListMenu {
    #[rename(name = "m_MenuItemListAll")]
    pub m_menu_item_list_all: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuitem::BasicMenuItem,
    >,
    #[rename(name = "m_MenuItemListGift")]
    pub m_menu_item_list_gift: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuitem::BasicMenuItem,
    >,
    #[rename(name = "m_MenuItemListFood")]
    pub m_menu_item_list_food: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuitem::BasicMenuItem,
    >,
    #[rename(name = "m_MenuItemListMaterial")]
    pub m_menu_item_list_material: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuitem::BasicMenuItem,
    >,
    #[rename(name = "m_MenuItemListOther")]
    pub m_menu_item_list_other: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuitem::BasicMenuItem,
    >,
    #[rename(name = "m_MenuSelectList")]
    pub m_menu_select_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuselect::BasicMenuSelect,
    >,
}

#[cfg(feature = "app-materiallistmenu")]
#[::unity2::methods]
impl MaterialListMenu {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
    ) -> crate::app::materiallistmenu::MaterialListMenu;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        menu_item_list_gift: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_item_list_food: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_item_list_material: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_item_list_other: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "SetItemHelpText", args = 1)]
    pub fn set_item_help_text(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "GetCategory", args = 0)]
    pub fn get_category(
        self,
    ) -> crate::app::materiallistmenucontent::MaterialListMenuContent_CategoryType;

    #[method(name = "SortCategory", args = 0)]
    pub fn sort_category(self) -> ();
}

#[cfg(feature = "app-materiallistmenu")]
impl MaterialListMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        menu_item_list_gift: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_item_list_food: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_item_list_material: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_item_list_other: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MaterialListMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMaterialListMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            menu_item_list_gift,
            menu_item_list_food,
            menu_item_list_material,
            menu_item_list_other,
        );
        this
    }
}
