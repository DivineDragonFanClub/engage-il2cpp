
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugmenu/DebugMenu_BindMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DebugMenu_BindMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DebugMenu_BindMode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DebugMenu.BindMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DebugMenu_BindMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DebugMenu_BindMode {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn hide() -> Self {
        Self { value: 1 }
    }

    pub fn alpha() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugmenu/DebugMenu_AnchorLocation.md")))]
#[::unity2::class(namespace = "App", name = "DebugMenu.AnchorLocation")]
#[parent(crate::system::object::Object)]
pub struct DebugMenu_AnchorLocation {}

#[cfg(feature = "app-debugmenu")]
#[::unity2::methods]
impl DebugMenu_AnchorLocation {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "GetX", args = 1)]
    pub fn get_x(anchor: crate::app::gx::GX_Anchor) -> i32;

    #[method(name = "GetY", args = 1)]
    pub fn get_y(anchor: crate::app::gx::GX_Anchor) -> i32;

    #[method(name = "GetNext", args = 3)]
    pub fn get_next(
        anchor: crate::app::gx::GX_Anchor,
        dx: i32,
        dy: i32,
    ) -> crate::app::gx::GX_Anchor;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-debugmenu")]
impl DebugMenu_AnchorLocation {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugMenu_AnchorLocation),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugMenu_AnchorLocationMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugmenu/DebugMenu.md")))]
#[::unity2::class(namespace = "App", name = "DebugMenu")]
#[parent(crate::app::procinst::ProcInst)]
pub struct DebugMenu {
    #[static_field]
    #[rename(name = "COLOR_WINDOW")]
    pub color_window: crate::unity_engine::color::Color,
    #[static_field]
    #[rename(name = "COLOR_ZERO")]
    pub color_zero: crate::unity_engine::color::Color,
    #[rename(name = "m_Rect")]
    pub m_rect: crate::unity_engine::rect::Rect,
    #[rename(name = "m_Select")]
    pub m_select: i32,
    #[rename(name = "m_Orient")]
    pub m_orient: f32,
    #[rename(name = "m_Color")]
    pub m_color: crate::unity_engine::color::Color,
    #[rename(name = "m_Depth")]
    pub m_depth: i32,
    #[rename(name = "m_Anchor")]
    pub m_anchor: crate::app::gx::GX_Anchor,
    #[rename(name = "m_Prefab")]
    pub m_prefab: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Window")]
    pub m_window: crate::app::debugwindow::DebugWindow,
    #[rename(name = "m_BindMode")]
    pub m_bind_mode: crate::app::debugmenu::DebugMenu_BindMode,
    #[rename(name = "m_Items")]
    pub m_items:
        crate::system::collections::generic::list_1::List_1<crate::app::menuitem::MenuItem>,
    #[rename(name = "m_IsFirst")]
    pub m_is_first: bool,
    #[rename(name = "m_IsRelay")]
    pub m_is_relay: bool,
    #[rename(name = "m_IsCalcW")]
    pub m_is_calc_w: bool,
    #[rename(name = "m_IsCalcH")]
    pub m_is_calc_h: bool,
    #[rename(name = "m_IsMovePage")]
    pub m_is_move_page: bool,
    #[rename(name = "m_IsMapCursor")]
    pub m_is_map_cursor: bool,
    #[rename(name = "m_RecordKey")]
    pub m_record_key: ::unity2::Il2CppString,
    #[rename(name = "m_SelectTick")]
    pub m_select_tick: i32,
    #[rename(name = "m_SelectTime")]
    pub m_select_time: f32,
    #[rename(name = "m_TimeScale")]
    pub m_time_scale: f32,
}

#[cfg(feature = "app-debugmenu")]
#[::unity2::methods]
impl DebugMenu {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnOpen", args = 0)]
    pub fn on_open(self) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnRebuild", args = 0)]
    pub fn on_rebuild(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "TickItem", args = 1)]
    pub fn tick_item(
        self,
        item: crate::app::menuitem::MenuItem,
    ) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "IsSelectable", args = 1)]
    pub fn is_selectable(self, index: i32) -> bool;

    #[method(name = "IsSelectable", args = 1)]
    pub fn is_selectable_2(self, item: crate::app::menuitem::MenuItem) -> bool;

    #[method(name = "IsVisible", args = 1)]
    pub fn is_visible(self, item: crate::app::menuitem::MenuItem) -> bool;

    #[method(name = "IsRelay", args = 0)]
    pub fn is_relay(self) -> bool;

    #[method(name = "MoveToPrev", args = 2)]
    pub fn move_to_prev(self, step: i32, is_trigger: bool) -> ();

    #[method(name = "MoveToNext", args = 2)]
    pub fn move_to_next(self, step: i32, is_trigger: bool) -> ();

    #[method(name = "GetWidth", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "GetHeight", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "GetPage", args = 0)]
    pub fn get_page(self) -> i32;

    #[method(name = "MoveTick", args = 0)]
    pub fn move_tick(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "GetItemBlank", args = 2)]
    pub fn get_item_blank(self, index: i32, dir: i32) -> f32;

    #[method(name = "GetOrientTarget", args = 0)]
    pub fn get_orient_target(self) -> f32;

    #[method(name = "UpdateSelect", args = 0)]
    pub fn update_select(self) -> ();

    #[method(name = "AddItem", args = 1)]
    pub fn add_item(self, item: crate::app::menuitem::MenuItem)
        -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddItem", args = 1)]
    pub fn add_item_2(
        self,
        items: crate::system::collections::generic::list_1::List_1<crate::app::menuitem::MenuItem>,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddString", args = 1)]
    pub fn add_string(self, name: ::unity2::Il2CppString) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddString", args = 2)]
    pub fn add_string_2(
        self,
        name: ::unity2::Il2CppString,
        english: ::unity2::Il2CppString,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddStringFormat", args = 2)]
    pub fn add_string_format(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddDisable", args = 1)]
    pub fn add_disable(self, name: ::unity2::Il2CppString) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddFunc", args = 2)]
    pub fn add_func(
        self,
        name: ::unity2::Il2CppString,
        func: crate::app::funcitem::FuncItem_Func,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddProperty", args = 2)]
    pub fn add_property(
        self,
        obj: crate::system::object::Object,
        info: crate::system::reflection::fieldinfo::FieldInfo,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddProperty", args = 2)]
    pub fn add_property_2(
        self,
        obj: crate::system::object::Object,
        info: crate::system::reflection::propertyinfo::PropertyInfo,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddProperty", args = 3)]
    pub fn add_property_3(
        self,
        obj: crate::system::object::Object,
        name: ::unity2::Il2CppString,
        flags: crate::system::reflection::bindingflags::BindingFlags,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddProperty", args = 2)]
    pub fn add_property_4(
        self,
        obj: crate::system::object::Object,
        flags: crate::system::reflection::bindingflags::BindingFlags,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddLabel", args = 1)]
    pub fn add_label(self, name: ::unity2::Il2CppString) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddLabel", args = 2)]
    pub fn add_label_2(
        self,
        name: ::unity2::Il2CppString,
        english: ::unity2::Il2CppString,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddLabelFormat", args = 2)]
    pub fn add_label_format(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddSeparator", args = 0)]
    pub fn add_separator(self) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddGroupBegin", args = 2)]
    pub fn add_group_begin(
        self,
        name: ::unity2::Il2CppString,
        state: crate::app::menuitem::MenuItem_State,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "AddGroupEnd", args = 0)]
    pub fn add_group_end(self) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "ClearItem", args = 0)]
    pub fn clear_item(self) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetPos", args = 2)]
    pub fn set_pos(self, x: f32, y: f32) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetPos", args = 1)]
    pub fn set_pos_2(
        self,
        pos: crate::unity_engine::vector2::Vector2,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetSize", args = 2)]
    pub fn set_size(self, w: f32, h: f32) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetSize", args = 1)]
    pub fn set_size_2(
        self,
        size: crate::unity_engine::vector2::Vector2,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetWidth", args = 1)]
    pub fn set_width(self, w: f32) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetHeight", args = 1)]
    pub fn set_height(self, h: f32) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetRect", args = 4)]
    pub fn set_rect(self, x: f32, y: f32, w: f32, h: f32) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetColot", args = 1)]
    pub fn set_colot(
        self,
        color: crate::unity_engine::color::Color,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "Move", args = 2)]
    pub fn r#move(self, x: f32, y: f32) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetMovePage", args = 1)]
    pub fn set_move_page(self, enable: bool) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetMapCursor", args = 1)]
    pub fn set_map_cursor(self, enable: bool) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "ResetSize", args = 0)]
    pub fn reset_size(self) -> ();

    #[method(name = "CalcWidth", args = 0)]
    pub fn calc_width(self) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "CalcHeight", args = 0)]
    pub fn calc_height(self) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "CalcRow", args = 1)]
    pub fn calc_row(self, row: i32) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetSelect", args = 1)]
    pub fn set_select(self, index: i32) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetDepth", args = 1)]
    pub fn set_depth(self, depth: i32) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetAncher", args = 1)]
    pub fn set_ancher(self, anchor: crate::app::gx::GX_Anchor) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetAncherPos", args = 2)]
    pub fn set_ancher_pos(
        self,
        anchor: crate::app::gx::GX_Anchor,
        offset: f32,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetRelay", args = 1)]
    pub fn set_relay(self, enable: bool) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "SetDebugTime", args = 1)]
    pub fn set_debug_time(self, time_scale: f32) -> ();

    #[method(name = "GetRect", args = 0)]
    pub fn get_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "GetOrient", args = 0)]
    pub fn get_orient(self) -> f32;

    #[method(name = "GetColor", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetDepth", args = 0)]
    pub fn get_depth(self) -> i32;

    #[method(name = "GetSelect", args = 0)]
    pub fn get_select(self) -> i32;

    #[method(name = "GetPos", args = 0)]
    pub fn get_pos(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetSize", args = 0)]
    pub fn get_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetAnchor", args = 0)]
    pub fn get_anchor(self) -> crate::app::gx::GX_Anchor;

    #[method(name = "GetIndentOffset", args = 1)]
    pub fn get_indent_offset(self, indent: i32) -> f32;

    #[method(name = "CalcIndent", args = 2)]
    pub fn calc_indent(self, indent: i32, item: crate::app::menuitem::MenuItem) -> i32;

    #[method(name = "ReadRecord", args = 0)]
    pub fn read_record(self) -> ();

    #[method(name = "WriteRecord", args = 0)]
    pub fn write_record(self) -> ();

    #[method(name = "SetRecord", args = 1)]
    pub fn set_record(self, key: ::unity2::Il2CppString) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "BuildHierarchy", args = 0)]
    pub fn build_hierarchy(self) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "GetTextWidth", args = 1)]
    pub fn get_text_width(self, text: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetTextHeight", args = 1)]
    pub fn get_text_height(self, text: ::unity2::Il2CppString) -> f32;

    #[method(name = "SetBindMode", args = 1)]
    pub fn set_bind_mode(
        self,
        mode: crate::app::debugmenu::DebugMenu_BindMode,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "GetBindMode", args = 0)]
    pub fn get_bind_mode(self) -> crate::app::debugmenu::DebugMenu_BindMode;

    #[method(name = "GetCurrentBindMode", args = 0)]
    pub fn get_current_bind_mode(self) -> crate::app::debugmenu::DebugMenu_BindMode;

    #[method(name = "GetBaseColor", args = 0)]
    pub fn get_base_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetItem", args = 1)]
    pub fn get_item(self, i: i32) -> crate::app::menuitem::MenuItem;

    #[method(name = "GetItems", args = 0)]
    pub fn get_items(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::menuitem::MenuItem>;

    #[method(name = "GetItemCount", args = 0)]
    pub fn get_item_count(self) -> i32;

    #[method(name = "GetSelectTick", args = 0)]
    pub fn get_select_tick(self) -> i32;

    #[method(name = "GetSelectTime", args = 0)]
    pub fn get_select_time(self) -> f32;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "GetTopMenu", args = 2)]
    pub fn get_top_menu(
        menu: crate::app::debugmenu::DebugMenu,
        is_relay: bool,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = "GetParentMenu", args = 1)]
    pub fn get_parent_menu(
        super_: crate::app::procinst::ProcInst,
    ) -> crate::app::debugmenu::DebugMenu;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-debugmenu")]
impl DebugMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugMenuMethods>::ctor(this);
        this
    }
}
