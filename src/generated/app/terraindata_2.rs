
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terraindata_2/TerrainData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TerrainData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TerrainData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TerrainData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TerrainData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TerrainData_Flags {
    pub fn door() -> Self {
        Self { value: 1 }
    }

    pub fn treasure() -> Self {
        Self { value: 2 }
    }

    pub fn visit() -> Self {
        Self { value: 4 }
    }

    pub fn bow_cannon() -> Self {
        Self { value: 8 }
    }

    pub fn magic_cannon() -> Self {
        Self { value: 16 }
    }

    pub fn fire_cannon() -> Self {
        Self { value: 32 }
    }

    pub fn no_shadow() -> Self {
        Self { value: 128 }
    }

    pub fn foot_smoke() -> Self {
        Self { value: 256 }
    }

    pub fn foot_print() -> Self {
        Self { value: 512 }
    }

    pub fn roof() -> Self {
        Self { value: 1024 }
    }

    pub fn sight_masking() -> Self {
        Self { value: 2048 }
    }

    pub fn not_stun() -> Self {
        Self { value: 4096 }
    }

    pub fn not_engage_add() -> Self {
        Self { value: 8192 }
    }

    pub fn fly_enable() -> Self {
        Self { value: 16384 }
    }

    pub fn engage_heal() -> Self {
        Self { value: 32768 }
    }

    pub fn not_target() -> Self {
        Self { value: 65536 }
    }

    pub fn not_warp() -> Self {
        Self { value: 131072 }
    }

    pub fn damage_half_display() -> Self {
        Self { value: 262144 }
    }

    pub fn hide_break_icon() -> Self {
        Self { value: 524288 }
    }

    pub fn show_phase_icon() -> Self {
        Self { value: 1048576 }
    }

    pub fn immobile() -> Self {
        Self { value: 536870912 }
    }

    pub fn minimap() -> Self {
        Self { value: 1073741824 }
    }

    pub fn help_spot() -> Self {
        Self { value: -2147483648 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terraindata_2/TerrainData_Destroyers.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TerrainData_Destroyers {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TerrainData_Destroyers {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TerrainData.Destroyers";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TerrainData_Destroyers {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TerrainData_Destroyers {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn player() -> Self {
        Self { value: 1 }
    }

    pub fn enemy() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terraindata_2/TerrainData_Commands.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TerrainData_Commands {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TerrainData_Commands {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TerrainData.Commands";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TerrainData_Commands {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TerrainData_Commands {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn torch_on() -> Self {
        Self { value: 1 }
    }

    pub fn torch_off() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terraindata_2/TerrainData_Prohibitions.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TerrainData_Prohibitions {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TerrainData_Prohibitions {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TerrainData.Prohibitions";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TerrainData_Prohibitions {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TerrainData_Prohibitions {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn all() -> Self {
        Self { value: 1 }
    }

    pub fn ground() -> Self {
        Self { value: 2 }
    }

    pub fn near() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terraindata_2/TerrainData_Layers.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TerrainData_Layers {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TerrainData_Layers {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TerrainData.Layers";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TerrainData_Layers {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TerrainData_Layers {
    pub fn lower() -> Self {
        Self { value: 0 }
    }

    pub fn upper() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terraindata_2/TerrainData_2.md")))]
#[::unity2::class(namespace = "App", name = "TerrainData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: terraindata_2 :: TerrainData_2 >)]
pub struct TerrainData_2 {}

#[cfg(feature = "app-terraindata_2")]
#[::unity2::methods]
impl TerrainData_2 {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Tid", args = 0)]
    pub fn get_tid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Tid", args = 1)]
    pub fn set_tid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CostName", args = 0)]
    pub fn get_cost_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CostName", args = 1)]
    pub fn set_cost_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CostType", args = 0)]
    pub fn get_cost_type(self) -> i32;

    #[method(name = "set_CostType", args = 1)]
    pub fn set_cost_type(self, value: i32) -> ();

    #[method(name = "get_Layer", args = 0)]
    pub fn get_layer(self) -> crate::app::terraindata_2::TerrainData_Layers;

    #[method(name = "set_Layer", args = 1)]
    pub fn set_layer(self, value: crate::app::terraindata_2::TerrainData_Layers) -> ();

    #[method(name = "get_Prohibition", args = 0)]
    pub fn get_prohibition(self) -> crate::app::terraindata_2::TerrainData_Prohibitions;

    #[method(name = "set_Prohibition", args = 1)]
    pub fn set_prohibition(self, value: crate::app::terraindata_2::TerrainData_Prohibitions) -> ();

    #[method(name = "get_Command", args = 0)]
    pub fn get_command(self) -> crate::app::terraindata_2::TerrainData_Commands;

    #[method(name = "set_Command", args = 1)]
    pub fn set_command(self, value: crate::app::terraindata_2::TerrainData_Commands) -> ();

    #[method(name = "get_Sight", args = 0)]
    pub fn get_sight(self) -> u8;

    #[method(name = "set_Sight", args = 1)]
    pub fn set_sight(self, value: u8) -> ();

    #[method(name = "get_Destroyer", args = 0)]
    pub fn get_destroyer(self) -> crate::app::terraindata_2::TerrainData_Destroyers;

    #[method(name = "set_Destroyer", args = 1)]
    pub fn set_destroyer(self, value: crate::app::terraindata_2::TerrainData_Destroyers) -> ();

    #[method(name = "get_Hp_N", args = 0)]
    pub fn get_hp_n(self) -> i32;

    #[method(name = "set_Hp_N", args = 1)]
    pub fn set_hp_n(self, value: i32) -> ();

    #[method(name = "get_Hp_H", args = 0)]
    pub fn get_hp_h(self) -> i32;

    #[method(name = "set_Hp_H", args = 1)]
    pub fn set_hp_h(self, value: i32) -> ();

    #[method(name = "get_Hp_L", args = 0)]
    pub fn get_hp_l(self) -> i32;

    #[method(name = "set_Hp_L", args = 1)]
    pub fn set_hp_l(self, value: i32) -> ();

    #[method(name = "get_Defense", args = 0)]
    pub fn get_defense(self) -> i8;

    #[method(name = "set_Defense", args = 1)]
    pub fn set_defense(self, value: i8) -> ();

    #[method(name = "get_Avoid", args = 0)]
    pub fn get_avoid(self) -> i8;

    #[method(name = "set_Avoid", args = 1)]
    pub fn set_avoid(self, value: i8) -> ();

    #[method(name = "get_PlayerDefense", args = 0)]
    pub fn get_player_defense(self) -> i8;

    #[method(name = "set_PlayerDefense", args = 1)]
    pub fn set_player_defense(self, value: i8) -> ();

    #[method(name = "get_EnemyDefense", args = 0)]
    pub fn get_enemy_defense(self) -> i8;

    #[method(name = "set_EnemyDefense", args = 1)]
    pub fn set_enemy_defense(self, value: i8) -> ();

    #[method(name = "get_PlayerAvoid", args = 0)]
    pub fn get_player_avoid(self) -> i8;

    #[method(name = "set_PlayerAvoid", args = 1)]
    pub fn set_player_avoid(self, value: i8) -> ();

    #[method(name = "get_EnemyAvoid", args = 0)]
    pub fn get_enemy_avoid(self) -> i8;

    #[method(name = "set_EnemyAvoid", args = 1)]
    pub fn set_enemy_avoid(self, value: i8) -> ();

    #[method(name = "get_Heal", args = 0)]
    pub fn get_heal(self) -> i8;

    #[method(name = "set_Heal", args = 1)]
    pub fn set_heal(self, value: i8) -> ();

    #[method(name = "get_Life", args = 0)]
    pub fn get_life(self) -> u8;

    #[method(name = "set_Life", args = 1)]
    pub fn set_life(self, value: u8) -> ();

    #[method(name = "get_MoveCost", args = 0)]
    pub fn get_move_cost(self) -> u8;

    #[method(name = "set_MoveCost", args = 1)]
    pub fn set_move_cost(self, value: u8) -> ();

    #[method(name = "get_FlyCost", args = 0)]
    pub fn get_fly_cost(self) -> u8;

    #[method(name = "set_FlyCost", args = 1)]
    pub fn set_fly_cost(self, value: u8) -> ();

    #[method(name = "get_MoveFirst", args = 0)]
    pub fn get_move_first(self) -> i8;

    #[method(name = "set_MoveFirst", args = 1)]
    pub fn set_move_first(self, value: i8) -> ();

    #[method(name = "get_Offset", args = 0)]
    pub fn get_offset(self) -> f32;

    #[method(name = "set_Offset", args = 1)]
    pub fn set_offset(self, value: f32) -> ();

    #[method(name = "get_ColorR", args = 0)]
    pub fn get_color_r(self) -> u8;

    #[method(name = "set_ColorR", args = 1)]
    pub fn set_color_r(self, value: u8) -> ();

    #[method(name = "get_ColorG", args = 0)]
    pub fn get_color_g(self) -> u8;

    #[method(name = "set_ColorG", args = 1)]
    pub fn set_color_g(self, value: u8) -> ();

    #[method(name = "get_ColorB", args = 0)]
    pub fn get_color_b(self) -> u8;

    #[method(name = "set_ColorB", args = 1)]
    pub fn set_color_b(self, value: u8) -> ();

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_Color", args = 1)]
    pub fn set_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ChangeTid", args = 0)]
    pub fn get_change_tid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ChangeTid", args = 1)]
    pub fn set_change_tid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ChangeEncount", args = 0)]
    pub fn get_change_encount(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ChangeEncount", args = 1)]
    pub fn set_change_encount(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Height", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "set_Height", args = 1)]
    pub fn set_height(self, value: f32) -> ();

    #[method(name = "get_PutEffect", args = 0)]
    pub fn get_put_effect(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PutEffect", args = 1)]
    pub fn set_put_effect(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Minimap", args = 0)]
    pub fn get_minimap(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Minimap", args = 1)]
    pub fn set_minimap(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CannonSkill", args = 0)]
    pub fn get_cannon_skill(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CannonSkill", args = 1)]
    pub fn set_cannon_skill(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CannonShellsN", args = 0)]
    pub fn get_cannon_shells_n(self) -> u8;

    #[method(name = "set_CannonShellsN", args = 1)]
    pub fn set_cannon_shells_n(self, value: u8) -> ();

    #[method(name = "get_CannonShellsH", args = 0)]
    pub fn get_cannon_shells_h(self) -> u8;

    #[method(name = "set_CannonShellsH", args = 1)]
    pub fn set_cannon_shells_h(self, value: u8) -> ();

    #[method(name = "get_CannonShellsL", args = 0)]
    pub fn get_cannon_shells_l(self) -> u8;

    #[method(name = "set_CannonShellsL", args = 1)]
    pub fn set_cannon_shells_l(self, value: u8) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::terraindata_2::TerrainData_Flags;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::terraindata_2::TerrainData_Flags) -> ();

    #[method(name = "get_PutAllow", args = 0)]
    pub fn get_put_allow(self) -> u8;

    #[method(name = "set_PutAllow", args = 1)]
    pub fn set_put_allow(self, value: u8) -> ();

    #[method(name = "get_AsciiName", args = 0)]
    pub fn get_ascii_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AsciiName", args = 1)]
    pub fn set_ascii_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "IsDoor", args = 0)]
    pub fn is_door(self) -> bool;

    #[method(name = "IsTreasure", args = 0)]
    pub fn is_treasure(self) -> bool;

    #[method(name = "IsVisit", args = 0)]
    pub fn is_visit(self) -> bool;

    #[method(name = "IsBowCannon", args = 0)]
    pub fn is_bow_cannon(self) -> bool;

    #[method(name = "IsMagicCannon", args = 0)]
    pub fn is_magic_cannon(self) -> bool;

    #[method(name = "IsFireCannon", args = 0)]
    pub fn is_fire_cannon(self) -> bool;

    #[method(name = "IsNotShadow", args = 0)]
    pub fn is_not_shadow(self) -> bool;

    #[method(name = "IsFootSmoke", args = 0)]
    pub fn is_foot_smoke(self) -> bool;

    #[method(name = "IsFootprint", args = 0)]
    pub fn is_footprint(self) -> bool;

    #[method(name = "IsRoof", args = 0)]
    pub fn is_roof(self) -> bool;

    #[method(name = "IsSightMasking", args = 0)]
    pub fn is_sight_masking(self) -> bool;

    #[method(name = "IsTorch", args = 0)]
    pub fn is_torch(self) -> bool;

    #[method(name = "IsCannon", args = 0)]
    pub fn is_cannon(self) -> bool;

    #[method(name = "IsNotStun", args = 0)]
    pub fn is_not_stun(self) -> bool;

    #[method(name = "IsNotEngageAdd", args = 0)]
    pub fn is_not_engage_add(self) -> bool;

    #[method(name = "IsFlyEnable", args = 0)]
    pub fn is_fly_enable(self) -> bool;

    #[method(name = "IsHelpSpot", args = 0)]
    pub fn is_help_spot(self) -> bool;

    #[method(name = "IsEngageHeal", args = 0)]
    pub fn is_engage_heal(self) -> bool;

    #[method(name = "IsNotTarget", args = 0)]
    pub fn is_not_target(self) -> bool;

    #[method(name = "IsNotWarp", args = 0)]
    pub fn is_not_warp(self) -> bool;

    #[method(name = "IsHideBreakIcon", args = 0)]
    pub fn is_hide_break_icon(self) -> bool;

    #[method(name = "IsShowPhaseIcon", args = 0)]
    pub fn is_show_phase_icon(self) -> bool;

    #[method(name = "IsImmobile", args = 0)]
    pub fn is_immobile(self) -> bool;

    #[method(name = "IsDamageHalfDisplay", args = 0)]
    pub fn is_damage_half_display(self) -> bool;

    #[method(name = "IsFlightOnly", args = 0)]
    pub fn is_flight_only(self) -> bool;

    #[method(name = "IsBreakable", args = 0)]
    pub fn is_breakable(self) -> bool;

    #[method(name = "CanPutAllow", args = 1)]
    pub fn can_put_allow(self, cost_type: i32) -> bool;

    #[method(name = "CanPutAllow", args = 0)]
    pub fn can_put_allow_2(self) -> bool;

    #[method(name = "CanBreakable", args = 1)]
    pub fn can_breakable(self, force: crate::app::force::Force_Type) -> bool;

    #[method(name = "GetHP", args = 0)]
    pub fn get_hp(self) -> i32;

    #[method(name = "CanUnitCommand", args = 1)]
    pub fn can_unit_command(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "GetCannonShells", args = 0)]
    pub fn get_cannon_shells(self) -> i32;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnCompleted", args = 0)]
    pub fn on_completed(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-terraindata_2")]
impl TerrainData_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainData_2),
                ::core::stringify!(new),
            )
        });
        <Self as ITerrainData_2Methods>::ctor(this);
        this
    }
}
