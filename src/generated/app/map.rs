
use crate::app::dynamicmesh::DynamicMesh;
use crate::app::dynamicmesh::IDynamicMesh;
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/map/Map_CellVertex.md")))]
#[::unity2::class(namespace = "App", name = "Map.CellVertex")]
#[parent(crate::system::object::Object)]
pub struct Map_CellVertex {
    #[rename(name = "m_Position")]
    pub m_position: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
}

#[cfg(feature = "app-map")]
#[::unity2::methods]
impl Map_CellVertex {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Calc", args = 2)]
    pub fn calc(self, x: i32, z: i32) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, i: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, i: i32, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_MinHeight", args = 0)]
    pub fn get_min_height(self) -> f32;

    #[method(name = "get_MaxHeight", args = 0)]
    pub fn get_max_height(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-map")]
impl Map_CellVertex {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Map_CellVertex),
                ::core::stringify!(new),
            )
        });
        <Self as IMap_CellVertexMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/map/Map_FillList.md")))]
#[::unity2::class(namespace = "App", name = "Map.FillList")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: app :: map :: Map_Pos >)]
pub struct Map_FillList {}

#[cfg(feature = "app-map")]
#[::unity2::methods]
impl Map_FillList {
    #[method(name = "FillTerrain", args = 4)]
    pub fn fill_terrain(self, x: i32, z: i32, before: i32, after: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-map")]
impl Map_FillList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Map_FillList),
                ::core::stringify!(new),
            )
        });
        <Self as IMap_FillListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/map/Map.md")))]
#[::unity2::class(namespace = "App", name = "Map")]
#[parent(crate::system::object::Object)]
pub struct Map {
    #[static_field]
    #[rename(name = "Layer_CameraTransparent")]
    pub layer_camera_transparent: i32,
    #[static_field]
    #[rename(name = "LOD_COUNT")]
    pub lod_count: i32,
    #[static_field]
    #[rename(name = "CELL_SIZE")]
    pub cell_size: f32,
    #[static_field]
    #[rename(name = "CELL_INV")]
    pub cell_inv: f32,
    #[static_field]
    #[rename(name = "CELL_HALF")]
    pub cell_half: f32,
    #[static_field]
    #[rename(name = "CELL_QUATER")]
    pub cell_quater: f32,
    #[static_field]
    #[rename(name = "CELL_OCTER")]
    pub cell_octer: f32,
    #[static_field]
    #[rename(name = "CELL_OFFSET")]
    pub cell_offset: f32,
    #[static_field]
    #[rename(name = "CELL_SLOPE")]
    pub cell_slope: f32,
    #[static_field]
    #[rename(name = "CELL_SIZE3")]
    pub cell_size3: crate::unity_engine::vector3::Vector3,
    #[static_field]
    #[rename(name = "RAY_RADUIS")]
    pub ray_raduis: f32,
    #[static_field]
    #[rename(name = "RAY_OFFSET")]
    pub ray_offset: f32,
    #[static_field]
    #[rename(name = "RAY_OFFSETS")]
    pub ray_offsets: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[static_field]
    #[rename(name = "CELL_OFFSETS")]
    pub cell_offsets: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[static_field]
    #[rename(name = "OFFSET_RATIO")]
    pub offset_ratio: f32,
    #[static_field]
    #[rename(name = "s_EventList")]
    pub s_event_list:
        crate::system::collections::generic::list_1::List_1<crate::app::mapobject::MapObject>,
    #[static_field]
    #[rename(name = "s_EventHits")]
    pub s_event_hits: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
    #[static_field]
    #[rename(name = "AttributeSeparator")]
    pub attribute_separator: ::unity2::Array<u16>,
    #[static_field]
    #[rename(name = "RenderLayerMask")]
    pub render_layer_mask: i32,
    #[static_field]
    #[rename(name = "ColliderLayerMask")]
    pub collider_layer_mask: i32,
    #[static_field]
    #[rename(name = "AttributeLayerMask")]
    pub attribute_layer_mask: i32,
    #[static_field]
    #[rename(name = "RAY_HEIGHT_EVENT")]
    pub ray_height_event: f32,
    #[static_field]
    #[rename(name = "RAY_HEIGHT_GROUND")]
    pub ray_height_ground: f32,
    #[static_field]
    #[rename(name = "RAY_DISTANCE_GROUND")]
    pub ray_distance_ground: f32,
    #[static_field]
    #[rename(name = "s_UpdateBinder")]
    pub s_update_binder: crate::app::bindholder::BindHolder,
}

#[cfg(feature = "app-map")]
#[::unity2::methods]
impl Map {
    #[method(name = "get_GridAlpha", args = 0)]
    pub fn get_grid_alpha() -> f32;

    #[method(name = "set_GridAlpha", args = 1)]
    pub fn set_grid_alpha(value: f32) -> ();

    #[method(name = "get_TerrAlpha", args = 0)]
    pub fn get_terr_alpha() -> f32;

    #[method(name = "set_TerrAlpha", args = 1)]
    pub fn set_terr_alpha(value: f32) -> ();

    #[method(name = "get_ShowLayer", args = 0)]
    pub fn get_show_layer() -> bool;

    #[method(name = "set_ShowLayer", args = 1)]
    pub fn set_show_layer(value: bool) -> ();

    #[method(name = "get_ShowOverlap", args = 0)]
    pub fn get_show_overlap() -> bool;

    #[method(name = "set_ShowOverlap", args = 1)]
    pub fn set_show_overlap(value: bool) -> ();

    #[method(name = "GetHeight", args = 1)]
    pub fn get_height(pos: crate::unity_engine::vector3::Vector3) -> f32;

    #[method(name = "GetHeight", args = 2)]
    pub fn get_height_2(x: f32, z: f32) -> f32;

    #[method(name = "GetUnder", args = 1)]
    pub fn get_under(pos: crate::unity_engine::vector3::Vector3) -> f32;

    #[method(name = "GetUnder", args = 2)]
    pub fn get_under_2(x: f32, z: f32) -> f32;

    #[method(name = "GetEdge", args = 3)]
    pub fn get_edge(x: i32, z: i32, dir: crate::app::dir_2::Dir_Type) -> f32;

    #[method(name = "IsConnect", args = 3)]
    pub fn is_connect(x: i32, z: i32, dir: crate::app::dir_2::Dir_Type) -> bool;

    #[method(name = "CellToWorld", args = 1)]
    pub fn cell_to_world(v: i32) -> f32;

    #[method(name = "WorldToCell", args = 1)]
    pub fn world_to_cell(v: f32) -> i32;

    #[method(name = "WorldToCenter", args = 1)]
    pub fn world_to_center(v: f32) -> f32;

    #[method(name = "CellToWorld", args = 2)]
    pub fn cell_to_world_2(x: i32, z: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SetTerrain", args = 4)]
    pub fn set_terrain(x: i32, z: i32, tid: ::unity2::Il2CppString, do_update: bool) -> ();

    #[method(name = "UpdateUnitPosition", args = 0)]
    pub fn update_unit_position() -> ();

    #[method(name = "UpdatePosition", args = 0)]
    pub fn update_position() -> ();

    #[method(name = "UpdateTerrain", args = 0)]
    pub fn update_terrain() -> ();

    #[method(name = "ApplyPreview", args = 0)]
    pub fn apply_preview() -> ();

    #[method(name = "HasEventObjects", args = 2)]
    pub fn has_event_objects(x: i32, z: i32) -> bool;

    #[method(name = "HasEventObject", args = 3)]
    pub fn has_event_object(x: i32, z: i32, kind: crate::app::mapobject::MapObject_Kinds) -> bool;

    #[method(name = "GetEventObject", args = 3)]
    pub fn get_event_object(
        x: i32,
        z: i32,
        kind: crate::app::mapobject::MapObject_Kinds,
    ) -> crate::app::mapobject::MapObject;

    #[method(name = "GetEventObjects", args = 2)]
    pub fn get_event_objects(
        x: i32,
        z: i32,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::mapobject::MapObject>;

    #[method(name = "GetActionImpl", args = 2)]
    pub fn get_action_impl(x: i32, z: i32) -> crate::app::mapobject::MapObject_Actions;

    #[method(name = "PlayActionImpl", args = 3)]
    pub fn play_action_impl(x: i32, z: i32, action: crate::app::mapobject::MapObject_Actions)
        -> ();

    #[method(name = "ResetActionImpl", args = 3)]
    pub fn reset_action_impl(
        x: i32,
        z: i32,
        action: crate::app::mapobject::MapObject_Actions,
    ) -> ();

    #[method(name = "PlayAction", args = 3)]
    pub fn play_action(x: i32, z: i32, action: crate::app::mapobject::MapObject_Actions) -> ();

    #[method(name = "PlayActionMove", args = 5)]
    pub fn play_action_move(
        x: i32,
        z: i32,
        moved_x: i32,
        moved_z: i32,
        action: crate::app::mapobject::MapObject_Actions,
    ) -> ();

    #[method(name = "GetStateImpl", args = 2)]
    pub fn get_state_impl(x: i32, z: i32) -> i32;

    #[method(name = "SetStateImpl", args = 4)]
    pub fn set_state_impl(x: i32, z: i32, state: i32, is_play: bool) -> ();

    #[method(name = "SetState", args = 3)]
    pub fn set_state(x: i32, z: i32, state: i32) -> ();

    #[method(name = "PlayState", args = 3)]
    pub fn play_state(x: i32, z: i32, state: i32) -> ();

    #[method(name = "ResetAction", args = 3)]
    pub fn reset_action(x: i32, z: i32, action: crate::app::mapobject::MapObject_Actions) -> ();

    #[method(name = "IsPlaying", args = 2)]
    pub fn is_playing(x: i32, z: i32) -> bool;

    #[method(name = "PlayEndurance", args = 4)]
    pub fn play_endurance(x: i32, z: i32, hp: i32, max_hp: i32) -> ();

    #[method(name = "PlayEnduranceForRewind", args = 4)]
    pub fn play_endurance_for_rewind(x: i32, z: i32, hp: i32, max_hp: i32) -> ();

    #[method(name = "PlayEnduranceImpl", args = 3)]
    pub fn play_endurance_impl(x: i32, z: i32, ratio: f32) -> ();

    #[method(name = "PlayAroundRoofImpl", args = 3)]
    pub fn play_around_roof_impl(
        x: i32,
        z: i32,
        action: crate::app::mapobject::MapObject_Actions,
    ) -> ();

    #[method(name = "CanLinkable", args = 1)]
    pub fn can_linkable(terrain: crate::app::terraindata_2::TerrainData_2) -> bool;

    #[method(name = "TryChangeTerrain", args = 2)]
    pub fn try_change_terrain(x: i32, z: i32) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "TryResumeTerrain", args = 2)]
    pub fn try_resume_terrain(x: i32, z: i32) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "Open", args = 2)]
    pub fn open(x: i32, z: i32) -> ();

    #[method(name = "Close", args = 2)]
    pub fn close(x: i32, z: i32) -> ();

    #[method(name = "Broken", args = 2)]
    pub fn broken(x: i32, z: i32) -> ();

    #[method(name = "ResetBroken", args = 2)]
    pub fn reset_broken(x: i32, z: i32) -> ();

    #[method(name = "TorchImpl", args = 4)]
    pub fn torch_impl(x: i32, z: i32, enable: bool, warning: bool) -> ();

    #[method(name = "TorchImpl", args = 2)]
    pub fn torch_impl_2(x: i32, z: i32) -> ();

    #[method(name = "UpdateTorch", args = 0)]
    pub fn update_torch() -> ();

    #[method(name = "SetTerrain", args = 4)]
    pub fn set_terrain_2(x: i32, z: i32, index: i32, do_update: bool) -> ();

    #[method(name = "FillTerrain", args = 3)]
    pub fn fill_terrain(
        x: i32,
        z: i32,
        tid: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::map::Map_Pos>;

    #[method(name = "GetGroundAttribute", args = 1)]
    pub fn get_ground_attribute(
        pos: crate::unity_engine::vector3::Vector3,
    ) -> crate::app::groundattribute::GroundAttribute;

    #[method(name = "GetGroundAttribute", args = 2)]
    pub fn get_ground_attribute_2(
        pos: crate::unity_engine::vector3::Vector3,
        layer_mask: i32,
    ) -> crate::app::groundattribute::GroundAttribute;

    #[method(name = "GetGroundAttribute", args = 5)]
    pub fn get_ground_attribute_3(
        origin: crate::unity_engine::vector3::Vector3,
        radius: f32,
        direction: crate::unity_engine::vector3::Vector3,
        max_distance: f32,
        layer_mask: i32,
    ) -> crate::app::groundattribute::GroundAttribute;

    #[method(name = "GetGroundAttribute", args = 1)]
    pub fn get_ground_attribute_4(
        hit: crate::unity_engine::raycasthit::RaycastHit,
    ) -> crate::app::groundattribute::GroundAttribute;

    #[method(name = "CanEnterTerrain", args = 3)]
    pub fn can_enter_terrain(unit: crate::app::unit::Unit, x: i32, z: i32) -> bool;

    #[method(name = "GetAttributeName", args = 1)]
    pub fn get_attribute_name(
        hit: crate::unity_engine::raycasthit::RaycastHit,
    ) -> ::unity2::Il2CppString;

    #[method(name = "SetEnable", args = 4)]
    pub fn set_enable(
        transform: crate::unity_engine::transform::Transform,
        enable: bool,
        render_mask: i32,
        collider_mask: i32,
    ) -> ();

    #[method(name = "SetRenderEnable", args = 2)]
    pub fn set_render_enable(
        transform: crate::unity_engine::transform::Transform,
        enable: bool,
    ) -> ();

    #[method(name = "SetColliderEnable", args = 2)]
    pub fn set_collider_enable(
        transform: crate::unity_engine::transform::Transform,
        enable: bool,
    ) -> ();

    #[method(name = "GetRange", args = 4)]
    pub fn get_range(x1: i32, z1: i32, x2: i32, z2: i32) -> i32;

    #[method(name = "GetRange", args = 2)]
    pub fn get_range_2(unit: crate::app::unit::Unit, target: crate::app::unit::Unit) -> i32;

    #[method(name = "GetRange", args = 3)]
    pub fn get_range_3(x: i32, z: i32, target: crate::app::unit::Unit) -> i32;

    #[method(name = "IsRange", args = 6)]
    pub fn is_range(
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
        near: i32,
        far: i32,
    ) -> bool;

    #[method(name = "IsRange", args = 3)]
    pub fn is_range_2(range: i32, near: i32, far: i32) -> bool;

    #[method(name = "IsRange", args = 4)]
    pub fn is_range_3(
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        near: i32,
        far: i32,
    ) -> bool;

    #[method(name = "IsRange", args = 8)]
    pub fn is_range_4(
        attack_unit_x: i32,
        attack_unit_z: i32,
        attack_unit_size: i32,
        target_unit_x: i32,
        target_unit_z: i32,
        target_unit_size: i32,
        near: i32,
        far: i32,
    ) -> bool;

    #[method(name = "GetRangePos", args = 8)]
    pub fn get_range_pos(
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        near: i32,
        far: i32,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
    ) -> bool;

    #[method(name = "GetRangePos", args = 12)]
    pub fn get_range_pos_2(
        attack_unit_x: i32,
        attack_unit_z: i32,
        attack_unit_size: i32,
        target_unit_x: i32,
        target_unit_z: i32,
        target_unit_size: i32,
        near: i32,
        far: i32,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
    ) -> bool;

    #[method(name = "GetNearestPos", args = 6)]
    pub fn get_nearest_pos(
        attack: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
    ) -> bool;

    #[method(name = "GetNearestPos", args = 8)]
    pub fn get_nearest_pos_2(
        attack: crate::app::unit::Unit,
        attack_unit_x: i32,
        attack_unit_z: i32,
        target: crate::app::unit::Unit,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
    ) -> bool;

    #[method(name = "GetNearestPos", args = 8)]
    pub fn get_nearest_pos_3(
        attack: crate::app::unit::Unit,
        attack_unit_x: i32,
        attack_unit_z: i32,
        target: crate::app::pokeinspector::PokeInspector,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
    ) -> bool;

    #[method(name = "GetNearestPos", args = 10)]
    pub fn get_nearest_pos_4(
        attack_unit_x: i32,
        attack_unit_z: i32,
        attack_unit_size: i32,
        target_unit_x: i32,
        target_unit_z: i32,
        target_unit_size: i32,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
    ) -> bool;

    #[method(name = "GetNearestPos", args = 12)]
    pub fn get_nearest_pos_5(
        attack_pos_x: i32,
        attack_pos_z: i32,
        attack_size_w: i32,
        attack_size_h: i32,
        target_pos_x: i32,
        target_pos_z: i32,
        target_size_w: i32,
        target_size_h: i32,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
    ) -> bool;

    #[method(name = "GetTargetPos", args = 11)]
    pub fn get_target_pos(
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        target: crate::app::unit::Unit,
        near: i32,
        far: i32,
        command_skill: crate::app::skilldata::SkillData,
        attack_x: i32,
        attack_z: i32,
        target_x: i32,
        target_z: i32,
    ) -> bool;

    #[method(name = "TestTerrainFlag", args = 3)]
    pub fn test_terrain_flag(
        x: i32,
        z: i32,
        flags: crate::app::terraindata_2::TerrainData_Flags,
    ) -> bool;

    #[method(name = "GetCannonObject", args = 2)]
    pub fn get_cannon_object(x: i32, z: i32) -> crate::app::mapobject::MapObject;

    #[method(name = "GetCannonObject", args = 1)]
    pub fn get_cannon_object_2(
        pos: crate::unity_engine::vector3::Vector3,
    ) -> crate::app::mapobject::MapObject;

    #[method(name = "HasBreakable", args = 2)]
    pub fn has_breakable(x: i32, z: i32) -> bool;

    #[method(name = "GetBreakableRect", args = 6)]
    pub fn get_breakable_rect(x: i32, z: i32, x1: i32, z1: i32, x2: i32, z2: i32) -> bool;

    #[method(name = "RegistGlobalKey", args = 0)]
    pub fn regist_global_key() -> ();

    #[method(name = "CommitGlobalKey", args = 0)]
    pub fn commit_global_key() -> ();

    #[method(name = "BreakableCenterPosition", args = 2)]
    pub fn breakable_center_position(x: i32, z: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetUnderPos", args = 2)]
    pub fn get_under_pos(x: i32, z: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetOverPos", args = 2)]
    pub fn get_over_pos(x: i32, z: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetEffectPos", args = 2)]
    pub fn get_effect_pos(x: i32, z: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SetMaterialFloat", args = 4)]
    pub fn set_material_float(
        name: ::unity2::Il2CppString,
        material: ::unity2::Il2CppString,
        property: ::unity2::Il2CppString,
        value: f32,
    ) -> ();

    #[method(name = "SetMaterialColor", args = 4)]
    pub fn set_material_color(
        name: ::unity2::Il2CppString,
        material: ::unity2::Il2CppString,
        property: ::unity2::Il2CppString,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "UpdateBindBegin", args = 0)]
    pub fn update_bind_begin() -> ();

    #[method(name = "UpdateBindEnd", args = 0)]
    pub fn update_bind_end() -> ();

    #[method(name = "IsUpdateBind", args = 0)]
    pub fn is_update_bind() -> bool;

    #[method(name = "TryCreateOverlap", args = 4)]
    pub fn try_create_overlap(
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        tid: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "TryCreateOverlap", args = 4)]
    pub fn try_create_overlap_2(
        unit: crate::app::unit::Unit,
        x: i32,
        z: i32,
        terrain: crate::app::terraindata_2::TerrainData_2,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-map")]
impl Map {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Map),
                ::core::stringify!(new),
            )
        });
        <Self as IMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/map/Map_Pos.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Map_Pos {
    pub x: i32,
    pub z: i32,
}

impl ::unity2::ClassIdentity for Map_Pos {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Map.Pos";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Map_Pos {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-map")]
#[::unity2::methods(value)]
impl Map_Pos {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, x: i32, z: i32) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/map/Map_CellMesh.md")))]
#[::unity2::class(namespace = "App", name = "Map.CellMesh")]
#[parent(crate::app::dynamicmesh::DynamicMesh)]
pub struct Map_CellMesh {
    #[rename(name = "m_CellVertex")]
    pub m_cell_vertex: crate::app::map::Map_CellVertex,
}

#[cfg(feature = "app-map")]
#[::unity2::methods]
impl Map_CellMesh {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        go: crate::unity_engine::gameobject::GameObject,
        sub_mesh_count: i32,
        vertex_capacity: i32,
    ) -> ();

    #[method(name = "AddTangents5", args = 1)]
    pub fn add_tangents5(self, tangent: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "AddCell", args = 4)]
    pub fn add_cell(
        self,
        x: i32,
        z: i32,
        color: crate::unity_engine::color::Color,
        distance: f32,
    ) -> ();

    #[method(name = "AddCell", args = 7)]
    pub fn add_cell_2(
        self,
        x: i32,
        z: i32,
        color: crate::unity_engine::color::Color,
        uv0: crate::unity_engine::vector2::Vector2,
        uv2: crate::unity_engine::vector2::Vector2,
        rotation: i32,
        distance: f32,
    ) -> ();

    #[method(name = "AddCell", args = 5)]
    pub fn add_cell_3(
        self,
        x: i32,
        z: i32,
        uv0: crate::unity_engine::vector2::Vector2,
        uv2: crate::unity_engine::vector2::Vector2,
        distance: f32,
    ) -> ();

    #[method(name = "AddCell", args = 7)]
    pub fn add_cell_4(
        self,
        x: i32,
        z: i32,
        color0: crate::unity_engine::color::Color,
        color1: crate::unity_engine::color::Color,
        color2: crate::unity_engine::color::Color,
        color3: crate::unity_engine::color::Color,
        distance: f32,
    ) -> ();

    #[method(name = "AddCell", args = 9)]
    pub fn add_cell_5(
        self,
        x: i32,
        z: i32,
        color0: crate::unity_engine::color::Color,
        color1: crate::unity_engine::color::Color,
        color2: crate::unity_engine::color::Color,
        color3: crate::unity_engine::color::Color,
        uv0: crate::unity_engine::vector2::Vector2,
        uv2: crate::unity_engine::vector2::Vector2,
        distance: f32,
    ) -> ();

    #[method(name = "AddCell", args = 7)]
    pub fn add_cell_6(
        self,
        x: i32,
        z: i32,
        dir: crate::app::dir_2::Dir_Type,
        color: crate::unity_engine::color::Color,
        uv0: crate::unity_engine::vector2::Vector2,
        uv2: crate::unity_engine::vector2::Vector2,
        distance: f32,
    ) -> ();

    #[method(name = "AddStraddleCell", args = 3)]
    pub fn add_straddle_cell(
        self,
        unit: crate::app::unit::Unit,
        color: crate::unity_engine::color::Color,
        uv_rotate: f32,
    ) -> ();

    #[method(name = "AddEventCell", args = 3)]
    pub fn add_event_cell(self, x: i32, z: i32, color: crate::unity_engine::color::Color) -> ();
}

#[cfg(feature = "app-map")]
impl Map_CellMesh {
    pub fn new(
        go: crate::unity_engine::gameobject::GameObject,
        sub_mesh_count: i32,
        vertex_capacity: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Map_CellMesh),
                ::core::stringify!(new),
            )
        });
        <Self as IMap_CellMeshMethods>::ctor(this, go, sub_mesh_count, vertex_capacity);
        this
    }
}
