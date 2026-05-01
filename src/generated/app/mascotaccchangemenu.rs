
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotaccchangemenu/MascotAccChangeMenu.md")))]
#[::unity2::class(namespace = "App", name = "MascotAccChangeMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MascotAccChangeMenu {
    #[rename(name = "m_MenuSelectList")]
    pub m_menu_select_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuselect::BasicMenuSelect,
    >,
}

#[cfg(feature = "app-mascotaccchangemenu")]
#[::unity2::methods]
impl MascotAccChangeMenu {
    #[method(name = "get_CurrentPartsType", args = 0)]
    pub fn get_current_parts_type(self) -> crate::app::mascotaccdata::MascotAccData_PartsType;

    #[method(name = "set_CurrentPartsType", args = 1)]
    pub fn set_current_parts_type(
        self,
        value: crate::app::mascotaccdata::MascotAccData_PartsType,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        r#type: crate::app::mascotaccdata::MascotAccData_PartsType,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "CreateMenuList", args = 1)]
    pub fn create_menu_list(
        r#type: crate::app::mascotaccdata::MascotAccData_PartsType,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = "ChangePartsType", args = 2)]
    pub fn change_parts_type(self, directional: i32, is_trigger: bool) -> bool;

    #[method(name = "UpdatePartsType", args = 0)]
    pub fn update_parts_type(self) -> ();

    #[method(name = "UpdateEquipAcc", args = 1)]
    pub fn update_equip_acc(
        self,
        item: crate::app::mascotaccchangemenuitem::MascotAccChangeMenuItem,
    ) -> ();

    #[method(name = "RebuildMenu", args = 1)]
    pub fn rebuild_menu(self, r#type: crate::app::mascotaccdata::MascotAccData_PartsType) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "SetHelpText", args = 2)]
    pub fn set_help_text(
        self,
        mascot_data: crate::app::mascotaccdata::MascotAccData,
        acc_data: crate::app::accessorydata::AccessoryData,
    ) -> ();
}

#[cfg(feature = "app-mascotaccchangemenu")]
impl MascotAccChangeMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotAccChangeMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotAccChangeMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
