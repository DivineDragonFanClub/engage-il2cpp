
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequencetrade/SortieSequenceTrade.md")))]
#[::unity2::class(namespace = "App", name = "SortieSequenceTrade")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: sortiesequencetrade :: SortieSequenceTrade >)]
pub struct SortieSequenceTrade {
    #[static_field]
    #[rename(name = "ResNameC")]
    pub res_name_c: ::unity2::Il2CppString,
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_menuL")]
    pub m_menu_l: crate::app::sortietradeitemmenu::SortieTradeItemMenu,
    #[rename(name = "m_menuR")]
    pub m_menu_r: crate::app::sortietradeitemmenu::SortieTradeItemMenu,
    #[rename(name = "m_bSelectLeft")]
    pub m_b_select_left: bool,
    #[rename(name = "m_Window")]
    pub m_window: crate::app::traderoot::TradeRoot,
    #[static_field]
    #[rename(name = "m_bSortieTrade")]
    pub m_b_sortie_trade: bool,
    #[static_field]
    #[rename(name = "m_bInvalidCharaImage")]
    pub m_b_invalid_chara_image: bool,
    #[static_field]
    #[rename(name = "m_TradeFromUnit")]
    pub m_trade_from_unit: crate::app::unit::Unit,
    #[static_field]
    #[rename(name = "m_TradeToUnit")]
    pub m_trade_to_unit: crate::app::unit::Unit,
    #[static_field]
    #[rename(name = "m_DefaultSelectFrom")]
    pub m_default_select_from: i32,
    #[static_field]
    #[rename(name = "m_DefaultSelectTo")]
    pub m_default_select_to: i32,
    #[static_field]
    #[rename(name = "m_InitialSelectSide")]
    pub m_initial_select_side: crate::app::sortietrademanager::SortieTradeManager_SideId,
    #[static_field]
    #[rename(name = "m_IsFirstSelectedOnInit")]
    pub m_is_first_selected_on_init: bool,
    #[rename(name = "m_CharaImage")]
    pub m_chara_image: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CharaImageRight")]
    pub m_chara_image_right: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-sortiesequencetrade")]
#[::unity2::methods]
impl SortieSequenceTrade {
    #[method(name = "CreateBindSortie", args = 1)]
    pub fn create_bind_sortie(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindSortieHideCharaImage", args = 1)]
    pub fn create_bind_sortie_hide_chara_image(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindMap", args = 4)]
    pub fn create_bind_map(
        super_: crate::app::procinst::ProcInst,
        from: crate::app::unit::Unit,
        to: crate::app::unit::Unit,
        default_select: i32,
    ) -> ();

    #[method(name = "CreateBindCommon", args = 7)]
    pub fn create_bind_common(
        super_: crate::app::procinst::ProcInst,
        from: crate::app::unit::Unit,
        to: crate::app::unit::Unit,
        select_from: i32,
        select_to: i32,
        select_side: crate::app::sortietrademanager::SortieTradeManager_SideId,
        is_first_selected: bool,
    ) -> ();

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "HasDoneAfterBuild", args = 0)]
    pub fn has_done_after_build(self) -> bool;

    #[method(name = "MenuTick", args = 0)]
    pub fn menu_tick(self) -> ();

    #[method(name = "SetActiveOnInitialize", args = 0)]
    pub fn set_active_on_initialize(self) -> ();

    #[method(name = "SetActiveLeft", args = 0)]
    pub fn set_active_left(self) -> ();

    #[method(name = "SetActiveRight", args = 0)]
    pub fn set_active_right(self) -> ();

    #[method(name = "SetActiveLeftForSecondSelect", args = 0)]
    pub fn set_active_left_for_second_select(self) -> ();

    #[method(name = "SetActiveRightForSecondSelect", args = 0)]
    pub fn set_active_right_for_second_select(self) -> ();

    #[method(name = "ChangeActiveByKey", args = 0)]
    pub fn change_active_by_key(self) -> ();

    #[method(name = "ChangeActive", args = 0)]
    pub fn change_active(self) -> ();

    #[method(name = "ItemSelect", args = 0)]
    pub fn item_select(self) -> ();

    #[method(name = "SetMenuActive", args = 4)]
    pub fn set_menu_active(
        self,
        menu: crate::app::sortietradeitemmenu::SortieTradeItemMenu,
        b_active: bool,
        b_on_initialize: bool,
        b_first_select: bool,
    ) -> ();

    #[method(name = "Cancel", args = 0)]
    pub fn cancel(self) -> ();

    #[method(name = "MenuRebuild", args = 0)]
    pub fn menu_rebuild(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "MenuClose", args = 0)]
    pub fn menu_close(self) -> ();

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "MenuCloseEnd", args = 0)]
    pub fn menu_close_end(self) -> ();

    #[method(name = "ItemTrade", args = 0)]
    pub fn item_trade(self) -> ();

    #[method(name = "ItemSwap", args = 3)]
    pub fn item_swap(
        self,
        menu: crate::app::sortietradeitemmenu::SortieTradeItemMenu,
        unit: crate::app::unit::Unit,
        b_done: bool,
    ) -> ();

    #[method(name = "GetAnotherMenu", args = 1)]
    pub fn get_another_menu(
        self,
        menu: crate::app::sortietradeitemmenu::SortieTradeItemMenu,
    ) -> crate::app::sortietradeitemmenu::SortieTradeItemMenu;

    #[method(name = "GetSelectedMenu", args = 0)]
    pub fn get_selected_menu(self) -> crate::app::sortietradeitemmenu::SortieTradeItemMenu;

    #[method(name = "GetUnselectedMenu", args = 0)]
    pub fn get_unselected_menu(self) -> crate::app::sortietradeitemmenu::SortieTradeItemMenu;

    #[method(name = "GetFirstSelectMenu", args = 0)]
    pub fn get_first_select_menu(self) -> crate::app::sortietradeitemmenu::SortieTradeItemMenu;

    #[method(name = "IsFirstSelectBlank", args = 0)]
    pub fn is_first_select_blank(self) -> bool;

    #[method(name = "UpdateSelectableBlankToDisable", args = 0)]
    pub fn update_selectable_blank_to_disable(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-sortiesequencetrade")]
impl SortieSequenceTrade {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieSequenceTrade),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieSequenceTradeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequencetrade/SortieSequenceTrade_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortieSequenceTrade_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortieSequenceTrade_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SortieSequenceTrade.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortieSequenceTrade_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortieSequenceTrade_Label {
    pub fn trade() -> Self {
        Self { value: 0 }
    }

    pub fn r#loop() -> Self {
        Self { value: 1 }
    }

    pub fn active_change() -> Self {
        Self { value: 2 }
    }

    pub fn active_left() -> Self {
        Self { value: 3 }
    }

    pub fn active_right() -> Self {
        Self { value: 4 }
    }

    pub fn item_select() -> Self {
        Self { value: 5 }
    }

    pub fn cancel() -> Self {
        Self { value: 6 }
    }

    pub fn menu_rebuild() -> Self {
        Self { value: 7 }
    }

    pub fn end() -> Self {
        Self { value: 8 }
    }
}
