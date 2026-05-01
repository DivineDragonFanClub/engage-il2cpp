
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/mathf/Mathf.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Mathf {}

impl ::unity2::ClassIdentity for Mathf {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Mathf";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Mathf {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-mathf")]
#[::unity2::methods(value)]
impl Mathf {
    #[method(name = "NextPowerOfTwo", args = 1)]
    pub fn next_power_of_two(value: i32) -> i32;

    #[method(name = "GammaToLinearSpace", args = 1)]
    pub fn gamma_to_linear_space(value: f32) -> f32;

    #[method(name = "LinearToGammaSpace", args = 1)]
    pub fn linear_to_gamma_space(value: f32) -> f32;

    #[method(name = "FloatToHalf", args = 1)]
    pub fn float_to_half(val: f32) -> u16;

    #[method(name = "PerlinNoise", args = 2)]
    pub fn perlin_noise(x: f32, y: f32) -> f32;

    #[method(name = "Sin", args = 1)]
    pub fn sin(f: f32) -> f32;

    #[method(name = "Cos", args = 1)]
    pub fn cos(f: f32) -> f32;

    #[method(name = "Tan", args = 1)]
    pub fn tan(f: f32) -> f32;

    #[method(name = "Asin", args = 1)]
    pub fn asin(f: f32) -> f32;

    #[method(name = "Acos", args = 1)]
    pub fn acos(f: f32) -> f32;

    #[method(name = "Atan", args = 1)]
    pub fn atan(f: f32) -> f32;

    #[method(name = "Atan2", args = 2)]
    pub fn atan2(y: f32, x: f32) -> f32;

    #[method(name = "Sqrt", args = 1)]
    pub fn sqrt(f: f32) -> f32;

    #[method(name = "Abs", args = 1)]
    pub fn abs(f: f32) -> f32;

    #[method(name = "Abs", args = 1)]
    pub fn abs_2(value: i32) -> i32;

    #[method(name = "Min", args = 2)]
    pub fn min(a: f32, b: f32) -> f32;

    #[method(name = "Min", args = 1)]
    pub fn min_2(values: ::unity2::Array<f32>) -> f32;

    #[method(name = "Min", args = 2)]
    pub fn min_3(a: i32, b: i32) -> i32;

    #[method(name = "Max", args = 2)]
    pub fn max(a: f32, b: f32) -> f32;

    #[method(name = "Max", args = 1)]
    pub fn max_2(values: ::unity2::Array<f32>) -> f32;

    #[method(name = "Max", args = 2)]
    pub fn max_3(a: i32, b: i32) -> i32;

    #[method(name = "Max", args = 1)]
    pub fn max_4(values: ::unity2::Array<i32>) -> i32;

    #[method(name = "Pow", args = 2)]
    pub fn pow(f: f32, p: f32) -> f32;

    #[method(name = "Exp", args = 1)]
    pub fn exp(power: f32) -> f32;

    #[method(name = "Log", args = 2)]
    pub fn log(f: f32, p: f32) -> f32;

    #[method(name = "Log", args = 1)]
    pub fn log_2(f: f32) -> f32;

    #[method(name = "Ceil", args = 1)]
    pub fn ceil(f: f32) -> f32;

    #[method(name = "Floor", args = 1)]
    pub fn floor(f: f32) -> f32;

    #[method(name = "Round", args = 1)]
    pub fn round(f: f32) -> f32;

    #[method(name = "CeilToInt", args = 1)]
    pub fn ceil_to_int(f: f32) -> i32;

    #[method(name = "FloorToInt", args = 1)]
    pub fn floor_to_int(f: f32) -> i32;

    #[method(name = "RoundToInt", args = 1)]
    pub fn round_to_int(f: f32) -> i32;

    #[method(name = "Sign", args = 1)]
    pub fn sign(f: f32) -> f32;

    #[method(name = "Clamp", args = 3)]
    pub fn clamp(value: f32, min: f32, max: f32) -> f32;

    #[method(name = "Clamp", args = 3)]
    pub fn clamp_2(value: i32, min: i32, max: i32) -> i32;

    #[method(name = "Clamp01", args = 1)]
    pub fn clamp01(value: f32) -> f32;

    #[method(name = "Lerp", args = 3)]
    pub fn lerp(a: f32, b: f32, t: f32) -> f32;

    #[method(name = "LerpUnclamped", args = 3)]
    pub fn lerp_unclamped(a: f32, b: f32, t: f32) -> f32;

    #[method(name = "LerpAngle", args = 3)]
    pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32;

    #[method(name = "MoveTowards", args = 3)]
    pub fn move_towards(current: f32, target: f32, max_delta: f32) -> f32;

    #[method(name = "MoveTowardsAngle", args = 3)]
    pub fn move_towards_angle(current: f32, target: f32, max_delta: f32) -> f32;

    #[method(name = "Approximately", args = 2)]
    pub fn approximately(a: f32, b: f32) -> bool;

    #[method(name = "SmoothDamp", args = 4)]
    pub fn smooth_damp(current: f32, target: f32, current_velocity: f32, smooth_time: f32) -> f32;

    #[method(name = "SmoothDamp", args = 6)]
    pub fn smooth_damp_2(
        current: f32,
        target: f32,
        current_velocity: f32,
        smooth_time: f32,
        max_speed: f32,
        delta_time: f32,
    ) -> f32;

    #[method(name = "SmoothDampAngle", args = 4)]
    pub fn smooth_damp_angle(
        current: f32,
        target: f32,
        current_velocity: f32,
        smooth_time: f32,
    ) -> f32;

    #[method(name = "SmoothDampAngle", args = 6)]
    pub fn smooth_damp_angle_2(
        current: f32,
        target: f32,
        current_velocity: f32,
        smooth_time: f32,
        max_speed: f32,
        delta_time: f32,
    ) -> f32;

    #[method(name = "Repeat", args = 2)]
    pub fn repeat(t: f32, length: f32) -> f32;

    #[method(name = "InverseLerp", args = 3)]
    pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32;

    #[method(name = "DeltaAngle", args = 2)]
    pub fn delta_angle(current: f32, target: f32) -> f32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
