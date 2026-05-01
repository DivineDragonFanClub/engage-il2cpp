
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/light2d_2/Light2D_2.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "Light2D"
)]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct Light2D_2 {
# [rename (name = "m_LightType")] pub m_light_type : crate :: unity_engine :: experimental :: rendering :: universal :: light2d_2 :: Light2D_LightType ,
# [rename (name = "m_BlendStyleIndex")] pub m_blend_style_index : i32 ,
# [rename (name = "m_FalloffIntensity")] pub m_falloff_intensity : f32 ,
# [rename (name = "m_Color")] pub m_color : crate :: unity_engine :: color :: Color ,
# [rename (name = "m_Intensity")] pub m_intensity : f32 ,
# [rename (name = "m_LightVolumeOpacity")] pub m_light_volume_opacity : f32 ,
# [rename (name = "m_ApplyToSortingLayers")] pub m_apply_to_sorting_layers : :: unity2 :: Array < i32 > ,
# [rename (name = "m_LightCookieSprite")] pub m_light_cookie_sprite : crate :: unity_engine :: sprite :: Sprite ,
# [rename (name = "m_UseNormalMap")] pub m_use_normal_map : bool ,
# [rename (name = "m_LightOrder")] pub m_light_order : i32 ,
# [rename (name = "m_AlphaBlendOnOverlap")] pub m_alpha_blend_on_overlap : bool ,
# [rename (name = "m_ShadowIntensity")] pub m_shadow_intensity : f32 ,
# [rename (name = "m_ShadowVolumeIntensity")] pub m_shadow_volume_intensity : f32 ,
# [rename (name = "m_PreviousLightCookieSprite")] pub m_previous_light_cookie_sprite : i32 ,
# [rename (name = "m_Mesh")] pub m_mesh : crate :: unity_engine :: mesh :: Mesh ,
# [rename (name = "m_LocalBounds")] pub m_local_bounds : crate :: unity_engine :: bounds :: Bounds ,
# [rename (name = "m_PointLightInnerAngle")] pub m_point_light_inner_angle : f32 ,
# [rename (name = "m_PointLightOuterAngle")] pub m_point_light_outer_angle : f32 ,
# [rename (name = "m_PointLightInnerRadius")] pub m_point_light_inner_radius : f32 ,
# [rename (name = "m_PointLightOuterRadius")] pub m_point_light_outer_radius : f32 ,
# [rename (name = "m_PointLightDistance")] pub m_point_light_distance : f32 ,
# [rename (name = "m_PointLightQuality")] pub m_point_light_quality : crate :: unity_engine :: experimental :: rendering :: universal :: light2d_2 :: Light2D_PointLightQuality ,
# [rename (name = "m_ShapeLightParametricSides")] pub m_shape_light_parametric_sides : i32 ,
# [rename (name = "m_ShapeLightParametricAngleOffset")] pub m_shape_light_parametric_angle_offset : f32 ,
# [rename (name = "m_ShapeLightParametricRadius")] pub m_shape_light_parametric_radius : f32 ,
# [rename (name = "m_ShapeLightFalloffSize")] pub m_shape_light_falloff_size : f32 ,
# [rename (name = "m_ShapeLightFalloffOffset")] pub m_shape_light_falloff_offset : crate :: unity_engine :: vector2 :: Vector2 ,
# [rename (name = "m_ShapePath")] pub m_shape_path : :: unity2 :: Array < crate :: unity_engine :: vector3 :: Vector3 > ,
# [rename (name = "m_PreviousShapeLightFalloffSize")] pub m_previous_shape_light_falloff_size : f32 ,
# [rename (name = "m_PreviousShapeLightParametricSides")] pub m_previous_shape_light_parametric_sides : i32 ,
# [rename (name = "m_PreviousShapeLightParametricAngleOffset")] pub m_previous_shape_light_parametric_angle_offset : f32 ,
# [rename (name = "m_PreviousShapeLightParametricRadius")] pub m_previous_shape_light_parametric_radius : f32 ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-light2d_2")]
#[::unity2::methods]
impl Light2D_2 {
    #[method(name = "get_affectedSortingLayers", args = 0)]
    pub fn get_affected_sorting_layers(self) -> ::unity2::Array<i32>;

    #[method(name = "get_lightCookieSpriteInstanceID", args = 0)]
    pub fn get_light_cookie_sprite_instance_id(self) -> i32;

    #[method(name = "get_boundingSphere", args = 0)]
    pub fn get_bounding_sphere(self) -> crate::unity_engine::boundingsphere::BoundingSphere;

    #[method(name = "set_boundingSphere", args = 1)]
    pub fn set_bounding_sphere(
        self,
        value: crate::unity_engine::boundingsphere::BoundingSphere,
    ) -> ();

    #[method(name = "get_lightMesh", args = 0)]
    pub fn get_light_mesh(self) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "get_lightType", args = 0)]
    pub fn get_light_type(
        self,
    ) -> crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_LightType;

    #[method(name = "set_lightType", args = 1)]
    pub fn set_light_type(
        self,
        value : crate :: unity_engine :: experimental :: rendering :: universal :: light2d_2 :: Light2D_LightType,
    ) -> ();

    #[method(name = "get_blendStyleIndex", args = 0)]
    pub fn get_blend_style_index(self) -> i32;

    #[method(name = "set_blendStyleIndex", args = 1)]
    pub fn set_blend_style_index(self, value: i32) -> ();

    #[method(name = "get_shadowIntensity", args = 0)]
    pub fn get_shadow_intensity(self) -> f32;

    #[method(name = "set_shadowIntensity", args = 1)]
    pub fn set_shadow_intensity(self, value: f32) -> ();

    #[method(name = "get_shadowVolumeIntensity", args = 0)]
    pub fn get_shadow_volume_intensity(self) -> f32;

    #[method(name = "set_shadowVolumeIntensity", args = 1)]
    pub fn set_shadow_volume_intensity(self, value: f32) -> ();

    #[method(name = "get_color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_color", args = 1)]
    pub fn set_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_intensity", args = 0)]
    pub fn get_intensity(self) -> f32;

    #[method(name = "set_intensity", args = 1)]
    pub fn set_intensity(self, value: f32) -> ();

    #[method(name = "get_volumeOpacity", args = 0)]
    pub fn get_volume_opacity(self) -> f32;

    #[method(name = "get_lightCookieSprite", args = 0)]
    pub fn get_light_cookie_sprite(self) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "get_falloffIntensity", args = 0)]
    pub fn get_falloff_intensity(self) -> f32;

    #[method(name = "get_useNormalMap", args = 0)]
    pub fn get_use_normal_map(self) -> bool;

    #[method(name = "get_alphaBlendOnOverlap", args = 0)]
    pub fn get_alpha_blend_on_overlap(self) -> bool;

    #[method(name = "get_lightOrder", args = 0)]
    pub fn get_light_order(self) -> i32;

    #[method(name = "set_lightOrder", args = 1)]
    pub fn set_light_order(self, value: i32) -> ();

    #[method(name = "GetTopMostLitLayer", args = 0)]
    pub fn get_top_most_lit_layer(self) -> i32;

    #[method(name = "UpdateMesh", args = 0)]
    pub fn update_mesh(self) -> ();

    #[method(name = "UpdateBoundingSphere", args = 0)]
    pub fn update_bounding_sphere(self) -> ();

    #[method(name = "IsLitLayer", args = 1)]
    pub fn is_lit_layer(self, layer: i32) -> bool;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "get_pointLightInnerAngle", args = 0)]
    pub fn get_point_light_inner_angle(self) -> f32;

    #[method(name = "set_pointLightInnerAngle", args = 1)]
    pub fn set_point_light_inner_angle(self, value: f32) -> ();

    #[method(name = "get_pointLightOuterAngle", args = 0)]
    pub fn get_point_light_outer_angle(self) -> f32;

    #[method(name = "set_pointLightOuterAngle", args = 1)]
    pub fn set_point_light_outer_angle(self, value: f32) -> ();

    #[method(name = "get_pointLightInnerRadius", args = 0)]
    pub fn get_point_light_inner_radius(self) -> f32;

    #[method(name = "set_pointLightInnerRadius", args = 1)]
    pub fn set_point_light_inner_radius(self, value: f32) -> ();

    #[method(name = "get_pointLightOuterRadius", args = 0)]
    pub fn get_point_light_outer_radius(self) -> f32;

    #[method(name = "set_pointLightOuterRadius", args = 1)]
    pub fn set_point_light_outer_radius(self, value: f32) -> ();

    #[method(name = "get_pointLightDistance", args = 0)]
    pub fn get_point_light_distance(self) -> f32;

    #[method(name = "get_pointLightQuality", args = 0)]
    pub fn get_point_light_quality(
        self,
    ) -> crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_PointLightQuality;

    #[method(name = "get_isPointLight", args = 0)]
    pub fn get_is_point_light(self) -> bool;

    #[method(name = "get_shapeLightParametricSides", args = 0)]
    pub fn get_shape_light_parametric_sides(self) -> i32;

    #[method(name = "get_shapeLightParametricAngleOffset", args = 0)]
    pub fn get_shape_light_parametric_angle_offset(self) -> f32;

    #[method(name = "get_shapeLightParametricRadius", args = 0)]
    pub fn get_shape_light_parametric_radius(self) -> f32;

    #[method(name = "get_shapeLightFalloffSize", args = 0)]
    pub fn get_shape_light_falloff_size(self) -> f32;

    #[method(name = "get_shapeLightFalloffOffset", args = 0)]
    pub fn get_shape_light_falloff_offset(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "get_shapePath", args = 0)]
    pub fn get_shape_path(self) -> ::unity2::Array<crate::unity_engine::vector3::Vector3>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-light2d_2")]
impl Light2D_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Light2D_2),
                ::core::stringify!(new),
            )
        });
        <Self as ILight2D_2Methods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/light2d_2/Light2D_PointLightQuality.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Light2D_PointLightQuality {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Light2D_PointLightQuality {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.Universal";

    const NAME: &'static str = "Light2D.PointLightQuality";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Light2D_PointLightQuality {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Light2D_PointLightQuality {
    pub fn fast() -> Self {
        Self { value: 0 }
    }

    pub fn accurate() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/light2d_2/Light2D_LightType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Light2D_LightType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Light2D_LightType {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.Universal";

    const NAME: &'static str = "Light2D.LightType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Light2D_LightType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Light2D_LightType {
    pub fn parametric() -> Self {
        Self { value: 0 }
    }

    pub fn freeform() -> Self {
        Self { value: 1 }
    }

    pub fn sprite() -> Self {
        Self { value: 2 }
    }

    pub fn point() -> Self {
        Self { value: 3 }
    }

    pub fn global() -> Self {
        Self { value: 4 }
    }
}
