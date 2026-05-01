
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmind/MapMind_Record.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapMind_Record {
    pub r#type: crate::app::mapmind::MapMind_Type,
    pub main: crate::app::mapmind::MapMind_Record_Value,
    pub link: crate::app::mapmind::MapMind_Record_Value,
}

impl ::unity2::ClassIdentity for MapMind_Record {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapMind.Record";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapMind_Record {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapmind")]
#[::unity2::methods(value)]
impl MapMind_Record {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        r#type: crate::app::mapmind::MapMind_Type,
        unit: crate::app::unit::Unit,
        link: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "Cancel", args = 0)]
    pub fn cancel(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmind/MapMind_Record_Value.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapMind_Record_Value {
    pub unit: crate::app::unit::Unit,
    pub x: i32,
    pub z: i32,
    pub is_changed: bool,
    pub engage_count: i32,
    pub unit_item: crate::app::unititem::UnitItem,
}

impl ::unity2::ClassIdentity for MapMind_Record_Value {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapMind.Record.Value";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapMind_Record_Value {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapmind")]
#[::unity2::methods(value)]
impl MapMind_Record_Value {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Cancel", args = 0)]
    pub fn cancel(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmind/MapMind_CommandStack.md")))]
#[::unity2::class(namespace = "App", name = "MapMind.CommandStack")]
#[parent(crate::system::object::Object)]
pub struct MapMind_CommandStack {
    #[rename(name = "m_Statck")]
    pub m_statck:
        crate::system::collections::generic::stack_1::Stack_1<crate::app::mapmind::MapMind_Record>,
}

#[cfg(feature = "app-mapmind")]
#[::unity2::methods]
impl MapMind_CommandStack {
    #[method(name = "Peek", args = 0)]
    pub fn peek(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "Push", args = 3)]
    pub fn push(
        self,
        r#type: crate::app::mapmind::MapMind_Type,
        unit: crate::app::unit::Unit,
        link: crate::app::unit::Unit,
    ) -> ();

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> bool;

    #[method(name = "Decide", args = 0)]
    pub fn decide(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_Exist", args = 0)]
    pub fn get_exist(self) -> bool;

    #[method(name = "TryAddAchieveEngage", args = 2)]
    pub fn try_add_achieve_engage(
        self,
        r#type: crate::app::mapmind::MapMind_Type,
        unit: crate::app::unit::Unit,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapmind")]
impl MapMind_CommandStack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapMind_CommandStack),
                ::core::stringify!(new),
            )
        });
        <Self as IMapMind_CommandStackMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmind/MapMind_Done.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapMind_Done {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapMind_Done {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapMind.Done";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapMind_Done {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapMind_Done {
    pub fn talk() -> Self {
        Self { value: 1 }
    }

    pub fn trade() -> Self {
        Self { value: 2 }
    }

    pub fn putoff() -> Self {
        Self { value: 4 }
    }

    pub fn transporter() -> Self {
        Self { value: 8 }
    }

    pub fn action() -> Self {
        Self { value: 16 }
    }

    pub fn sight() -> Self {
        Self { value: 32 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmind/MapMind_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapMind_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapMind_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapMind.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapMind_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapMind_Type {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn fixed() -> Self {
        Self { value: 1 }
    }

    pub fn talk() -> Self {
        Self { value: 2 }
    }

    pub fn attack() -> Self {
        Self { value: 3 }
    }

    pub fn engage_start() -> Self {
        Self { value: 4 }
    }

    pub fn engage_link() -> Self {
        Self { value: 5 }
    }

    pub fn engage_attack() -> Self {
        Self { value: 6 }
    }

    pub fn engage_rod() -> Self {
        Self { value: 7 }
    }

    pub fn engage_rewarp() -> Self {
        Self { value: 8 }
    }

    pub fn engage_charge() -> Self {
        Self { value: 9 }
    }

    pub fn cannon() -> Self {
        Self { value: 10 }
    }

    pub fn destroy() -> Self {
        Self { value: 11 }
    }

    pub fn rod() -> Self {
        Self { value: 12 }
    }

    pub fn item_use() -> Self {
        Self { value: 13 }
    }

    pub fn item() -> Self {
        Self { value: 14 }
    }

    pub fn trade() -> Self {
        Self { value: 15 }
    }

    pub fn visit() -> Self {
        Self { value: 16 }
    }

    pub fn breakdown() -> Self {
        Self { value: 17 }
    }

    pub fn breakdown_enemy() -> Self {
        Self { value: 18 }
    }

    pub fn escape() -> Self {
        Self { value: 19 }
    }

    pub fn breakthrough() -> Self {
        Self { value: 20 }
    }

    pub fn door() -> Self {
        Self { value: 21 }
    }

    pub fn torch() -> Self {
        Self { value: 22 }
    }

    pub fn treasure_box() -> Self {
        Self { value: 23 }
    }

    pub fn transporter() -> Self {
        Self { value: 24 }
    }

    pub fn rod_warp() -> Self {
        Self { value: 25 }
    }

    pub fn rod_warp_dest() -> Self {
        Self { value: 26 }
    }

    pub fn rod_rewarp() -> Self {
        Self { value: 27 }
    }

    pub fn rod_rewarp_dest() -> Self {
        Self { value: 28 }
    }

    pub fn rod_rescue() -> Self {
        Self { value: 29 }
    }

    pub fn rod_interference() -> Self {
        Self { value: 30 }
    }

    pub fn rod_torch() -> Self {
        Self { value: 31 }
    }

    pub fn rod_creation() -> Self {
        Self { value: 32 }
    }

    pub fn rod_nodus() -> Self {
        Self { value: 33 }
    }

    pub fn dance() -> Self {
        Self { value: 34 }
    }

    pub fn guard() -> Self {
        Self { value: 35 }
    }

    pub fn dragon_vein() -> Self {
        Self { value: 36 }
    }

    pub fn overlap_skill() -> Self {
        Self { value: 37 }
    }

    pub fn command_skill() -> Self {
        Self { value: 38 }
    }

    pub fn vision_create() -> Self {
        Self { value: 39 }
    }

    pub fn vision_delete() -> Self {
        Self { value: 40 }
    }

    pub fn god_change() -> Self {
        Self { value: 41 }
    }

    pub fn destroy_village() -> Self {
        Self { value: 42 }
    }

    pub fn turn_end() -> Self {
        Self { value: 43 }
    }

    pub fn surrender() -> Self {
        Self { value: 44 }
    }

    pub fn informal() -> Self {
        Self { value: 45 }
    }

    pub fn rod_heal() -> Self {
        Self { value: 46 }
    }

    pub fn rod_magic_shield() -> Self {
        Self { value: 47 }
    }

    pub fn full_bullet() -> Self {
        Self { value: 48 }
    }

    pub fn engage_wait() -> Self {
        Self { value: 49 }
    }

    pub fn engage_summon() -> Self {
        Self { value: 50 }
    }

    pub fn item_menu() -> Self {
        Self { value: 51 }
    }

    pub fn enchant_menu() -> Self {
        Self { value: 52 }
    }

    pub fn enchant() -> Self {
        Self { value: 53 }
    }

    pub fn contract() -> Self {
        Self { value: 54 }
    }

    pub fn sub_menu() -> Self {
        Self { value: 55 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmind/MapMind_MultiTargets.md")))]
#[::unity2::class(namespace = "App", name = "MapMind.MultiTargets")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: mapmind :: MapMind_Target >)]
pub struct MapMind_MultiTargets {}

#[cfg(feature = "app-mapmind")]
#[::unity2::methods]
impl MapMind_MultiTargets {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Exists", args = 1)]
    pub fn exists(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, unit: crate::app::unit::Unit) -> i32;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of_2(self, terrain: crate::app::terraindata_2::TerrainData_2) -> i32;

    #[method(name = "IndexOf", args = 2)]
    pub fn index_of_3(self, x: i32, z: i32) -> i32;

    #[method(name = "Add", args = 1)]
    pub fn add(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_2(self, x: i32, z: i32, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_3(self, x: i32, z: i32, tid: ::unity2::Il2CppString) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add_4(self, x: i32, z: i32, terrain: crate::app::terraindata_2::TerrainData_2) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_5(self, unit: crate::app::unit::Unit, value: i32) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add_6(self, x: i32, z: i32) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetCneter", args = 0)]
    pub fn get_cneter(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetUnitCount", args = 0)]
    pub fn get_unit_count(self) -> i32;
}

#[cfg(feature = "app-mapmind")]
impl MapMind_MultiTargets {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapMind_MultiTargets),
                ::core::stringify!(new),
            )
        });
        <Self as IMapMind_MultiTargetsMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmind/MapMind_DoneField.md")))]
#[::unity2::class(namespace = "App", name = "MapMind.DoneField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: mapmind :: MapMind_Done >)]
pub struct MapMind_DoneField {}

#[cfg(feature = "app-mapmind")]
#[::unity2::methods]
impl MapMind_DoneField {
    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::mapmind::MapMind_Done) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapmind")]
impl MapMind_DoneField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapMind_DoneField),
                ::core::stringify!(new),
            )
        });
        <Self as IMapMind_DoneFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmind/MapMind_Target.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapMind_Target {
    pub unit: crate::app::unit::Unit,
    pub terrain: crate::app::terraindata_2::TerrainData_2,
    pub x: i8,
    pub z: i8,
}

impl ::unity2::ClassIdentity for MapMind_Target {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapMind.Target";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapMind_Target {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-mapmind")]
#[::unity2::methods(value)]
impl MapMind_Target {
    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> i8;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: i8) -> ();

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmind/MapMind.md")))]
#[::unity2::class(namespace = "App", name = "MapMind")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapmind :: MapMind >)]
pub struct MapMind {
    #[static_field]
    #[rename(name = "RouteMax")]
    pub route_max: i32,
    #[rename(name = "m_UnitIndex")]
    pub m_unit_index: u8,
    #[rename(name = "m_FirstUnitIndex")]
    pub m_first_unit_index: u8,
    #[rename(name = "m_FirstX")]
    pub m_first_x: i8,
    #[rename(name = "m_FirstZ")]
    pub m_first_z: i8,
    #[rename(name = "m_UnitShowX")]
    pub m_unit_show_x: i8,
    #[rename(name = "m_UnitShowZ")]
    pub m_unit_show_z: i8,
    #[rename(name = "m_X")]
    pub m_x: i8,
    #[rename(name = "m_Z")]
    pub m_z: i8,
    #[rename(name = "m_Mind")]
    pub m_mind: crate::app::mapmind::MapMind_Type,
    #[rename(name = "m_AttackX")]
    pub m_attack_x: i8,
    #[rename(name = "m_AttackZ")]
    pub m_attack_z: i8,
    #[rename(name = "m_ItemIndex")]
    pub m_item_index: i8,
    #[rename(name = "m_TargetUnitIndex")]
    pub m_target_unit_index: u8,
    #[rename(name = "m_TargetX")]
    pub m_target_x: i8,
    #[rename(name = "m_TargetZ")]
    pub m_target_z: i8,
    #[rename(name = "m_FocusX")]
    pub m_focus_x: i8,
    #[rename(name = "m_FocusZ")]
    pub m_focus_z: i8,
    #[rename(name = "m_TargetArgument")]
    pub m_target_argument: i16,
    #[rename(name = "m_TradeUnitIndex")]
    pub m_trade_unit_index: u8,
    #[rename(name = "m_EventUnitIndex")]
    pub m_event_unit_index: u8,
    #[rename(name = "m_Done")]
    pub m_done: crate::app::mapmind::MapMind_DoneField,
    #[rename(name = "m_MovePower")]
    pub m_move_power: u8,
    #[rename(name = "m_TransporterIndex")]
    pub m_transporter_index: i16,
    #[rename(name = "m_CommandSkill")]
    pub m_command_skill: crate::app::skilldata::SkillData,
    #[rename(name = "m_SpecifiedItem")]
    pub m_specified_item: crate::app::itemdata::ItemData,
    #[rename(name = "m_AIEngageRewarpX")]
    pub m_ai_engage_rewarp_x: i8,
    #[rename(name = "m_AIEngageRewarpZ")]
    pub m_ai_engage_rewarp_z: i8,
    #[rename(name = "m_Routes")]
    pub m_routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
}

#[cfg(feature = "app-mapmind")]
#[::unity2::methods]
impl MapMind {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 2)]
    pub fn reset(self, unit: crate::app::unit::Unit, for_remove: bool) -> ();

    #[method(name = "ResetMind", args = 0)]
    pub fn reset_mind(self) -> ();

    #[method(name = "ResetTarget", args = 0)]
    pub fn reset_target(self) -> ();

    #[method(name = "ResetRoute", args = 0)]
    pub fn reset_route(self) -> ();

    #[method(name = "IsMoved", args = 0)]
    pub fn is_moved(self) -> bool;

    #[method(name = "IsDone", args = 0)]
    pub fn is_done(self) -> bool;

    #[method(name = "IsDone", args = 1)]
    pub fn is_done_2(self, mask: crate::app::mapmind::MapMind_Done) -> bool;

    #[method(name = "SetDone", args = 1)]
    pub fn set_done(self, mask: crate::app::mapmind::MapMind_Done) -> ();

    #[method(name = "IsAIEngageRewarp", args = 0)]
    pub fn is_ai_engage_rewarp(self) -> bool;

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "get_TargetUnit", args = 0)]
    pub fn get_target_unit(self) -> crate::app::unit::Unit;

    #[method(name = "get_TradeUnit", args = 0)]
    pub fn get_trade_unit(self) -> crate::app::unit::Unit;

    #[method(name = "get_UnitIndex", args = 0)]
    pub fn get_unit_index(self) -> i32;

    #[method(name = "get_FirstUnitIndex", args = 0)]
    pub fn get_first_unit_index(self) -> i32;

    #[method(name = "get_FirstX", args = 0)]
    pub fn get_first_x(self) -> i32;

    #[method(name = "get_FirstZ", args = 0)]
    pub fn get_first_z(self) -> i32;

    #[method(name = "get_UnitX", args = 0)]
    pub fn get_unit_x(self) -> i32;

    #[method(name = "set_UnitX", args = 1)]
    pub fn set_unit_x(self, value: i32) -> ();

    #[method(name = "get_UnitZ", args = 0)]
    pub fn get_unit_z(self) -> i32;

    #[method(name = "set_UnitZ", args = 1)]
    pub fn set_unit_z(self, value: i32) -> ();

    #[method(name = "SetUnitPos", args = 2)]
    pub fn set_unit_pos(self, x: i32, z: i32) -> ();

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_X", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "set_Z", args = 1)]
    pub fn set_z(self, value: i32) -> ();

    #[method(name = "get_Mind", args = 0)]
    pub fn get_mind(self) -> crate::app::mapmind::MapMind_Type;

    #[method(name = "set_Mind", args = 1)]
    pub fn set_mind(self, value: crate::app::mapmind::MapMind_Type) -> ();

    #[method(name = "get_AttackX", args = 0)]
    pub fn get_attack_x(self) -> i32;

    #[method(name = "set_AttackX", args = 1)]
    pub fn set_attack_x(self, value: i32) -> ();

    #[method(name = "get_AttackZ", args = 0)]
    pub fn get_attack_z(self) -> i32;

    #[method(name = "set_AttackZ", args = 1)]
    pub fn set_attack_z(self, value: i32) -> ();

    #[method(name = "get_ItemIndex", args = 0)]
    pub fn get_item_index(self) -> i32;

    #[method(name = "set_ItemIndex", args = 1)]
    pub fn set_item_index(self, value: i32) -> ();

    #[method(name = "get_TargetUnitIndex", args = 0)]
    pub fn get_target_unit_index(self) -> i32;

    #[method(name = "set_TargetUnitIndex", args = 1)]
    pub fn set_target_unit_index(self, value: i32) -> ();

    #[method(name = "get_TargetX", args = 0)]
    pub fn get_target_x(self) -> i32;

    #[method(name = "set_TargetX", args = 1)]
    pub fn set_target_x(self, value: i32) -> ();

    #[method(name = "get_TargetZ", args = 0)]
    pub fn get_target_z(self) -> i32;

    #[method(name = "set_TargetZ", args = 1)]
    pub fn set_target_z(self, value: i32) -> ();

    #[method(name = "get_FocusX", args = 0)]
    pub fn get_focus_x(self) -> i32;

    #[method(name = "set_FocusX", args = 1)]
    pub fn set_focus_x(self, value: i32) -> ();

    #[method(name = "get_FocusZ", args = 0)]
    pub fn get_focus_z(self) -> i32;

    #[method(name = "set_FocusZ", args = 1)]
    pub fn set_focus_z(self, value: i32) -> ();

    #[method(name = "get_TargetArgument", args = 0)]
    pub fn get_target_argument(self) -> i32;

    #[method(name = "set_TargetArgument", args = 1)]
    pub fn set_target_argument(self, value: i32) -> ();

    #[method(name = "get_TradeUnitIndex", args = 0)]
    pub fn get_trade_unit_index(self) -> i32;

    #[method(name = "set_TradeUnitIndex", args = 1)]
    pub fn set_trade_unit_index(self, value: i32) -> ();

    #[method(name = "get_MovePower", args = 0)]
    pub fn get_move_power(self) -> i32;

    #[method(name = "set_MovePower", args = 1)]
    pub fn set_move_power(self, value: i32) -> ();

    #[method(name = "get_Routes", args = 0)]
    pub fn get_routes(self) -> ::unity2::Array<crate::app::dir_2::Dir_Type>;

    #[method(name = "set_Routes", args = 1)]
    pub fn set_routes(self, value: ::unity2::Array<crate::app::dir_2::Dir_Type>) -> ();

    #[method(name = "get_TransporterIndex", args = 0)]
    pub fn get_transporter_index(self) -> i32;

    #[method(name = "set_TransporterIndex", args = 1)]
    pub fn set_transporter_index(self, value: i32) -> ();

    #[method(name = "get_EventUnitIndex", args = 0)]
    pub fn get_event_unit_index(self) -> i32;

    #[method(name = "set_EventUnitIndex", args = 1)]
    pub fn set_event_unit_index(self, value: i32) -> ();

    #[method(name = "get_CommandSkill", args = 0)]
    pub fn get_command_skill(self) -> crate::app::skilldata::SkillData;

    #[method(name = "set_CommandSkill", args = 1)]
    pub fn set_command_skill(self, value: crate::app::skilldata::SkillData) -> ();

    #[method(name = "get_SpecifiedItem", args = 0)]
    pub fn get_specified_item(self) -> crate::app::itemdata::ItemData;

    #[method(name = "set_SpecifiedItem", args = 1)]
    pub fn set_specified_item(self, value: crate::app::itemdata::ItemData) -> ();

    #[method(name = "get_AIEngageRewarpX", args = 0)]
    pub fn get_ai_engage_rewarp_x(self) -> i32;

    #[method(name = "set_AIEngageRewarpX", args = 1)]
    pub fn set_ai_engage_rewarp_x(self, value: i32) -> ();

    #[method(name = "get_AIEngageRewarpZ", args = 0)]
    pub fn get_ai_engage_rewarp_z(self) -> i32;

    #[method(name = "set_AIEngageRewarpZ", args = 1)]
    pub fn set_ai_engage_rewarp_z(self, value: i32) -> ();

    #[method(name = "get_SummonColor", args = 0)]
    pub fn get_summon_color(self) -> crate::app::persondata::PersonData_Colors;

    #[method(name = "set_SummonColor", args = 1)]
    pub fn set_summon_color(self, value: crate::app::persondata::PersonData_Colors) -> ();

    #[method(name = "get_MultiTarget", args = 0)]
    pub fn get_multi_target(self) -> crate::app::mapmind::MapMind_MultiTargets;

    #[method(name = "get_Targets", args = 0)]
    pub fn get_targets(self) -> crate::app::mapmind::MapMind_MultiTargets;

    #[method(name = "get_Stack", args = 0)]
    pub fn get_stack(self) -> crate::app::mapmind::MapMind_CommandStack;

    #[method(name = "GetTargetUnit", args = 1)]
    pub fn get_target_unit_2(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetTargetX", args = 1)]
    pub fn get_target_x_2(self, index: i32) -> i32;

    #[method(name = "GetTargetZ", args = 1)]
    pub fn get_target_z_2(self, index: i32) -> i32;

    #[method(name = "GetTargetLastX", args = 0)]
    pub fn get_target_last_x(self) -> i32;

    #[method(name = "GetTargetLastZ", args = 0)]
    pub fn get_target_last_z(self) -> i32;

    #[method(name = "GetDestroyTarget", args = 0)]
    pub fn get_destroy_target(self) -> crate::app::pokeinspector::PokeInspector;

    #[method(name = "GetItem", args = 0)]
    pub fn get_item(self) -> crate::app::itemdata::ItemData;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetUnitItem", args = 1)]
    pub fn get_unit_item_2(self, index: i32) -> crate::app::unititem::UnitItem;

    #[method(name = "CanGodChange", args = 0)]
    pub fn can_god_change(self) -> bool;

    #[method(name = "IsFocus", args = 0)]
    pub fn is_focus(self) -> bool;

    #[method(name = "GetMind", args = 2)]
    pub fn get_mind_2(
        mind: crate::app::mapmind::MapMind_Type,
        skill: crate::app::skilldata::SkillData,
    ) -> crate::app::mapmind::MapMind_Type;
}

#[cfg(feature = "app-mapmind")]
impl MapMind {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapMind),
                ::core::stringify!(new),
            )
        });
        <Self as IMapMindMethods>::ctor(this);
        this
    }
}
