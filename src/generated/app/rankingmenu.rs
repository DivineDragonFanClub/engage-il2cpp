
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingmenu/RankingMenu.md")))]
#[::unity2::class(namespace = "App", name = "RankingMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RankingMenu {
    #[static_field]
    #[rename(name = "MaxRankingCount")]
    pub max_ranking_count: i32,
}

#[cfg(feature = "app-rankingmenu")]
#[::unity2::methods]
impl RankingMenu {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        chapter: crate::app::chapterdata::ChapterData,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::rankingmenucontent::RankingMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-rankingmenu")]
impl RankingMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::rankingmenucontent::RankingMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rankingmenu/RankingMenu_RankingMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RankingMenu.RankingMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RankingMenu_RankingMenuItem {
    #[rename(name = "m_rank")]
    pub m_rank: i32,
    #[rename(name = "m_unit")]
    pub m_unit: crate::app::persondata::PersonData,
    #[rename(name = "m_god")]
    pub m_god: crate::app::goddata::GodData,
}

#[cfg(feature = "app-rankingmenu")]
#[::unity2::methods]
impl RankingMenu_RankingMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, index: i32, data: crate::app::nexranking::NexRanking_Data) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPersonData", args = 0)]
    pub fn get_person_data(self) -> crate::app::persondata::PersonData;

    #[method(name = "GetGodData", args = 0)]
    pub fn get_god_data(self) -> crate::app::goddata::GodData;
}

#[cfg(feature = "app-rankingmenu")]
impl RankingMenu_RankingMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingMenu_RankingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRankingMenu_RankingMenuItemMethods>::ctor(this);
        this
    }

    pub fn new_2(index: i32, data: crate::app::nexranking::NexRanking_Data) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RankingMenu_RankingMenuItem),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRankingMenu_RankingMenuItemMethods>::ctor_2(this, index, data);
        this
    }
}
