
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/flattenedvec3curve/FlattenedVec3Curve.md")))]
#[::unity2::class(namespace = "Combat", name = "FlattenedVec3Curve")]
#[parent(crate::system::object::Object)]
pub struct FlattenedVec3Curve {
    #[static_field]
    #[rename(name = "U")]
    pub u: f32,
    #[rename(name = "tip")]
    pub tip:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "roo")]
    pub roo:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector3::Vector3>,
    #[rename(name = "dir")]
    pub dir:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector3::Vector3>,
}

#[cfg(feature = "combat-flattenedvec3curve")]
#[::unity2::methods]
impl FlattenedVec3Curve {
    #[method(name = "get_timeLength", args = 0)]
    pub fn get_time_length(self) -> f32;

    #[method(name = "set_timeLength", args = 1)]
    pub fn set_time_length(self, value: f32) -> ();

    #[method(name = "get_length", args = 0)]
    pub fn get_length(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, i: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        rx: crate::unity_engine::animationcurve::AnimationCurve,
        ry: crate::unity_engine::animationcurve::AnimationCurve,
        rz: crate::unity_engine::animationcurve::AnimationCurve,
        tx: crate::unity_engine::animationcurve::AnimationCurve,
        ty: crate::unity_engine::animationcurve::AnimationCurve,
        tz: crate::unity_engine::animationcurve::AnimationCurve,
    ) -> ();

    #[method(name = "CopyFrom", args = 3)]
    pub fn copy_from(
        self,
        rhs: crate::combat::flattenedvec3curve::FlattenedVec3Curve,
        lp: i32,
        rp: i32,
    ) -> ();

    #[method(name = "Export", args = 6)]
    pub fn export(
        self,
        rx: crate::unity_engine::animationcurve::AnimationCurve,
        ry: crate::unity_engine::animationcurve::AnimationCurve,
        rz: crate::unity_engine::animationcurve::AnimationCurve,
        tx: crate::unity_engine::animationcurve::AnimationCurve,
        ty: crate::unity_engine::animationcurve::AnimationCurve,
        tz: crate::unity_engine::animationcurve::AnimationCurve,
    ) -> ();
}

#[cfg(feature = "combat-flattenedvec3curve")]
impl FlattenedVec3Curve {
    pub fn new(
        rx: crate::unity_engine::animationcurve::AnimationCurve,
        ry: crate::unity_engine::animationcurve::AnimationCurve,
        rz: crate::unity_engine::animationcurve::AnimationCurve,
        tx: crate::unity_engine::animationcurve::AnimationCurve,
        ty: crate::unity_engine::animationcurve::AnimationCurve,
        tz: crate::unity_engine::animationcurve::AnimationCurve,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FlattenedVec3Curve),
                ::core::stringify!(new),
            )
        });
        <Self as IFlattenedVec3CurveMethods>::ctor(this, rx, ry, rz, tx, ty, tz);
        this
    }
}
