
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/parabola/Parabola.md")))]
#[::unity2::class(namespace = "Combat", name = "Parabola")]
#[parent(crate::system::object::Object)]
pub struct Parabola {}

#[cfg(feature = "combat-parabola")]
#[::unity2::methods]
impl Parabola {
    #[method(name = "get_in_Speed", args = 0)]
    pub fn get_in_speed(self) -> f32;

    #[method(name = "set_in_Speed", args = 1)]
    pub fn set_in_speed(self, value: f32) -> ();

    #[method(name = "get_in_Mass", args = 0)]
    pub fn get_in_mass(self) -> f32;

    #[method(name = "set_in_Mass", args = 1)]
    pub fn set_in_mass(self, value: f32) -> ();

    #[method(name = "get_in_StartPos", args = 0)]
    pub fn get_in_start_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_in_StartPos", args = 1)]
    pub fn set_in_start_pos(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_in_EndPos", args = 0)]
    pub fn get_in_end_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_in_EndPos", args = 1)]
    pub fn set_in_end_pos(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_in_WeaponLength", args = 0)]
    pub fn get_in_weapon_length(self) -> f32;

    #[method(name = "set_in_WeaponLength", args = 1)]
    pub fn set_in_weapon_length(self, value: f32) -> ();

    #[method(name = "get_out_FlyingTime", args = 0)]
    pub fn get_out_flying_time(self) -> f32;

    #[method(name = "set_out_FlyingTime", args = 1)]
    pub fn set_out_flying_time(self, value: f32) -> ();

    #[method(name = "get_out_InitialVelocity", args = 0)]
    pub fn get_out_initial_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_out_InitialVelocity", args = 1)]
    pub fn set_out_initial_velocity(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_out_WorldHitDir", args = 0)]
    pub fn get_out_world_hit_dir(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_out_WorldHitDir", args = 1)]
    pub fn set_out_world_hit_dir(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_WorldCentroidOffset", args = 0)]
    pub fn get_world_centroid_offset(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Calculate", args = 0)]
    pub fn calculate(self) -> ();

    #[method(name = "CalcHitDir", args = 0)]
    pub fn calc_hit_dir(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "RecalcInitialVelocity", args = 1)]
    pub fn recalc_initial_velocity(self, flying_time: f32) -> ();

    #[method(name = "Kmph_mps", args = 1)]
    pub fn kmph_mps(kmph: f32) -> f32;

    #[method(name = "CalculateTrajectory", args = 3)]
    pub fn calculate_trajectory(
        time: f32,
        mass: f32,
        velocity: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CalculateTrajectoryVelocity2D", args = 2)]
    pub fn calculate_trajectory_velocity2_d(
        target: crate::unity_engine::vector2::Vector2,
        power: f32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "CalculateTrajectoryVelocity", args = 3)]
    pub fn calculate_trajectory_velocity(
        target: crate::unity_engine::vector3::Vector3,
        power: f32,
        time_to_hit: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CalculateTrajectoryVelocityByTime", args = 2)]
    pub fn calculate_trajectory_velocity_by_time(
        target: crate::unity_engine::vector3::Vector3,
        time: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-parabola")]
impl Parabola {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Parabola),
                ::core::stringify!(new),
            )
        });
        <Self as IParabolaMethods>::ctor(this);
        this
    }
}
