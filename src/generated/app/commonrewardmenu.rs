
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonrewardmenu/CommonRewardMenu.md")))]
#[::unity2::class(namespace = "App", name = "CommonRewardMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct CommonRewardMenu {
    #[rename(name = "m_IsGaugeAllEnd")]
    pub m_is_gauge_all_end: bool,
    #[rename(name = "m_AddLevelUpUnitList")]
    pub m_add_level_up_unit_list: crate::system::action_2::Action_2<crate::app::unit::Unit, i32>,
    #[rename(name = "m_WaitTimeStart")]
    pub m_wait_time_start: f32,
    #[rename(name = "m_WaitTimeEnd")]
    pub m_wait_time_end: f32,
    #[rename(name = "m_ShowIndex")]
    pub m_show_index: i32,
}

#[cfg(feature = "app-commonrewardmenu")]
#[::unity2::methods]
impl CommonRewardMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        reward_exp_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
            crate::app::unit::Unit,
            i32,
        >,
        add_level_up_unit_list: crate::system::action_2::Action_2<crate::app::unit::Unit, i32>,
    ) -> crate::app::commonrewardmenu::CommonRewardMenu;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        add_level_up_unit_list: crate::system::action_2::Action_2<crate::app::unit::Unit, i32>,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "MoveUp", args = 1)]
    pub fn move_up(self, is_trigger: bool) -> ();

    #[method(name = "MoveDown", args = 1)]
    pub fn move_down(self, is_trigger: bool) -> ();
}

#[cfg(feature = "app-commonrewardmenu")]
impl CommonRewardMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        add_level_up_unit_list: crate::system::action_2::Action_2<crate::app::unit::Unit, i32>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommonRewardMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ICommonRewardMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            add_level_up_unit_list,
        );
        this
    }
}
