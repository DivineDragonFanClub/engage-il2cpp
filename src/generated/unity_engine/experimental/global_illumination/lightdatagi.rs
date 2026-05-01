
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/global_illumination/lightdatagi/LightDataGI.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct LightDataGI {
    pub instance_id: i32,
    pub cookie_id: i32,
    pub cookie_scale: f32,
    pub color: crate::unity_engine::experimental::global_illumination::linearcolor::LinearColor,
    pub indirect_color:
        crate::unity_engine::experimental::global_illumination::linearcolor::LinearColor,
    pub orientation: crate::unity_engine::quaternion::Quaternion,
    pub position: crate::unity_engine::vector3::Vector3,
    pub range: f32,
    pub cone_angle: f32,
    pub inner_cone_angle: f32,
    pub shape0: f32,
    pub shape1: f32,
    pub r#type: crate::unity_engine::experimental::global_illumination::lighttype_2::LightType_2,
    pub mode: crate::unity_engine::experimental::global_illumination::lightmode::LightMode,
    pub shadow: u8,
    pub falloff: crate::unity_engine::experimental::global_illumination::fallofftype::FalloffType,
}

impl ::unity2::ClassIdentity for LightDataGI {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.GlobalIllumination";

    const NAME: &'static str = "LightDataGI";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for LightDataGI {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-global_illumination-lightdatagi")]
#[::unity2::methods(value)]
impl LightDataGI {
    #[method(name = "Init", args = 2)]
    pub fn init(
        self,
        light : crate :: unity_engine :: experimental :: global_illumination :: directionallight :: DirectionalLight,
        cookie: crate::unity_engine::experimental::global_illumination::cookie::Cookie,
    ) -> ();

    #[method(name = "Init", args = 2)]
    pub fn init_2(
        self,
        light: crate::unity_engine::experimental::global_illumination::pointlight::PointLight,
        cookie: crate::unity_engine::experimental::global_illumination::cookie::Cookie,
    ) -> ();

    #[method(name = "Init", args = 2)]
    pub fn init_3(
        self,
        light: crate::unity_engine::experimental::global_illumination::spotlight::SpotLight,
        cookie: crate::unity_engine::experimental::global_illumination::cookie::Cookie,
    ) -> ();

    #[method(name = "Init", args = 2)]
    pub fn init_4(
        self,
        light : crate :: unity_engine :: experimental :: global_illumination :: rectanglelight :: RectangleLight,
        cookie: crate::unity_engine::experimental::global_illumination::cookie::Cookie,
    ) -> ();

    #[method(name = "Init", args = 2)]
    pub fn init_5(
        self,
        light: crate::unity_engine::experimental::global_illumination::disclight::DiscLight,
        cookie: crate::unity_engine::experimental::global_illumination::cookie::Cookie,
    ) -> ();

    #[method(name = "InitNoBake", args = 1)]
    pub fn init_no_bake(self, light_instance_id: i32) -> ();
}
