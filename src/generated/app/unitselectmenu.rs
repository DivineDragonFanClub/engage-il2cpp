
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitselectmenu/UnitSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "UnitSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct UnitSelectMenu {}

#[cfg(feature = "app-unitselectmenu")]
#[::unity2::methods]
impl UnitSelectMenu {
    #[method(name = "Create", args = 2)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::unitselectmenucontent::UnitSelectMenuContent,
    ) -> crate::app::unitselectmenu::UnitSelectMenu;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::unitselectmenucontent::UnitSelectMenuContent,
    ) -> ();

    #[method(name = "RebuildMenu", args = 0)]
    pub fn rebuild_menu(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "SetSelectIndexFromUnit", args = 1)]
    pub fn set_select_index_from_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetRootGameObject", args = 0)]
    pub fn get_root_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "PlayCursorAnimIdle", args = 0)]
    pub fn play_cursor_anim_idle(self) -> ();

    #[method(name = "SetCursorAlpha", args = 1)]
    pub fn set_cursor_alpha(self, alpha: f32) -> ();
}

#[cfg(feature = "app-unitselectmenu")]
impl UnitSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::unitselectmenucontent::UnitSelectMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
