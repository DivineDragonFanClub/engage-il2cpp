
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/xrpass/XRPass.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "XRPass")]
#[parent(crate::system::object::Object)]
pub struct XRPass {
    #[rename(name = "views")]
    pub views: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::rendering::universal::xrview::XRView,
    >,
    #[static_field]
    #[rename(name = "invalidRT")]
    pub invalid_rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    #[rename(name = "occlusionMeshMaterial")]
    pub occlusion_mesh_material: crate::unity_engine::material::Material,
    #[rename(name = "occlusionMeshCombined")]
    pub occlusion_mesh_combined: crate::unity_engine::mesh::Mesh,
    #[rename(name = "occlusionMeshCombinedHashCode")]
    pub occlusion_mesh_combined_hash_code: i32,
    #[static_field]
    #[rename(name = "k_XRCustomMirrorTag")]
    pub k_xr_custom_mirror_tag: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "_XRCustomMirrorProfilingSampler")]
    pub xr_custom_mirror_profiling_sampler:
        crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    #[static_field]
    #[rename(name = "k_XROcclusionTag")]
    pub k_xr_occlusion_tag: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "_XROcclusionProfilingSampler")]
    pub xr_occlusion_profiling_sampler:
        crate::unity_engine::rendering::profilingsampler::ProfilingSampler,
    #[rename(name = "stereoEyeIndices")]
    pub stereo_eye_indices: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    #[rename(name = "stereoProjectionMatrix")]
    pub stereo_projection_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    #[rename(name = "stereoViewMatrix")]
    pub stereo_view_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    #[rename(name = "stereoCameraProjectionMatrix")]
    pub stereo_camera_projection_matrix: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
}

#[cfg(feature = "unity_engine-rendering-universal-xrpass")]
#[::unity2::methods]
impl XRPass {
    #[method(name = "get_enabled", args = 0)]
    pub fn get_enabled(self) -> bool;

    #[method(name = "get_xrSdkEnabled", args = 0)]
    pub fn get_xr_sdk_enabled(self) -> bool;

    #[method(name = "set_xrSdkEnabled", args = 1)]
    pub fn set_xr_sdk_enabled(self, value: bool) -> ();

    #[method(name = "get_copyDepth", args = 0)]
    pub fn get_copy_depth(self) -> bool;

    #[method(name = "set_copyDepth", args = 1)]
    pub fn set_copy_depth(self, value: bool) -> ();

    #[method(name = "get_multipassId", args = 0)]
    pub fn get_multipass_id(self) -> i32;

    #[method(name = "set_multipassId", args = 1)]
    pub fn set_multipass_id(self, value: i32) -> ();

    #[method(name = "get_cullingPassId", args = 0)]
    pub fn get_culling_pass_id(self) -> i32;

    #[method(name = "set_cullingPassId", args = 1)]
    pub fn set_culling_pass_id(self, value: i32) -> ();

    #[method(name = "get_renderTarget", args = 0)]
    pub fn get_render_target(
        self,
    ) -> crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier;

    #[method(name = "set_renderTarget", args = 1)]
    pub fn set_render_target(
        self,
        value: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
    ) -> ();

    #[method(name = "get_renderTargetDesc", args = 0)]
    pub fn get_render_target_desc(
        self,
    ) -> crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor;

    #[method(name = "set_renderTargetDesc", args = 1)]
    pub fn set_render_target_desc(
        self,
        value: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();

    #[method(name = "get_renderTargetValid", args = 0)]
    pub fn get_render_target_valid(self) -> bool;

    #[method(name = "get_renderTargetIsRenderTexture", args = 0)]
    pub fn get_render_target_is_render_texture(self) -> bool;

    #[method(name = "set_renderTargetIsRenderTexture", args = 1)]
    pub fn set_render_target_is_render_texture(self, value: bool) -> ();

    #[method(name = "GetProjMatrix", args = 1)]
    pub fn get_proj_matrix(self, view_index: i32) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "GetViewMatrix", args = 1)]
    pub fn get_view_matrix(self, view_index: i32) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "GetTextureArraySlice", args = 1)]
    pub fn get_texture_array_slice(self, view_index: i32) -> i32;

    #[method(name = "GetViewport", args = 1)]
    pub fn get_viewport(self, view_index: i32) -> crate::unity_engine::rect::Rect;

    #[method(name = "get_cullingParams", args = 0)]
    pub fn get_culling_params(
        self,
    ) -> crate::unity_engine::rendering::scriptablecullingparameters::ScriptableCullingParameters;

    #[method(name = "set_cullingParams", args = 1)]
    pub fn set_culling_params(
        self,
        value : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
    ) -> ();

    #[method(name = "get_viewCount", args = 0)]
    pub fn get_view_count(self) -> i32;

    #[method(name = "get_singlePassEnabled", args = 0)]
    pub fn get_single_pass_enabled(self) -> bool;

    #[method(name = "get_isOcclusionMeshSupported", args = 0)]
    pub fn get_is_occlusion_mesh_supported(self) -> bool;

    #[method(name = "get_hasValidOcclusionMesh", args = 0)]
    pub fn get_has_valid_occlusion_mesh(self) -> bool;

    #[method(name = "SetCustomMirrorView", args = 1)]
    pub fn set_custom_mirror_view(
        self,
        callback: crate::unity_engine::rendering::universal::xrpass::XRPass_CustomMirrorView,
    ) -> ();

    #[method(name = "Create", args = 1)]
    pub fn create(
        create_info: crate::unity_engine::rendering::universal::xrpasscreateinfo::XRPassCreateInfo,
    ) -> crate::unity_engine::rendering::universal::xrpass::XRPass;

    #[method(name = "UpdateView", args = 3)]
    pub fn update_view(
        self,
        view_id: i32,
        xr_sdk_render_pass : crate :: unity_engine :: xr :: xrdisplaysubsystem :: XRDisplaySubsystem_XRRenderPass,
        xr_sdk_render_parameter : crate :: unity_engine :: xr :: xrdisplaysubsystem :: XRDisplaySubsystem_XRRenderParameter,
    ) -> ();

    #[method(name = "UpdateView", args = 5)]
    pub fn update_view_2(
        self,
        view_id: i32,
        proj: crate::unity_engine::matrix4x4::Matrix4x4,
        view: crate::unity_engine::matrix4x4::Matrix4x4,
        vp: crate::unity_engine::rect::Rect,
        texture_array_slice: i32,
    ) -> ();

    #[method(name = "UpdateCullingParams", args = 2)]
    pub fn update_culling_params(
        self,
        culling_pass_id: i32,
        culling_params : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
    ) -> ();

    #[method(name = "AddView", args = 4)]
    pub fn add_view(
        self,
        proj: crate::unity_engine::matrix4x4::Matrix4x4,
        view: crate::unity_engine::matrix4x4::Matrix4x4,
        vp: crate::unity_engine::rect::Rect,
        texture_array_slice: i32,
    ) -> ();

    #[method(name = "Create", args = 4)]
    pub fn create_2(
        xr_render_pass : crate :: unity_engine :: xr :: xrdisplaysubsystem :: XRDisplaySubsystem_XRRenderPass,
        multipass_id: i32,
        culling_parameters : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
        occlusion_mesh_material: crate::unity_engine::material::Material,
    ) -> crate::unity_engine::rendering::universal::xrpass::XRPass;

    #[method(name = "AddView", args = 2)]
    pub fn add_view_2(
        self,
        xr_sdk_render_pass : crate :: unity_engine :: xr :: xrdisplaysubsystem :: XRDisplaySubsystem_XRRenderPass,
        xr_sdk_render_parameter : crate :: unity_engine :: xr :: xrdisplaysubsystem :: XRDisplaySubsystem_XRRenderParameter,
    ) -> ();

    #[method(name = "Release", args = 1)]
    pub fn release(xr_pass: crate::unity_engine::rendering::universal::xrpass::XRPass) -> ();

    #[method(name = "AddViewInternal", args = 1)]
    pub fn add_view_internal(
        self,
        xr_view: crate::unity_engine::rendering::universal::xrview::XRView,
    ) -> ();

    #[method(name = "UpdateOcclusionMesh", args = 0)]
    pub fn update_occlusion_mesh(self) -> ();

    #[method(name = "TryGetOcclusionMeshCombinedHashCode", args = 1)]
    pub fn try_get_occlusion_mesh_combined_hash_code(self, hash_code: i32) -> bool;

    #[method(name = "CreateOcclusionMeshCombined", args = 0)]
    pub fn create_occlusion_mesh_combined(self) -> ();

    #[method(name = "StartSinglePass", args = 1)]
    pub fn start_single_pass(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "StopSinglePass", args = 1)]
    pub fn stop_single_pass(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "EndCamera", args = 2)]
    pub fn end_camera(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        camera_data: crate::unity_engine::rendering::universal::cameradata::CameraData,
    ) -> ();

    #[method(name = "RenderOcclusionMesh", args = 1)]
    pub fn render_occlusion_mesh(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "UpdateGPUViewAndProjectionMatrices", args = 3)]
    pub fn update_gpu_view_and_projection_matrices(
        self,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        camera_data: crate::unity_engine::rendering::universal::cameradata::CameraData,
        is_render_to_texture: bool,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-xrpass")]
impl XRPass {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(XRPass),
                ::core::stringify!(new),
            )
        });
        <Self as IXRPassMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/xrpass/XRPass_CustomMirrorView.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "XRPass.CustomMirrorView"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct XRPass_CustomMirrorView {}

#[cfg(feature = "unity_engine-rendering-universal-xrpass")]
#[::unity2::methods]
impl XRPass_CustomMirrorView {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        pass: crate::unity_engine::rendering::universal::xrpass::XRPass,
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        rt: crate::unity_engine::rendertexture::RenderTexture,
        viewport: crate::unity_engine::rect::Rect,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-xrpass")]
impl XRPass_CustomMirrorView {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(XRPass_CustomMirrorView),
                ::core::stringify!(new),
            )
        });
        <Self as IXRPass_CustomMirrorViewMethods>::ctor(this, object, method);
        this
    }
}
