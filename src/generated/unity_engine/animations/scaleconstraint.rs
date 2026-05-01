
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/animations/scaleconstraint/ScaleConstraint.md")))]
#[::unity2::class(namespace = "UnityEngine.Animations", name = "ScaleConstraint")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct ScaleConstraint {}

#[cfg(feature = "unity_engine-animations-scaleconstraint")]
#[::unity2::methods]
impl ScaleConstraint {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Internal_Create", args = 1)]
    pub fn internal_create(
        self_: crate::unity_engine::animations::scaleconstraint::ScaleConstraint,
    ) -> ();

    #[method(name = "get_weight", args = 0)]
    pub fn get_weight(self) -> f32;

    #[method(name = "set_weight", args = 1)]
    pub fn set_weight(self, value: f32) -> ();

    #[method(name = "get_scaleAtRest", args = 0)]
    pub fn get_scale_at_rest(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_scaleAtRest", args = 1)]
    pub fn set_scale_at_rest(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_scaleOffset", args = 0)]
    pub fn get_scale_offset(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_scaleOffset", args = 1)]
    pub fn set_scale_offset(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_scalingAxis", args = 0)]
    pub fn get_scaling_axis(self) -> crate::unity_engine::animations::axis::Axis;

    #[method(name = "set_scalingAxis", args = 1)]
    pub fn set_scaling_axis(self, value: crate::unity_engine::animations::axis::Axis) -> ();

    #[method(name = "get_constraintActive", args = 0)]
    pub fn get_constraint_active(self) -> bool;

    #[method(name = "set_constraintActive", args = 1)]
    pub fn set_constraint_active(self, value: bool) -> ();

    #[method(name = "get_locked", args = 0)]
    pub fn get_locked(self) -> bool;

    #[method(name = "set_locked", args = 1)]
    pub fn set_locked(self, value: bool) -> ();

    #[method(name = "get_sourceCount", args = 0)]
    pub fn get_source_count(self) -> i32;

    #[method(name = "GetSourceCountInternal", args = 1)]
    pub fn get_source_count_internal(
        self_: crate::unity_engine::animations::scaleconstraint::ScaleConstraint,
    ) -> i32;

    #[method(name = "GetSources", args = 1)]
    pub fn get_sources(
        self,
        sources: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::animations::constraintsource::ConstraintSource,
        >,
    ) -> ();

    #[method(name = "SetSources", args = 1)]
    pub fn set_sources(
        self,
        sources: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::animations::constraintsource::ConstraintSource,
        >,
    ) -> ();

    #[method(name = "SetSourcesInternal", args = 2)]
    pub fn set_sources_internal(
        self_: crate::unity_engine::animations::scaleconstraint::ScaleConstraint,
        sources: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::animations::constraintsource::ConstraintSource,
        >,
    ) -> ();

    #[method(name = "AddSource", args = 1)]
    pub fn add_source(
        self,
        source: crate::unity_engine::animations::constraintsource::ConstraintSource,
    ) -> i32;

    #[method(name = "RemoveSource", args = 1)]
    pub fn remove_source(self, index: i32) -> ();

    #[method(name = "RemoveSourceInternal", args = 1)]
    pub fn remove_source_internal(self, index: i32) -> ();

    #[method(name = "GetSource", args = 1)]
    pub fn get_source(
        self,
        index: i32,
    ) -> crate::unity_engine::animations::constraintsource::ConstraintSource;

    #[method(name = "GetSourceInternal", args = 1)]
    pub fn get_source_internal(
        self,
        index: i32,
    ) -> crate::unity_engine::animations::constraintsource::ConstraintSource;

    #[method(name = "SetSource", args = 2)]
    pub fn set_source(
        self,
        index: i32,
        source: crate::unity_engine::animations::constraintsource::ConstraintSource,
    ) -> ();

    #[method(name = "SetSourceInternal", args = 2)]
    pub fn set_source_internal(
        self,
        index: i32,
        source: crate::unity_engine::animations::constraintsource::ConstraintSource,
    ) -> ();

    #[method(name = "ValidateSourceIndex", args = 1)]
    pub fn validate_source_index(self, index: i32) -> ();

    #[method(name = "get_scaleAtRest_Injected", args = 1)]
    pub fn get_scale_at_rest_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_scaleAtRest_Injected", args = 1)]
    pub fn set_scale_at_rest_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_scaleOffset_Injected", args = 1)]
    pub fn get_scale_offset_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_scaleOffset_Injected", args = 1)]
    pub fn set_scale_offset_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "AddSource_Injected", args = 1)]
    pub fn add_source_injected(
        self,
        source: crate::unity_engine::animations::constraintsource::ConstraintSource,
    ) -> i32;

    #[method(name = "GetSourceInternal_Injected", args = 2)]
    pub fn get_source_internal_injected(
        self,
        index: i32,
        ret: crate::unity_engine::animations::constraintsource::ConstraintSource,
    ) -> ();

    #[method(name = "SetSourceInternal_Injected", args = 2)]
    pub fn set_source_internal_injected(
        self,
        index: i32,
        source: crate::unity_engine::animations::constraintsource::ConstraintSource,
    ) -> ();
}

#[cfg(feature = "unity_engine-animations-scaleconstraint")]
impl ScaleConstraint {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScaleConstraint),
                ::core::stringify!(new),
            )
        });
        <Self as IScaleConstraintMethods>::ctor(this);
        this
    }
}
