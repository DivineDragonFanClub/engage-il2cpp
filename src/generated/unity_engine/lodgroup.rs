
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/lodgroup/LODGroup.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "LODGroup")]
#[parent(crate::unity_engine::component::Component)]
pub struct LODGroup {}

#[cfg(feature = "unity_engine-lodgroup")]
#[::unity2::methods]
impl LODGroup {
    #[method(name = "get_localReferencePoint", args = 0)]
    pub fn get_local_reference_point(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_localReferencePoint", args = 1)]
    pub fn set_local_reference_point(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_size", args = 0)]
    pub fn get_size(self) -> f32;

    #[method(name = "set_size", args = 1)]
    pub fn set_size(self, value: f32) -> ();

    #[method(name = "get_lodCount", args = 0)]
    pub fn get_lod_count(self) -> i32;

    #[method(name = "get_fadeMode", args = 0)]
    pub fn get_fade_mode(self) -> crate::unity_engine::lodfademode::LODFadeMode;

    #[method(name = "set_fadeMode", args = 1)]
    pub fn set_fade_mode(self, value: crate::unity_engine::lodfademode::LODFadeMode) -> ();

    #[method(name = "get_animateCrossFading", args = 0)]
    pub fn get_animate_cross_fading(self) -> bool;

    #[method(name = "set_animateCrossFading", args = 1)]
    pub fn set_animate_cross_fading(self, value: bool) -> ();

    #[method(name = "get_enabled", args = 0)]
    pub fn get_enabled(self) -> bool;

    #[method(name = "set_enabled", args = 1)]
    pub fn set_enabled(self, value: bool) -> ();

    #[method(name = "RecalculateBounds", args = 0)]
    pub fn recalculate_bounds(self) -> ();

    #[method(name = "GetLODs", args = 0)]
    pub fn get_lo_ds(self) -> ::unity2::Array<crate::unity_engine::lod::LOD>;

    #[method(name = "SetLODS", args = 1)]
    pub fn set_lods(self, lods: ::unity2::Array<crate::unity_engine::lod::LOD>) -> ();

    #[method(name = "SetLODs", args = 1)]
    pub fn set_lo_ds(self, lods: ::unity2::Array<crate::unity_engine::lod::LOD>) -> ();

    #[method(name = "ForceLOD", args = 1)]
    pub fn force_lod(self, index: i32) -> ();

    #[method(name = "get_crossFadeAnimationDuration", args = 0)]
    pub fn get_cross_fade_animation_duration() -> f32;

    #[method(name = "set_crossFadeAnimationDuration", args = 1)]
    pub fn set_cross_fade_animation_duration(value: f32) -> ();

    #[method(name = "get_worldReferencePoint", args = 0)]
    pub fn get_world_reference_point(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_localReferencePoint_Injected", args = 1)]
    pub fn get_local_reference_point_injected(
        self,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "set_localReferencePoint_Injected", args = 1)]
    pub fn set_local_reference_point_injected(
        self,
        value: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_worldReferencePoint_Injected", args = 1)]
    pub fn get_world_reference_point_injected(
        self,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();
}

#[cfg(feature = "unity_engine-lodgroup")]
impl LODGroup {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LODGroup),
                ::core::stringify!(new),
            )
        });
        <Self as ILODGroupMethods>::ctor(this);
        this
    }
}
