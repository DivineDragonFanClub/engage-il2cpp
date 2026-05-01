
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/ipixelperfectcamera_interface/IPixelPerfectCamera_Interface.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "IPixelPerfectCamera"
)]
pub struct IPixelPerfectCamera_Interface {}

#[cfg(feature = "unity_engine-experimental-rendering-universal-ipixelperfectcamera_interface")]
#[::unity2::methods]
impl IPixelPerfectCamera_Interface {
    #[method(name = "get_assetsPPU", args = 0)]
    pub fn get_assets_ppu(self) -> i32;

    #[method(name = "set_assetsPPU", args = 1)]
    pub fn set_assets_ppu(self, value: i32) -> ();

    #[method(name = "get_refResolutionX", args = 0)]
    pub fn get_ref_resolution_x(self) -> i32;

    #[method(name = "set_refResolutionX", args = 1)]
    pub fn set_ref_resolution_x(self, value: i32) -> ();

    #[method(name = "get_refResolutionY", args = 0)]
    pub fn get_ref_resolution_y(self) -> i32;

    #[method(name = "set_refResolutionY", args = 1)]
    pub fn set_ref_resolution_y(self, value: i32) -> ();

    #[method(name = "get_upscaleRT", args = 0)]
    pub fn get_upscale_rt(self) -> bool;

    #[method(name = "set_upscaleRT", args = 1)]
    pub fn set_upscale_rt(self, value: bool) -> ();

    #[method(name = "get_pixelSnapping", args = 0)]
    pub fn get_pixel_snapping(self) -> bool;

    #[method(name = "set_pixelSnapping", args = 1)]
    pub fn set_pixel_snapping(self, value: bool) -> ();

    #[method(name = "get_cropFrameX", args = 0)]
    pub fn get_crop_frame_x(self) -> bool;

    #[method(name = "set_cropFrameX", args = 1)]
    pub fn set_crop_frame_x(self, value: bool) -> ();

    #[method(name = "get_cropFrameY", args = 0)]
    pub fn get_crop_frame_y(self) -> bool;

    #[method(name = "set_cropFrameY", args = 1)]
    pub fn set_crop_frame_y(self, value: bool) -> ();

    #[method(name = "get_stretchFill", args = 0)]
    pub fn get_stretch_fill(self) -> bool;

    #[method(name = "set_stretchFill", args = 1)]
    pub fn set_stretch_fill(self, value: bool) -> ();
}
