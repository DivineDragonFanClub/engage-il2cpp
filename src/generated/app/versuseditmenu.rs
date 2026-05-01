
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versuseditmenu/VersusEditMenu.md")))]
#[::unity2::class(namespace = "App", name = "VersusEditMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct VersusEditMenu {
    #[static_field]
    #[rename(name = "m_MenuEditContent")]
    pub m_menu_edit_content: crate::app::versusmapeditcontent::VersusMapEditContent,
    #[static_field]
    #[rename(name = "m_ToOneCallCallback")]
    pub m_to_one_call_callback: crate::system::action_1::Action_1<i32>,
    #[static_field]
    #[rename(name = "m_FirstSelect")]
    pub m_first_select: i32,
    #[static_field]
    #[rename(name = "m_OnCloseCallback")]
    pub m_on_close_callback: crate::system::action::Action,
}

#[cfg(feature = "app-versuseditmenu")]
#[::unity2::methods]
impl VersusEditMenu {
    #[method(name = "get_MenuItemContentOld", args = 0)]
    pub fn get_menu_item_content_old(
        self,
    ) -> crate::app::versusmapeditobjlistmenuitemcontent::VersusMapEditObjListMenuItemContent;

    #[method(name = "set_MenuItemContentOld", args = 1)]
    pub fn set_menu_item_content_old(
        self,
        value: crate::app::versusmapeditobjlistmenuitemcontent::VersusMapEditObjListMenuItemContent,
    ) -> ();

    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        obj_list: crate::system::collections::generic::list_1::List_1<
            crate::app::mapeditorobjectdata::MapEditorObjectData,
        >,
        menu_edit_content: crate::app::versusmapeditcontent::VersusMapEditContent,
        to_one_call_callback: crate::system::action_1::Action_1<i32>,
        first_select: i32,
        on_select_callback: crate::system::action_1::Action_1<
            crate::app::mapeditorcategorydata::MapEditorCategoryData,
        >,
        on_close_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versusmapeditobjlistmenucontent::VersusMapEditObjListMenuContent,
        first_select: i32,
    ) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "TickInput", args = 0)]
    pub fn tick_input(self) -> bool;

    #[method(name = "GetThumbSprite", args = 1)]
    pub fn get_thumb_sprite(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetCategorySprite", args = 1)]
    pub fn get_category_sprite(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "ToOne", args = 1)]
    pub fn to_one(self, select_index: i32) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versuseditmenu")]
impl VersusEditMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versusmapeditobjlistmenucontent::VersusMapEditObjListMenuContent,
        first_select: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusEditMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusEditMenuMethods>::ctor(this, menu_item_list, menu_content, first_select);
        this
    }
}
