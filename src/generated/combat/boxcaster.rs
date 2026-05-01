
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/boxcaster/BoxCaster.md")))]
#[::unity2::class(namespace = "Combat", name = "BoxCaster")]
#[parent(crate::system::object::Object)]
pub struct BoxCaster {
    #[rename(name = "BoxHalfXZ")]
    pub box_half_xz: f32,
    #[rename(name = "BoxHalfY")]
    pub box_half_y: f32,
    #[rename(name = "BoxFlotingHeight")]
    pub box_floting_height: f32,
    #[rename(name = "IsFlying")]
    pub is_flying: bool,
    #[rename(name = "in_Pos0")]
    pub in_pos0: crate::unity_engine::vector3::Vector3,
    #[rename(name = "in_Pos1")]
    pub in_pos1: crate::unity_engine::vector3::Vector3,
    #[rename(name = "Dir2D")]
    pub dir2_d: crate::combat::fxz::FXZ,
    #[rename(name = "WorldPos0")]
    pub world_pos0: crate::unity_engine::vector3::Vector3,
    #[rename(name = "WorldPos1")]
    pub world_pos1: crate::unity_engine::vector3::Vector3,
    #[rename(name = "Dir3D")]
    pub dir3_d: crate::unity_engine::vector3::Vector3,
    #[rename(name = "Result0")]
    pub result0: crate::combat::boxcaster::BoxCaster_CastResult,
    #[rename(name = "Result1")]
    pub result1: crate::combat::boxcaster::BoxCaster_CastResult,
}

#[cfg(feature = "combat-boxcaster")]
#[::unity2::methods]
impl BoxCaster {
    #[method(name = "get_HalfBoxSize", args = 0)]
    pub fn get_half_box_size(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_HalfBoxSize", args = 1)]
    pub fn set_half_box_size(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_TryCount", args = 0)]
    pub fn get_try_count(self) -> i32;

    #[method(name = "set_TryCount", args = 1)]
    pub fn set_try_count(self, value: i32) -> ();

    #[method(name = "get_Center2D", args = 0)]
    pub fn get_center2_d(self) -> crate::combat::fxz::FXZ;

    #[method(name = "get_Center3D", args = 0)]
    pub fn get_center3_d(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_DebugColor", args = 0)]
    pub fn get_debug_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_DebugColor", args = 1)]
    pub fn set_debug_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_Right", args = 0)]
    pub fn get_right(self) -> crate::combat::fxz::FXZ;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, pos0: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(
        self,
        pos0: crate::unity_engine::vector3::Vector3,
        pos1: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "MakeParams", args = 0)]
    pub fn make_params(self) -> ();

    #[method(name = "Cast", args = 0)]
    pub fn cast(self) -> ();

    #[method(name = "CastOne", args = 3)]
    pub fn cast_one(
        self,
        mask: i32,
        hit0: crate::unity_engine::raycasthit::RaycastHit,
        hit1: crate::unity_engine::raycasthit::RaycastHit,
    ) -> bool;

    #[method(name = "RayCast", args = 3)]
    pub fn ray_cast(
        self,
        hit_info: crate::unity_engine::raycasthit::RaycastHit,
        mask: i32,
        max_length: f32,
    ) -> bool;

    #[method(name = "WillCollide", args = 2)]
    pub fn will_collide(
        self,
        rotation: crate::unity_engine::quaternion::Quaternion,
        mask: i32,
    ) -> bool;

    #[method(name = "DebugCastTrajectory", args = 4)]
    pub fn debug_cast_trajectory(
        self,
        world_pos: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
        forward: crate::unity_engine::quaternion::Quaternion,
        distance: f32,
    ) -> ();

    #[method(name = "DrawGizmos", args = 0)]
    pub fn draw_gizmos(self) -> ();

    #[method(name = "IsDividedBy", args = 1)]
    pub fn is_divided_by(self, mask: i32) -> bool;

    #[method(name = "Pullup", args = 0)]
    pub fn pullup(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "combat-boxcaster")]
impl BoxCaster {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BoxCaster),
                ::core::stringify!(new),
            )
        });
        <Self as IBoxCasterMethods>::ctor(this);
        this
    }

    pub fn new_2(pos0: crate::unity_engine::vector3::Vector3) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BoxCaster),
                ::core::stringify!(new_2),
            )
        });
        <Self as IBoxCasterMethods>::ctor_2(this, pos0);
        this
    }

    pub fn new_3(
        pos0: crate::unity_engine::vector3::Vector3,
        pos1: crate::unity_engine::vector3::Vector3,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BoxCaster),
                ::core::stringify!(new_3),
            )
        });
        <Self as IBoxCasterMethods>::ctor_3(this, pos0, pos1);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/boxcaster/BoxCaster_CastResult.md")))]
#[::unity2::class(namespace = "Combat", name = "BoxCaster.CastResult")]
#[parent(crate::system::object::Object)]
pub struct BoxCaster_CastResult {
    #[rename(name = "startPos")]
    pub start_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "endPos")]
    pub end_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "dir")]
    pub dir: crate::unity_engine::vector3::Vector3,
    #[rename(name = "sizeXZ")]
    pub size_xz: f32,
    #[rename(name = "sizeY")]
    pub size_y: f32,
    #[rename(name = "forward")]
    pub forward: crate::unity_engine::quaternion::Quaternion,
    #[rename(name = "NumHits")]
    pub num_hits: i32,
    #[rename(name = "Hits")]
    pub hits: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
}

#[cfg(feature = "combat-boxcaster")]
#[::unity2::methods]
impl BoxCaster_CastResult {
    #[method(name = "get_centerPos", args = 0)]
    pub fn get_center_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, setting_xz: f32, setting_y: f32) -> ();

    #[method(name = "Cast", args = 4)]
    pub fn cast(
        self,
        start_pos: crate::unity_engine::vector3::Vector3,
        end_pos: crate::unity_engine::vector3::Vector3,
        half_box_size: crate::unity_engine::vector3::Vector3,
        mask: i32,
    ) -> ();

    #[method(name = "IsIntersect", args = 2)]
    pub fn is_intersect(self, pos: crate::combat::fxz::FXZ, radius: f32) -> bool;

    #[method(name = "IsDividedBy", args = 1)]
    pub fn is_divided_by(self, mask: i32) -> bool;

    #[method(name = "DrawGizmos", args = 0)]
    pub fn draw_gizmos(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "combat-boxcaster")]
impl BoxCaster_CastResult {
    pub fn new(setting_xz: f32, setting_y: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BoxCaster_CastResult),
                ::core::stringify!(new),
            )
        });
        <Self as IBoxCaster_CastResultMethods>::ctor(this, setting_xz, setting_y);
        this
    }
}
