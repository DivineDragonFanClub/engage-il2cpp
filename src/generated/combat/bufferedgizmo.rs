
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/bufferedgizmo/BufferedGizmo_Segment.md")))]
#[::unity2::class(namespace = "Combat", name = "BufferedGizmo.Segment")]
#[parent(crate::system::object::Object)]
pub struct BufferedGizmo_Segment {
    #[rename(name = "A")]
    pub a: crate::unity_engine::vector3::Vector3,
    #[rename(name = "B")]
    pub b: crate::unity_engine::vector3::Vector3,
    #[rename(name = "color")]
    pub color: crate::unity_engine::color::Color,
}

#[cfg(feature = "combat-bufferedgizmo")]
#[::unity2::methods]
impl BufferedGizmo_Segment {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        color: crate::unity_engine::color::Color,
    ) -> ();
}

#[cfg(feature = "combat-bufferedgizmo")]
impl BufferedGizmo_Segment {
    pub fn new(
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        color: crate::unity_engine::color::Color,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BufferedGizmo_Segment),
                ::core::stringify!(new),
            )
        });
        <Self as IBufferedGizmo_SegmentMethods>::ctor(this, a, b, color);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/bufferedgizmo/BufferedGizmo.md")))]
#[::unity2::class(namespace = "Combat", name = "BufferedGizmo")]
#[parent(crate::system::object::Object)]
pub struct BufferedGizmo {
    #[rename(name = "segments")]
    pub segments: crate::system::collections::generic::list_1::List_1<
        crate::combat::bufferedgizmo::BufferedGizmo_Segment,
    >,
}

#[cfg(feature = "combat-bufferedgizmo")]
#[::unity2::methods]
impl BufferedGizmo {
    #[method(name = "Add", args = 3)]
    pub fn add(
        self,
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "Draw", args = 0)]
    pub fn draw(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-bufferedgizmo")]
impl BufferedGizmo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BufferedGizmo),
                ::core::stringify!(new),
            )
        });
        <Self as IBufferedGizmoMethods>::ctor(this);
        this
    }
}
