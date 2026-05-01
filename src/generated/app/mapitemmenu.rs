
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::gridmenu::GridMenu;
use crate::app::gridmenu::IGridMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::trademenu::ITradeMenu;
use crate::app::trademenu::TradeMenu;
use crate::app::trademenuitem::ITradeMenuItem;
use crate::app::trademenuitem::TradeMenuItem;
use crate::app::yesnodialog::IYesNoDialog;
use crate::app::yesnodialog::YesNoDialog;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemUseMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.SubItemUseMenuItem")]
#[parent(crate::app::mapitemmenu::MapItemMenu_SubItemMenuItem)]
pub struct MapItemMenu_SubItemUseMenuItem {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemUseMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemUseMenuItem {
    pub fn new(unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemUseMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemUseMenuItemMethods>::ctor(this, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_MapTradeMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.MapTradeMenu")]
#[parent(crate::app::trademenu::TradeMenu)]
pub struct MapItemMenu_MapTradeMenu {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_MapTradeMenu {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_MapTradeMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_MapTradeMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_MapTradeMenuMethods>::ctor(this, menu_item_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_ItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.ItemMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct MapItemMenu_ItemMenuItem {
    #[rename(name = "m_BasicItemMenuContent")]
    pub m_basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    #[rename(name = "m_UnitItemIndex")]
    pub m_unit_item_index: i32,
}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_ItemMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        unit_item_index: i32,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitItemIndex", args = 0)]
    pub fn get_unit_item_index(self) -> i32;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "IsEffective", args = 0)]
    pub fn is_effective(self) -> bool;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_ItemMenuItem {
    pub fn new(
        unit_item_index: i32,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_ItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_ItemMenuItemMethods>::ctor(
            this,
            unit_item_index,
            basic_item_menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.SubItemMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MapItemMenu_SubItemMenuItem {
    #[rename(name = "m_UnitItemIndex")]
    pub m_unit_item_index: i32,
}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit_item_index: i32) -> ();

    #[method(name = "GetParentMenu", args = 0)]
    pub fn get_parent_menu(self) -> crate::app::basicmenu::BasicMenu;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemMenuItem {
    pub fn new(unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemMenuItemMethods>::ctor(this, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemEquipMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.SubItemEquipMenuItem")]
#[parent(crate::app::mapitemmenu::MapItemMenu_SubItemMenuItem)]
pub struct MapItemMenu_SubItemEquipMenuItem {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemEquipMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemEquipMenuItem {
    pub fn new(unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemEquipMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemEquipMenuItemMethods>::ctor(this, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_EnchantItemMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.EnchantItemMenuItem")]
#[parent(crate::app::mapitemmenu::MapItemMenu_ItemMenuItem)]
pub struct MapItemMenu_EnchantItemMenuItem {
    #[rename(name = "m_EnchantType")]
    pub m_enchant_type: crate::app::mapitemmenu::MapItemMenu_EnchantType,
}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_EnchantItemMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        unit_item_index: i32,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
        enchant_type: crate::app::mapitemmenu::MapItemMenu_EnchantType,
    ) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "CanEnchant", args = 1)]
    pub fn can_enchant(self, unit_item: crate::app::unititem::UnitItem) -> bool;

    #[method(name = "Enumerate", args = 0)]
    pub fn enumerate(self) -> bool;

    #[method(name = "IsEffective", args = 0)]
    pub fn is_effective(self) -> bool;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_EnchantItemMenuItem {
    pub fn new(
        unit_item_index: i32,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
        enchant_type: crate::app::mapitemmenu::MapItemMenu_EnchantType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_EnchantItemMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_EnchantItemMenuItemMethods>::ctor(
            this,
            unit_item_index,
            basic_item_menu_content,
            enchant_type,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_RodMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.RodMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MapItemMenu_RodMenu {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_RodMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_RodMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_RodMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_RodMenuMethods>::ctor(this, menu_item_list, basic_item_menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_MapTradeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.MapTradeMenuItem")]
#[parent(crate::app::trademenuitem::TradeMenuItem)]
pub struct MapItemMenu_MapTradeMenuItem {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_MapTradeMenuItem {
    #[method(name = "GetLeftUnit", args = 0)]
    pub fn get_left_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetRightUnit", args = 0)]
    pub fn get_right_unit(self) -> crate::app::unit::Unit;

    #[method(name = "SetDone", args = 0)]
    pub fn set_done(self) -> ();

    #[method(name = "IsDone", args = 0)]
    pub fn is_done(self) -> bool;

    #[method(name = "OnEnd", args = 0)]
    pub fn on_end(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_MapTradeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_MapTradeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_MapTradeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_TargetMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.TargetMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct MapItemMenu_TargetMenuItem {
    #[rename(name = "m_UnitItemIndex")]
    pub m_unit_item_index: i32,
}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_TargetMenuItem {
    #[method(name = "get_ActiveMind", args = 0)]
    pub fn get_active_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "get_DeployMode", args = 0)]
    pub fn get_deploy_mode(self) -> crate::app::mappaneldeploy::MapPanelDeploy_Mode;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit_item_index: i32) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_TargetMenuItem {
    pub fn new(unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_TargetMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_TargetMenuItemMethods>::ctor(this, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_EnchantType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapItemMenu_EnchantType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapItemMenu_EnchantType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapItemMenu.EnchantType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapItemMenu_EnchantType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapItemMenu_EnchantType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn item() -> Self {
        Self { value: 1 }
    }

    pub fn weapon() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemSortMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.SubItemSortMenuItem")]
#[parent(crate::app::mapitemmenu::MapItemMenu_SubItemMenuItem)]
pub struct MapItemMenu_SubItemSortMenuItem {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemSortMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemSortMenuItem {
    pub fn new(unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemSortMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemSortMenuItemMethods>::ctor(this, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.SubItemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MapItemMenu_SubItemMenu {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::basicmenu::BasicMenu,
        unit_item_index: i32,
        parent_menu_item: crate::app::basicmenuitem::BasicMenuItem,
    ) -> ();
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_RodMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.RodMenuItem")]
#[parent(crate::app::mapitemmenu::MapItemMenu_TargetMenuItem)]
pub struct MapItemMenu_RodMenuItem {
    #[rename(name = "m_BasicItemMenuContent")]
    pub m_basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_RodMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        unit_item_index: i32,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_RodMenuItem {
    pub fn new(
        unit_item_index: i32,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_RodMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_RodMenuItemMethods>::ctor(
            this,
            unit_item_index,
            basic_item_menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemPutOffMenuItem_ConfirmDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapItemMenu.SubItemPutOffMenuItem.ConfirmDialog"
)]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct MapItemMenu_SubItemPutOffMenuItem_ConfirmDialog {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemPutOffMenuItem_ConfirmDialog {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
    ) -> ();
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemPutOffMenuItem_ConfirmDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemPutOffMenuItem_ConfirmDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemPutOffMenuItem_ConfirmDialogMethods>::ctor(
            this,
            menu_item_list,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_AttackMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.AttackMenuItem")]
#[parent(crate::app::mapitemmenu::MapItemMenu_TargetMenuItem)]
pub struct MapItemMenu_AttackMenuItem {
    #[rename(name = "m_Mind")]
    pub m_mind: crate::app::mapmind::MapMind_Type,
    #[rename(name = "m_ActionMask")]
    pub m_action_mask: crate::app::maptarget::MapTarget_ActionMask,
    #[rename(name = "m_Skill")]
    pub m_skill: crate::app::skilldata::SkillData,
    #[rename(name = "m_BasicItemMenuContent")]
    pub m_basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_AttackMenuItem {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        unit_item_index: i32,
        mind: crate::app::mapmind::MapMind_Type,
        action_mask: crate::app::maptarget::MapTarget_ActionMask,
        skill: crate::app::skilldata::SkillData,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_AttackMenuItem {
    pub fn new(
        unit_item_index: i32,
        mind: crate::app::mapmind::MapMind_Type,
        action_mask: crate::app::maptarget::MapTarget_ActionMask,
        skill: crate::app::skilldata::SkillData,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_AttackMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_AttackMenuItemMethods>::ctor(
            this,
            unit_item_index,
            mind,
            action_mask,
            skill,
            basic_item_menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemPutOffMenuItem_ConfirmDialogItemYes.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MapItemMenu.SubItemPutOffMenuItem.ConfirmDialogItemYes"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct MapItemMenu_SubItemPutOffMenuItem_ConfirmDialogItemYes {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItemIndex")]
    pub m_unit_item_index: i32,
}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemPutOffMenuItem_ConfirmDialogItemYes {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        text: ::unity2::Il2CppString,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemPutOffMenuItem_ConfirmDialogItemYes {
    pub fn new(
        text: ::unity2::Il2CppString,
        unit: crate::app::unit::Unit,
        unit_item_index: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemPutOffMenuItem_ConfirmDialogItemYes),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemPutOffMenuItem_ConfirmDialogItemYesMethods>::ctor(
            this,
            text,
            unit,
            unit_item_index,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemTakeOffMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.SubItemTakeOffMenuItem")]
#[parent(crate::app::mapitemmenu::MapItemMenu_SubItemMenuItem)]
pub struct MapItemMenu_SubItemTakeOffMenuItem {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemTakeOffMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemTakeOffMenuItem {
    pub fn new(unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemTakeOffMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemTakeOffMenuItemMethods>::ctor(this, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemTradeMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.SubItemTradeMenuItem")]
#[parent(crate::app::mapitemmenu::MapItemMenu_SubItemMenuItem)]
pub struct MapItemMenu_SubItemTradeMenuItem {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemTradeMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemTradeMenuItem {
    pub fn new(unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemTradeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemTradeMenuItemMethods>::ctor(this, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu")]
#[parent(crate::system::object::Object)]
pub struct MapItemMenu {
    #[static_field]
    #[rename(name = "s_RodSelectIndex")]
    pub s_rod_select_index: i32,
    #[static_field]
    #[rename(name = "s_EnchantSelectIndex")]
    pub s_enchant_select_index: i32,
}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu {
    #[method(name = "ResetSelectIndex", args = 0)]
    pub fn reset_select_index() -> ();

    #[method(name = "JumpTargetSelect", args = 1)]
    pub fn jump_target_select(mind: crate::app::mapmind::MapMind_Type) -> ();

    #[method(name = "CreateBindTarget", args = 4)]
    pub fn create_bind_target(
        super_: crate::app::procinst::ProcInst,
        mind: crate::app::mapmind::MapMind_Type,
        mask: crate::app::maptarget::MapTarget_ActionMask,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CreateBindAttack", args = 2)]
    pub fn create_bind_attack(
        super_: crate::app::procinst::ProcInst,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CreateBindMagic", args = 2)]
    pub fn create_bind_magic(
        super_: crate::app::procinst::ProcInst,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CreateBindEngageAttack", args = 2)]
    pub fn create_bind_engage_attack(
        super_: crate::app::procinst::ProcInst,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CreateBindEngageCharge", args = 2)]
    pub fn create_bind_engage_charge(
        super_: crate::app::procinst::ProcInst,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CreateBindEngageWait", args = 2)]
    pub fn create_bind_engage_wait(
        super_: crate::app::procinst::ProcInst,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CreateBindEngageRewarp", args = 2)]
    pub fn create_bind_engage_rewarp(
        super_: crate::app::procinst::ProcInst,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CreateBindEngageRod", args = 2)]
    pub fn create_bind_engage_rod(
        super_: crate::app::procinst::ProcInst,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CreateBindCannon", args = 1)]
    pub fn create_bind_cannon(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindDestroy", args = 1)]
    pub fn create_bind_destroy(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindRod", args = 1)]
    pub fn create_bind_rod(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindItem", args = 1)]
    pub fn create_bind_item(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindEnchantItem", args = 1)]
    pub fn create_bind_enchant_item(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindEnchantWeapon", args = 1)]
    pub fn create_bind_enchant_weapon(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindTrade", args = 1)]
    pub fn create_bind_trade(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_SubItemPutOffMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.SubItemPutOffMenuItem")]
#[parent(crate::app::mapitemmenu::MapItemMenu_SubItemMenuItem)]
pub struct MapItemMenu_SubItemPutOffMenuItem {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_SubItemPutOffMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit_item_index: i32) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_SubItemPutOffMenuItem {
    pub fn new(unit_item_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_SubItemPutOffMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_SubItemPutOffMenuItemMethods>::ctor(this, unit_item_index);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_AttackMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.AttackMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MapItemMenu_AttackMenu {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_AttackMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        mind: crate::app::mapmind::MapMind_Type,
        mask: crate::app::maptarget::MapTarget_ActionMask,
        skill: crate::app::skilldata::SkillData,
    ) -> ();
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_AttackMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_AttackMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_AttackMenuMethods>::ctor(
            this,
            menu_item_list,
            basic_item_menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapitemmenu/MapItemMenu_ItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapItemMenu.ItemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MapItemMenu_ItemMenu {}

#[cfg(feature = "app-mapitemmenu")]
#[::unity2::methods]
impl MapItemMenu_ItemMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> ();

    #[method(name = "OnBuild", args = 1)]
    pub fn on_build(self, is_first_build: bool) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        enchant_type: crate::app::mapitemmenu::MapItemMenu_EnchantType,
    ) -> ();

    #[method(name = "GetMenuItemIndexEquipped", args = 0)]
    pub fn get_menu_item_index_equipped(self) -> i32;
}

#[cfg(feature = "app-mapitemmenu")]
impl MapItemMenu_ItemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        basic_item_menu_content: crate::app::basicitemmenucontent::BasicItemMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapItemMenu_ItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapItemMenu_ItemMenuMethods>::ctor(this, menu_item_list, basic_item_menu_content);
        this
    }
}
