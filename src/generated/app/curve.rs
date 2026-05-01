
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/curve/Curve.md")))]
#[::unity2::class(namespace = "App", name = "Curve")]
#[parent(crate::system::object::Object)]
pub struct Curve {}

#[cfg(feature = "app-curve")]
#[::unity2::methods]
impl Curve {
    #[method(name = "Calc", args = 6)]
    pub fn calc(
        r#type: crate::app::curve::Curve_Type,
        prev: f32,
        next: f32,
        now: f32,
        term: f32,
        num: i32,
    ) -> f32;

    #[method(name = "Lerp", args = 5)]
    pub fn lerp(prev: f32, next: f32, now: f32, term: f32, unused: i32) -> f32;

    #[method(name = "Lerp", args = 5)]
    pub fn lerp_2(
        prev: crate::unity_engine::vector3::Vector3,
        next: crate::unity_engine::vector3::Vector3,
        now: f32,
        term: f32,
        unused: i32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Lerp", args = 5)]
    pub fn lerp_3(
        prev: crate::unity_engine::color::Color,
        next: crate::unity_engine::color::Color,
        now: f32,
        term: f32,
        unused: i32,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "Accel", args = 5)]
    pub fn accel(prev: f32, next: f32, now: f32, term: f32, num: i32) -> f32;

    #[method(name = "Decel", args = 5)]
    pub fn decel(prev: f32, next: f32, now: f32, term: f32, num: i32) -> f32;

    #[method(name = "AcDecel", args = 5)]
    pub fn ac_decel(prev: f32, next: f32, now: f32, term: f32, num: i32) -> f32;

    #[method(name = "DecAccel", args = 5)]
    pub fn dec_accel(prev: f32, next: f32, now: f32, term: f32, num: i32) -> f32;

    #[method(name = "LerpDecel", args = 5)]
    pub fn lerp_decel(prev: f32, next: f32, now: f32, term: f32, num: i32) -> f32;

    #[method(name = "LerpAccel", args = 5)]
    pub fn lerp_accel(prev: f32, next: f32, now: f32, term: f32, num: i32) -> f32;

    #[method(name = "DecelLerp", args = 5)]
    pub fn decel_lerp(prev: f32, next: f32, now: f32, term: f32, num: i32) -> f32;

    #[method(name = "AccelLerp", args = 5)]
    pub fn accel_lerp(prev: f32, next: f32, now: f32, term: f32, num: i32) -> f32;

    #[method(name = "LerpSyncTime", args = 5)]
    pub fn lerp_sync_time(prev: f32, next: f32, now: f32, term: f32, unused: i32) -> f32;

    #[method(name = "LerpSyncTime", args = 5)]
    pub fn lerp_sync_time_2(
        prev: crate::unity_engine::vector3::Vector3,
        next: crate::unity_engine::vector3::Vector3,
        now: f32,
        term: f32,
        unused: i32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "LerpSyncTime", args = 5)]
    pub fn lerp_sync_time_3(
        prev: crate::unity_engine::color::Color,
        next: crate::unity_engine::color::Color,
        now: f32,
        term: f32,
        unused: i32,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "Hermite", args = 5)]
    pub fn hermite(
        v0: crate::unity_engine::vector3::Vector3,
        v1: crate::unity_engine::vector3::Vector3,
        v2: crate::unity_engine::vector3::Vector3,
        v3: crate::unity_engine::vector3::Vector3,
        t: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Hermite", args = 5)]
    pub fn hermite_2(
        v0: crate::unity_engine::vector2::Vector2,
        v1: crate::unity_engine::vector2::Vector2,
        v2: crate::unity_engine::vector2::Vector2,
        v3: crate::unity_engine::vector2::Vector2,
        t: f32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "CatmullRom", args = 5)]
    pub fn catmull_rom(
        v0: crate::unity_engine::vector3::Vector3,
        v1: crate::unity_engine::vector3::Vector3,
        v2: crate::unity_engine::vector3::Vector3,
        v3: crate::unity_engine::vector3::Vector3,
        t: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CatmullRom", args = 5)]
    pub fn catmull_rom_2(
        v0: crate::unity_engine::vector2::Vector2,
        v1: crate::unity_engine::vector2::Vector2,
        v2: crate::unity_engine::vector2::Vector2,
        v3: crate::unity_engine::vector2::Vector2,
        t: f32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-curve")]
impl Curve {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Curve),
                ::core::stringify!(new),
            )
        });
        <Self as ICurveMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/curve/Curve_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Curve_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Curve_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Curve.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Curve_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Curve_Type {
    pub fn linear() -> Self {
        Self { value: 0 }
    }

    pub fn accel() -> Self {
        Self { value: 1 }
    }

    pub fn decel() -> Self {
        Self { value: 2 }
    }

    pub fn accel_decel() -> Self {
        Self { value: 3 }
    }

    pub fn decel_accel() -> Self {
        Self { value: 4 }
    }

    pub fn linear_decel() -> Self {
        Self { value: 5 }
    }

    pub fn linear_accel() -> Self {
        Self { value: 6 }
    }

    pub fn decel_linear() -> Self {
        Self { value: 7 }
    }

    pub fn accel_linear() -> Self {
        Self { value: 8 }
    }
}
