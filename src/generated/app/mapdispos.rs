
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdispos/MapDispos.md")))]
#[::unity2::class(namespace = "App", name = "MapDispos")]
#[parent(crate::system::object::Object)]
pub struct MapDispos {}

#[cfg(feature = "app-mapdispos")]
#[::unity2::methods]
impl MapDispos {
    #[method(name = "Load", args = 1)]
    pub fn load(file_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "CreateFirst", args = 0)]
    pub fn create_first() -> ();

    #[method(name = "CreateFirstEncount", args = 0)]
    pub fn create_first_encount() -> ();

    #[method(name = "GetEncountPlayeGroup", args = 1)]
    pub fn get_encount_playe_group(player_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetEncountMobGroup", args = 2)]
    pub fn get_encount_mob_group(player_index: i32, mob_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetEncountTerrainGroup", args = 1)]
    pub fn get_encount_terrain_group(player_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetEncountPlayerCount", args = 0)]
    pub fn get_encount_player_count() -> i32;

    #[method(name = "GetEncountMobCount", args = 1)]
    pub fn get_encount_mob_count(player_index: i32) -> i32;

    #[method(name = "GetFakeIndex", args = 3)]
    pub fn get_fake_index(index: i32, key: ::unity2::Il2CppString, count: i32) -> i32;

    #[method(name = "CreateFirstChallenge", args = 0)]
    pub fn create_first_challenge() -> ();

    #[method(name = "CreateFirstDlcGod", args = 0)]
    pub fn create_first_dlc_god() -> ();

    #[method(name = "CreateFirstVersusCasual", args = 0)]
    pub fn create_first_versus_casual() -> ();

    #[method(name = "CreateFirstVersusRanked", args = 0)]
    pub fn create_first_versus_ranked() -> ();

    #[method(name = "CreateFirstVersusMock", args = 0)]
    pub fn create_first_versus_mock() -> ();

    #[method(name = "CreateVersusOpponentTeam", args = 1)]
    pub fn create_versus_opponent_team(edit_data: crate::app::mapeditdata::MapEditData) -> ();

    #[method(name = "CreateVersusPlayerTeam", args = 1)]
    pub fn create_versus_player_team(edit_data: crate::app::mapeditdata::MapEditData) -> ();

    #[method(name = "CreateFirstEdit", args = 0)]
    pub fn create_first_edit() -> ();

    #[method(name = "CreateVersusEditTeam", args = 1)]
    pub fn create_versus_edit_team(edit_data: crate::app::mapeditdata::MapEditData) -> ();

    #[method(name = "ResetSortiePositionForEdit", args = 1)]
    pub fn reset_sortie_position_for_edit(
        pos_stack: crate::system::collections::generic::stack_1::Stack_1<
            crate::app::mapdispos::MapDispos_Pos,
        >,
    ) -> ();

    #[method(name = "CreateProcess", args = 4)]
    pub fn create_process(
        super_: crate::app::procinst::ProcInst,
        group: ::unity2::Il2CppString,
        dispos_flag: crate::app::mapdispos::MapDispos_Flag,
        position_group: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "IsWaitProcess", args = 0)]
    pub fn is_wait_process() -> bool;

    #[method(name = "GetDebugHeader", args = 0)]
    pub fn get_debug_header() -> ::unity2::Il2CppString;

    #[method(name = "CreatePlayerTeam", args = 1)]
    pub fn create_player_team(group: ::unity2::Il2CppString) -> ();

    #[method(name = "TryCreateDisposTeam", args = 1)]
    pub fn try_create_dispos_team(group: ::unity2::Il2CppString) -> ();

    #[method(name = "CreateDisposTeam", args = 2)]
    pub fn create_dispos_team(
        group: ::unity2::Il2CppString,
        position_group: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateEncountDisposTeam", args = 1)]
    pub fn create_encount_dispos_team(group: ::unity2::Il2CppString) -> ();

    #[method(name = "CreateChallengeDisposTeam", args = 2)]
    pub fn create_challenge_dispos_team(
        group: ::unity2::Il2CppString,
        random: crate::app::random_2::Random_2,
    ) -> ();

    #[method(name = "CreateDlcGodDisposTeam", args = 2)]
    pub fn create_dlc_god_dispos_team(
        group: ::unity2::Il2CppString,
        position_group: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreateUnit", args = 1)]
    pub fn create_unit(data: crate::app::disposdata::DisposData) -> crate::app::unit::Unit;

    #[method(name = "CreateEncountTerrain", args = 1)]
    pub fn create_encount_terrain(player_index: i32) -> ();

    #[method(name = "TryCreateTerrain", args = 1)]
    pub fn try_create_terrain(group: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetForceAngle", args = 2)]
    pub fn get_force_angle(
        unit: crate::app::unit::Unit,
        dir: crate::app::disposdata::DisposData_Directions,
    ) -> f32;

    #[method(name = "GetForceAngle", args = 2)]
    pub fn get_force_angle_2(
        force: crate::app::force::Force_Type,
        dir: crate::app::disposdata::DisposData_Directions,
    ) -> f32;

    #[method(name = "SetupUnit", args = 5)]
    pub fn setup_unit(
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        force_type: crate::app::force::Force_Type,
        dir: crate::app::disposdata::DisposData_Directions,
    ) -> ();

    #[method(name = "GetSortieLimit", args = 0)]
    pub fn get_sortie_limit() -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapdispos")]
impl MapDispos {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDispos),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDisposMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdispos/MapDispos_ProcDispos.md")))]
#[::unity2::class(namespace = "App", name = "MapDispos.ProcDispos")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapDispos_ProcDispos {
    #[static_field]
    #[rename(name = "s_ProcessCount")]
    pub s_process_count: i32,
    #[rename(name = "m_List")]
    pub m_list: crate::app::mapdispos::MapDispos_ActualDataList,
    #[rename(name = "m_DisposFlag")]
    pub m_dispos_flag: crate::app::mapdispos::MapDispos_FlagField,
}

#[cfg(feature = "app-mapdispos")]
#[::unity2::methods]
impl MapDispos_ProcDispos {
    #[method(name = "Create", args = 3)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        list: crate::app::mapdispos::MapDispos_ActualDataList,
        dispos_flag: crate::app::mapdispos::MapDispos_Flag,
    ) -> ();

    #[method(name = "IsExist", args = 0)]
    pub fn is_exist() -> bool;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        list: crate::app::mapdispos::MapDispos_ActualDataList,
        dispos_flag: crate::app::mapdispos::MapDispos_Flag,
    ) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load(self) -> ();

    #[method(name = "WaitLoad", args = 0)]
    pub fn wait_load(self) -> ();

    #[method(name = "Focus", args = 0)]
    pub fn focus(self) -> ();

    #[method(name = "Dispos", args = 0)]
    pub fn dispos(self) -> ();

    #[method(name = "WaitDispos", args = 0)]
    pub fn wait_dispos(self) -> ();

    #[method(name = "InstantIfPossible", args = 2)]
    pub fn instant_if_possible(
        list: crate::app::mapdispos::MapDispos_ActualDataList,
        dispos_flag: crate::app::mapdispos::MapDispos_Flag,
    ) -> bool;

    #[method(name = "CheckHide", args = 1)]
    pub fn check_hide(list: crate::app::mapdispos::MapDispos_ActualDataList) -> bool;

    #[method(name = "CalcFocusPos", args = 3)]
    pub fn calc_focus_pos(
        list: crate::app::mapdispos::MapDispos_ActualDataList,
        focus_x: i32,
        focus_z: i32,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapdispos")]
impl MapDispos_ProcDispos {
    pub fn new(
        list: crate::app::mapdispos::MapDispos_ActualDataList,
        dispos_flag: crate::app::mapdispos::MapDispos_Flag,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDispos_ProcDispos),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDispos_ProcDisposMethods>::ctor(this, list, dispos_flag);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdispos/MapDispos_Pos.md")))]
#[::unity2::class(namespace = "App", name = "MapDispos.Pos")]
#[parent(crate::system::object::Object)]
pub struct MapDispos_Pos {}

#[cfg(feature = "app-mapdispos")]
#[::unity2::methods]
impl MapDispos_Pos {
    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "set_X", args = 1)]
    pub fn set_x(self, value: i32) -> ();

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "set_Z", args = 1)]
    pub fn set_z(self, value: i32) -> ();

    #[method(name = "get_Dir", args = 0)]
    pub fn get_dir(self) -> crate::app::disposdata::DisposData_Directions;

    #[method(name = "set_Dir", args = 1)]
    pub fn set_dir(self, value: crate::app::disposdata::DisposData_Directions) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapdispos")]
impl MapDispos_Pos {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDispos_Pos),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDispos_PosMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdispos/MapDispos_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "MapDispos.FlagField")]
#[parent(crate::app::bitfield32::BitField32)]
pub struct MapDispos_FlagField {}

#[cfg(feature = "app-mapdispos")]
#[::unity2::methods]
impl MapDispos_FlagField {
    #[method(name = "Set", args = 1)]
    pub fn set(self, f: crate::app::mapdispos::MapDispos_Flag) -> ();

    #[method(name = "Test", args = 1)]
    pub fn test(self, f: crate::app::mapdispos::MapDispos_Flag) -> bool;

    #[method(name = "Not", args = 1)]
    pub fn not(self, f: crate::app::mapdispos::MapDispos_Flag) -> bool;

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, f: crate::app::mapdispos::MapDispos_Flag) -> ();

    #[method(name = "Reset", args = 1)]
    pub fn reset(self, f: crate::app::mapdispos::MapDispos_Flag) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapdispos")]
impl MapDispos_FlagField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDispos_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDispos_FlagFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdispos/MapDispos_ActualDataList.md")))]
#[::unity2::class(namespace = "App", name = "MapDispos.ActualDataList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: mapdispos :: MapDispos_ActualData >)]
pub struct MapDispos_ActualDataList {}

#[cfg(feature = "app-mapdispos")]
#[::unity2::methods]
impl MapDispos_ActualDataList {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::disposdata::DisposData,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::disposdata::DisposData,
        >,
        position_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::disposdata::DisposData,
        >,
    ) -> ();

    #[method(name = "Calc", args = 1)]
    pub fn calc(self, dispos_flag: crate::app::mapdispos::MapDispos_Flag) -> bool;

    #[method(name = "Filter", args = 1)]
    pub fn filter(
        self,
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::disposdata::DisposData,
        >,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::disposdata::DisposData>;
}

#[cfg(feature = "app-mapdispos")]
impl MapDispos_ActualDataList {
    pub fn new(
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::disposdata::DisposData,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDispos_ActualDataList),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDispos_ActualDataListMethods>::ctor(this, data_list);
        this
    }

    pub fn new_2(
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::disposdata::DisposData,
        >,
        position_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::disposdata::DisposData,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDispos_ActualDataList),
                ::core::stringify!(new_2),
            )
        });
        <Self as IMapDispos_ActualDataListMethods>::ctor_2(this, data_list, position_data_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdispos/MapDispos_ActualData.md")))]
#[::unity2::class(namespace = "App", name = "MapDispos.ActualData")]
#[parent(crate::system::object::Object)]
pub struct MapDispos_ActualData {
    #[rename(name = "m_Data")]
    pub m_data: crate::app::disposdata::DisposData,
    #[rename(name = "m_PositionData")]
    pub m_position_data: crate::app::disposdata::DisposData,
    #[rename(name = "m_CalcResult")]
    pub m_calc_result: crate::app::mapdispos::MapDispos_ActualData_CalcResults,
    #[rename(name = "m_UnitIndex")]
    pub m_unit_index: i32,
    #[rename(name = "m_CalcAppearX")]
    pub m_calc_appear_x: i32,
    #[rename(name = "m_CalcAppearZ")]
    pub m_calc_appear_z: i32,
    #[rename(name = "m_CalcDisposX")]
    pub m_calc_dispos_x: i32,
    #[rename(name = "m_CalcDisposZ")]
    pub m_calc_dispos_z: i32,
    #[rename(name = "m_Direction")]
    pub m_direction: crate::app::disposdata::DisposData_Directions,
}

#[cfg(feature = "app-mapdispos")]
#[::unity2::methods]
impl MapDispos_ActualData {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        data: crate::app::disposdata::DisposData,
        position_data: crate::app::disposdata::DisposData,
    ) -> ();

    #[method(name = "Calc", args = 1)]
    pub fn calc(self, dispos_flag: crate::app::mapdispos::MapDispos_Flag) -> bool;

    #[method(name = "UnitMove", args = 4)]
    pub fn unit_move(
        self,
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        move_flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<
            crate::app::mapdeploy::MapDeploy,
        >,
    ) -> ();

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::app::disposdata::DisposData;

    #[method(name = "get_DataDisposX", args = 0)]
    pub fn get_data_dispos_x(self) -> i32;

    #[method(name = "get_DataDisposZ", args = 0)]
    pub fn get_data_dispos_z(self) -> i32;

    #[method(name = "get_CalcResult", args = 0)]
    pub fn get_calc_result(self) -> crate::app::mapdispos::MapDispos_ActualData_CalcResults;

    #[method(name = "get_UnitIndex", args = 0)]
    pub fn get_unit_index(self) -> i32;

    #[method(name = "get_CalcAppearX", args = 0)]
    pub fn get_calc_appear_x(self) -> i32;

    #[method(name = "get_CalcAppearZ", args = 0)]
    pub fn get_calc_appear_z(self) -> i32;

    #[method(name = "get_CalcDisposX", args = 0)]
    pub fn get_calc_dispos_x(self) -> i32;

    #[method(name = "get_CalcDisposZ", args = 0)]
    pub fn get_calc_dispos_z(self) -> i32;

    #[method(name = "get_DisposDir", args = 0)]
    pub fn get_dispos_dir(self) -> crate::app::disposdata::DisposData_Directions;

    #[method(name = "CalcDisposXZ", args = 3)]
    pub fn calc_dispos_xz(self, unit: crate::app::unit::Unit, dispos_x: i32, dispos_z: i32)
        -> bool;

    #[method(name = "CalcApperXZ", args = 5)]
    pub fn calc_apper_xz(
        self,
        unit: crate::app::unit::Unit,
        dispos_x: i32,
        dispos_z: i32,
        appear_x: i32,
        appear_z: i32,
    ) -> bool;

    #[method(name = "IsForced", args = 1)]
    pub fn is_forced(self, flag: crate::app::mapdispos::MapDispos_Flag) -> bool;

    #[method(name = "IsEnable", args = 1)]
    pub fn is_enable(self, flag: crate::app::mapdispos::MapDispos_Flag) -> bool;

    #[method(name = "IsCreate", args = 0)]
    pub fn is_create(self) -> bool;

    #[method(name = "GetUnit", args = 1)]
    pub fn get_unit(
        self,
        dispos_flag: crate::app::mapdispos::MapDispos_Flag,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetTerrain", args = 2)]
    pub fn get_terrain(self, x: i32, z: i32) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "IsOutOfPlayArea", args = 3)]
    pub fn is_out_of_play_area(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "IsUnitExist", args = 3)]
    pub fn is_unit_exist(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "IsTerrainImmobile", args = 2)]
    pub fn is_terrain_immobile(self, x: i32, z: i32) -> bool;

    #[method(name = "IsTerrainImmobile", args = 3)]
    pub fn is_terrain_immobile_2(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "UnitAIMoveLimitForBigUnit", args = 2)]
    pub fn unit_ai_move_limit_for_big_unit(
        self,
        unit: crate::app::unit::Unit,
        move_flag: crate::app::mapdeploytemplate_1::MapDeployTemplate_1_Flag<
            crate::app::mapdeploy::MapDeploy,
        >,
    ) -> ();

    #[method(name = "get_DataPerson", args = 0)]
    pub fn get_data_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "get_DataForceType", args = 0)]
    pub fn get_data_force_type(self) -> crate::app::force::Force_Type;

    #[method(name = "get_DataAppearX", args = 0)]
    pub fn get_data_appear_x(self) -> i32;

    #[method(name = "get_DataAppearZ", args = 0)]
    pub fn get_data_appear_z(self) -> i32;
}

#[cfg(feature = "app-mapdispos")]
impl MapDispos_ActualData {
    pub fn new(
        data: crate::app::disposdata::DisposData,
        position_data: crate::app::disposdata::DisposData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDispos_ActualData),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDispos_ActualDataMethods>::ctor(this, data, position_data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdispos/MapDispos_Flag.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapDispos_Flag {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapDispos_Flag {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapDispos.Flag";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapDispos_Flag {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapDispos_Flag {
    pub fn focus() -> Self {
        Self { value: 1 }
    }

    pub fn focus_unit() -> Self {
        Self { value: 2 }
    }

    pub fn forced() -> Self {
        Self { value: 4 }
    }

    pub fn warp() -> Self {
        Self { value: 8 }
    }

    pub fn not_forced() -> Self {
        Self { value: 16 }
    }

    pub fn loose() -> Self {
        Self { value: 32 }
    }

    pub fn fixed_speed() -> Self {
        Self { value: 65536 }
    }

    pub fn dead() -> Self {
        Self { value: 131072 }
    }

    pub fn status_fixed() -> Self {
        Self { value: 262144 }
    }

    pub fn suppress_warning() -> Self {
        Self { value: 524288 }
    }

    pub fn instant() -> Self {
        Self { value: 1048576 }
    }

    pub fn bind() -> Self {
        Self { value: 2097152 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdispos/MapDispos_PosList.md")))]
#[::unity2::class(namespace = "App", name = "MapDispos.PosList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: mapdispos :: MapDispos_Pos >)]
pub struct MapDispos_PosList {}

#[cfg(feature = "app-mapdispos")]
#[::unity2::methods]
impl MapDispos_PosList {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Register", args = 2)]
    pub fn register(
        self,
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::disposdata::DisposData,
        >,
        limit: i32,
    ) -> ();

    #[method(name = "PopFront", args = 0)]
    pub fn pop_front(self) -> crate::app::mapdispos::MapDispos_Pos;

    #[method(name = "TryRemove", args = 1)]
    pub fn try_remove(self, data: crate::app::disposdata::DisposData) -> bool;
}

#[cfg(feature = "app-mapdispos")]
impl MapDispos_PosList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapDispos_PosList),
                ::core::stringify!(new),
            )
        });
        <Self as IMapDispos_PosListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapdispos/MapDispos_ActualData_CalcResults.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapDispos_ActualData_CalcResults {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapDispos_ActualData_CalcResults {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapDispos.ActualData.CalcResults";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapDispos_ActualData_CalcResults {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapDispos_ActualData_CalcResults {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn success() -> Self {
        Self { value: 1 }
    }

    pub fn failure() -> Self {
        Self { value: 2 }
    }

    pub fn disable() -> Self {
        Self { value: 3 }
    }
}
