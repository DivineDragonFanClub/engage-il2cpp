
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineringtimesselectmenuitem/RefineRingTimesSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineRingTimesSelectMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineRingTimesSelectMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refineringtimesselectmenu::RefineRingTimesSelectMenu_DecideEventHandler,
}

#[cfg(feature = "app-refineringtimesselectmenuitem")]
#[::unity2::methods]
impl RefineRingTimesSelectMenuItem {
    #[method(name = "get_m_Times", args = 0)]
    pub fn get_m_times(self) -> i32;

    #[method(name = "set_m_Times", args = 1)]
    pub fn set_m_times(self, value: i32) -> ();

    #[method(name = "get_m_PiecesOfBond", args = 0)]
    pub fn get_m_pieces_of_bond(self) -> i32;

    #[method(name = "set_m_PiecesOfBond", args = 1)]
    pub fn set_m_pieces_of_bond(self, value: i32) -> ();

    #[method(name = "get_m_IsEnoughPieceOfBond", args = 0)]
    pub fn get_m_is_enough_piece_of_bond(self) -> bool;

    #[method(name = "set_m_IsEnoughPieceOfBond", args = 1)]
    pub fn set_m_is_enough_piece_of_bond(self, value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        times: i32,
        decide_event_handler : crate :: app :: refineringtimesselectmenu :: RefineRingTimesSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineringtimesselectmenuitem")]
impl RefineRingTimesSelectMenuItem {
    pub fn new(
        times: i32,
        decide_event_handler : crate :: app :: refineringtimesselectmenu :: RefineRingTimesSelectMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineRingTimesSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineRingTimesSelectMenuItemMethods>::ctor(this, times, decide_event_handler);
        this
    }
}
