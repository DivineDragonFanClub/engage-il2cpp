
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::gridmenu::GridMenu;
use crate::app::gridmenu::IGridMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectsortiemenu/UnitSelectSortieMenu.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectSortieMenu")]
#[parent(crate::app::gridmenu::GridMenu)]
pub struct UnitSelectSortieMenu {}

#[cfg(feature = "app-unitselectsortiemenu")]
#[::unity2::methods]
impl UnitSelectSortieMenu {
    #[method(name = "Create", args = 2)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::unitselectsortiemenucontent::UnitSelectSortieMenuContent,
    ) -> crate::app::unitselectsortiemenu::UnitSelectSortieMenu;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::unitselectsortiemenucontent::UnitSelectSortieMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "SetSelectIndexFromUnit", args = 1)]
    pub fn set_select_index_from_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetRootGameObject", args = 0)]
    pub fn get_root_game_object(self) -> crate::unity_engine::gameobject::GameObject;
}

#[cfg(feature = "app-unitselectsortiemenu")]
impl UnitSelectSortieMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::unitselectsortiemenucontent::UnitSelectSortieMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectSortieMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectSortieMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
