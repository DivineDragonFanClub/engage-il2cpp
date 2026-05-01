
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::stackprocinst_1::IStackProcInst_1;
use crate::app::stackprocinst_1::StackProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomreliancesubselect/MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MyRoomRelianceSubSelect.MyRoomRelianceSubSelectItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem {
    #[rename(name = "m_UnitL")]
    pub m_unit_l: crate::app::unit::Unit,
    #[rename(name = "m_UnitR")]
    pub m_unit_r: crate::app::unit::Unit,
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
}

#[cfg(feature = "app-myroomreliancesubselect")]
#[::unity2::methods]
impl MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem {
    #[method(name = "get_IsSelected", args = 0)]
    pub fn get_is_selected(self) -> bool;

    #[method(name = "set_IsSelected", args = 1)]
    pub fn set_is_selected(self, value: bool) -> ();

    #[method(name = "get_IsTalk", args = 0)]
    pub fn get_is_talk(self) -> bool;

    #[method(name = "set_IsTalk", args = 1)]
    pub fn set_is_talk(self, value: bool) -> ();

    #[method(name = "get_Cursor", args = 0)]
    pub fn get_cursor(
        self,
    ) -> crate::app::myroomrelianceselectroot::MyRoomRelianceSelectRoot_CursorTop;

    #[method(name = "get_IsBlank", args = 0)]
    pub fn get_is_blank(self) -> bool;

    #[method(name = "get_IsAPlus", args = 0)]
    pub fn get_is_a_plus(self) -> bool;

    #[method(name = "GetSortOrder", args = 0)]
    pub fn get_sort_order(self) -> i32;

    #[method(name = "GetCommandColor", args = 0)]
    pub fn get_command_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "GetOpenCount", args = 0)]
    pub fn get_open_count(self) -> i32;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit_l: crate::app::unit::Unit, unit_r: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        unit_l: crate::app::unit::Unit,
        god_unit: crate::app::godunit::GodUnit,
    ) -> ();

    #[method(name = "UpdateTalk", args = 0)]
    pub fn update_talk(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "GetPlayerGodLevel", args = 2)]
    pub fn get_player_god_level(
        self,
        unit_l: crate::app::unit::Unit,
        unit_r: crate::app::unit::Unit,
    ) -> i32;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnitL", args = 0)]
    pub fn get_unit_l(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitR", args = 0)]
    pub fn get_unit_r(self) -> crate::app::unit::Unit;

    #[method(name = "GetGodUnit", args = 0)]
    pub fn get_god_unit(self) -> crate::app::godunit::GodUnit;
}

#[cfg(feature = "app-myroomreliancesubselect")]
impl MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem {
    pub fn new(unit_l: crate::app::unit::Unit, unit_r: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRelianceSubSelect_MyRoomRelianceSubSelectItemMethods>::ctor(
            this, unit_l, unit_r,
        );
        this
    }

    pub fn new_2(unit_l: crate::app::unit::Unit, god_unit: crate::app::godunit::GodUnit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRelianceSubSelect_MyRoomRelianceSubSelectItem),
                ::core::stringify!(new_2),
            )
        });
        <Self as IMyRoomRelianceSubSelect_MyRoomRelianceSubSelectItemMethods>::ctor_2(
            this, unit_l, god_unit,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomreliancesubselect/MyRoomRelianceSubSelect_MyRoomRelianceCallSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MyRoomRelianceSubSelect.MyRoomRelianceCallSequence"
)]
# [parent (crate :: app :: stackprocinst_1 :: StackProcInst_1 < crate :: app :: myroomreliancesubselect :: MyRoomRelianceSubSelect_MyRoomRelianceCallSequence >)]
pub struct MyRoomRelianceSubSelect_MyRoomRelianceCallSequence {
    #[rename(name = "m_bind")]
    pub m_bind: bool,
    #[rename(name = "m_camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
}

#[cfg(feature = "app-myroomreliancesubselect")]
#[::unity2::methods]
impl MyRoomRelianceSubSelect_MyRoomRelianceCallSequence {
    #[method(name = "get_Self", args = 0)]
    pub fn get_self(self) -> crate::app::unit::Unit;

    #[method(name = "set_Self", args = 1)]
    pub fn set_self(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_Target", args = 0)]
    pub fn get_target(self) -> crate::app::unit::Unit;

    #[method(name = "set_Target", args = 1)]
    pub fn set_target(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> crate::app::reliancedata::RelianceData_Level;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: crate::app::reliancedata::RelianceData_Level) -> ();

    #[method(name = "get_GodTarget", args = 0)]
    pub fn get_god_target(self) -> crate::app::godunit::GodUnit;

    #[method(name = "set_GodTarget", args = 1)]
    pub fn set_god_target(self, value: crate::app::godunit::GodUnit) -> ();

    #[method(name = "get_GodTargetData", args = 0)]
    pub fn get_god_target_data(self) -> crate::app::goddata::GodData;

    #[method(name = "set_GodTargetData", args = 1)]
    pub fn set_god_target_data(self, value: crate::app::goddata::GodData) -> ();

    #[method(name = "get_NextRelianceLevel", args = 0)]
    pub fn get_next_reliance_level(self) -> crate::app::goddata::GodData_RelianceLevel;

    #[method(name = "set_NextRelianceLevel", args = 1)]
    pub fn set_next_reliance_level(self, value: crate::app::goddata::GodData_RelianceLevel) -> ();

    #[method(name = "get_PrevLevel", args = 0)]
    pub fn get_prev_level(self) -> i32;

    #[method(name = "set_PrevLevel", args = 1)]
    pub fn set_prev_level(self, value: i32) -> ();

    #[method(name = "get_NextLevel", args = 0)]
    pub fn get_next_level(self) -> i32;

    #[method(name = "set_NextLevel", args = 1)]
    pub fn set_next_level(self, value: i32) -> ();

    #[method(name = "get_EntryCallback", args = 0)]
    pub fn get_entry_callback(self) -> crate::system::action::Action;

    #[method(name = "set_EntryCallback", args = 1)]
    pub fn set_entry_callback(self, value: crate::system::action::Action) -> ();

    #[method(name = "get_ExitCallback", args = 0)]
    pub fn get_exit_callback(self) -> crate::system::action::Action;

    #[method(name = "set_ExitCallback", args = 1)]
    pub fn set_exit_callback(self, value: crate::system::action::Action) -> ();

    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        self_: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        level: crate::app::reliancedata::RelianceData_Level,
        pre_level: i32,
        next_level: i32,
    ) -> ();

    #[method(name = "CreateBind", args = 9)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        self_: crate::app::unit::Unit,
        target: crate::app::godunit::GodUnit,
        next_reliance_level: crate::app::goddata::GodData_RelianceLevel,
        prev_level: i32,
        next_level: i32,
        entry_callback: crate::system::action::Action,
        exit_callback: crate::system::action::Action,
        god_data: crate::app::goddata::GodData,
    ) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "Entry", args = 0)]
    pub fn entry(self) -> ();

    #[method(name = "Main", args = 0)]
    pub fn main(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "UpdateAchive", args = 3)]
    pub fn update_achive(
        self,
        a: crate::app::unit::Unit,
        b: crate::app::unit::Unit,
        level: crate::app::reliancedata::RelianceData_Level,
    ) -> ();

    #[method(name = "LevelUp", args = 0)]
    pub fn level_up(self) -> ();

    #[method(name = "LevelUpAfter", args = 0)]
    pub fn level_up_after(self) -> ();

    #[method(name = "AmiiboBenefit", args = 0)]
    pub fn amiibo_benefit(self) -> ();

    #[method(name = "FadeIn", args = 0)]
    pub fn fade_in(self) -> ();

    #[method(name = "CreateReliance", args = 0)]
    pub fn create_reliance(self) -> ();

    #[method(name = "CreateRelianceGod", args = 0)]
    pub fn create_reliance_god(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomreliancesubselect")]
impl MyRoomRelianceSubSelect_MyRoomRelianceCallSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRelianceSubSelect_MyRoomRelianceCallSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRelianceSubSelect_MyRoomRelianceCallSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomreliancesubselect/MyRoomRelianceSubSelect_MyRoomRelianceCallSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomRelianceSubSelect_MyRoomRelianceCallSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomRelianceSubSelect_MyRoomRelianceCallSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomRelianceSubSelect.MyRoomRelianceCallSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomRelianceSubSelect_MyRoomRelianceCallSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomRelianceSubSelect_MyRoomRelianceCallSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn main() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomreliancesubselect/MyRoomRelianceSubSelect.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomRelianceSubSelect")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct MyRoomRelianceSubSelect {
    #[rename(name = "IsRankPhase")]
    pub is_rank_phase: bool,
    #[rename(name = "m_MenuSelectList")]
    pub m_menu_select_list: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuselect::BasicMenuSelect,
    >,
}

#[cfg(feature = "app-myroomreliancesubselect")]
#[::unity2::methods]
impl MyRoomRelianceSubSelect {
    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        select_unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "CreateMenuList", args = 1)]
    pub fn create_menu_list(
        select_unit: crate::app::unit::Unit,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::myroomreliancesubselectcontent::MyRoomRelianceSubSelectContent,
    ) -> ();

    #[method(name = "RebuildMenu", args = 1)]
    pub fn rebuild_menu(self, select_unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateTalkState", args = 0)]
    pub fn update_talk_state(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-myroomreliancesubselect")]
impl MyRoomRelianceSubSelect {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::myroomreliancesubselectcontent::MyRoomRelianceSubSelectContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRelianceSubSelect),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRelianceSubSelectMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomreliancesubselect/MyRoomRelianceSubSelect_AmiiboBenefitSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "MyRoomRelianceSubSelect.AmiiboBenefitSequence"
)]
#[parent(crate::app::procinst::ProcInst)]
pub struct MyRoomRelianceSubSelect_AmiiboBenefitSequence {
    #[rename(name = "m_GodUnit")]
    pub m_god_unit: crate::app::godunit::GodUnit,
    #[rename(name = "m_PieceCount")]
    pub m_piece_count: i32,
}

#[cfg(feature = "app-myroomreliancesubselect")]
#[::unity2::methods]
impl MyRoomRelianceSubSelect_AmiiboBenefitSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, god_unit: crate::app::godunit::GodUnit) -> ();

    #[method(name = "Accessory", args = 0)]
    pub fn accessory(self) -> ();

    #[method(name = "Music", args = 0)]
    pub fn music(self) -> ();

    #[method(name = "PieceOfBond", args = 0)]
    pub fn piece_of_bond(self) -> ();

    #[method(name = "Message", args = 3)]
    pub fn message(
        self,
        god_unit: crate::app::godunit::GodUnit,
        sprite_name: ::unity2::Il2CppString,
        item_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        god_unit: crate::app::godunit::GodUnit,
    ) -> ();
}

#[cfg(feature = "app-myroomreliancesubselect")]
impl MyRoomRelianceSubSelect_AmiiboBenefitSequence {
    pub fn new(god_unit: crate::app::godunit::GodUnit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomRelianceSubSelect_AmiiboBenefitSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomRelianceSubSelect_AmiiboBenefitSequenceMethods>::ctor(this, god_unit);
        this
    }
}
