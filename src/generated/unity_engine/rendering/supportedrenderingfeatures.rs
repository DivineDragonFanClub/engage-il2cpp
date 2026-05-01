
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/supportedrenderingfeatures/SupportedRenderingFeatures.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering",
    name = "SupportedRenderingFeatures"
)]
#[parent(crate::system::object::Object)]
pub struct SupportedRenderingFeatures {
    #[static_field]
    #[rename(name = "s_Active")]
    pub s_active:
        crate::unity_engine::rendering::supportedrenderingfeatures::SupportedRenderingFeatures,
}

#[cfg(feature = "unity_engine-rendering-supportedrenderingfeatures")]
#[::unity2::methods]
impl SupportedRenderingFeatures {
    #[method(name = "get_active", args = 0)]
    pub fn get_active(
    ) -> crate::unity_engine::rendering::supportedrenderingfeatures::SupportedRenderingFeatures;

    #[method(name = "set_active", args = 1)]
    pub fn set_active(
        value : crate :: unity_engine :: rendering :: supportedrenderingfeatures :: SupportedRenderingFeatures,
    ) -> ();

    #[method(name = "get_defaultMixedLightingModes", args = 0)]
    pub fn get_default_mixed_lighting_modes (self ,) -> crate :: unity_engine :: rendering :: supportedrenderingfeatures :: SupportedRenderingFeatures_LightmapMixedBakeModes ;

    #[method(name = "get_mixedLightingModes", args = 0)]
    pub fn get_mixed_lighting_modes (self ,) -> crate :: unity_engine :: rendering :: supportedrenderingfeatures :: SupportedRenderingFeatures_LightmapMixedBakeModes ;

    #[method(name = "get_lightmapBakeTypes", args = 0)]
    pub fn get_lightmap_bake_types(self)
        -> crate::unity_engine::lightmapbaketype::LightmapBakeType;

    #[method(name = "get_lightmapsModes", args = 0)]
    pub fn get_lightmaps_modes(self) -> crate::unity_engine::lightmapsmode::LightmapsMode;

    #[method(name = "get_enlighten", args = 0)]
    pub fn get_enlighten(self) -> bool;

    #[method(name = "get_rendersUIOverlay", args = 0)]
    pub fn get_renders_ui_overlay(self) -> bool;

    #[method(name = "get_autoAmbientProbeBaking", args = 0)]
    pub fn get_auto_ambient_probe_baking(self) -> bool;

    #[method(name = "get_autoDefaultReflectionProbeBaking", args = 0)]
    pub fn get_auto_default_reflection_probe_baking(self) -> bool;

    #[method(name = "FallbackMixedLightingModeByRef", args = 1)]
    pub fn fallback_mixed_lighting_mode_by_ref(fallback_mode_ptr: ::unity2::IntPtr) -> ();

    #[method(name = "IsMixedLightingModeSupported", args = 1)]
    pub fn is_mixed_lighting_mode_supported(
        mixed_mode: crate::unity_engine::mixedlightingmode::MixedLightingMode,
    ) -> bool;

    #[method(name = "IsMixedLightingModeSupportedByRef", args = 2)]
    pub fn is_mixed_lighting_mode_supported_by_ref(
        mixed_mode: crate::unity_engine::mixedlightingmode::MixedLightingMode,
        is_supported_ptr: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "IsLightmapBakeTypeSupported", args = 1)]
    pub fn is_lightmap_bake_type_supported(
        bake_type: crate::unity_engine::lightmapbaketype::LightmapBakeType,
    ) -> bool;

    #[method(name = "IsLightmapBakeTypeSupportedByRef", args = 2)]
    pub fn is_lightmap_bake_type_supported_by_ref(
        bake_type: crate::unity_engine::lightmapbaketype::LightmapBakeType,
        is_supported_ptr: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "IsLightmapsModeSupportedByRef", args = 2)]
    pub fn is_lightmaps_mode_supported_by_ref(
        mode: crate::unity_engine::lightmapsmode::LightmapsMode,
        is_supported_ptr: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "IsLightmapperSupportedByRef", args = 2)]
    pub fn is_lightmapper_supported_by_ref(
        lightmapper: i32,
        is_supported_ptr: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "IsUIOverlayRenderedBySRP", args = 1)]
    pub fn is_ui_overlay_rendered_by_srp(is_supported_ptr: ::unity2::IntPtr) -> ();

    #[method(name = "IsAutoAmbientProbeBakingSupported", args = 1)]
    pub fn is_auto_ambient_probe_baking_supported(is_supported_ptr: ::unity2::IntPtr) -> ();

    #[method(name = "IsAutoDefaultReflectionProbeBakingSupported", args = 1)]
    pub fn is_auto_default_reflection_probe_baking_supported(
        is_supported_ptr: ::unity2::IntPtr,
    ) -> ();

    #[method(name = "FallbackLightmapperByRef", args = 1)]
    pub fn fallback_lightmapper_by_ref(lightmapper_ptr: ::unity2::IntPtr) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-supportedrenderingfeatures")]
impl SupportedRenderingFeatures {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SupportedRenderingFeatures),
                ::core::stringify!(new),
            )
        });
        <Self as ISupportedRenderingFeaturesMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/supportedrenderingfeatures/SupportedRenderingFeatures_LightmapMixedBakeModes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SupportedRenderingFeatures_LightmapMixedBakeModes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SupportedRenderingFeatures_LightmapMixedBakeModes {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "SupportedRenderingFeatures.LightmapMixedBakeModes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SupportedRenderingFeatures_LightmapMixedBakeModes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SupportedRenderingFeatures_LightmapMixedBakeModes {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn indirect_only() -> Self {
        Self { value: 1 }
    }

    pub fn subtractive() -> Self {
        Self { value: 2 }
    }

    pub fn shadowmask() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/supportedrenderingfeatures/SupportedRenderingFeatures_ReflectionProbeModes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SupportedRenderingFeatures_ReflectionProbeModes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SupportedRenderingFeatures_ReflectionProbeModes {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "SupportedRenderingFeatures.ReflectionProbeModes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SupportedRenderingFeatures_ReflectionProbeModes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SupportedRenderingFeatures_ReflectionProbeModes {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn rotation() -> Self {
        Self { value: 1 }
    }
}
