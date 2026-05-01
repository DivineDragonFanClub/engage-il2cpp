
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::wellitemselectmenuitem::IWellItemSelectMenuItem;
use crate::app::wellitemselectmenuitem::WellItemSelectMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellitemselectmenu/WellItemSelectMenu_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct WellItemSelectMenu_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for WellItemSelectMenu_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "WellItemSelectMenu.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for WellItemSelectMenu_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl WellItemSelectMenu_Kinds {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn first() -> Self {
        Self { value: 1 }
    }

    pub fn last() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellitemselectmenu/WellItemSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "WellItemSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct WellItemSelectMenu {
    #[rename(name = "ItemKindList")]
    pub item_kind_list: ::unity2::Array<crate::app::itemdata::ItemData_Kinds>,
    #[static_field]
    #[rename(name = "SELECT_ITEM_MAX")]
    pub select_item_max: i32,
    #[rename(name = "m_SelectedKindIndex")]
    pub m_selected_kind_index: i32,
    #[static_field]
    #[rename(name = "ShowRowNum")]
    pub show_row_num: i32,
    #[rename(name = "m_SavedFullMenuItemList")]
    pub m_saved_full_menu_item_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuitem::BasicMenuItem,
    >,
    #[rename(name = "m_SortMenuItemList")]
    pub m_sort_menu_item_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuitem::BasicMenuItem,
    >,
    #[rename(name = "m_ItemKind")]
    pub m_item_kind: crate::app::itemdata::ItemData_Kinds,
    #[rename(name = "m_Selects")]
    pub m_selects: ::unity2::Array<crate::app::basicmenuselect::BasicMenuSelect>,
    #[rename(name = "m_SelectedItemList")]
    pub m_selected_item_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicitemmenuitem::BasicItemMenuItem,
    >,
}

#[cfg(feature = "app-wellitemselectmenu")]
#[::unity2::methods]
impl WellItemSelectMenu {
    #[method(name = "get_SelectedKind", args = 0)]
    pub fn get_selected_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "get_m_CommonDisplayIndex", args = 0)]
    pub fn get_m_common_display_index(self) -> i32;

    #[method(name = "set_m_CommonDisplayIndex", args = 1)]
    pub fn set_m_common_display_index(self, value: i32) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(
        parent: crate::app::procinst::ProcInst,
    ) -> crate::app::wellitemselectmenu::WellItemSelectMenu;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::wellitemselectmenucontent::WellItemSelectMenuContent,
    ) -> ();

    #[method(name = "GetSelectItem", args = 0)]
    pub fn get_select_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "SaveSelectItem", args = 0)]
    pub fn save_select_item(self) -> ();

    #[method(name = "GetSelectUnit", args = 0)]
    pub fn get_select_unit(self) -> crate::app::unit::Unit;

    #[method(name = "UpdateUnit", args = 1)]
    pub fn update_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateUnit", args = 2)]
    pub fn update_unit_2(self, unit: crate::app::unit::Unit, is_chara_only_on: bool) -> ();

    #[method(name = "UpdateItemHelp", args = 2)]
    pub fn update_item_help(
        self,
        unit: crate::app::unit::Unit,
        item: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "IsMarkingItemFull", args = 0)]
    pub fn is_marking_item_full(self) -> bool;

    #[method(name = "IsMarkingItemEmpty", args = 0)]
    pub fn is_marking_item_empty(self) -> bool;

    #[method(name = "AddSelectedItem", args = 3)]
    pub fn add_selected_item(
        self,
        unit: crate::app::unit::Unit,
        owner_item_index: i32,
        menu_item: crate::app::wellitemselectmenuitem::WellItemSelectMenuItem,
    ) -> ();

    #[method(name = "RemoveSelectedItem", args = 2)]
    pub fn remove_selected_item(self, unit: crate::app::unit::Unit, owner_item_index: i32) -> ();

    #[method(name = "RemoveSelectedLastItem", args = 0)]
    pub fn remove_selected_last_item(self) -> bool;

    #[method(name = "GetLastSelectedItem", args = 0)]
    pub fn get_last_selected_item(
        self,
    ) -> crate::app::wellitemselectmenuitem::WellItemSelectMenuItem;

    #[method(name = "SetItemKindIndex", args = 2)]
    pub fn set_item_kind_index(self, kind_index: i32, is_auto_select: bool) -> ();

    #[method(name = "SetFirstKind", args = 1)]
    pub fn set_first_kind(self, is_auto_select: bool) -> ();

    #[method(name = "SetLastKind", args = 1)]
    pub fn set_last_kind(self, is_auto_select: bool) -> ();

    #[method(name = "SetFirstSelection", args = 0)]
    pub fn set_first_selection(self) -> ();

    #[method(name = "ResetFirstSelection", args = 0)]
    pub fn reset_first_selection(self) -> ();

    #[method(name = "SetSelectIndexOnChangeMenu", args = 2)]
    pub fn set_select_index_on_change_menu(
        self,
        common_display_index: i32,
        kind: crate::app::wellitemselectmenu::WellItemSelectMenu_Kinds,
    ) -> ();

    #[method(name = "HoldSelection", args = 0)]
    pub fn hold_selection(self) -> ();

    #[method(name = "UpdateMenu", args = 1)]
    pub fn update_menu(self, is_auto_select: bool) -> ();

    #[method(name = "GetSelectableItemCount", args = 0)]
    pub fn get_selectable_item_count(self) -> i32;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "ClampMenuItemIndex", args = 1)]
    pub fn clamp_menu_item_index(self, item_index: i32) -> i32;

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "Sort", args = 0)]
    pub fn sort(self) -> ();

    #[method(name = "ComparePoolMenuItem", args = 2)]
    pub fn compare_pool_menu_item(
        x: crate::app::basicmenuitem::BasicMenuItem,
        y: crate::app::basicmenuitem::BasicMenuItem,
    ) -> i32;

    #[method(name = "GetMarkingItemList", args = 0)]
    pub fn get_marking_item_list(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::unititem::UnitItem>;

    #[method(name = "PutOffMarkingItems", args = 0)]
    pub fn put_off_marking_items(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-wellitemselectmenu")]
impl WellItemSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::wellitemselectmenucontent::WellItemSelectMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WellItemSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IWellItemSelectMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellitemselectmenu/WellItemSelectMenu_SelectedItem.md")))]
#[::unity2::class(namespace = "App", name = "WellItemSelectMenu.SelectedItem")]
#[parent(crate::app::wellitemselectmenuitem::WellItemSelectMenuItem)]
pub struct WellItemSelectMenu_SelectedItem {
    #[rename(name = "m_MenuItem")]
    pub m_menu_item: crate::app::wellitemselectmenuitem::WellItemSelectMenuItem,
}

#[cfg(feature = "app-wellitemselectmenu")]
#[::unity2::methods]
impl WellItemSelectMenu_SelectedItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        owner_item_index: i32,
        menu_item: crate::app::wellitemselectmenuitem::WellItemSelectMenuItem,
    ) -> ();

    #[method(name = "IsSame", args = 2)]
    pub fn is_same(self, unit: crate::app::unit::Unit, owner_item_index: i32) -> bool;

    #[method(name = "get_OriginalMenuItem", args = 0)]
    pub fn get_original_menu_item(
        self,
    ) -> crate::app::wellitemselectmenuitem::WellItemSelectMenuItem;
}

#[cfg(feature = "app-wellitemselectmenu")]
impl WellItemSelectMenu_SelectedItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        owner_item_index: i32,
        menu_item: crate::app::wellitemselectmenuitem::WellItemSelectMenuItem,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WellItemSelectMenu_SelectedItem),
                ::core::stringify!(new),
            )
        });
        <Self as IWellItemSelectMenu_SelectedItemMethods>::ctor(
            this,
            unit,
            owner_item_index,
            menu_item,
        );
        this
    }
}
