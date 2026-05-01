
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimage/MapImage.md")))]
#[::unity2::class(namespace = "App", name = "MapImage")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapimage :: MapImage >)]
pub struct MapImage {
    #[static_field]
    #[rename(name = "MaxWidthBit")]
    pub max_width_bit: i32,
    #[static_field]
    #[rename(name = "MaxHeightBit")]
    pub max_height_bit: i32,
    #[static_field]
    #[rename(name = "MaxWidth")]
    pub max_width: i32,
    #[static_field]
    #[rename(name = "MaxHeight")]
    pub max_height: i32,
    #[static_field]
    #[rename(name = "MaxSize")]
    pub max_size: i32,
    #[static_field]
    #[rename(name = "MaxOuter")]
    pub max_outer: i32,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::mapimageunit::MapImageUnit,
    #[rename(name = "m_Terrain")]
    pub m_terrain: crate::app::mapimageterrain::MapImageTerrain,
    #[rename(name = "m_Cost")]
    pub m_cost: crate::app::mapimagecost::MapImageCost,
    #[rename(name = "m_Danger")]
    pub m_danger: crate::app::mapimagedanger::MapImageDanger,
    #[rename(name = "m_Talk")]
    pub m_talk: crate::app::mapimagetalk::MapImageTalk,
    #[rename(name = "m_Range")]
    pub m_range: crate::app::mapimagerange::MapImageRange,
    #[rename(name = "m_History")]
    pub m_history: crate::app::mapimagehistory::MapImageHistory,
    #[rename(name = "m_BackupTerrains")]
    pub m_backup_terrains: crate::system::collections::generic::list_1::List_1<
        crate::app::mapimage::MapImage_BackupTerrain,
    >,
}

#[cfg(feature = "app-mapimage")]
#[::unity2::methods]
impl MapImage {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetPutTerrain", args = 2)]
    pub fn get_put_terrain(
        tid: ::unity2::Il2CppString,
        nullable: bool,
    ) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "Setup", args = 2)]
    pub fn setup(self, terrain: crate::app::mapterrain::MapTerrain, is_resume: bool) -> ();

    #[method(name = "SetSize", args = 2)]
    pub fn set_size(self, w: i32, h: i32) -> ();

    #[method(name = "IsOutOfMap", args = 2)]
    pub fn is_out_of_map(self, x: i32, z: i32) -> bool;

    #[method(name = "IsOutOfPlayArea", args = 2)]
    pub fn is_out_of_play_area(self, x: i32, z: i32) -> bool;

    #[method(name = "CanTarget", args = 2)]
    pub fn can_target(self, x: i32, z: i32) -> bool;

    #[method(name = "CanTarget", args = 3)]
    pub fn can_target_2(self, x: i32, z: i32, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CanTarget", args = 3)]
    pub fn can_target_3(self, x: i32, z: i32, is_consider_not_target: bool) -> bool;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "get_X", args = 0)]
    pub fn get_x(self) -> i32;

    #[method(name = "get_Z", args = 0)]
    pub fn get_z(self) -> i32;

    #[method(name = "get_W", args = 0)]
    pub fn get_w(self) -> i32;

    #[method(name = "set_W", args = 1)]
    pub fn set_w(self, value: i32) -> ();

    #[method(name = "get_H", args = 0)]
    pub fn get_h(self) -> i32;

    #[method(name = "set_H", args = 1)]
    pub fn set_h(self, value: i32) -> ();

    #[method(name = "get_PlayAreaX", args = 0)]
    pub fn get_play_area_x(self) -> i32;

    #[method(name = "set_PlayAreaX", args = 1)]
    pub fn set_play_area_x(self, value: i32) -> ();

    #[method(name = "get_PlayAreaZ", args = 0)]
    pub fn get_play_area_z(self) -> i32;

    #[method(name = "set_PlayAreaZ", args = 1)]
    pub fn set_play_area_z(self, value: i32) -> ();

    #[method(name = "get_PlayAreaW", args = 0)]
    pub fn get_play_area_w(self) -> i32;

    #[method(name = "set_PlayAreaW", args = 1)]
    pub fn set_play_area_w(self, value: i32) -> ();

    #[method(name = "get_PlayAreaH", args = 0)]
    pub fn get_play_area_h(self) -> i32;

    #[method(name = "set_PlayAreaH", args = 1)]
    pub fn set_play_area_h(self, value: i32) -> ();

    #[method(name = "get_X1", args = 0)]
    pub fn get_x1(self) -> i32;

    #[method(name = "set_X1", args = 1)]
    pub fn set_x1(self, value: i32) -> ();

    #[method(name = "get_Z1", args = 0)]
    pub fn get_z1(self) -> i32;

    #[method(name = "set_Z1", args = 1)]
    pub fn set_z1(self, value: i32) -> ();

    #[method(name = "get_X2", args = 0)]
    pub fn get_x2(self) -> i32;

    #[method(name = "set_X2", args = 1)]
    pub fn set_x2(self, value: i32) -> ();

    #[method(name = "get_Z2", args = 0)]
    pub fn get_z2(self) -> i32;

    #[method(name = "set_Z2", args = 1)]
    pub fn set_z2(self, value: i32) -> ();

    #[method(name = "get_PlayAreaX1", args = 0)]
    pub fn get_play_area_x1(self) -> i32;

    #[method(name = "set_PlayAreaX1", args = 1)]
    pub fn set_play_area_x1(self, value: i32) -> ();

    #[method(name = "get_PlayAreaZ1", args = 0)]
    pub fn get_play_area_z1(self) -> i32;

    #[method(name = "set_PlayAreaZ1", args = 1)]
    pub fn set_play_area_z1(self, value: i32) -> ();

    #[method(name = "get_PlayAreaX2", args = 0)]
    pub fn get_play_area_x2(self) -> i32;

    #[method(name = "set_PlayAreaX2", args = 1)]
    pub fn set_play_area_x2(self, value: i32) -> ();

    #[method(name = "get_PlayAreaZ2", args = 0)]
    pub fn get_play_area_z2(self) -> i32;

    #[method(name = "set_PlayAreaZ2", args = 1)]
    pub fn set_play_area_z2(self, value: i32) -> ();

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::mapimageunit::MapImageUnit;

    #[method(name = "get_Terrain", args = 0)]
    pub fn get_terrain(self) -> crate::app::mapimageterrain::MapImageTerrain;

    #[method(name = "get_Cost", args = 0)]
    pub fn get_cost(self) -> crate::app::mapimagecost::MapImageCost;

    #[method(name = "get_Danger", args = 0)]
    pub fn get_danger(self) -> crate::app::mapimagedanger::MapImageDanger;

    #[method(name = "get_Talk", args = 0)]
    pub fn get_talk(self) -> crate::app::mapimagetalk::MapImageTalk;

    #[method(name = "get_Range", args = 0)]
    pub fn get_range(self) -> crate::app::mapimagerange::MapImageRange;

    #[method(name = "get_History", args = 0)]
    pub fn get_history(self) -> crate::app::mapimagehistory::MapImageHistory;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTargetUnit", args = 2)]
    pub fn get_target_unit(self, x: i32, z: i32) -> crate::app::unit::Unit;

    #[method(name = "GetTargetUnit", args = 3)]
    pub fn get_target_unit_2(
        self,
        x: i32,
        z: i32,
        allow_asphyxiation: bool,
    ) -> crate::app::unit::Unit;

    #[method(name = "ClampX", args = 1)]
    pub fn clamp_x(self, x: i32) -> i32;

    #[method(name = "ClampZ", args = 1)]
    pub fn clamp_z(self, z: i32) -> i32;
}

#[cfg(feature = "app-mapimage")]
impl MapImage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImage),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimage/MapImage_BackupTerrain.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MapImage_BackupTerrain {
    pub x: i32,
    pub z: i32,
    pub hash: i32,
    pub index: i32,
}

impl ::unity2::ClassIdentity for MapImage_BackupTerrain {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapImage.BackupTerrain";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapImage_BackupTerrain {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
