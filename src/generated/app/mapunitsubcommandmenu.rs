
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::mapbasicmenu::IMapBasicMenu;
use crate::app::mapbasicmenu::MapBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitsubcommandmenu/MapUnitSubCommandMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitSubCommandMenu")]
#[parent(crate::app::mapbasicmenu::MapBasicMenu)]
pub struct MapUnitSubCommandMenu {
    #[static_field]
    #[rename(name = "s_SelectIndex")]
    pub s_select_index: i32,
    #[rename(name = "m_MapUnitCommandMenuContent")]
    pub m_map_unit_command_menu_content:
        crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
}

#[cfg(feature = "app-mapunitsubcommandmenu")]
#[::unity2::methods]
impl MapUnitSubCommandMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
    ) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "ResetSelectIndex", args = 0)]
    pub fn reset_select_index() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapunitsubcommandmenu")]
impl MapUnitSubCommandMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitSubCommandMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitSubCommandMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
