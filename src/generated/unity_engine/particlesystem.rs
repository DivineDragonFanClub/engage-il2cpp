
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState {
    pub m_accumulated_dt: f32,
    pub m_start_delay: f32,
    pub m_playback_time: f32,
    pub m_ring_buffer_index: i32,
    pub m_emission: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Emission,
    pub m_initial: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Initial,
    pub m_shape: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Shape,
    pub m_force: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Force,
    pub m_collision: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Collision,
    pub m_noise: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Noise,
    pub m_lights: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Lights,
    pub m_trail: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Trail,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_EmitParams.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_EmitParams {
    pub m_particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    pub m_position_set: bool,
    pub m_velocity_set: bool,
    pub m_axis_of_rotation_set: bool,
    pub m_rotation_set: bool,
    pub m_angular_velocity_set: bool,
    pub m_start_size_set: bool,
    pub m_start_color_set: bool,
    pub m_random_seed_set: bool,
    pub m_start_lifetime_set: bool,
    pub m_mesh_index_set: bool,
    pub m_apply_shape_to_position: bool,
}

impl ::unity2::ClassIdentity for ParticleSystem_EmitParams {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.EmitParams";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_EmitParams {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_ColorBySpeedModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_ColorBySpeedModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_ColorBySpeedModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.ColorBySpeedModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_ColorBySpeedModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_ColorBySpeedModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_Trails.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_Trails {
    pub positions:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::vector4::Vector4>,
    pub front_positions: crate::system::collections::generic::list_1::List_1<i32>,
    pub back_positions: crate::system::collections::generic::list_1::List_1<i32>,
    pub position_counts: crate::system::collections::generic::list_1::List_1<i32>,
    pub max_trail_count: i32,
    pub max_positions_per_trail_count: i32,
}

impl ::unity2::ClassIdentity for ParticleSystem_Trails {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.Trails";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_Trails {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Seed4.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Seed4 {
    pub x: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed,
    pub y: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed,
    pub z: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed,
    pub w: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Seed4 {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Seed4";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Seed4 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_NoiseModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_NoiseModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_NoiseModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.NoiseModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_NoiseModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_NoiseModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_SubEmittersModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_SubEmittersModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_SubEmittersModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.SubEmittersModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_SubEmittersModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_SubEmittersModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_MinMaxGradient.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_MinMaxGradient {
    pub m_mode: crate::unity_engine::particlesystemgradientmode::ParticleSystemGradientMode,
    pub m_gradient_min: crate::unity_engine::gradient::Gradient,
    pub m_gradient_max: crate::unity_engine::gradient::Gradient,
    pub m_color_min: crate::unity_engine::color::Color,
    pub m_color_max: crate::unity_engine::color::Color,
}

impl ::unity2::ClassIdentity for ParticleSystem_MinMaxGradient {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.MinMaxGradient";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_MinMaxGradient {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_MinMaxGradient {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        color: crate::unity_engine::color::Color,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_MinMaxGradient;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_ExternalForcesModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_ExternalForcesModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_ExternalForcesModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.ExternalForcesModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_ExternalForcesModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_ExternalForcesModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_LimitVelocityOverLifetimeModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_LimitVelocityOverLifetimeModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_LimitVelocityOverLifetimeModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.LimitVelocityOverLifetimeModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_LimitVelocityOverLifetimeModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_LimitVelocityOverLifetimeModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_RotationBySpeedModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_RotationBySpeedModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_RotationBySpeedModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.RotationBySpeedModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_RotationBySpeedModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_RotationBySpeedModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_InheritVelocityModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_InheritVelocityModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_InheritVelocityModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.InheritVelocityModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_InheritVelocityModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_InheritVelocityModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_CustomDataModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_CustomDataModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_CustomDataModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.CustomDataModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_CustomDataModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_CustomDataModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_ShapeModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_ShapeModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_ShapeModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.ShapeModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_ShapeModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_ShapeModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();

    #[method(name = "set_enabled", args = 1)]
    pub fn set_enabled(self, value: bool) -> ();

    #[method(name = "set_shapeType", args = 1)]
    pub fn set_shape_type(
        self,
        value: crate::unity_engine::particlesystemshapetype::ParticleSystemShapeType,
    ) -> ();

    #[method(name = "get_skinnedMeshRenderer", args = 0)]
    pub fn get_skinned_mesh_renderer(
        self,
    ) -> crate::unity_engine::skinnedmeshrenderer::SkinnedMeshRenderer;

    #[method(name = "set_skinnedMeshRenderer", args = 1)]
    pub fn set_skinned_mesh_renderer(
        self,
        value: crate::unity_engine::skinnedmeshrenderer::SkinnedMeshRenderer,
    ) -> ();

    #[method(name = "set_enabled_Injected", args = 2)]
    pub fn set_enabled_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_ShapeModule,
        value: bool,
    ) -> ();

    #[method(name = "set_shapeType_Injected", args = 2)]
    pub fn set_shape_type_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_ShapeModule,
        value: crate::unity_engine::particlesystemshapetype::ParticleSystemShapeType,
    ) -> ();

    #[method(name = "get_skinnedMeshRenderer_Injected", args = 1)]
    pub fn get_skinned_mesh_renderer_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_ShapeModule,
    ) -> crate::unity_engine::skinnedmeshrenderer::SkinnedMeshRenderer;

    #[method(name = "set_skinnedMeshRenderer_Injected", args = 2)]
    pub fn set_skinned_mesh_renderer_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_ShapeModule,
        value: crate::unity_engine::skinnedmeshrenderer::SkinnedMeshRenderer,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_LightsModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_LightsModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_LightsModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.LightsModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_LightsModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_LightsModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Force.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Force {
    pub m_random: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed4,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Force {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Force";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Force {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_ForceOverLifetimeModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_ForceOverLifetimeModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_ForceOverLifetimeModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.ForceOverLifetimeModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_ForceOverLifetimeModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_ForceOverLifetimeModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_LifetimeByEmitterSpeedModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_LifetimeByEmitterSpeedModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_LifetimeByEmitterSpeedModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.LifetimeByEmitterSpeedModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_LifetimeByEmitterSpeedModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_LifetimeByEmitterSpeedModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_RotationOverLifetimeModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_RotationOverLifetimeModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_RotationOverLifetimeModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.RotationOverLifetimeModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_RotationOverLifetimeModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_RotationOverLifetimeModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Emission.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Emission {
    pub m_particle_spacing: f32,
    pub m_to_emit_accumulator: f32,
    pub m_random: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Emission {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Emission";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Emission {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_CollisionModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_CollisionModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_CollisionModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.CollisionModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_CollisionModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_CollisionModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_VelocityOverLifetimeModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_VelocityOverLifetimeModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_VelocityOverLifetimeModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.VelocityOverLifetimeModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_VelocityOverLifetimeModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_VelocityOverLifetimeModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ParticleSystem")]
#[parent(crate::unity_engine::component::Component)]
pub struct ParticleSystem {}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods]
impl ParticleSystem {
    #[method(name = "Emit", args = 5)]
    pub fn emit(
        self,
        position: crate::unity_engine::vector3::Vector3,
        velocity: crate::unity_engine::vector3::Vector3,
        size: f32,
        lifetime: f32,
        color: crate::unity_engine::color32::Color32,
    ) -> ();

    #[method(name = "Emit", args = 1)]
    pub fn emit_2(
        self,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    ) -> ();

    #[method(name = "get_startDelay", args = 0)]
    pub fn get_start_delay(self) -> f32;

    #[method(name = "set_startDelay", args = 1)]
    pub fn set_start_delay(self, value: f32) -> ();

    #[method(name = "get_loop", args = 0)]
    pub fn get_loop(self) -> bool;

    #[method(name = "set_loop", args = 1)]
    pub fn set_loop(self, value: bool) -> ();

    #[method(name = "get_playOnAwake", args = 0)]
    pub fn get_play_on_awake(self) -> bool;

    #[method(name = "set_playOnAwake", args = 1)]
    pub fn set_play_on_awake(self, value: bool) -> ();

    #[method(name = "get_duration", args = 0)]
    pub fn get_duration(self) -> f32;

    #[method(name = "get_playbackSpeed", args = 0)]
    pub fn get_playback_speed(self) -> f32;

    #[method(name = "set_playbackSpeed", args = 1)]
    pub fn set_playback_speed(self, value: f32) -> ();

    #[method(name = "get_enableEmission", args = 0)]
    pub fn get_enable_emission(self) -> bool;

    #[method(name = "set_enableEmission", args = 1)]
    pub fn set_enable_emission(self, value: bool) -> ();

    #[method(name = "get_emissionRate", args = 0)]
    pub fn get_emission_rate(self) -> f32;

    #[method(name = "set_emissionRate", args = 1)]
    pub fn set_emission_rate(self, value: f32) -> ();

    #[method(name = "get_startSpeed", args = 0)]
    pub fn get_start_speed(self) -> f32;

    #[method(name = "set_startSpeed", args = 1)]
    pub fn set_start_speed(self, value: f32) -> ();

    #[method(name = "get_startSize", args = 0)]
    pub fn get_start_size(self) -> f32;

    #[method(name = "set_startSize", args = 1)]
    pub fn set_start_size(self, value: f32) -> ();

    #[method(name = "get_startColor", args = 0)]
    pub fn get_start_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_startColor", args = 1)]
    pub fn set_start_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_startRotation", args = 0)]
    pub fn get_start_rotation(self) -> f32;

    #[method(name = "set_startRotation", args = 1)]
    pub fn set_start_rotation(self, value: f32) -> ();

    #[method(name = "get_startRotation3D", args = 0)]
    pub fn get_start_rotation3_d(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_startRotation3D", args = 1)]
    pub fn set_start_rotation3_d(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_startLifetime", args = 0)]
    pub fn get_start_lifetime(self) -> f32;

    #[method(name = "set_startLifetime", args = 1)]
    pub fn set_start_lifetime(self, value: f32) -> ();

    #[method(name = "get_gravityModifier", args = 0)]
    pub fn get_gravity_modifier(self) -> f32;

    #[method(name = "set_gravityModifier", args = 1)]
    pub fn set_gravity_modifier(self, value: f32) -> ();

    #[method(name = "get_maxParticles", args = 0)]
    pub fn get_max_particles(self) -> i32;

    #[method(name = "set_maxParticles", args = 1)]
    pub fn set_max_particles(self, value: i32) -> ();

    #[method(name = "get_simulationSpace", args = 0)]
    pub fn get_simulation_space(
        self,
    ) -> crate::unity_engine::particlesystemsimulationspace::ParticleSystemSimulationSpace;

    #[method(name = "set_simulationSpace", args = 1)]
    pub fn set_simulation_space(
        self,
        value: crate::unity_engine::particlesystemsimulationspace::ParticleSystemSimulationSpace,
    ) -> ();

    #[method(name = "get_scalingMode", args = 0)]
    pub fn get_scaling_mode(
        self,
    ) -> crate::unity_engine::particlesystemscalingmode::ParticleSystemScalingMode;

    #[method(name = "set_scalingMode", args = 1)]
    pub fn set_scaling_mode(
        self,
        value: crate::unity_engine::particlesystemscalingmode::ParticleSystemScalingMode,
    ) -> ();

    #[method(name = "get_automaticCullingEnabled", args = 0)]
    pub fn get_automatic_culling_enabled(self) -> bool;

    #[method(name = "get_isPlaying", args = 0)]
    pub fn get_is_playing(self) -> bool;

    #[method(name = "get_isEmitting", args = 0)]
    pub fn get_is_emitting(self) -> bool;

    #[method(name = "get_isStopped", args = 0)]
    pub fn get_is_stopped(self) -> bool;

    #[method(name = "get_isPaused", args = 0)]
    pub fn get_is_paused(self) -> bool;

    #[method(name = "get_particleCount", args = 0)]
    pub fn get_particle_count(self) -> i32;

    #[method(name = "get_time", args = 0)]
    pub fn get_time(self) -> f32;

    #[method(name = "set_time", args = 1)]
    pub fn set_time(self, value: f32) -> ();

    #[method(name = "get_randomSeed", args = 0)]
    pub fn get_random_seed(self) -> u32;

    #[method(name = "set_randomSeed", args = 1)]
    pub fn set_random_seed(self, value: u32) -> ();

    #[method(name = "get_useAutoRandomSeed", args = 0)]
    pub fn get_use_auto_random_seed(self) -> bool;

    #[method(name = "set_useAutoRandomSeed", args = 1)]
    pub fn set_use_auto_random_seed(self, value: bool) -> ();

    #[method(name = "get_proceduralSimulationSupported", args = 0)]
    pub fn get_procedural_simulation_supported(self) -> bool;

    #[method(name = "GetParticleCurrentSize", args = 1)]
    pub fn get_particle_current_size(
        self,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    ) -> f32;

    #[method(name = "GetParticleCurrentSize3D", args = 1)]
    pub fn get_particle_current_size3_d(
        self,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetParticleCurrentColor", args = 1)]
    pub fn get_particle_current_color(
        self,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    ) -> crate::unity_engine::color32::Color32;

    #[method(name = "GetParticleMeshIndex", args = 1)]
    pub fn get_particle_mesh_index(
        self,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    ) -> i32;

    #[method(name = "SetParticles", args = 3)]
    pub fn set_particles(
        self,
        particles: ::unity2::Array<crate::unity_engine::particlesystem::ParticleSystem_Particle>,
        size: i32,
        offset: i32,
    ) -> ();

    #[method(name = "SetParticles", args = 2)]
    pub fn set_particles_2(
        self,
        particles: ::unity2::Array<crate::unity_engine::particlesystem::ParticleSystem_Particle>,
        size: i32,
    ) -> ();

    #[method(name = "SetParticles", args = 1)]
    pub fn set_particles_3(
        self,
        particles: ::unity2::Array<crate::unity_engine::particlesystem::ParticleSystem_Particle>,
    ) -> ();

    #[method(name = "SetParticlesWithNativeArray", args = 4)]
    pub fn set_particles_with_native_array(
        self,
        particles: ::unity2::IntPtr,
        particles_length: i32,
        size: i32,
        offset: i32,
    ) -> ();

    #[method(name = "GetParticles", args = 3)]
    pub fn get_particles(
        self,
        particles: ::unity2::Array<crate::unity_engine::particlesystem::ParticleSystem_Particle>,
        size: i32,
        offset: i32,
    ) -> i32;

    #[method(name = "GetParticles", args = 2)]
    pub fn get_particles_2(
        self,
        particles: ::unity2::Array<crate::unity_engine::particlesystem::ParticleSystem_Particle>,
        size: i32,
    ) -> i32;

    #[method(name = "GetParticles", args = 1)]
    pub fn get_particles_3(
        self,
        particles: ::unity2::Array<crate::unity_engine::particlesystem::ParticleSystem_Particle>,
    ) -> i32;

    #[method(name = "GetParticlesWithNativeArray", args = 4)]
    pub fn get_particles_with_native_array(
        self,
        particles: ::unity2::IntPtr,
        particles_length: i32,
        size: i32,
        offset: i32,
    ) -> i32;

    #[method(name = "SetCustomParticleData", args = 2)]
    pub fn set_custom_particle_data(
        self,
        custom_data: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        stream_index: crate::unity_engine::particlesystemcustomdata::ParticleSystemCustomData,
    ) -> ();

    #[method(name = "GetCustomParticleData", args = 2)]
    pub fn get_custom_particle_data(
        self,
        custom_data: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
        stream_index: crate::unity_engine::particlesystemcustomdata::ParticleSystemCustomData,
    ) -> i32;

    #[method(name = "GetPlaybackState", args = 0)]
    pub fn get_playback_state(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_PlaybackState;

    #[method(name = "SetPlaybackState", args = 1)]
    pub fn set_playback_state(
        self,
        playback_state: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState,
    ) -> ();

    #[method(name = "GetTrailDataInternal", args = 1)]
    pub fn get_trail_data_internal(
        self,
        trail_data: crate::unity_engine::particlesystem::ParticleSystem_Trails,
    ) -> ();

    #[method(name = "GetTrails", args = 0)]
    pub fn get_trails(self) -> crate::unity_engine::particlesystem::ParticleSystem_Trails;

    #[method(name = "SetTrails", args = 1)]
    pub fn set_trails(
        self,
        trail_data: crate::unity_engine::particlesystem::ParticleSystem_Trails,
    ) -> ();

    #[method(name = "Simulate", args = 4)]
    pub fn simulate(self, t: f32, with_children: bool, restart: bool, fixed_time_step: bool) -> ();

    #[method(name = "Simulate", args = 3)]
    pub fn simulate_2(self, t: f32, with_children: bool, restart: bool) -> ();

    #[method(name = "Simulate", args = 2)]
    pub fn simulate_3(self, t: f32, with_children: bool) -> ();

    #[method(name = "Simulate", args = 1)]
    pub fn simulate_4(self, t: f32) -> ();

    #[method(name = "Play", args = 1)]
    pub fn play(self, with_children: bool) -> ();

    #[method(name = "Play", args = 0)]
    pub fn play_2(self) -> ();

    #[method(name = "Pause", args = 1)]
    pub fn pause(self, with_children: bool) -> ();

    #[method(name = "Pause", args = 0)]
    pub fn pause_2(self) -> ();

    #[method(name = "Stop", args = 2)]
    pub fn stop(
        self,
        with_children: bool,
        stop_behavior: crate::unity_engine::particlesystemstopbehavior::ParticleSystemStopBehavior,
    ) -> ();

    #[method(name = "Stop", args = 1)]
    pub fn stop_2(self, with_children: bool) -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop_3(self) -> ();

    #[method(name = "Clear", args = 1)]
    pub fn clear(self, with_children: bool) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear_2(self) -> ();

    #[method(name = "IsAlive", args = 1)]
    pub fn is_alive(self, with_children: bool) -> bool;

    #[method(name = "IsAlive", args = 0)]
    pub fn is_alive_2(self) -> bool;

    #[method(name = "Emit", args = 1)]
    pub fn emit_3(self, count: i32) -> ();

    #[method(name = "Emit_Internal", args = 1)]
    pub fn emit_internal(self, count: i32) -> ();

    #[method(name = "Emit", args = 2)]
    pub fn emit_4(
        self,
        emit_params: crate::unity_engine::particlesystem::ParticleSystem_EmitParams,
        count: i32,
    ) -> ();

    #[method(name = "EmitOld_Internal", args = 1)]
    pub fn emit_old_internal(
        self,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    ) -> ();

    #[method(name = "TriggerSubEmitter", args = 1)]
    pub fn trigger_sub_emitter(self, sub_emitter_index: i32) -> ();

    #[method(name = "TriggerSubEmitter", args = 2)]
    pub fn trigger_sub_emitter_2(
        self,
        sub_emitter_index: i32,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    ) -> ();

    #[method(name = "TriggerSubEmitterForParticle", args = 2)]
    pub fn trigger_sub_emitter_for_particle(
        self,
        sub_emitter_index: i32,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    ) -> ();

    #[method(name = "TriggerSubEmitter", args = 2)]
    pub fn trigger_sub_emitter_3(
        self,
        sub_emitter_index: i32,
        particles: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::particlesystem::ParticleSystem_Particle,
        >,
    ) -> ();

    #[method(name = "ResetPreMappedBufferMemory", args = 0)]
    pub fn reset_pre_mapped_buffer_memory() -> ();

    #[method(name = "SetMaximumPreMappedBufferCounts", args = 2)]
    pub fn set_maximum_pre_mapped_buffer_counts(
        vertex_buffers_count: i32,
        index_buffers_count: i32,
    ) -> ();

    #[method(name = "AllocateAxisOfRotationAttribute", args = 0)]
    pub fn allocate_axis_of_rotation_attribute(self) -> ();

    #[method(name = "AllocateMeshIndexAttribute", args = 0)]
    pub fn allocate_mesh_index_attribute(self) -> ();

    #[method(name = "AllocateCustomDataAttribute", args = 1)]
    pub fn allocate_custom_data_attribute(
        self,
        stream: crate::unity_engine::particlesystemcustomdata::ParticleSystemCustomData,
    ) -> ();

    #[method(name = "get_main", args = 0)]
    pub fn get_main(self) -> crate::unity_engine::particlesystem::ParticleSystem_MainModule;

    #[method(name = "get_emission", args = 0)]
    pub fn get_emission(self)
        -> crate::unity_engine::particlesystem::ParticleSystem_EmissionModule;

    #[method(name = "get_shape", args = 0)]
    pub fn get_shape(self) -> crate::unity_engine::particlesystem::ParticleSystem_ShapeModule;

    #[method(name = "get_velocityOverLifetime", args = 0)]
    pub fn get_velocity_over_lifetime(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_VelocityOverLifetimeModule;

    #[method(name = "get_limitVelocityOverLifetime", args = 0)]
    pub fn get_limit_velocity_over_lifetime(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_LimitVelocityOverLifetimeModule;

    #[method(name = "get_inheritVelocity", args = 0)]
    pub fn get_inherit_velocity(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_InheritVelocityModule;

    #[method(name = "get_lifetimeByEmitterSpeed", args = 0)]
    pub fn get_lifetime_by_emitter_speed(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_LifetimeByEmitterSpeedModule;

    #[method(name = "get_forceOverLifetime", args = 0)]
    pub fn get_force_over_lifetime(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_ForceOverLifetimeModule;

    #[method(name = "get_colorOverLifetime", args = 0)]
    pub fn get_color_over_lifetime(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_ColorOverLifetimeModule;

    #[method(name = "get_colorBySpeed", args = 0)]
    pub fn get_color_by_speed(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_ColorBySpeedModule;

    #[method(name = "get_sizeOverLifetime", args = 0)]
    pub fn get_size_over_lifetime(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_SizeOverLifetimeModule;

    #[method(name = "get_sizeBySpeed", args = 0)]
    pub fn get_size_by_speed(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_SizeBySpeedModule;

    #[method(name = "get_rotationOverLifetime", args = 0)]
    pub fn get_rotation_over_lifetime(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_RotationOverLifetimeModule;

    #[method(name = "get_rotationBySpeed", args = 0)]
    pub fn get_rotation_by_speed(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_RotationBySpeedModule;

    #[method(name = "get_externalForces", args = 0)]
    pub fn get_external_forces(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_ExternalForcesModule;

    #[method(name = "get_noise", args = 0)]
    pub fn get_noise(self) -> crate::unity_engine::particlesystem::ParticleSystem_NoiseModule;

    #[method(name = "get_collision", args = 0)]
    pub fn get_collision(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_CollisionModule;

    #[method(name = "get_trigger", args = 0)]
    pub fn get_trigger(self) -> crate::unity_engine::particlesystem::ParticleSystem_TriggerModule;

    #[method(name = "get_subEmitters", args = 0)]
    pub fn get_sub_emitters(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_SubEmittersModule;

    #[method(name = "get_textureSheetAnimation", args = 0)]
    pub fn get_texture_sheet_animation(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_TextureSheetAnimationModule;

    #[method(name = "get_lights", args = 0)]
    pub fn get_lights(self) -> crate::unity_engine::particlesystem::ParticleSystem_LightsModule;

    #[method(name = "get_customData", args = 0)]
    pub fn get_custom_data(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_CustomDataModule;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetParticleCurrentSize3D_Injected", args = 2)]
    pub fn get_particle_current_size3_d_injected(
        self,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetParticleCurrentColor_Injected", args = 2)]
    pub fn get_particle_current_color_injected(
        self,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
        ret: crate::unity_engine::color32::Color32,
    ) -> ();

    #[method(name = "GetPlaybackState_Injected", args = 1)]
    pub fn get_playback_state_injected(
        self,
        ret: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState,
    ) -> ();

    #[method(name = "SetPlaybackState_Injected", args = 1)]
    pub fn set_playback_state_injected(
        self,
        playback_state: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState,
    ) -> ();

    #[method(name = "SetTrails_Injected", args = 1)]
    pub fn set_trails_injected(
        self,
        trail_data: crate::unity_engine::particlesystem::ParticleSystem_Trails,
    ) -> ();

    #[method(name = "Emit_Injected", args = 2)]
    pub fn emit_injected(
        self,
        emit_params: crate::unity_engine::particlesystem::ParticleSystem_EmitParams,
        count: i32,
    ) -> ();

    #[method(name = "TriggerSubEmitterForParticle_Injected", args = 2)]
    pub fn trigger_sub_emitter_for_particle_injected(
        self,
        sub_emitter_index: i32,
        particle: crate::unity_engine::particlesystem::ParticleSystem_Particle,
    ) -> ();
}

#[cfg(feature = "unity_engine-particlesystem")]
impl ParticleSystem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ParticleSystem),
                ::core::stringify!(new),
            )
        });
        <Self as IParticleSystemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Noise.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Noise {
    pub m_scroll_offset: f32,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Noise {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Noise";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Noise {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Initial.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Initial {
    pub m_random: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed4,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Initial {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Initial";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Initial {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_TrailModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_TrailModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_TrailModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.TrailModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_TrailModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_TrailModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_Particle.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_Particle {
    pub m_position: crate::unity_engine::vector3::Vector3,
    pub m_velocity: crate::unity_engine::vector3::Vector3,
    pub m_animated_velocity: crate::unity_engine::vector3::Vector3,
    pub m_initial_velocity: crate::unity_engine::vector3::Vector3,
    pub m_axis_of_rotation: crate::unity_engine::vector3::Vector3,
    pub m_rotation: crate::unity_engine::vector3::Vector3,
    pub m_angular_velocity: crate::unity_engine::vector3::Vector3,
    pub m_start_size: crate::unity_engine::vector3::Vector3,
    pub m_start_color: crate::unity_engine::color32::Color32,
    pub m_random_seed: u32,
    pub m_parent_random_seed: u32,
    pub m_lifetime: f32,
    pub m_start_lifetime: f32,
    pub m_mesh_index: i32,
    pub m_emit_accumulator0: f32,
    pub m_emit_accumulator1: f32,
    pub m_flags: u32,
}

impl ::unity2::ClassIdentity for ParticleSystem_Particle {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.Particle";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_Particle {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_Particle {
    #[method(name = "set_lifetime", args = 1)]
    pub fn set_lifetime(self, value: f32) -> ();

    #[method(name = "set_position", args = 1)]
    pub fn set_position(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_velocity", args = 1)]
    pub fn set_velocity(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_remainingLifetime", args = 1)]
    pub fn set_remaining_lifetime(self, value: f32) -> ();

    #[method(name = "set_startLifetime", args = 1)]
    pub fn set_start_lifetime(self, value: f32) -> ();

    #[method(name = "set_startColor", args = 1)]
    pub fn set_start_color(self, value: crate::unity_engine::color32::Color32) -> ();

    #[method(name = "set_randomSeed", args = 1)]
    pub fn set_random_seed(self, value: u32) -> ();

    #[method(name = "set_startSize", args = 1)]
    pub fn set_start_size(self, value: f32) -> ();

    #[method(name = "set_rotation", args = 1)]
    pub fn set_rotation(self, value: f32) -> ();

    #[method(name = "set_rotation3D", args = 1)]
    pub fn set_rotation3_d(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_angularVelocity3D", args = 1)]
    pub fn set_angular_velocity3_d(self, value: crate::unity_engine::vector3::Vector3) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Shape.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Shape {
    pub m_random: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed4,
    pub m_radius_timer: f32,
    pub m_radius_timer_prev: f32,
    pub m_arc_timer: f32,
    pub m_arc_timer_prev: f32,
    pub m_mesh_spawn_timer: f32,
    pub m_mesh_spawn_timer_prev: f32,
    pub m_ordered_mesh_vertex_index: i32,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Shape {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Shape";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Shape {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Trail.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Trail {
    pub m_timer: f32,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Trail {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Trail";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Trail {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_ColorOverLifetimeModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_ColorOverLifetimeModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_ColorOverLifetimeModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.ColorOverLifetimeModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_ColorOverLifetimeModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_ColorOverLifetimeModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_MainModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_MainModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_MainModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.MainModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_MainModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_MainModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();

    #[method(name = "get_duration", args = 0)]
    pub fn get_duration(self) -> f32;

    #[method(name = "get_loop", args = 0)]
    pub fn get_loop(self) -> bool;

    #[method(name = "set_loop", args = 1)]
    pub fn set_loop(self, value: bool) -> ();

    #[method(name = "get_startDelayMultiplier", args = 0)]
    pub fn get_start_delay_multiplier(self) -> f32;

    #[method(name = "set_startDelayMultiplier", args = 1)]
    pub fn set_start_delay_multiplier(self, value: f32) -> ();

    #[method(name = "get_startLifetimeMultiplier", args = 0)]
    pub fn get_start_lifetime_multiplier(self) -> f32;

    #[method(name = "set_startLifetimeMultiplier", args = 1)]
    pub fn set_start_lifetime_multiplier(self, value: f32) -> ();

    #[method(name = "get_startSpeedMultiplier", args = 0)]
    pub fn get_start_speed_multiplier(self) -> f32;

    #[method(name = "set_startSpeedMultiplier", args = 1)]
    pub fn set_start_speed_multiplier(self, value: f32) -> ();

    #[method(name = "get_startSizeMultiplier", args = 0)]
    pub fn get_start_size_multiplier(self) -> f32;

    #[method(name = "set_startSizeMultiplier", args = 1)]
    pub fn set_start_size_multiplier(self, value: f32) -> ();

    #[method(name = "set_startRotation", args = 1)]
    pub fn set_start_rotation(
        self,
        value: crate::unity_engine::particlesystem::ParticleSystem_MinMaxCurve,
    ) -> ();

    #[method(name = "get_startRotationMultiplier", args = 0)]
    pub fn get_start_rotation_multiplier(self) -> f32;

    #[method(name = "set_startRotationMultiplier", args = 1)]
    pub fn set_start_rotation_multiplier(self, value: f32) -> ();

    #[method(name = "get_startRotationXMultiplier", args = 0)]
    pub fn get_start_rotation_x_multiplier(self) -> f32;

    #[method(name = "set_startRotationXMultiplier", args = 1)]
    pub fn set_start_rotation_x_multiplier(self, value: f32) -> ();

    #[method(name = "get_startRotationYMultiplier", args = 0)]
    pub fn get_start_rotation_y_multiplier(self) -> f32;

    #[method(name = "set_startRotationYMultiplier", args = 1)]
    pub fn set_start_rotation_y_multiplier(self, value: f32) -> ();

    #[method(name = "get_startRotationZMultiplier", args = 0)]
    pub fn get_start_rotation_z_multiplier(self) -> f32;

    #[method(name = "set_startRotationZMultiplier", args = 1)]
    pub fn set_start_rotation_z_multiplier(self, value: f32) -> ();

    #[method(name = "get_startColor", args = 0)]
    pub fn get_start_color(
        self,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_MinMaxGradient;

    #[method(name = "set_startColor", args = 1)]
    pub fn set_start_color(
        self,
        value: crate::unity_engine::particlesystem::ParticleSystem_MinMaxGradient,
    ) -> ();

    #[method(name = "get_gravityModifierMultiplier", args = 0)]
    pub fn get_gravity_modifier_multiplier(self) -> f32;

    #[method(name = "set_gravityModifierMultiplier", args = 1)]
    pub fn set_gravity_modifier_multiplier(self, value: f32) -> ();

    #[method(name = "get_simulationSpace", args = 0)]
    pub fn get_simulation_space(
        self,
    ) -> crate::unity_engine::particlesystemsimulationspace::ParticleSystemSimulationSpace;

    #[method(name = "set_simulationSpace", args = 1)]
    pub fn set_simulation_space(
        self,
        value: crate::unity_engine::particlesystemsimulationspace::ParticleSystemSimulationSpace,
    ) -> ();

    #[method(name = "get_simulationSpeed", args = 0)]
    pub fn get_simulation_speed(self) -> f32;

    #[method(name = "set_simulationSpeed", args = 1)]
    pub fn set_simulation_speed(self, value: f32) -> ();

    #[method(name = "get_scalingMode", args = 0)]
    pub fn get_scaling_mode(
        self,
    ) -> crate::unity_engine::particlesystemscalingmode::ParticleSystemScalingMode;

    #[method(name = "set_scalingMode", args = 1)]
    pub fn set_scaling_mode(
        self,
        value: crate::unity_engine::particlesystemscalingmode::ParticleSystemScalingMode,
    ) -> ();

    #[method(name = "get_playOnAwake", args = 0)]
    pub fn get_play_on_awake(self) -> bool;

    #[method(name = "set_playOnAwake", args = 1)]
    pub fn set_play_on_awake(self, value: bool) -> ();

    #[method(name = "get_maxParticles", args = 0)]
    pub fn get_max_particles(self) -> i32;

    #[method(name = "set_maxParticles", args = 1)]
    pub fn set_max_particles(self, value: i32) -> ();

    #[method(name = "get_stopAction", args = 0)]
    pub fn get_stop_action(
        self,
    ) -> crate::unity_engine::particlesystemstopaction::ParticleSystemStopAction;

    #[method(name = "set_stopAction", args = 1)]
    pub fn set_stop_action(
        self,
        value: crate::unity_engine::particlesystemstopaction::ParticleSystemStopAction,
    ) -> ();

    #[method(name = "get_cullingMode", args = 0)]
    pub fn get_culling_mode(
        self,
    ) -> crate::unity_engine::particlesystemcullingmode::ParticleSystemCullingMode;

    #[method(name = "set_cullingMode", args = 1)]
    pub fn set_culling_mode(
        self,
        value: crate::unity_engine::particlesystemcullingmode::ParticleSystemCullingMode,
    ) -> ();

    #[method(name = "get_duration_Injected", args = 1)]
    pub fn get_duration_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "get_loop_Injected", args = 1)]
    pub fn get_loop_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> bool;

    #[method(name = "set_loop_Injected", args = 2)]
    pub fn set_loop_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: bool,
    ) -> ();

    #[method(name = "get_startDelayMultiplier_Injected", args = 1)]
    pub fn get_start_delay_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_startDelayMultiplier_Injected", args = 2)]
    pub fn set_start_delay_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "get_startLifetimeMultiplier_Injected", args = 1)]
    pub fn get_start_lifetime_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_startLifetimeMultiplier_Injected", args = 2)]
    pub fn set_start_lifetime_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "get_startSpeedMultiplier_Injected", args = 1)]
    pub fn get_start_speed_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_startSpeedMultiplier_Injected", args = 2)]
    pub fn set_start_speed_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "get_startSizeMultiplier_Injected", args = 1)]
    pub fn get_start_size_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_startSizeMultiplier_Injected", args = 2)]
    pub fn set_start_size_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "set_startRotation_Injected", args = 2)]
    pub fn set_start_rotation_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: crate::unity_engine::particlesystem::ParticleSystem_MinMaxCurve,
    ) -> ();

    #[method(name = "get_startRotationMultiplier_Injected", args = 1)]
    pub fn get_start_rotation_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_startRotationMultiplier_Injected", args = 2)]
    pub fn set_start_rotation_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "get_startRotationXMultiplier_Injected", args = 1)]
    pub fn get_start_rotation_x_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_startRotationXMultiplier_Injected", args = 2)]
    pub fn set_start_rotation_x_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "get_startRotationYMultiplier_Injected", args = 1)]
    pub fn get_start_rotation_y_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_startRotationYMultiplier_Injected", args = 2)]
    pub fn set_start_rotation_y_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "get_startRotationZMultiplier_Injected", args = 1)]
    pub fn get_start_rotation_z_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_startRotationZMultiplier_Injected", args = 2)]
    pub fn set_start_rotation_z_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "get_startColor_Injected", args = 2)]
    pub fn get_start_color_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        ret: crate::unity_engine::particlesystem::ParticleSystem_MinMaxGradient,
    ) -> ();

    #[method(name = "set_startColor_Injected", args = 2)]
    pub fn set_start_color_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: crate::unity_engine::particlesystem::ParticleSystem_MinMaxGradient,
    ) -> ();

    #[method(name = "get_gravityModifierMultiplier_Injected", args = 1)]
    pub fn get_gravity_modifier_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_gravityModifierMultiplier_Injected", args = 2)]
    pub fn set_gravity_modifier_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "get_simulationSpace_Injected", args = 1)]
    pub fn get_simulation_space_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> crate::unity_engine::particlesystemsimulationspace::ParticleSystemSimulationSpace;

    #[method(name = "set_simulationSpace_Injected", args = 2)]
    pub fn set_simulation_space_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: crate::unity_engine::particlesystemsimulationspace::ParticleSystemSimulationSpace,
    ) -> ();

    #[method(name = "get_simulationSpeed_Injected", args = 1)]
    pub fn get_simulation_speed_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> f32;

    #[method(name = "set_simulationSpeed_Injected", args = 2)]
    pub fn set_simulation_speed_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: f32,
    ) -> ();

    #[method(name = "get_scalingMode_Injected", args = 1)]
    pub fn get_scaling_mode_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> crate::unity_engine::particlesystemscalingmode::ParticleSystemScalingMode;

    #[method(name = "set_scalingMode_Injected", args = 2)]
    pub fn set_scaling_mode_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: crate::unity_engine::particlesystemscalingmode::ParticleSystemScalingMode,
    ) -> ();

    #[method(name = "get_playOnAwake_Injected", args = 1)]
    pub fn get_play_on_awake_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> bool;

    #[method(name = "set_playOnAwake_Injected", args = 2)]
    pub fn set_play_on_awake_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: bool,
    ) -> ();

    #[method(name = "get_maxParticles_Injected", args = 1)]
    pub fn get_max_particles_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> i32;

    #[method(name = "set_maxParticles_Injected", args = 2)]
    pub fn set_max_particles_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: i32,
    ) -> ();

    #[method(name = "get_stopAction_Injected", args = 1)]
    pub fn get_stop_action_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> crate::unity_engine::particlesystemstopaction::ParticleSystemStopAction;

    #[method(name = "set_stopAction_Injected", args = 2)]
    pub fn set_stop_action_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: crate::unity_engine::particlesystemstopaction::ParticleSystemStopAction,
    ) -> ();

    #[method(name = "get_cullingMode_Injected", args = 1)]
    pub fn get_culling_mode_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
    ) -> crate::unity_engine::particlesystemcullingmode::ParticleSystemCullingMode;

    #[method(name = "set_cullingMode_Injected", args = 2)]
    pub fn set_culling_mode_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_MainModule,
        value: crate::unity_engine::particlesystemcullingmode::ParticleSystemCullingMode,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Lights.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Lights {
    pub m_random: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed,
    pub m_particle_emission_counter: f32,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Lights {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Lights";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Lights {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_TextureSheetAnimationModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_TextureSheetAnimationModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_TextureSheetAnimationModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.TextureSheetAnimationModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_TextureSheetAnimationModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_TextureSheetAnimationModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Seed.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Seed {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Seed {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Seed";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Seed {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_PlaybackState_Collision.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_PlaybackState_Collision {
    pub m_random: crate::unity_engine::particlesystem::ParticleSystem_PlaybackState_Seed4,
}

impl ::unity2::ClassIdentity for ParticleSystem_PlaybackState_Collision {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.PlaybackState.Collision";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_PlaybackState_Collision {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_TriggerModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_TriggerModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_TriggerModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.TriggerModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_TriggerModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_TriggerModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_SizeOverLifetimeModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_SizeOverLifetimeModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_SizeOverLifetimeModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.SizeOverLifetimeModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_SizeOverLifetimeModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_SizeOverLifetimeModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_MinMaxCurve.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_MinMaxCurve {
    pub m_mode: crate::unity_engine::particlesystemcurvemode::ParticleSystemCurveMode,
    pub m_curve_multiplier: f32,
    pub m_curve_min: crate::unity_engine::animationcurve::AnimationCurve,
    pub m_curve_max: crate::unity_engine::animationcurve::AnimationCurve,
    pub m_constant_min: f32,
    pub m_constant_max: f32,
}

impl ::unity2::ClassIdentity for ParticleSystem_MinMaxCurve {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.MinMaxCurve";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_MinMaxCurve {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_MinMaxCurve {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, constant: f32) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        constant: f32,
    ) -> crate::unity_engine::particlesystem::ParticleSystem_MinMaxCurve;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_SizeBySpeedModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_SizeBySpeedModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_SizeBySpeedModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.SizeBySpeedModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_SizeBySpeedModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_SizeBySpeedModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystem/ParticleSystem_EmissionModule.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ParticleSystem_EmissionModule {
    pub m_particle_system: crate::unity_engine::particlesystem::ParticleSystem,
}

impl ::unity2::ClassIdentity for ParticleSystem_EmissionModule {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystem.EmissionModule";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystem_EmissionModule {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-particlesystem")]
#[::unity2::methods(value)]
impl ParticleSystem_EmissionModule {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, particle_system: crate::unity_engine::particlesystem::ParticleSystem) -> ();

    #[method(name = "get_enabled", args = 0)]
    pub fn get_enabled(self) -> bool;

    #[method(name = "set_enabled", args = 1)]
    pub fn set_enabled(self, value: bool) -> ();

    #[method(name = "set_rateOverTime", args = 1)]
    pub fn set_rate_over_time(
        self,
        value: crate::unity_engine::particlesystem::ParticleSystem_MinMaxCurve,
    ) -> ();

    #[method(name = "get_rateOverTimeMultiplier", args = 0)]
    pub fn get_rate_over_time_multiplier(self) -> f32;

    #[method(name = "get_enabled_Injected", args = 1)]
    pub fn get_enabled_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_EmissionModule,
    ) -> bool;

    #[method(name = "set_enabled_Injected", args = 2)]
    pub fn set_enabled_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_EmissionModule,
        value: bool,
    ) -> ();

    #[method(name = "set_rateOverTime_Injected", args = 2)]
    pub fn set_rate_over_time_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_EmissionModule,
        value: crate::unity_engine::particlesystem::ParticleSystem_MinMaxCurve,
    ) -> ();

    #[method(name = "get_rateOverTimeMultiplier_Injected", args = 1)]
    pub fn get_rate_over_time_multiplier_injected(
        unity_self: crate::unity_engine::particlesystem::ParticleSystem_EmissionModule,
    ) -> f32;
}
