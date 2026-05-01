
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terraincostdata/TerrainCostData.md")))]
#[::unity2::class(namespace = "App", name = "TerrainCostData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: terraincostdata :: TerrainCostData >)]
pub struct TerrainCostData {
    #[rename(name = "Costs")]
    pub costs: ::unity2::Array<u8>,
}

#[cfg(feature = "app-terraincostdata")]
#[::unity2::methods]
impl TerrainCostData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_None", args = 0)]
    pub fn get_none(self) -> u8;

    #[method(name = "set_None", args = 1)]
    pub fn set_none(self, value: u8) -> ();

    #[method(name = "get_Foot", args = 0)]
    pub fn get_foot(self) -> u8;

    #[method(name = "set_Foot", args = 1)]
    pub fn set_foot(self, value: u8) -> ();

    #[method(name = "get_Horse", args = 0)]
    pub fn get_horse(self) -> u8;

    #[method(name = "set_Horse", args = 1)]
    pub fn set_horse(self, value: u8) -> ();

    #[method(name = "get_Fly", args = 0)]
    pub fn get_fly(self) -> u8;

    #[method(name = "set_Fly", args = 1)]
    pub fn set_fly(self, value: u8) -> ();

    #[method(name = "get_Dragon", args = 0)]
    pub fn get_dragon(self) -> u8;

    #[method(name = "set_Dragon", args = 1)]
    pub fn set_dragon(self, value: u8) -> ();

    #[method(name = "get_Pad", args = 0)]
    pub fn get_pad(self) -> u8;

    #[method(name = "set_Pad", args = 1)]
    pub fn set_pad(self, value: u8) -> ();

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

    #[method(name = "get_ColorA", args = 0)]
    pub fn get_color_a(self) -> u8;

    #[method(name = "set_ColorA", args = 1)]
    pub fn set_color_a(self, value: u8) -> ();

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_Color", args = 1)]
    pub fn set_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetCost", args = 3)]
    pub fn get_cost(move_type: crate::app::jobdata::JobData_MoveTypes, x: i32, z: i32) -> i32;

    #[method(name = "GetCost", args = 3)]
    pub fn get_cost_2(unit: crate::app::unit::Unit, x: i32, z: i32) -> i32;

    #[method(name = "GetCostWithoutOverlap", args = 3)]
    pub fn get_cost_without_overlap(
        move_type: crate::app::jobdata::JobData_MoveTypes,
        x: i32,
        z: i32,
    ) -> i32;

    #[method(name = "IsNoMove", args = 3)]
    pub fn is_no_move(unit: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "IsNoMove", args = 3)]
    pub fn is_no_move_2(move_type: crate::app::jobdata::JobData_MoveTypes, x: i32, z: i32) -> bool;

    #[method(name = "IsNoMove", args = 1)]
    pub fn is_no_move_3(cost: i32) -> bool;
}

#[cfg(feature = "app-terraincostdata")]
impl TerrainCostData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainCostData),
                ::core::stringify!(new),
            )
        });
        <Self as ITerrainCostDataMethods>::ctor(this);
        this
    }
}
