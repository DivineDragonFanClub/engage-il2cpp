
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/exchangeyesnodialog/ExchangeYesNoDialog_MoneyParam.md")))]
#[::unity2::class(namespace = "App", name = "ExchangeYesNoDialog.MoneyParam")]
#[parent(crate::system::object::Object)]
pub struct ExchangeYesNoDialog_MoneyParam {
    #[rename(name = "num")]
    pub num: i32,
}

#[cfg(feature = "app-exchangeyesnodialog")]
#[::unity2::methods]
impl ExchangeYesNoDialog_MoneyParam {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-exchangeyesnodialog")]
impl ExchangeYesNoDialog_MoneyParam {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeYesNoDialog_MoneyParam),
                ::core::stringify!(new),
            )
        });
        <Self as IExchangeYesNoDialog_MoneyParamMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/exchangeyesnodialog/ExchangeYesNoDialog.md")))]
#[::unity2::class(namespace = "App", name = "ExchangeYesNoDialog")]
#[parent(crate::app::basicdialog::BasicDialog)]
pub struct ExchangeYesNoDialog {
    #[rename(name = "m_DisposeEventHandler")]
    pub m_dispose_event_handler: crate::system::action::Action,
    #[rename(name = "m_IsForWell")]
    pub m_is_for_well: bool,
}

#[cfg(feature = "app-exchangeyesnodialog")]
#[::unity2::methods]
impl ExchangeYesNoDialog {
    #[method(name = "get_m_GetItemTitle", args = 0)]
    pub fn get_m_get_item_title(self) -> ::unity2::Il2CppString;

    #[method(name = "set_m_GetItemTitle", args = 1)]
    pub fn set_m_get_item_title(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_m_GetItemParamList", args = 0)]
    pub fn get_m_get_item_param_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
    >;

    #[method(name = "set_m_GetItemParamList", args = 1)]
    pub fn set_m_get_item_param_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
    ) -> ();

    #[method(name = "get_m_GetMoneyParam", args = 0)]
    pub fn get_m_get_money_param(
        self,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam;

    #[method(name = "set_m_GetMoneyParam", args = 1)]
    pub fn set_m_get_money_param(
        self,
        value: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
    ) -> ();

    #[method(name = "get_m_GetItemBeforeParam", args = 0)]
    pub fn get_m_get_item_before_param(
        self,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam;

    #[method(name = "set_m_GetItemBeforeParam", args = 1)]
    pub fn set_m_get_item_before_param(
        self,
        value: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
    ) -> ();

    #[method(name = "get_m_GetItemAfterParam", args = 0)]
    pub fn get_m_get_item_after_param(
        self,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam;

    #[method(name = "set_m_GetItemAfterParam", args = 1)]
    pub fn set_m_get_item_after_param(
        self,
        value: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
    ) -> ();

    #[method(name = "get_m_CostItemTitle", args = 0)]
    pub fn get_m_cost_item_title(self) -> ::unity2::Il2CppString;

    #[method(name = "set_m_CostItemTitle", args = 1)]
    pub fn set_m_cost_item_title(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_m_CostItemParamList", args = 0)]
    pub fn get_m_cost_item_param_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
    >;

    #[method(name = "set_m_CostItemParamList", args = 1)]
    pub fn set_m_cost_item_param_list(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
    ) -> ();

    #[method(name = "get_m_CostMoneyParam", args = 0)]
    pub fn get_m_cost_money_param(
        self,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam;

    #[method(name = "set_m_CostMoneyParam", args = 1)]
    pub fn set_m_cost_money_param(
        self,
        value: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
    ) -> ();

    #[method(name = "get_m_UsingGetItemList", args = 0)]
    pub fn get_m_using_get_item_list(self) -> bool;

    #[method(name = "set_m_UsingGetItemList", args = 1)]
    pub fn set_m_using_get_item_list(self, value: bool) -> ();

    #[method(name = "get_m_UsingGetBeforeAfter", args = 0)]
    pub fn get_m_using_get_before_after(self) -> bool;

    #[method(name = "set_m_UsingGetBeforeAfter", args = 1)]
    pub fn set_m_using_get_before_after(self, value: bool) -> ();

    #[method(name = "get_m_UsingGetLongName", args = 0)]
    pub fn get_m_using_get_long_name(self) -> bool;

    #[method(name = "set_m_UsingGetLongName", args = 1)]
    pub fn set_m_using_get_long_name(self, value: bool) -> ();

    #[method(name = "get_m_UsingGetNameOnly", args = 0)]
    pub fn get_m_using_get_name_only(self) -> bool;

    #[method(name = "set_m_UsingGetNameOnly", args = 1)]
    pub fn set_m_using_get_name_only(self, value: bool) -> ();

    #[method(name = "get_m_EnabledKeyWaitStyle", args = 0)]
    pub fn get_m_enabled_key_wait_style(self) -> bool;

    #[method(name = "set_m_EnabledKeyWaitStyle", args = 1)]
    pub fn set_m_enabled_key_wait_style(self, value: bool) -> ();

    #[method(name = "CreateBind", args = 10)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        get_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
        yes_menu_item: crate::app::basicdialogitemyes::BasicDialogItemYes,
        no_menu_item: crate::app::basicdialogitemno::BasicDialogItemNo,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = "CreateBind", args = 10)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_before_item_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        get_after_item_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
        yes_menu_item: crate::app::basicdialogitemyes::BasicDialogItemYes,
        no_menu_item: crate::app::basicdialogitemno::BasicDialogItemNo,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = "CreateBind", args = 9)]
    pub fn create_bind_3(
        super_: crate::app::procinst::ProcInst,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
        yes_menu_item: crate::app::basicdialogitemyes::BasicDialogItemYes,
        no_menu_item: crate::app::basicdialogitemno::BasicDialogItemNo,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = "CreateBind", args = 8)]
    pub fn create_bind_4(
        super_: crate::app::procinst::ProcInst,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_name: ::unity2::Il2CppString,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_name: ::unity2::Il2CppString,
        yes_menu_item: crate::app::basicdialogitemyes::BasicDialogItemYes,
        no_menu_item: crate::app::basicdialogitemno::BasicDialogItemNo,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = "CreateBindForWell", args = 8)]
    pub fn create_bind_for_well(
        super_: crate::app::procinst::ProcInst,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_name: ::unity2::Il2CppString,
        yes_menu_item: crate::app::basicdialogitemyes::BasicDialogItemYes,
        no_menu_item: crate::app::basicdialogitemno::BasicDialogItemNo,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = "CreateDifficultyChangeBind", args = 3)]
    pub fn create_difficulty_change_bind(
        super_: crate::app::procinst::ProcInst,
        now_difficulty_name: ::unity2::Il2CppString,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> crate::app::exchangeyesnodialog::ExchangeYesNoDialog;

    #[method(name = ".ctor", args = 9)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        get_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
    ) -> ();

    #[method(name = ".ctor", args = 9)]
    pub fn ctor_2(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_before_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        get_item_after_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
    ) -> ();

    #[method(name = ".ctor", args = 8)]
    pub fn ctor_3(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
    ) -> ();

    #[method(name = ".ctor", args = 7)]
    pub fn ctor_4(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_name: ::unity2::Il2CppString,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 7)]
    pub fn ctor_5(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "SetKeyWaitStyle", args = 1)]
    pub fn set_key_wait_style(self, enabled: bool) -> ();

    #[method(name = "GetGetItemParamMax", args = 0)]
    pub fn get_get_item_param_max() -> i32;

    #[method(name = "GetCostItemParamMax", args = 0)]
    pub fn get_cost_item_param_max() -> i32;

    #[method(name = "SetDisposeEventHandler", args = 1)]
    pub fn set_dispose_event_handler(
        self,
        dispose_event_handler: crate::system::action::Action,
    ) -> ();
}

#[cfg(feature = "app-exchangeyesnodialog")]
impl ExchangeYesNoDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        get_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeYesNoDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IExchangeYesNoDialogMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            top_message,
            get_item_title,
            get_item_param_list,
            get_money_param,
            cost_item_title,
            cost_item_param_list,
            cost_money_param,
        );
        this
    }

    pub fn new_2(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_before_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        get_item_after_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeYesNoDialog),
                ::core::stringify!(new_2),
            )
        });
        <Self as IExchangeYesNoDialogMethods>::ctor_2(
            this,
            menu_item_list,
            menu_content,
            top_message,
            get_item_title,
            get_item_before_param,
            get_item_after_param,
            cost_item_title,
            cost_item_param_list,
            cost_money_param,
        );
        this
    }

    pub fn new_3(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_param_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_money_param: crate::app::exchangeyesnodialog::ExchangeYesNoDialog_MoneyParam,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeYesNoDialog),
                ::core::stringify!(new_3),
            )
        });
        <Self as IExchangeYesNoDialogMethods>::ctor_3(
            this,
            menu_item_list,
            menu_content,
            top_message,
            get_item_title,
            get_item_param,
            cost_item_title,
            cost_item_param_list,
            cost_money_param,
        );
        this
    }

    pub fn new_4(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_name: ::unity2::Il2CppString,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeYesNoDialog),
                ::core::stringify!(new_4),
            )
        });
        <Self as IExchangeYesNoDialogMethods>::ctor_4(
            this,
            menu_item_list,
            menu_content,
            top_message,
            get_item_title,
            get_item_name,
            cost_item_title,
            cost_item_name,
        );
        this
    }

    pub fn new_5(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::exchangedialogcontent::ExchangeDialogContent,
        top_message: ::unity2::Il2CppString,
        get_item_title: ::unity2::Il2CppString,
        get_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::exchangeyesnodialog::ExchangeYesNoDialog_ItemParam,
        >,
        cost_item_title: ::unity2::Il2CppString,
        cost_item_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeYesNoDialog),
                ::core::stringify!(new_5),
            )
        });
        <Self as IExchangeYesNoDialogMethods>::ctor_5(
            this,
            menu_item_list,
            menu_content,
            top_message,
            get_item_title,
            get_item_list,
            cost_item_title,
            cost_item_name,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/exchangeyesnodialog/ExchangeYesNoDialog_ItemParam.md")))]
#[::unity2::class(namespace = "App", name = "ExchangeYesNoDialog.ItemParam")]
#[parent(crate::system::object::Object)]
pub struct ExchangeYesNoDialog_ItemParam {
    #[rename(name = "sprite")]
    pub sprite: crate::unity_engine::sprite::Sprite,
    #[rename(name = "name")]
    pub name: ::unity2::Il2CppString,
    #[rename(name = "num")]
    pub num: i32,
    #[rename(name = "enabledNum")]
    pub enabled_num: bool,
}

#[cfg(feature = "app-exchangeyesnodialog")]
#[::unity2::methods]
impl ExchangeYesNoDialog_ItemParam {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-exchangeyesnodialog")]
impl ExchangeYesNoDialog_ItemParam {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExchangeYesNoDialog_ItemParam),
                ::core::stringify!(new),
            )
        });
        <Self as IExchangeYesNoDialog_ItemParamMethods>::ctor(this);
        this
    }
}
