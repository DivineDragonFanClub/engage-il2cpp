
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/disposdata/DisposData_Item.md")))]
#[::unity2::class(namespace = "App", name = "DisposData.Item")]
#[parent(crate::system::object::Object)]
pub struct DisposData_Item {}

#[cfg(feature = "app-disposdata")]
#[::unity2::methods]
impl DisposData_Item {
    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Drop", args = 0)]
    pub fn get_drop(self) -> i32;

    #[method(name = "set_Drop", args = 1)]
    pub fn set_drop(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-disposdata")]
impl DisposData_Item {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisposData_Item),
                ::core::stringify!(new),
            )
        });
        <Self as IDisposData_ItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/disposdata/DisposData_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DisposData_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DisposData_State {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DisposData.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DisposData_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DisposData_State {
    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn rampage() -> Self {
        Self { value: 1 }
    }

    pub fn keep() -> Self {
        Self { value: -1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/disposdata/DisposData_Directions.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DisposData_Directions {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DisposData_Directions {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DisposData.Directions";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DisposData_Directions {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DisposData_Directions {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn up() -> Self {
        Self { value: 1 }
    }

    pub fn upper_right() -> Self {
        Self { value: 2 }
    }

    pub fn right() -> Self {
        Self { value: 3 }
    }

    pub fn lower_right() -> Self {
        Self { value: 4 }
    }

    pub fn down() -> Self {
        Self { value: 5 }
    }

    pub fn lower_left() -> Self {
        Self { value: 6 }
    }

    pub fn left() -> Self {
        Self { value: 7 }
    }

    pub fn upper_left() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/disposdata/DisposData_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "DisposData.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: disposdata :: DisposData_Flags >)]
pub struct DisposData_FlagField {}

#[cfg(feature = "app-disposdata")]
#[::unity2::methods]
impl DisposData_FlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: crate::app::disposdata::DisposData_Flags) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::disposdata::DisposData_Flags) -> i32;
}

#[cfg(feature = "app-disposdata")]
impl DisposData_FlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisposData_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IDisposData_FlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::disposdata::DisposData_Flags) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisposData_FlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDisposData_FlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/disposdata/DisposData.md")))]
#[::unity2::class(namespace = "App", name = "DisposData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: disposdata :: DisposData >)]
pub struct DisposData {
    #[static_field]
    #[rename(name = "ITEM_COUNT")]
    pub item_count: i32,
}

#[cfg(feature = "app-disposdata")]
#[::unity2::methods]
impl DisposData {
    #[method(name = "Load", args = 1)]
    pub fn load(file_name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Group", args = 0)]
    pub fn get_group(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Group", args = 1)]
    pub fn set_group(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Tid", args = 0)]
    pub fn get_tid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Tid", args = 1)]
    pub fn set_tid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::disposdata::DisposData_FlagField;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::disposdata::DisposData_FlagField) -> ();

    #[method(name = "get_Jid", args = 0)]
    pub fn get_jid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Jid", args = 1)]
    pub fn set_jid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Sid", args = 0)]
    pub fn get_sid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Sid", args = 1)]
    pub fn set_sid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Bid", args = 0)]
    pub fn get_bid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bid", args = 1)]
    pub fn set_bid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AppearX", args = 0)]
    pub fn get_appear_x(self) -> i8;

    #[method(name = "set_AppearX", args = 1)]
    pub fn set_appear_x(self, value: i8) -> ();

    #[method(name = "get_AppearY", args = 0)]
    pub fn get_appear_y(self) -> i8;

    #[method(name = "set_AppearY", args = 1)]
    pub fn set_appear_y(self, value: i8) -> ();

    #[method(name = "get_DisposX", args = 0)]
    pub fn get_dispos_x(self) -> i8;

    #[method(name = "set_DisposX", args = 1)]
    pub fn set_dispos_x(self, value: i8) -> ();

    #[method(name = "get_DisposY", args = 0)]
    pub fn get_dispos_y(self) -> i8;

    #[method(name = "set_DisposY", args = 1)]
    pub fn set_dispos_y(self, value: i8) -> ();

    #[method(name = "get_Direction", args = 0)]
    pub fn get_direction(self) -> crate::app::disposdata::DisposData_Directions;

    #[method(name = "set_Direction", args = 1)]
    pub fn set_direction(self, value: crate::app::disposdata::DisposData_Directions) -> ();

    #[method(name = "get_Rotation", args = 0)]
    pub fn get_rotation(self) -> i8;

    #[method(name = "set_Rotation", args = 1)]
    pub fn set_rotation(self, value: i8) -> ();

    #[method(name = "get_LevelN", args = 0)]
    pub fn get_level_n(self) -> u8;

    #[method(name = "set_LevelN", args = 1)]
    pub fn set_level_n(self, value: u8) -> ();

    #[method(name = "get_LevelH", args = 0)]
    pub fn get_level_h(self) -> u8;

    #[method(name = "set_LevelH", args = 1)]
    pub fn set_level_h(self, value: u8) -> ();

    #[method(name = "get_LevelL", args = 0)]
    pub fn get_level_l(self) -> u8;

    #[method(name = "set_LevelL", args = 1)]
    pub fn set_level_l(self, value: u8) -> ();

    #[method(name = "get_Items", args = 0)]
    pub fn get_items(self) -> ::unity2::Array<crate::app::disposdata::DisposData_Item>;

    #[method(name = "set_Items", args = 1)]
    pub fn set_items(self, value: ::unity2::Array<crate::app::disposdata::DisposData_Item>) -> ();

    #[method(name = "get_Item1", args = 0)]
    pub fn get_item1(self) -> crate::app::disposdata::DisposData_Item;

    #[method(name = "set_Item1", args = 1)]
    pub fn set_item1(self, value: crate::app::disposdata::DisposData_Item) -> ();

    #[method(name = "get_Item2", args = 0)]
    pub fn get_item2(self) -> crate::app::disposdata::DisposData_Item;

    #[method(name = "set_Item2", args = 1)]
    pub fn set_item2(self, value: crate::app::disposdata::DisposData_Item) -> ();

    #[method(name = "get_Item3", args = 0)]
    pub fn get_item3(self) -> crate::app::disposdata::DisposData_Item;

    #[method(name = "set_Item3", args = 1)]
    pub fn set_item3(self, value: crate::app::disposdata::DisposData_Item) -> ();

    #[method(name = "get_Item4", args = 0)]
    pub fn get_item4(self) -> crate::app::disposdata::DisposData_Item;

    #[method(name = "set_Item4", args = 1)]
    pub fn set_item4(self, value: crate::app::disposdata::DisposData_Item) -> ();

    #[method(name = "get_Item5", args = 0)]
    pub fn get_item5(self) -> crate::app::disposdata::DisposData_Item;

    #[method(name = "set_Item5", args = 1)]
    pub fn set_item5(self, value: crate::app::disposdata::DisposData_Item) -> ();

    #[method(name = "get_Item6", args = 0)]
    pub fn get_item6(self) -> crate::app::disposdata::DisposData_Item;

    #[method(name = "set_Item6", args = 1)]
    pub fn set_item6(self, value: crate::app::disposdata::DisposData_Item) -> ();

    #[method(name = "get_Gid", args = 0)]
    pub fn get_gid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Gid", args = 1)]
    pub fn set_gid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HpStockCount", args = 0)]
    pub fn get_hp_stock_count(self) -> u8;

    #[method(name = "set_HpStockCount", args = 1)]
    pub fn set_hp_stock_count(self, value: u8) -> ();

    #[method(name = "get_State0", args = 0)]
    pub fn get_state0(self) -> crate::app::disposdata::DisposData_State;

    #[method(name = "set_State0", args = 1)]
    pub fn set_state0(self, value: crate::app::disposdata::DisposData_State) -> ();

    #[method(name = "get_State1", args = 0)]
    pub fn get_state1(self) -> crate::app::disposdata::DisposData_State;

    #[method(name = "set_State1", args = 1)]
    pub fn set_state1(self, value: crate::app::disposdata::DisposData_State) -> ();

    #[method(name = "get_State2", args = 0)]
    pub fn get_state2(self) -> crate::app::disposdata::DisposData_State;

    #[method(name = "set_State2", args = 1)]
    pub fn set_state2(self, value: crate::app::disposdata::DisposData_State) -> ();

    #[method(name = "get_State3", args = 0)]
    pub fn get_state3(self) -> crate::app::disposdata::DisposData_State;

    #[method(name = "set_State3", args = 1)]
    pub fn set_state3(self, value: crate::app::disposdata::DisposData_State) -> ();

    #[method(name = "get_State4", args = 0)]
    pub fn get_state4(self) -> crate::app::disposdata::DisposData_State;

    #[method(name = "set_State4", args = 1)]
    pub fn set_state4(self, value: crate::app::disposdata::DisposData_State) -> ();

    #[method(name = "get_State5", args = 0)]
    pub fn get_state5(self) -> crate::app::disposdata::DisposData_State;

    #[method(name = "set_State5", args = 1)]
    pub fn set_state5(self, value: crate::app::disposdata::DisposData_State) -> ();

    #[method(name = "get_AI_ActionName", args = 0)]
    pub fn get_ai_action_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_ActionName", args = 1)]
    pub fn set_ai_action_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_ActionVal", args = 0)]
    pub fn get_ai_action_val(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_ActionVal", args = 1)]
    pub fn set_ai_action_val(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_MindName", args = 0)]
    pub fn get_ai_mind_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_MindName", args = 1)]
    pub fn set_ai_mind_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_MindVal", args = 0)]
    pub fn get_ai_mind_val(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_MindVal", args = 1)]
    pub fn set_ai_mind_val(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_AttackName", args = 0)]
    pub fn get_ai_attack_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_AttackName", args = 1)]
    pub fn set_ai_attack_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_AttackVal", args = 0)]
    pub fn get_ai_attack_val(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_AttackVal", args = 1)]
    pub fn set_ai_attack_val(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_MoveName", args = 0)]
    pub fn get_ai_move_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_MoveName", args = 1)]
    pub fn set_ai_move_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_MoveVal", args = 0)]
    pub fn get_ai_move_val(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_MoveVal", args = 1)]
    pub fn set_ai_move_val(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_BattleRate", args = 0)]
    pub fn get_ai_battle_rate(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_BattleRate", args = 1)]
    pub fn set_ai_battle_rate(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_Priority", args = 0)]
    pub fn get_ai_priority(self) -> u8;

    #[method(name = "set_AI_Priority", args = 1)]
    pub fn set_ai_priority(self, value: u8) -> ();

    #[method(name = "get_AI_HealRateA", args = 0)]
    pub fn get_ai_heal_rate_a(self) -> i8;

    #[method(name = "set_AI_HealRateA", args = 1)]
    pub fn set_ai_heal_rate_a(self, value: i8) -> ();

    #[method(name = "get_AI_HealRateB", args = 0)]
    pub fn get_ai_heal_rate_b(self) -> i8;

    #[method(name = "set_AI_HealRateB", args = 1)]
    pub fn set_ai_heal_rate_b(self, value: i8) -> ();

    #[method(name = "get_AI_BandNo", args = 0)]
    pub fn get_ai_band_no(self) -> u32;

    #[method(name = "set_AI_BandNo", args = 1)]
    pub fn set_ai_band_no(self, value: u32) -> ();

    #[method(name = "get_AI_MoveLimit", args = 0)]
    pub fn get_ai_move_limit(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AI_MoveLimit", args = 1)]
    pub fn set_ai_move_limit(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AI_Flag", args = 0)]
    pub fn get_ai_flag(self) -> crate::app::disposdata::DisposData_AIFlagField;

    #[method(name = "set_AI_Flag", args = 1)]
    pub fn set_ai_flag(self, value: crate::app::disposdata::DisposData_AIFlagField) -> ();

    #[method(name = "get_LevelMin", args = 0)]
    pub fn get_level_min(self) -> u8;

    #[method(name = "set_LevelMin", args = 1)]
    pub fn set_level_min(self, value: u8) -> ();

    #[method(name = "get_LevelMax", args = 0)]
    pub fn get_level_max(self) -> u8;

    #[method(name = "set_LevelMax", args = 1)]
    pub fn set_level_max(self, value: u8) -> ();

    #[method(name = "get_Force", args = 0)]
    pub fn get_force(self) -> i8;

    #[method(name = "set_Force", args = 1)]
    pub fn set_force(self, value: i8) -> ();

    #[method(name = "get_ForceType", args = 0)]
    pub fn get_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "GetPerson", args = 0)]
    pub fn get_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "GetTerrain", args = 0)]
    pub fn get_terrain(self) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "GetJob", args = 0)]
    pub fn get_job(self) -> crate::app::jobdata::JobData;

    #[method(name = "GetItem", args = 1)]
    pub fn get_item(self, index: i32) -> crate::app::itemdata::ItemData;

    #[method(name = "AddUnitItem", args = 1)]
    pub fn add_unit_item(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "GetLevel", args = 1)]
    pub fn get_level(self, difficulty: crate::app::difficulty::Difficulty) -> i32;

    #[method(name = "GetLevel", args = 0)]
    pub fn get_level_2(self) -> i32;

    #[method(name = "GetVariableName", args = 1)]
    pub fn get_variable_name(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "HasPosition", args = 0)]
    pub fn has_position(self) -> bool;

    #[method(name = "CanDispos", args = 0)]
    pub fn can_dispos(self) -> bool;

    #[method(name = "CanTerrain", args = 0)]
    pub fn can_terrain(self) -> bool;

    #[method(name = "CanUsePosition", args = 0)]
    pub fn can_use_position(self) -> bool;

    #[method(name = "IsDifficulty", args = 0)]
    pub fn is_difficulty(self) -> bool;

    #[method(name = "ReplaceBattleRate", args = 0)]
    pub fn replace_battle_rate(self) -> crate::app::unitai::UnitAI_BattleRate;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnRelease", args = 0)]
    pub fn on_release(self) -> ();

    #[method(name = "DbgValidateState", args = 0)]
    pub fn dbg_validate_state(self) -> ();
}

#[cfg(feature = "app-disposdata")]
impl DisposData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisposData),
                ::core::stringify!(new),
            )
        });
        <Self as IDisposDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/disposdata/DisposData_AIFlagField.md")))]
#[::unity2::class(namespace = "App", name = "DisposData.AIFlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: disposdata :: DisposData_AIFlags >)]
pub struct DisposData_AIFlagField {}

#[cfg(feature = "app-disposdata")]
#[::unity2::methods]
impl DisposData_AIFlagField {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, f: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, f: crate::app::disposdata::DisposData_AIFlags) -> ();

    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::disposdata::DisposData_AIFlags) -> i32;
}

#[cfg(feature = "app-disposdata")]
impl DisposData_AIFlagField {
    pub fn new(f: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisposData_AIFlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IDisposData_AIFlagFieldMethods>::ctor(this, f);
        this
    }

    pub fn new_2(f: crate::app::disposdata::DisposData_AIFlags) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisposData_AIFlagField),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDisposData_AIFlagFieldMethods>::ctor_2(this, f);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/disposdata/DisposData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DisposData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DisposData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DisposData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DisposData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DisposData_Flags {
    pub fn normal() -> Self {
        Self { value: 1 }
    }

    pub fn hard() -> Self {
        Self { value: 2 }
    }

    pub fn lunatic() -> Self {
        Self { value: 4 }
    }

    pub fn create() -> Self {
        Self { value: 8 }
    }

    pub fn leader() -> Self {
        Self { value: 16 }
    }

    pub fn not_move() -> Self {
        Self { value: 32 }
    }

    pub fn edge() -> Self {
        Self { value: 64 }
    }

    pub fn pos() -> Self {
        Self { value: 128 }
    }

    pub fn must() -> Self {
        Self { value: 256 }
    }

    pub fn fix() -> Self {
        Self { value: 512 }
    }

    pub fn guest() -> Self {
        Self { value: 1024 }
    }

    pub fn mask_sortie() -> Self {
        Self { value: 896 }
    }

    pub fn mask_difficulty() -> Self {
        Self { value: 7 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/disposdata/DisposData_AIFlags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DisposData_AIFlags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DisposData_AIFlags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DisposData.AIFlags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DisposData_AIFlags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DisposData_AIFlags {
    pub fn not_activate_by_attacked() -> Self {
        Self { value: 1 }
    }

    pub fn dummy() -> Self {
        Self { value: 2 }
    }

    pub fn zero_attack() -> Self {
        Self { value: 4 }
    }

    pub fn heal() -> Self {
        Self { value: 8 }
    }

    pub fn r#break() -> Self {
        Self { value: 16 }
    }

    pub fn chain() -> Self {
        Self { value: 32 }
    }

    pub fn equip_short_after_long_range() -> Self {
        Self { value: 64 }
    }

    pub fn move_break() -> Self {
        Self { value: 128 }
    }

    pub fn engage_attack_once() -> Self {
        Self { value: 256 }
    }
}
