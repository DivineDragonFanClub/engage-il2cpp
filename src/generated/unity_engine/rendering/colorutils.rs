
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/colorutils/ColorUtils.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "ColorUtils")]
#[parent(crate::system::object::Object)]
pub struct ColorUtils {
    #[static_field]
    #[rename(name = "s_LightMeterCalibrationConstant")]
    pub s_light_meter_calibration_constant: f32,
    #[static_field]
    #[rename(name = "s_LensAttenuation")]
    pub s_lens_attenuation: f32,
}

#[cfg(feature = "unity_engine-rendering-colorutils")]
#[::unity2::methods]
impl ColorUtils {
    #[method(name = "get_lensImperfectionExposureScale", args = 0)]
    pub fn get_lens_imperfection_exposure_scale() -> f32;

    #[method(name = "StandardIlluminantY", args = 1)]
    pub fn standard_illuminant_y(x: f32) -> f32;

    #[method(name = "CIExyToLMS", args = 2)]
    pub fn ci_exy_to_lms(x: f32, y: f32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ColorBalanceToLMSCoeffs", args = 2)]
    pub fn color_balance_to_lms_coeffs(
        temperature: f32,
        tint: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Luminance", args = 1)]
    pub fn luminance(color: crate::unity_engine::color::Color) -> f32;

    #[method(name = "ComputeEV100", args = 3)]
    pub fn compute_ev100(aperture: f32, shutter_speed: f32, iso: f32) -> f32;

    #[method(name = "ConvertEV100ToExposure", args = 1)]
    pub fn convert_ev100_to_exposure(ev100: f32) -> f32;

    #[method(name = "ConvertExposureToEV100", args = 1)]
    pub fn convert_exposure_to_ev100(exposure: f32) -> f32;

    #[method(name = "ComputeEV100FromAvgLuminance", args = 1)]
    pub fn compute_ev100_from_avg_luminance(avg_luminance: f32) -> f32;

    #[method(name = "ComputeISO", args = 3)]
    pub fn compute_iso(aperture: f32, shutter_speed: f32, target_ev100: f32) -> f32;

    #[method(name = "ToHex", args = 1)]
    pub fn to_hex(c: crate::unity_engine::color::Color) -> u32;

    #[method(name = "ToRGBA", args = 1)]
    pub fn to_rgba(hex: u32) -> crate::unity_engine::color::Color;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
