
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::propertyattribute::IPropertyAttribute;
use crate::unity_engine::propertyattribute::PropertyAttribute;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/colorusageattribute/ColorUsageAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ColorUsageAttribute")]
#[parent(crate::unity_engine::propertyattribute::PropertyAttribute)]
pub struct ColorUsageAttribute {
    #[rename(name = "showAlpha")]
    pub show_alpha: bool,
    #[rename(name = "hdr")]
    pub hdr: bool,
    #[rename(name = "minBrightness")]
    pub min_brightness: f32,
    #[rename(name = "maxBrightness")]
    pub max_brightness: f32,
    #[rename(name = "minExposureValue")]
    pub min_exposure_value: f32,
    #[rename(name = "maxExposureValue")]
    pub max_exposure_value: f32,
}

#[cfg(feature = "unity_engine-colorusageattribute")]
#[::unity2::methods]
impl ColorUsageAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, show_alpha: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, show_alpha: bool, hdr: bool) -> ();
}

#[cfg(feature = "unity_engine-colorusageattribute")]
impl ColorUsageAttribute {
    pub fn new(show_alpha: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ColorUsageAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IColorUsageAttributeMethods>::ctor(this, show_alpha);
        this
    }

    pub fn new_2(show_alpha: bool, hdr: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ColorUsageAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as IColorUsageAttributeMethods>::ctor_2(this, show_alpha, hdr);
        this
    }
}
