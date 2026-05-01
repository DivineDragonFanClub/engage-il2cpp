
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/material/Material.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Material")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct Material {}

#[cfg(feature = "unity_engine-material")]
#[::unity2::methods]
impl Material {
    #[method(name = "Create", args = 1)]
    pub fn create(
        script_contents: ::unity2::Il2CppString,
    ) -> crate::unity_engine::material::Material;

    #[method(name = "CreateWithShader", args = 2)]
    pub fn create_with_shader(
        self_: crate::unity_engine::material::Material,
        shader: crate::unity_engine::shader::Shader,
    ) -> ();

    #[method(name = "CreateWithMaterial", args = 2)]
    pub fn create_with_material(
        self_: crate::unity_engine::material::Material,
        source: crate::unity_engine::material::Material,
    ) -> ();

    #[method(name = "CreateWithString", args = 1)]
    pub fn create_with_string(self_: crate::unity_engine::material::Material) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, shader: crate::unity_engine::shader::Shader) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, source: crate::unity_engine::material::Material) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, contents: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDefaultMaterial", args = 0)]
    pub fn get_default_material() -> crate::unity_engine::material::Material;

    #[method(name = "GetDefaultParticleMaterial", args = 0)]
    pub fn get_default_particle_material() -> crate::unity_engine::material::Material;

    #[method(name = "GetDefaultLineMaterial", args = 0)]
    pub fn get_default_line_material() -> crate::unity_engine::material::Material;

    #[method(name = "get_shader", args = 0)]
    pub fn get_shader(self) -> crate::unity_engine::shader::Shader;

    #[method(name = "set_shader", args = 1)]
    pub fn set_shader(self, value: crate::unity_engine::shader::Shader) -> ();

    #[method(name = "get_color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_color", args = 1)]
    pub fn set_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_mainTexture", args = 0)]
    pub fn get_main_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "set_mainTexture", args = 1)]
    pub fn set_main_texture(self, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "get_mainTextureOffset", args = 0)]
    pub fn get_main_texture_offset(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_mainTextureOffset", args = 1)]
    pub fn set_main_texture_offset(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_mainTextureScale", args = 0)]
    pub fn get_main_texture_scale(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_mainTextureScale", args = 1)]
    pub fn set_main_texture_scale(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "GetFirstPropertyNameIdByAttribute", args = 1)]
    pub fn get_first_property_name_id_by_attribute(
        self,
        attribute_flag: crate::unity_engine::rendering::shaderpropertyflags::ShaderPropertyFlags,
    ) -> i32;

    #[method(name = "HasProperty", args = 1)]
    pub fn has_property(self, name_id: i32) -> bool;

    #[method(name = "HasProperty", args = 1)]
    pub fn has_property_2(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "get_renderQueue", args = 0)]
    pub fn get_render_queue(self) -> i32;

    #[method(name = "set_renderQueue", args = 1)]
    pub fn set_render_queue(self, value: i32) -> ();

    #[method(name = "get_rawRenderQueue", args = 0)]
    pub fn get_raw_render_queue(self) -> i32;

    #[method(name = "EnableKeyword", args = 1)]
    pub fn enable_keyword(self, keyword: ::unity2::Il2CppString) -> ();

    #[method(name = "DisableKeyword", args = 1)]
    pub fn disable_keyword(self, keyword: ::unity2::Il2CppString) -> ();

    #[method(name = "IsKeywordEnabled", args = 1)]
    pub fn is_keyword_enabled(self, keyword: ::unity2::Il2CppString) -> bool;

    #[method(name = "get_globalIlluminationFlags", args = 0)]
    pub fn get_global_illumination_flags(
        self,
    ) -> crate::unity_engine::materialglobalilluminationflags::MaterialGlobalIlluminationFlags;

    #[method(name = "set_globalIlluminationFlags", args = 1)]
    pub fn set_global_illumination_flags(
        self,
        value : crate :: unity_engine :: materialglobalilluminationflags :: MaterialGlobalIlluminationFlags,
    ) -> ();

    #[method(name = "get_doubleSidedGI", args = 0)]
    pub fn get_double_sided_gi(self) -> bool;

    #[method(name = "set_doubleSidedGI", args = 1)]
    pub fn set_double_sided_gi(self, value: bool) -> ();

    #[method(name = "get_enableInstancing", args = 0)]
    pub fn get_enable_instancing(self) -> bool;

    #[method(name = "set_enableInstancing", args = 1)]
    pub fn set_enable_instancing(self, value: bool) -> ();

    #[method(name = "get_passCount", args = 0)]
    pub fn get_pass_count(self) -> i32;

    #[method(name = "SetShaderPassEnabled", args = 2)]
    pub fn set_shader_pass_enabled(self, pass_name: ::unity2::Il2CppString, enabled: bool) -> ();

    #[method(name = "GetShaderPassEnabled", args = 1)]
    pub fn get_shader_pass_enabled(self, pass_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetPassName", args = 1)]
    pub fn get_pass_name(self, pass: i32) -> ::unity2::Il2CppString;

    #[method(name = "FindPass", args = 1)]
    pub fn find_pass(self, pass_name: ::unity2::Il2CppString) -> i32;

    #[method(name = "SetOverrideTag", args = 2)]
    pub fn set_override_tag(self, tag: ::unity2::Il2CppString, val: ::unity2::Il2CppString) -> ();

    #[method(name = "GetTagImpl", args = 3)]
    pub fn get_tag_impl(
        self,
        tag: ::unity2::Il2CppString,
        current_sub_shader_only: bool,
        default_value: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetTag", args = 3)]
    pub fn get_tag(
        self,
        tag: ::unity2::Il2CppString,
        search_fallbacks: bool,
        default_value: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetTag", args = 2)]
    pub fn get_tag_2(
        self,
        tag: ::unity2::Il2CppString,
        search_fallbacks: bool,
    ) -> ::unity2::Il2CppString;

    #[method(name = "Lerp", args = 3)]
    pub fn lerp(
        self,
        start: crate::unity_engine::material::Material,
        end: crate::unity_engine::material::Material,
        t: f32,
    ) -> ();

    #[method(name = "SetPass", args = 1)]
    pub fn set_pass(self, pass: i32) -> bool;

    #[method(name = "CopyPropertiesFromMaterial", args = 1)]
    pub fn copy_properties_from_material(self, mat: crate::unity_engine::material::Material) -> ();

    #[method(name = "GetShaderKeywords", args = 0)]
    pub fn get_shader_keywords(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "SetShaderKeywords", args = 1)]
    pub fn set_shader_keywords(self, names: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "ComputeCRC", args = 0)]
    pub fn compute_crc(self) -> i32;

    #[method(name = "GetTexturePropertyNames", args = 0)]
    pub fn get_texture_property_names(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetTexturePropertyNameIDs", args = 0)]
    pub fn get_texture_property_name_i_ds(self) -> ::unity2::Array<i32>;

    #[method(name = "GetTexturePropertyNamesInternal", args = 1)]
    pub fn get_texture_property_names_internal(
        self,
        out_names: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetTexturePropertyNameIDsInternal", args = 1)]
    pub fn get_texture_property_name_i_ds_internal(
        self,
        out_names: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetTexturePropertyNames", args = 1)]
    pub fn get_texture_property_names_2(
        self,
        out_names: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "GetTexturePropertyNameIDs", args = 1)]
    pub fn get_texture_property_name_i_ds_2(
        self,
        out_names: crate::system::collections::generic::list_1::List_1<i32>,
    ) -> ();

    #[method(name = "SetFloatImpl", args = 2)]
    pub fn set_float_impl(self, name: i32, value: f32) -> ();

    #[method(name = "SetColorImpl", args = 2)]
    pub fn set_color_impl(self, name: i32, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetMatrixImpl", args = 2)]
    pub fn set_matrix_impl(self, name: i32, value: crate::unity_engine::matrix4x4::Matrix4x4)
        -> ();

    #[method(name = "SetTextureImpl", args = 2)]
    pub fn set_texture_impl(self, name: i32, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "SetRenderTextureImpl", args = 3)]
    pub fn set_render_texture_impl(
        self,
        name: i32,
        value: crate::unity_engine::rendertexture::RenderTexture,
        element: crate::unity_engine::rendering::rendertexturesubelement::RenderTextureSubElement,
    ) -> ();

    #[method(name = "SetBufferImpl", args = 2)]
    pub fn set_buffer_impl(
        self,
        name: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "SetGraphicsBufferImpl", args = 2)]
    pub fn set_graphics_buffer_impl(
        self,
        name: i32,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
    ) -> ();

    #[method(name = "SetConstantBufferImpl", args = 4)]
    pub fn set_constant_buffer_impl(
        self,
        name: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetConstantGraphicsBufferImpl", args = 4)]
    pub fn set_constant_graphics_buffer_impl(
        self,
        name: i32,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "GetFloatImpl", args = 1)]
    pub fn get_float_impl(self, name: i32) -> f32;

    #[method(name = "GetColorImpl", args = 1)]
    pub fn get_color_impl(self, name: i32) -> crate::unity_engine::color::Color;

    #[method(name = "GetMatrixImpl", args = 1)]
    pub fn get_matrix_impl(self, name: i32) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "GetTextureImpl", args = 1)]
    pub fn get_texture_impl(self, name: i32) -> crate::unity_engine::texture::Texture;

    #[method(name = "SetFloatArrayImpl", args = 3)]
    pub fn set_float_array_impl(self, name: i32, values: ::unity2::Array<f32>, count: i32) -> ();

    #[method(name = "SetVectorArrayImpl", args = 3)]
    pub fn set_vector_array_impl(
        self,
        name: i32,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
        count: i32,
    ) -> ();

    #[method(name = "SetColorArrayImpl", args = 3)]
    pub fn set_color_array_impl(
        self,
        name: i32,
        values: ::unity2::Array<crate::unity_engine::color::Color>,
        count: i32,
    ) -> ();

    #[method(name = "SetMatrixArrayImpl", args = 3)]
    pub fn set_matrix_array_impl(
        self,
        name: i32,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
        count: i32,
    ) -> ();

    #[method(name = "GetFloatArrayImpl", args = 1)]
    pub fn get_float_array_impl(self, name: i32) -> ::unity2::Array<f32>;

    #[method(name = "GetVectorArrayImpl", args = 1)]
    pub fn get_vector_array_impl(
        self,
        name: i32,
    ) -> ::unity2::Array<crate::unity_engine::vector4::Vector4>;

    #[method(name = "GetColorArrayImpl", args = 1)]
    pub fn get_color_array_impl(
        self,
        name: i32,
    ) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "GetMatrixArrayImpl", args = 1)]
    pub fn get_matrix_array_impl(
        self,
        name: i32,
    ) -> ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>;

    #[method(name = "GetFloatArrayCountImpl", args = 1)]
    pub fn get_float_array_count_impl(self, name: i32) -> i32;

    #[method(name = "GetVectorArrayCountImpl", args = 1)]
    pub fn get_vector_array_count_impl(self, name: i32) -> i32;

    #[method(name = "GetColorArrayCountImpl", args = 1)]
    pub fn get_color_array_count_impl(self, name: i32) -> i32;

    #[method(name = "GetMatrixArrayCountImpl", args = 1)]
    pub fn get_matrix_array_count_impl(self, name: i32) -> i32;

    #[method(name = "ExtractFloatArrayImpl", args = 2)]
    pub fn extract_float_array_impl(self, name: i32, val: ::unity2::Array<f32>) -> ();

    #[method(name = "ExtractVectorArrayImpl", args = 2)]
    pub fn extract_vector_array_impl(
        self,
        name: i32,
        val: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    ) -> ();

    #[method(name = "ExtractColorArrayImpl", args = 2)]
    pub fn extract_color_array_impl(
        self,
        name: i32,
        val: ::unity2::Array<crate::unity_engine::color::Color>,
    ) -> ();

    #[method(name = "ExtractMatrixArrayImpl", args = 2)]
    pub fn extract_matrix_array_impl(
        self,
        name: i32,
        val: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    ) -> ();

    #[method(name = "GetTextureScaleAndOffsetImpl", args = 1)]
    pub fn get_texture_scale_and_offset_impl(
        self,
        name: i32,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "SetTextureOffsetImpl", args = 2)]
    pub fn set_texture_offset_impl(
        self,
        name: i32,
        offset: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "SetTextureScaleImpl", args = 2)]
    pub fn set_texture_scale_impl(
        self,
        name: i32,
        scale: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "SetFloatArray", args = 3)]
    pub fn set_float_array(self, name: i32, values: ::unity2::Array<f32>, count: i32) -> ();

    #[method(name = "SetVectorArray", args = 3)]
    pub fn set_vector_array(
        self,
        name: i32,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
        count: i32,
    ) -> ();

    #[method(name = "SetColorArray", args = 3)]
    pub fn set_color_array(
        self,
        name: i32,
        values: ::unity2::Array<crate::unity_engine::color::Color>,
        count: i32,
    ) -> ();

    #[method(name = "SetMatrixArray", args = 3)]
    pub fn set_matrix_array(
        self,
        name: i32,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
        count: i32,
    ) -> ();

    #[method(name = "ExtractFloatArray", args = 2)]
    pub fn extract_float_array(
        self,
        name: i32,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "ExtractVectorArray", args = 2)]
    pub fn extract_vector_array(
        self,
        name: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "ExtractColorArray", args = 2)]
    pub fn extract_color_array(
        self,
        name: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
    ) -> ();

    #[method(name = "ExtractMatrixArray", args = 2)]
    pub fn extract_matrix_array(
        self,
        name: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "SetFloat", args = 2)]
    pub fn set_float(self, name: ::unity2::Il2CppString, value: f32) -> ();

    #[method(name = "SetFloat", args = 2)]
    pub fn set_float_2(self, name_id: i32, value: f32) -> ();

    #[method(name = "SetInt", args = 2)]
    pub fn set_int(self, name: ::unity2::Il2CppString, value: i32) -> ();

    #[method(name = "SetInt", args = 2)]
    pub fn set_int_2(self, name_id: i32, value: i32) -> ();

    #[method(name = "SetColor", args = 2)]
    pub fn set_color_2(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetColor", args = 2)]
    pub fn set_color_3(self, name_id: i32, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetVector", args = 2)]
    pub fn set_vector(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetVector", args = 2)]
    pub fn set_vector_2(self, name_id: i32, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "SetMatrix", args = 2)]
    pub fn set_matrix(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetMatrix", args = 2)]
    pub fn set_matrix_2(self, name_id: i32, value: crate::unity_engine::matrix4x4::Matrix4x4)
        -> ();

    #[method(name = "SetTexture", args = 2)]
    pub fn set_texture(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::texture::Texture,
    ) -> ();

    #[method(name = "SetTexture", args = 2)]
    pub fn set_texture_2(self, name_id: i32, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "SetTexture", args = 3)]
    pub fn set_texture_3(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::rendertexture::RenderTexture,
        element: crate::unity_engine::rendering::rendertexturesubelement::RenderTextureSubElement,
    ) -> ();

    #[method(name = "SetTexture", args = 3)]
    pub fn set_texture_4(
        self,
        name_id: i32,
        value: crate::unity_engine::rendertexture::RenderTexture,
        element: crate::unity_engine::rendering::rendertexturesubelement::RenderTextureSubElement,
    ) -> ();

    #[method(name = "SetBuffer", args = 2)]
    pub fn set_buffer(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "SetBuffer", args = 2)]
    pub fn set_buffer_2(
        self,
        name_id: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "SetBuffer", args = 2)]
    pub fn set_buffer_3(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
    ) -> ();

    #[method(name = "SetBuffer", args = 2)]
    pub fn set_buffer_4(
        self,
        name_id: i32,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
    ) -> ();

    #[method(name = "SetConstantBuffer", args = 4)]
    pub fn set_constant_buffer(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetConstantBuffer", args = 4)]
    pub fn set_constant_buffer_2(
        self,
        name_id: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetConstantBuffer", args = 4)]
    pub fn set_constant_buffer_3(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetConstantBuffer", args = 4)]
    pub fn set_constant_buffer_4(
        self,
        name_id: i32,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetFloatArray", args = 2)]
    pub fn set_float_array_2(
        self,
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "SetFloatArray", args = 2)]
    pub fn set_float_array_3(
        self,
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "SetFloatArray", args = 2)]
    pub fn set_float_array_4(
        self,
        name: ::unity2::Il2CppString,
        values: ::unity2::Array<f32>,
    ) -> ();

    #[method(name = "SetFloatArray", args = 2)]
    pub fn set_float_array_5(self, name_id: i32, values: ::unity2::Array<f32>) -> ();

    #[method(name = "SetColorArray", args = 2)]
    pub fn set_color_array_2(
        self,
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
    ) -> ();

    #[method(name = "SetColorArray", args = 2)]
    pub fn set_color_array_3(
        self,
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
    ) -> ();

    #[method(name = "SetColorArray", args = 2)]
    pub fn set_color_array_4(
        self,
        name: ::unity2::Il2CppString,
        values: ::unity2::Array<crate::unity_engine::color::Color>,
    ) -> ();

    #[method(name = "SetColorArray", args = 2)]
    pub fn set_color_array_5(
        self,
        name_id: i32,
        values: ::unity2::Array<crate::unity_engine::color::Color>,
    ) -> ();

    #[method(name = "SetVectorArray", args = 2)]
    pub fn set_vector_array_2(
        self,
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "SetVectorArray", args = 2)]
    pub fn set_vector_array_3(
        self,
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "SetVectorArray", args = 2)]
    pub fn set_vector_array_4(
        self,
        name: ::unity2::Il2CppString,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    ) -> ();

    #[method(name = "SetVectorArray", args = 2)]
    pub fn set_vector_array_5(
        self,
        name_id: i32,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    ) -> ();

    #[method(name = "SetMatrixArray", args = 2)]
    pub fn set_matrix_array_2(
        self,
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "SetMatrixArray", args = 2)]
    pub fn set_matrix_array_3(
        self,
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "SetMatrixArray", args = 2)]
    pub fn set_matrix_array_4(
        self,
        name: ::unity2::Il2CppString,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    ) -> ();

    #[method(name = "SetMatrixArray", args = 2)]
    pub fn set_matrix_array_5(
        self,
        name_id: i32,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    ) -> ();

    #[method(name = "GetFloat", args = 1)]
    pub fn get_float(self, name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetFloat", args = 1)]
    pub fn get_float_2(self, name_id: i32) -> f32;

    #[method(name = "GetInt", args = 1)]
    pub fn get_int(self, name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetInt", args = 1)]
    pub fn get_int_2(self, name_id: i32) -> i32;

    #[method(name = "GetColor", args = 1)]
    pub fn get_color_2(self, name: ::unity2::Il2CppString) -> crate::unity_engine::color::Color;

    #[method(name = "GetColor", args = 1)]
    pub fn get_color_3(self, name_id: i32) -> crate::unity_engine::color::Color;

    #[method(name = "GetVector", args = 1)]
    pub fn get_vector(self, name: ::unity2::Il2CppString) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetVector", args = 1)]
    pub fn get_vector_2(self, name_id: i32) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetMatrix", args = 1)]
    pub fn get_matrix(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "GetMatrix", args = 1)]
    pub fn get_matrix_2(self, name_id: i32) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "GetTexture", args = 1)]
    pub fn get_texture(self, name: ::unity2::Il2CppString)
        -> crate::unity_engine::texture::Texture;

    #[method(name = "GetTexture", args = 1)]
    pub fn get_texture_2(self, name_id: i32) -> crate::unity_engine::texture::Texture;

    #[method(name = "GetFloatArray", args = 1)]
    pub fn get_float_array(self, name: ::unity2::Il2CppString) -> ::unity2::Array<f32>;

    #[method(name = "GetFloatArray", args = 1)]
    pub fn get_float_array_2(self, name_id: i32) -> ::unity2::Array<f32>;

    #[method(name = "GetColorArray", args = 1)]
    pub fn get_color_array(
        self,
        name: ::unity2::Il2CppString,
    ) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "GetColorArray", args = 1)]
    pub fn get_color_array_2(
        self,
        name_id: i32,
    ) -> ::unity2::Array<crate::unity_engine::color::Color>;

    #[method(name = "GetVectorArray", args = 1)]
    pub fn get_vector_array(
        self,
        name: ::unity2::Il2CppString,
    ) -> ::unity2::Array<crate::unity_engine::vector4::Vector4>;

    #[method(name = "GetVectorArray", args = 1)]
    pub fn get_vector_array_2(
        self,
        name_id: i32,
    ) -> ::unity2::Array<crate::unity_engine::vector4::Vector4>;

    #[method(name = "GetMatrixArray", args = 1)]
    pub fn get_matrix_array(
        self,
        name: ::unity2::Il2CppString,
    ) -> ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>;

    #[method(name = "GetMatrixArray", args = 1)]
    pub fn get_matrix_array_2(
        self,
        name_id: i32,
    ) -> ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>;

    #[method(name = "GetFloatArray", args = 2)]
    pub fn get_float_array_3(
        self,
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "GetFloatArray", args = 2)]
    pub fn get_float_array_4(
        self,
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "GetColorArray", args = 2)]
    pub fn get_color_array_3(
        self,
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
    ) -> ();

    #[method(name = "GetColorArray", args = 2)]
    pub fn get_color_array_4(
        self,
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::color::Color,
        >,
    ) -> ();

    #[method(name = "GetVectorArray", args = 2)]
    pub fn get_vector_array_3(
        self,
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "GetVectorArray", args = 2)]
    pub fn get_vector_array_4(
        self,
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "GetMatrixArray", args = 2)]
    pub fn get_matrix_array_3(
        self,
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "GetMatrixArray", args = 2)]
    pub fn get_matrix_array_4(
        self,
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "SetTextureOffset", args = 2)]
    pub fn set_texture_offset(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "SetTextureOffset", args = 2)]
    pub fn set_texture_offset_2(
        self,
        name_id: i32,
        value: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "SetTextureScale", args = 2)]
    pub fn set_texture_scale(
        self,
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "SetTextureScale", args = 2)]
    pub fn set_texture_scale_2(
        self,
        name_id: i32,
        value: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "GetTextureOffset", args = 1)]
    pub fn get_texture_offset(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetTextureOffset", args = 1)]
    pub fn get_texture_offset_2(self, name_id: i32) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetTextureScale", args = 1)]
    pub fn get_texture_scale(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetTextureScale", args = 1)]
    pub fn get_texture_scale_2(self, name_id: i32) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "SetColorImpl_Injected", args = 2)]
    pub fn set_color_impl_injected(self, name: i32, value: crate::unity_engine::color::Color)
        -> ();

    #[method(name = "SetMatrixImpl_Injected", args = 2)]
    pub fn set_matrix_impl_injected(
        self,
        name: i32,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "GetColorImpl_Injected", args = 2)]
    pub fn get_color_impl_injected(self, name: i32, ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "GetMatrixImpl_Injected", args = 2)]
    pub fn get_matrix_impl_injected(
        self,
        name: i32,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "GetTextureScaleAndOffsetImpl_Injected", args = 2)]
    pub fn get_texture_scale_and_offset_impl_injected(
        self,
        name: i32,
        ret: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetTextureOffsetImpl_Injected", args = 2)]
    pub fn set_texture_offset_impl_injected(
        self,
        name: i32,
        offset: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "SetTextureScaleImpl_Injected", args = 2)]
    pub fn set_texture_scale_impl_injected(
        self,
        name: i32,
        scale: crate::unity_engine::vector2::Vector2,
    ) -> ();
}

#[cfg(feature = "unity_engine-material")]
impl Material {
    pub fn new(shader: crate::unity_engine::shader::Shader) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Material),
                ::core::stringify!(new),
            )
        });
        <Self as IMaterialMethods>::ctor(this, shader);
        this
    }

    pub fn new_2(source: crate::unity_engine::material::Material) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Material),
                ::core::stringify!(new_2),
            )
        });
        <Self as IMaterialMethods>::ctor_2(this, source);
        this
    }

    pub fn new_3(contents: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Material),
                ::core::stringify!(new_3),
            )
        });
        <Self as IMaterialMethods>::ctor_3(this, contents);
        this
    }
}
