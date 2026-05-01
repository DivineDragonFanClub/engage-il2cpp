
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::renderer::IRenderer;
use crate::unity_engine::renderer::Renderer;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/linerenderer/LineRenderer.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "LineRenderer")]
#[parent(crate::unity_engine::renderer::Renderer)]
pub struct LineRenderer {}

#[cfg(feature = "unity_engine-linerenderer")]
#[::unity2::methods]
impl LineRenderer {
    #[method(name = "SetWidth", args = 2)]
    pub fn set_width(self, start: f32, end: f32) -> ();

    #[method(name = "SetColors", args = 2)]
    pub fn set_colors(
        self,
        start: crate::unity_engine::color::Color,
        end: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetVertexCount", args = 1)]
    pub fn set_vertex_count(self, count: i32) -> ();

    #[method(name = "get_numPositions", args = 0)]
    pub fn get_num_positions(self) -> i32;

    #[method(name = "set_numPositions", args = 1)]
    pub fn set_num_positions(self, value: i32) -> ();

    #[method(name = "get_startWidth", args = 0)]
    pub fn get_start_width(self) -> f32;

    #[method(name = "set_startWidth", args = 1)]
    pub fn set_start_width(self, value: f32) -> ();

    #[method(name = "get_endWidth", args = 0)]
    pub fn get_end_width(self) -> f32;

    #[method(name = "set_endWidth", args = 1)]
    pub fn set_end_width(self, value: f32) -> ();

    #[method(name = "get_widthMultiplier", args = 0)]
    pub fn get_width_multiplier(self) -> f32;

    #[method(name = "set_widthMultiplier", args = 1)]
    pub fn set_width_multiplier(self, value: f32) -> ();

    #[method(name = "get_numCornerVertices", args = 0)]
    pub fn get_num_corner_vertices(self) -> i32;

    #[method(name = "set_numCornerVertices", args = 1)]
    pub fn set_num_corner_vertices(self, value: i32) -> ();

    #[method(name = "get_numCapVertices", args = 0)]
    pub fn get_num_cap_vertices(self) -> i32;

    #[method(name = "set_numCapVertices", args = 1)]
    pub fn set_num_cap_vertices(self, value: i32) -> ();

    #[method(name = "get_useWorldSpace", args = 0)]
    pub fn get_use_world_space(self) -> bool;

    #[method(name = "set_useWorldSpace", args = 1)]
    pub fn set_use_world_space(self, value: bool) -> ();

    #[method(name = "get_loop", args = 0)]
    pub fn get_loop(self) -> bool;

    #[method(name = "set_loop", args = 1)]
    pub fn set_loop(self, value: bool) -> ();

    #[method(name = "get_startColor", args = 0)]
    pub fn get_start_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_startColor", args = 1)]
    pub fn set_start_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_endColor", args = 0)]
    pub fn get_end_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_endColor", args = 1)]
    pub fn set_end_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_positionCount", args = 0)]
    pub fn get_position_count(self) -> i32;

    #[method(name = "set_positionCount", args = 1)]
    pub fn set_position_count(self, value: i32) -> ();

    #[method(name = "SetPosition", args = 2)]
    pub fn set_position(self, index: i32, position: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "GetPosition", args = 1)]
    pub fn get_position(self, index: i32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_shadowBias", args = 0)]
    pub fn get_shadow_bias(self) -> f32;

    #[method(name = "set_shadowBias", args = 1)]
    pub fn set_shadow_bias(self, value: f32) -> ();

    #[method(name = "get_generateLightingData", args = 0)]
    pub fn get_generate_lighting_data(self) -> bool;

    #[method(name = "set_generateLightingData", args = 1)]
    pub fn set_generate_lighting_data(self, value: bool) -> ();

    #[method(name = "get_textureMode", args = 0)]
    pub fn get_texture_mode(self) -> crate::unity_engine::linetexturemode::LineTextureMode;

    #[method(name = "set_textureMode", args = 1)]
    pub fn set_texture_mode(
        self,
        value: crate::unity_engine::linetexturemode::LineTextureMode,
    ) -> ();

    #[method(name = "get_alignment", args = 0)]
    pub fn get_alignment(self) -> crate::unity_engine::linealignment::LineAlignment;

    #[method(name = "set_alignment", args = 1)]
    pub fn set_alignment(self, value: crate::unity_engine::linealignment::LineAlignment) -> ();

    #[method(name = "Simplify", args = 1)]
    pub fn simplify(self, tolerance: f32) -> ();

    #[method(name = "BakeMesh", args = 2)]
    pub fn bake_mesh(self, mesh: crate::unity_engine::mesh::Mesh, use_transform: bool) -> ();

    #[method(name = "BakeMesh", args = 3)]
    pub fn bake_mesh_2(
        self,
        mesh: crate::unity_engine::mesh::Mesh,
        camera: crate::unity_engine::camera::Camera,
        use_transform: bool,
    ) -> ();

    #[method(name = "get_widthCurve", args = 0)]
    pub fn get_width_curve(self) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "set_widthCurve", args = 1)]
    pub fn set_width_curve(self, value: crate::unity_engine::animationcurve::AnimationCurve) -> ();

    #[method(name = "get_colorGradient", args = 0)]
    pub fn get_color_gradient(self) -> crate::unity_engine::gradient::Gradient;

    #[method(name = "set_colorGradient", args = 1)]
    pub fn set_color_gradient(self, value: crate::unity_engine::gradient::Gradient) -> ();

    #[method(name = "GetWidthCurveCopy", args = 0)]
    pub fn get_width_curve_copy(self) -> crate::unity_engine::animationcurve::AnimationCurve;

    #[method(name = "GetColorGradientCopy", args = 0)]
    pub fn get_color_gradient_copy(self) -> crate::unity_engine::gradient::Gradient;

    #[method(name = "GetPositions", args = 1)]
    pub fn get_positions(
        self,
        positions: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> i32;

    #[method(name = "SetPositions", args = 1)]
    pub fn set_positions(
        self,
        positions: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> ();

    #[method(name = "SetPositionsWithNativeContainer", args = 2)]
    pub fn set_positions_with_native_container(self, positions: ::unity2::IntPtr, count: i32)
        -> ();

    #[method(name = "GetPositionsWithNativeContainer", args = 2)]
    pub fn get_positions_with_native_container(
        self,
        positions: ::unity2::IntPtr,
        length: i32,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_startColor_Injected", args = 1)]
    pub fn get_start_color_injected(self, ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_startColor_Injected", args = 1)]
    pub fn set_start_color_injected(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_endColor_Injected", args = 1)]
    pub fn get_end_color_injected(self, ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_endColor_Injected", args = 1)]
    pub fn set_end_color_injected(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetPosition_Injected", args = 2)]
    pub fn set_position_injected(
        self,
        index: i32,
        position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetPosition_Injected", args = 2)]
    pub fn get_position_injected(
        self,
        index: i32,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();
}

#[cfg(feature = "unity_engine-linerenderer")]
impl LineRenderer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LineRenderer),
                ::core::stringify!(new),
            )
        });
        <Self as ILineRendererMethods>::ctor(this);
        this
    }
}
