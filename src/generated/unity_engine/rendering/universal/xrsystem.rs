
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/xrsystem/XRSystem_XRShaderIDs.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "XRSystem.XRShaderIDs"
)]
#[parent(crate::system::object::Object)]
pub struct XRSystem_XRShaderIDs {
    #[static_field]
    #[rename(name = "_SourceTexArraySlice")]
    pub source_tex_array_slice: i32,
    #[static_field]
    #[rename(name = "_SRGBRead")]
    pub srgb_read: i32,
    #[static_field]
    #[rename(name = "_SRGBWrite")]
    pub srgb_write: i32,
}

#[cfg(feature = "unity_engine-rendering-universal-xrsystem")]
#[::unity2::methods]
impl XRSystem_XRShaderIDs {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/xrsystem/XRSystem.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "XRSystem")]
#[parent(crate::system::object::Object)]
pub struct XRSystem {
    #[rename(name = "emptyPass")]
    pub empty_pass: crate::unity_engine::rendering::universal::xrpass::XRPass,
    #[rename(name = "framePasses")]
    pub frame_passes: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::universal::xrpass::XRPass,
    >,
    #[static_field]
    #[rename(name = "displayList")]
    pub display_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::xr::xrdisplaysubsystem::XRDisplaySubsystem,
    >,
    #[rename(name = "display")]
    pub display: crate::unity_engine::xr::xrdisplaysubsystem::XRDisplaySubsystem,
    #[static_field]
    #[rename(name = "msaaLevel")]
    pub msaa_level: i32,
    #[rename(name = "occlusionMeshMaterial")]
    pub occlusion_mesh_material: crate::unity_engine::material::Material,
    #[rename(name = "mirrorViewMaterial")]
    pub mirror_view_material: crate::unity_engine::material::Material,
    #[rename(name = "mirrorViewMaterialProperty")]
    pub mirror_view_material_property:
        crate::unity_engine::materialpropertyblock::MaterialPropertyBlock,
    #[rename(name = "testRenderTexture")]
    pub test_render_texture: crate::unity_engine::rendertexture::RenderTexture,
    #[static_field]
    #[rename(name = "k_XRMirrorTag")]
    pub k_xr_mirror_tag: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "_XRMirrorProfilingSampler")]
    pub xr_mirror_profiling_sampler:
        crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
}

#[cfg(feature = "unity_engine-rendering-universal-xrsystem")]
#[::unity2::methods]
impl XRSystem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "InitializeXRSystemData", args = 1)]
    pub fn initialize_xr_system_data(
        self,
        data: crate::unity_engine::rendering::universal::xrsystemdata::XRSystemData,
    ) -> ();

    #[method(name = "GetDisplaySubsystem", args = 0)]
    pub fn get_display_subsystem() -> ();

    #[method(name = "XRSystemInit", args = 0)]
    pub fn xr_system_init() -> ();

    #[method(name = "UpdateMSAALevel", args = 1)]
    pub fn update_msaa_level(level: i32) -> ();

    #[method(name = "GetMSAALevel", args = 0)]
    pub fn get_msaa_level() -> i32;

    #[method(name = "UpdateRenderScale", args = 1)]
    pub fn update_render_scale(render_scale: f32) -> ();

    #[method(name = "GetMaxViews", args = 0)]
    pub fn get_max_views(self) -> i32;

    #[method(name = "SetupFrame", args = 1)]
    pub fn setup_frame(
        self,
        camera_data: crate::unity_engine::rendering::universal::cameradata::CameraData,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::universal::xrpass::XRPass,
    >;

    #[method(name = "ReleaseFrame", args = 0)]
    pub fn release_frame(self) -> ();

    #[method(name = "RefreshXrSdk", args = 0)]
    pub fn refresh_xr_sdk(self) -> bool;

    #[method(name = "UpdateCameraData", args = 2)]
    pub fn update_camera_data(
        self,
        base_camera_data: crate::unity_engine::rendering::universal::cameradata::CameraData,
        xr: crate::unity_engine::rendering::universal::xrpass::XRPass,
    ) -> ();

    #[method(name = "UpdateFromCamera", args = 2)]
    pub fn update_from_camera(
        self,
        xr_pass: crate::unity_engine::rendering::universal::xrpass::XRPass,
        camera_data: crate::unity_engine::rendering::universal::cameradata::CameraData,
    ) -> ();

    #[method(name = "CreateLayoutFromXrSdk", args = 2)]
    pub fn create_layout_from_xr_sdk(
        self,
        camera: crate::unity_engine::camera::Camera,
        single_pass_allowed: bool,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "AddPassToFrame", args = 1)]
    pub fn add_pass_to_frame(
        self,
        xr_pass: crate::unity_engine::rendering::universal::xrpass::XRPass,
    ) -> ();

    #[method(name = "RenderMirrorView", args = 2)]
    pub fn render_mirror_view(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        camera: crate::unity_engine::camera::Camera,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-xrsystem")]
impl XRSystem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(XRSystem),
                ::core::stringify!(new),
            )
        });
        <Self as IXRSystemMethods>::ctor(this);
        this
    }
}
