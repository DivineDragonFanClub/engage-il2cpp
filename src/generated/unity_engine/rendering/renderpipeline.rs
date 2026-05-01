
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/renderpipeline/RenderPipeline.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "RenderPipeline")]
#[parent(crate::system::object::Object)]
pub struct RenderPipeline {}

#[cfg(feature = "unity_engine-rendering-renderpipeline")]
#[::unity2::methods]
impl RenderPipeline {
    #[method(name = "Render", args = 2)]
    pub fn render(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        cameras: ::unity2::Array<crate::unity_engine::camera::Camera>,
    ) -> ();

    #[method(name = "ProcessRenderRequests", args = 3)]
    pub fn process_render_requests(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        camera: crate::unity_engine::camera::Camera,
        render_requests: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::camera::Camera_RenderRequest,
        >,
    ) -> ();

    #[method(name = "BeginFrameRendering", args = 2)]
    pub fn begin_frame_rendering(
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        cameras: ::unity2::Array<crate::unity_engine::camera::Camera>,
    ) -> ();

    #[method(name = "BeginCameraRendering", args = 2)]
    pub fn begin_camera_rendering(
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        camera: crate::unity_engine::camera::Camera,
    ) -> ();

    #[method(name = "EndFrameRendering", args = 2)]
    pub fn end_frame_rendering(
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        cameras: ::unity2::Array<crate::unity_engine::camera::Camera>,
    ) -> ();

    #[method(name = "EndCameraRendering", args = 2)]
    pub fn end_camera_rendering(
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        camera: crate::unity_engine::camera::Camera,
    ) -> ();

    #[method(name = "InternalRender", args = 2)]
    pub fn internal_render(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        cameras: ::unity2::Array<crate::unity_engine::camera::Camera>,
    ) -> ();

    #[method(name = "InternalRenderWithRequests", args = 3)]
    pub fn internal_render_with_requests(
        self,
        context: crate::unity_engine::rendering::scriptablerendercontext::ScriptableRenderContext,
        cameras: ::unity2::Array<crate::unity_engine::camera::Camera>,
        render_requests: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::camera::Camera_RenderRequest,
        >,
    ) -> ();

    #[method(name = "get_disposed", args = 0)]
    pub fn get_disposed(self) -> bool;

    #[method(name = "set_disposed", args = 1)]
    pub fn set_disposed(self, value: bool) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose_2(self, disposing: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-renderpipeline")]
impl RenderPipeline {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderPipeline),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderPipelineMethods>::ctor(this);
        this
    }
}
