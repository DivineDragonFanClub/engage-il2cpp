
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/look_dev/idataprovider/IDataProvider.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.LookDev", name = "IDataProvider")]
pub struct IDataProvider {}

#[cfg(feature = "unity_engine-rendering-look_dev-idataprovider")]
#[::unity2::methods]
impl IDataProvider {
    #[method(name = "FirstInitScene", args = 1)]
    pub fn first_init_scene(
        self,
        stage : crate :: unity_engine :: rendering :: look_dev :: stageruntimeinterface :: StageRuntimeInterface,
    ) -> ();

    #[method(name = "UpdateSky", args = 3)]
    pub fn update_sky(
        self,
        camera: crate::unity_engine::camera::Camera,
        sky: crate::unity_engine::rendering::look_dev::sky::Sky,
        stage : crate :: unity_engine :: rendering :: look_dev :: stageruntimeinterface :: StageRuntimeInterface,
    ) -> ();

    #[method(name = "get_supportedDebugModes", args = 0)]
    pub fn get_supported_debug_modes(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<::unity2::Il2CppString>;

    #[method(name = "UpdateDebugMode", args = 1)]
    pub fn update_debug_mode(self, debug_index: i32) -> ();

    #[method(name = "GetShadowMask", args = 2)]
    pub fn get_shadow_mask(
        self,
        output: crate::unity_engine::rendertexture::RenderTexture,
        stage : crate :: unity_engine :: rendering :: look_dev :: stageruntimeinterface :: StageRuntimeInterface,
    ) -> ();

    #[method(name = "OnBeginRendering", args = 1)]
    pub fn on_begin_rendering(
        self,
        stage : crate :: unity_engine :: rendering :: look_dev :: stageruntimeinterface :: StageRuntimeInterface,
    ) -> ();

    #[method(name = "OnEndRendering", args = 1)]
    pub fn on_end_rendering(
        self,
        stage : crate :: unity_engine :: rendering :: look_dev :: stageruntimeinterface :: StageRuntimeInterface,
    ) -> ();

    #[method(name = "Cleanup", args = 1)]
    pub fn cleanup(
        self,
        sri: crate::unity_engine::rendering::look_dev::stageruntimeinterface::StageRuntimeInterface,
    ) -> ();
}
