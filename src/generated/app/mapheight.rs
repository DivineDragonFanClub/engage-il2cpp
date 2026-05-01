
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapheight/MapHeight_CellMap.md")))]
#[::unity2::class(namespace = "App", name = "MapHeight.CellMap")]
#[parent(crate::system::object::Object)]
pub struct MapHeight_CellMap {
    #[rename(name = "m_Cells")]
    pub m_cells: ::unity2::Array<crate::app::mapheight::MapHeight_CellInfo>,
    #[rename(name = "m_LayerMaskA")]
    pub m_layer_mask_a: i32,
    #[rename(name = "m_LayerMaskB")]
    pub m_layer_mask_b: i32,
}

#[cfg(feature = "app-mapheight")]
#[::unity2::methods]
impl MapHeight_CellMap {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, layer_mask_a: i32, layer_mask_b: i32) -> ();

    #[method(name = "GetX", args = 2)]
    pub fn get_x(self, x: i32, index: i32) -> f32;

    #[method(name = "GetZ", args = 2)]
    pub fn get_z(self, z: i32, index: i32) -> f32;

    #[method(name = "GetHeight", args = 3)]
    pub fn get_height(self, x: i32, z: i32, index: i32) -> f32;

    #[method(name = "GetMaxHeight", args = 2)]
    pub fn get_max_height(self, x: i32, z: i32) -> f32;

    #[method(name = "GetMinHeight", args = 2)]
    pub fn get_min_height(self, x: i32, z: i32) -> f32;

    #[method(name = "Get", args = 2)]
    pub fn get(self, x: i32, z: i32) -> crate::app::mapheight::MapHeight_CellInfo;

    #[method(name = "Update", args = 2)]
    pub fn update(self, x: i32, z: i32) -> ();

    #[method(name = "IsLayerMask", args = 3)]
    pub fn is_layer_mask(self, x: i32, z: i32, layer_mask: i32) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}

#[cfg(feature = "app-mapheight")]
impl MapHeight_CellMap {
    pub fn new(layer_mask_a: i32, layer_mask_b: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHeight_CellMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHeight_CellMapMethods>::ctor(this, layer_mask_a, layer_mask_b);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapheight/MapHeight_CellInfo.md")))]
#[::unity2::class(namespace = "App", name = "MapHeight.CellInfo")]
#[parent(crate::system::object::Object)]
pub struct MapHeight_CellInfo {
    #[rename(name = "heights")]
    pub heights: ::unity2::Array<f32>,
    #[rename(name = "normals")]
    pub normals: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "layers")]
    pub layers: ::unity2::Array<i32>,
}

#[cfg(feature = "app-mapheight")]
#[::unity2::methods]
impl MapHeight_CellInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "GetMin", args = 0)]
    pub fn get_min(self) -> f32;

    #[method(name = "GetMax", args = 0)]
    pub fn get_max(self) -> f32;

    #[method(name = "GetDiff", args = 0)]
    pub fn get_diff(self) -> f32;

    #[method(name = "Update", args = 5)]
    pub fn update(self, index: i32, x: f32, z: f32, layer_mask_a: i32, layer_mask_b: i32) -> ();

    #[method(name = "IsFlat", args = 1)]
    pub fn is_flat(self, index: i32) -> bool;

    #[method(name = "IsFlat", args = 0)]
    pub fn is_flat_2(self) -> bool;

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, src: crate::app::mapheight::MapHeight_CellInfo) -> ();
}

#[cfg(feature = "app-mapheight")]
impl MapHeight_CellInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHeight_CellInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHeight_CellInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapheight/MapHeight_Plane.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHeight_Plane {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHeight_Plane {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHeight.Plane";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHeight_Plane {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHeight_Plane {
    pub fn up() -> Self {
        Self { value: 0 }
    }

    pub fn down() -> Self {
        Self { value: 1 }
    }

    pub fn left() -> Self {
        Self { value: 2 }
    }

    pub fn right() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapheight/MapHeight_Layers.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapHeight_Layers {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapHeight_Layers {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapHeight.Layers";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapHeight_Layers {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapHeight_Layers {
    pub fn under() -> Self {
        Self { value: 0 }
    }

    pub fn over() -> Self {
        Self { value: 1 }
    }

    pub fn num() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapheight/MapHeight.md")))]
#[::unity2::class(namespace = "App", name = "MapHeight")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapheight :: MapHeight >)]
pub struct MapHeight {
    #[static_field]
    #[rename(name = "R")]
    pub r: i32,
    #[static_field]
    #[rename(name = "N")]
    pub n: i32,
    #[static_field]
    #[rename(name = "W")]
    pub w: i32,
    #[static_field]
    #[rename(name = "H")]
    pub h: i32,
    #[rename(name = "m_CellMaps")]
    pub m_cell_maps: ::unity2::Array<crate::app::mapheight::MapHeight_CellMap>,
    #[rename(name = "m_LayerMasksA")]
    pub m_layer_masks_a: ::unity2::Array<i32>,
    #[rename(name = "m_LayerMasksB")]
    pub m_layer_masks_b: ::unity2::Array<i32>,
    #[rename(name = "m_LayerMaskOver")]
    pub m_layer_mask_over: i32,
    #[static_field]
    #[rename(name = "ZERO")]
    pub zero: crate::unity_engine::vector3::Vector3,
    #[static_field]
    #[rename(name = "s_EdgeIndexes")]
    pub s_edge_indexes: ::unity2::Array<crate::app::mapheight::MapHeight_EdgeIndex>,
}

#[cfg(feature = "app-mapheight")]
#[::unity2::methods]
impl MapHeight {
    #[method(name = "CalcRayCast", args = 5)]
    pub fn calc_ray_cast(
        hit: crate::unity_engine::raycasthit::RaycastHit,
        x: f32,
        z: f32,
        layer_mask_a: i32,
        layer_mask_b: i32,
    ) -> bool;

    #[method(name = "CalcRayCast", args = 4)]
    pub fn calc_ray_cast_2(x: f32, z: f32, layer_mask_a: i32, layer_mask_b: i32) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "GetCellMap", args = 1)]
    pub fn get_cell_map(
        self,
        layer: crate::app::mapheight::MapHeight_Layers,
    ) -> crate::app::mapheight::MapHeight_CellMap;

    #[method(name = "GetOver", args = 0)]
    pub fn get_over(self) -> crate::app::mapheight::MapHeight_CellMap;

    #[method(name = "GetUnder", args = 0)]
    pub fn get_under(self) -> crate::app::mapheight::MapHeight_CellMap;

    #[method(name = "Update", args = 2)]
    pub fn update(self, x: i32, z: i32) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "Get", args = 2)]
    pub fn get(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        layer: crate::app::mapheight::MapHeight_Layers,
    ) -> f32;

    #[method(name = "GetNormal", args = 2)]
    pub fn get_normal(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        layer: crate::app::mapheight::MapHeight_Layers,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetMaxHeight", args = 2)]
    pub fn get_max_height(self, x: i32, z: i32) -> f32;

    #[method(name = "GetMinHeight", args = 2)]
    pub fn get_min_height(self, x: i32, z: i32) -> f32;

    #[method(name = "GetPlane", args = 2)]
    pub fn get_plane(ox: f32, oz: f32) -> crate::app::mapheight::MapHeight_Plane;

    #[method(name = "IsOutSide", args = 2)]
    pub fn is_out_side(x: i32, z: i32) -> bool;

    #[method(name = "Get", args = 3)]
    pub fn get_2(self, x: f32, z: f32, layer: crate::app::mapheight::MapHeight_Layers) -> f32;

    #[method(name = "Get", args = 4)]
    pub fn get_3(
        self,
        x: i32,
        z: i32,
        index: i32,
        layer: crate::app::mapheight::MapHeight_Layers,
    ) -> f32;

    #[method(name = "GetNormal", args = 3)]
    pub fn get_normal_2(
        self,
        x: f32,
        z: f32,
        layer: crate::app::mapheight::MapHeight_Layers,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetEdge", args = 3)]
    pub fn get_edge(self, x: i32, z: i32, dir: crate::app::dir_2::Dir_Type) -> f32;

    #[method(name = "GetMinEdge", args = 3)]
    pub fn get_min_edge(self, x: i32, z: i32, dir: crate::app::dir_2::Dir_Type) -> f32;

    #[method(name = "GetMaxEdge", args = 3)]
    pub fn get_max_edge(self, x: i32, z: i32, dir: crate::app::dir_2::Dir_Type) -> f32;

    #[method(name = "IsSlope", args = 3)]
    pub fn is_slope(self, x: i32, z: i32, slope: f32) -> bool;

    #[method(name = "GetTiltAngle", args = 2)]
    pub fn get_tilt_angle(self, x: i32, z: i32) -> f32;

    #[method(name = "GetLength_point_plane", args = 3)]
    pub fn get_length_point_plane(
        p: crate::unity_engine::vector3::Vector3,
        d: crate::unity_engine::vector3::Vector3,
        normalized_n: crate::unity_engine::vector3::Vector3,
    ) -> f32;

    #[method(name = "GetLength_point_plane", args = 3)]
    pub fn get_length_point_plane_2(
        p: crate::unity_engine::vector3::Vector3,
        d: f32,
        normalized_n: crate::unity_engine::vector3::Vector3,
    ) -> f32;

    #[method(name = "GetIntersectPt_segment_plane", args = 6)]
    pub fn get_intersect_pt_segment_plane(
        p: crate::unity_engine::vector3::Vector3,
        q: crate::unity_engine::vector3::Vector3,
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        c: crate::unity_engine::vector3::Vector3,
        o_pt: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "GetIntersectPt_segment_triangle", args = 6)]
    pub fn get_intersect_pt_segment_triangle(
        p: crate::unity_engine::vector3::Vector3,
        q: crate::unity_engine::vector3::Vector3,
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        c: crate::unity_engine::vector3::Vector3,
        o_pt: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapheight")]
impl MapHeight {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHeight),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHeightMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapheight/MapHeight_EdgeIndex.md")))]
#[::unity2::class(namespace = "App", name = "MapHeight.EdgeIndex")]
#[parent(crate::system::object::Object)]
pub struct MapHeight_EdgeIndex {
    #[rename(name = "Index1")]
    pub index1: i32,
    #[rename(name = "Index2")]
    pub index2: i32,
}

#[cfg(feature = "app-mapheight")]
#[::unity2::methods]
impl MapHeight_EdgeIndex {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapheight")]
impl MapHeight_EdgeIndex {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapHeight_EdgeIndex),
                ::core::stringify!(new),
            )
        });
        <Self as IMapHeight_EdgeIndexMethods>::ctor(this);
        this
    }
}
