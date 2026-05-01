
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/global_illumination/linearcolor/LinearColor.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct LinearColor {
    pub m_red: f32,
    pub m_green: f32,
    pub m_blue: f32,
    pub m_intensity: f32,
}

impl ::unity2::ClassIdentity for LinearColor {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.GlobalIllumination";

    const NAME: &'static str = "LinearColor";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for LinearColor {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-global_illumination-linearcolor")]
#[::unity2::methods(value)]
impl LinearColor {
    #[method(name = "get_red", args = 0)]
    pub fn get_red(self) -> f32;

    #[method(name = "set_red", args = 1)]
    pub fn set_red(self, value: f32) -> ();

    #[method(name = "get_green", args = 0)]
    pub fn get_green(self) -> f32;

    #[method(name = "set_green", args = 1)]
    pub fn set_green(self, value: f32) -> ();

    #[method(name = "get_blue", args = 0)]
    pub fn get_blue(self) -> f32;

    #[method(name = "set_blue", args = 1)]
    pub fn set_blue(self, value: f32) -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        color: crate::unity_engine::color::Color,
        intensity: f32,
    ) -> crate::unity_engine::experimental::global_illumination::linearcolor::LinearColor;

    #[method(name = "Black", args = 0)]
    pub fn black(
    ) -> crate::unity_engine::experimental::global_illumination::linearcolor::LinearColor;
}
