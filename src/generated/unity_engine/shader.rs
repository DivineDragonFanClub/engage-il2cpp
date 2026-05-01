
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/shader/Shader.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Shader")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct Shader {}

#[cfg(feature = "unity_engine-shader")]
#[::unity2::methods]
impl Shader {
    #[method(name = "get_globalShaderHardwareTier", args = 0)]
    pub fn get_global_shader_hardware_tier(
    ) -> crate::unity_engine::rendering::shaderhardwaretier::ShaderHardwareTier;

    #[method(name = "set_globalShaderHardwareTier", args = 1)]
    pub fn set_global_shader_hardware_tier(
        value: crate::unity_engine::rendering::shaderhardwaretier::ShaderHardwareTier,
    ) -> ();

    #[method(name = "Find", args = 1)]
    pub fn find(name: ::unity2::Il2CppString) -> crate::unity_engine::shader::Shader;

    #[method(name = "FindBuiltin", args = 1)]
    pub fn find_builtin(name: ::unity2::Il2CppString) -> crate::unity_engine::shader::Shader;

    #[method(name = "get_maximumLOD", args = 0)]
    pub fn get_maximum_lod(self) -> i32;

    #[method(name = "set_maximumLOD", args = 1)]
    pub fn set_maximum_lod(self, value: i32) -> ();

    #[method(name = "get_globalMaximumLOD", args = 0)]
    pub fn get_global_maximum_lod() -> i32;

    #[method(name = "set_globalMaximumLOD", args = 1)]
    pub fn set_global_maximum_lod(value: i32) -> ();

    #[method(name = "get_isSupported", args = 0)]
    pub fn get_is_supported(self) -> bool;

    #[method(name = "get_globalRenderPipeline", args = 0)]
    pub fn get_global_render_pipeline() -> ::unity2::Il2CppString;

    #[method(name = "set_globalRenderPipeline", args = 1)]
    pub fn set_global_render_pipeline(value: ::unity2::Il2CppString) -> ();

    #[method(name = "EnableKeyword", args = 1)]
    pub fn enable_keyword(keyword: ::unity2::Il2CppString) -> ();

    #[method(name = "DisableKeyword", args = 1)]
    pub fn disable_keyword(keyword: ::unity2::Il2CppString) -> ();

    #[method(name = "IsKeywordEnabled", args = 1)]
    pub fn is_keyword_enabled(keyword: ::unity2::Il2CppString) -> bool;

    #[method(name = "get_renderQueue", args = 0)]
    pub fn get_render_queue(self) -> i32;

    #[method(name = "get_disableBatching", args = 0)]
    pub fn get_disable_batching(
        self,
    ) -> crate::unity_engine::disablebatchingtype::DisableBatchingType;

    #[method(name = "WarmupAllShaders", args = 0)]
    pub fn warmup_all_shaders() -> ();

    #[method(name = "TagToID", args = 1)]
    pub fn tag_to_id(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "IDToTag", args = 1)]
    pub fn id_to_tag(name: i32) -> ::unity2::Il2CppString;

    #[method(name = "PropertyToID", args = 1)]
    pub fn property_to_id(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetDependency", args = 1)]
    pub fn get_dependency(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::shader::Shader;

    #[method(name = "get_passCount", args = 0)]
    pub fn get_pass_count(self) -> i32;

    #[method(name = "FindPassTagValue", args = 2)]
    pub fn find_pass_tag_value(
        self,
        pass_index: i32,
        tag_name: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    ) -> crate::unity_engine::rendering::shadertagid::ShaderTagId;

    #[method(name = "Internal_FindPassTagValue", args = 2)]
    pub fn internal_find_pass_tag_value(self, pass_index: i32, tag_name: i32) -> i32;

    #[method(name = "SetGlobalFloatImpl", args = 2)]
    pub fn set_global_float_impl(name: i32, value: f32) -> ();

    #[method(name = "SetGlobalVectorImpl", args = 2)]
    pub fn set_global_vector_impl(name: i32, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "SetGlobalMatrixImpl", args = 2)]
    pub fn set_global_matrix_impl(
        name: i32,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetGlobalTextureImpl", args = 2)]
    pub fn set_global_texture_impl(name: i32, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "SetGlobalRenderTextureImpl", args = 3)]
    pub fn set_global_render_texture_impl(
        name: i32,
        value: crate::unity_engine::rendertexture::RenderTexture,
        element: crate::unity_engine::rendering::rendertexturesubelement::RenderTextureSubElement,
    ) -> ();

    #[method(name = "SetGlobalBufferImpl", args = 2)]
    pub fn set_global_buffer_impl(
        name: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "SetGlobalGraphicsBufferImpl", args = 2)]
    pub fn set_global_graphics_buffer_impl(
        name: i32,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
    ) -> ();

    #[method(name = "SetGlobalConstantBufferImpl", args = 4)]
    pub fn set_global_constant_buffer_impl(
        name: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetGlobalConstantGraphicsBufferImpl", args = 4)]
    pub fn set_global_constant_graphics_buffer_impl(
        name: i32,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "GetGlobalFloatImpl", args = 1)]
    pub fn get_global_float_impl(name: i32) -> f32;

    #[method(name = "GetGlobalVectorImpl", args = 1)]
    pub fn get_global_vector_impl(name: i32) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetGlobalMatrixImpl", args = 1)]
    pub fn get_global_matrix_impl(name: i32) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "GetGlobalTextureImpl", args = 1)]
    pub fn get_global_texture_impl(name: i32) -> crate::unity_engine::texture::Texture;

    #[method(name = "SetGlobalFloatArrayImpl", args = 3)]
    pub fn set_global_float_array_impl(name: i32, values: ::unity2::Array<f32>, count: i32) -> ();

    #[method(name = "SetGlobalVectorArrayImpl", args = 3)]
    pub fn set_global_vector_array_impl(
        name: i32,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
        count: i32,
    ) -> ();

    #[method(name = "SetGlobalMatrixArrayImpl", args = 3)]
    pub fn set_global_matrix_array_impl(
        name: i32,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
        count: i32,
    ) -> ();

    #[method(name = "GetGlobalFloatArrayImpl", args = 1)]
    pub fn get_global_float_array_impl(name: i32) -> ::unity2::Array<f32>;

    #[method(name = "GetGlobalVectorArrayImpl", args = 1)]
    pub fn get_global_vector_array_impl(
        name: i32,
    ) -> ::unity2::Array<crate::unity_engine::vector4::Vector4>;

    #[method(name = "GetGlobalMatrixArrayImpl", args = 1)]
    pub fn get_global_matrix_array_impl(
        name: i32,
    ) -> ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>;

    #[method(name = "GetGlobalFloatArrayCountImpl", args = 1)]
    pub fn get_global_float_array_count_impl(name: i32) -> i32;

    #[method(name = "GetGlobalVectorArrayCountImpl", args = 1)]
    pub fn get_global_vector_array_count_impl(name: i32) -> i32;

    #[method(name = "GetGlobalMatrixArrayCountImpl", args = 1)]
    pub fn get_global_matrix_array_count_impl(name: i32) -> i32;

    #[method(name = "ExtractGlobalFloatArrayImpl", args = 2)]
    pub fn extract_global_float_array_impl(name: i32, val: ::unity2::Array<f32>) -> ();

    #[method(name = "ExtractGlobalVectorArrayImpl", args = 2)]
    pub fn extract_global_vector_array_impl(
        name: i32,
        val: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    ) -> ();

    #[method(name = "ExtractGlobalMatrixArrayImpl", args = 2)]
    pub fn extract_global_matrix_array_impl(
        name: i32,
        val: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    ) -> ();

    #[method(name = "SetGlobalFloatArray", args = 3)]
    pub fn set_global_float_array(name: i32, values: ::unity2::Array<f32>, count: i32) -> ();

    #[method(name = "SetGlobalVectorArray", args = 3)]
    pub fn set_global_vector_array(
        name: i32,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
        count: i32,
    ) -> ();

    #[method(name = "SetGlobalMatrixArray", args = 3)]
    pub fn set_global_matrix_array(
        name: i32,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
        count: i32,
    ) -> ();

    #[method(name = "ExtractGlobalFloatArray", args = 2)]
    pub fn extract_global_float_array(
        name: i32,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "ExtractGlobalVectorArray", args = 2)]
    pub fn extract_global_vector_array(
        name: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "ExtractGlobalMatrixArray", args = 2)]
    pub fn extract_global_matrix_array(
        name: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "SetGlobalFloat", args = 2)]
    pub fn set_global_float(name: ::unity2::Il2CppString, value: f32) -> ();

    #[method(name = "SetGlobalFloat", args = 2)]
    pub fn set_global_float_2(name_id: i32, value: f32) -> ();

    #[method(name = "SetGlobalInt", args = 2)]
    pub fn set_global_int(name: ::unity2::Il2CppString, value: i32) -> ();

    #[method(name = "SetGlobalInt", args = 2)]
    pub fn set_global_int_2(name_id: i32, value: i32) -> ();

    #[method(name = "SetGlobalVector", args = 2)]
    pub fn set_global_vector(
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetGlobalVector", args = 2)]
    pub fn set_global_vector_2(name_id: i32, value: crate::unity_engine::vector4::Vector4) -> ();

    #[method(name = "SetGlobalColor", args = 2)]
    pub fn set_global_color(
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "SetGlobalColor", args = 2)]
    pub fn set_global_color_2(name_id: i32, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetGlobalMatrix", args = 2)]
    pub fn set_global_matrix(
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetGlobalMatrix", args = 2)]
    pub fn set_global_matrix_2(
        name_id: i32,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetGlobalTexture", args = 2)]
    pub fn set_global_texture(
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::texture::Texture,
    ) -> ();

    #[method(name = "SetGlobalTexture", args = 2)]
    pub fn set_global_texture_2(name_id: i32, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "SetGlobalTexture", args = 3)]
    pub fn set_global_texture_3(
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::rendertexture::RenderTexture,
        element: crate::unity_engine::rendering::rendertexturesubelement::RenderTextureSubElement,
    ) -> ();

    #[method(name = "SetGlobalTexture", args = 3)]
    pub fn set_global_texture_4(
        name_id: i32,
        value: crate::unity_engine::rendertexture::RenderTexture,
        element: crate::unity_engine::rendering::rendertexturesubelement::RenderTextureSubElement,
    ) -> ();

    #[method(name = "SetGlobalBuffer", args = 2)]
    pub fn set_global_buffer(
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "SetGlobalBuffer", args = 2)]
    pub fn set_global_buffer_2(
        name_id: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
    ) -> ();

    #[method(name = "SetGlobalBuffer", args = 2)]
    pub fn set_global_buffer_3(
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
    ) -> ();

    #[method(name = "SetGlobalBuffer", args = 2)]
    pub fn set_global_buffer_4(
        name_id: i32,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
    ) -> ();

    #[method(name = "SetGlobalConstantBuffer", args = 4)]
    pub fn set_global_constant_buffer(
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetGlobalConstantBuffer", args = 4)]
    pub fn set_global_constant_buffer_2(
        name_id: i32,
        value: crate::unity_engine::computebuffer::ComputeBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetGlobalConstantBuffer", args = 4)]
    pub fn set_global_constant_buffer_3(
        name: ::unity2::Il2CppString,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetGlobalConstantBuffer", args = 4)]
    pub fn set_global_constant_buffer_4(
        name_id: i32,
        value: crate::unity_engine::graphicsbuffer::GraphicsBuffer,
        offset: i32,
        size: i32,
    ) -> ();

    #[method(name = "SetGlobalFloatArray", args = 2)]
    pub fn set_global_float_array_2(
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "SetGlobalFloatArray", args = 2)]
    pub fn set_global_float_array_3(
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "SetGlobalFloatArray", args = 2)]
    pub fn set_global_float_array_4(
        name: ::unity2::Il2CppString,
        values: ::unity2::Array<f32>,
    ) -> ();

    #[method(name = "SetGlobalFloatArray", args = 2)]
    pub fn set_global_float_array_5(name_id: i32, values: ::unity2::Array<f32>) -> ();

    #[method(name = "SetGlobalVectorArray", args = 2)]
    pub fn set_global_vector_array_2(
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "SetGlobalVectorArray", args = 2)]
    pub fn set_global_vector_array_3(
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "SetGlobalVectorArray", args = 2)]
    pub fn set_global_vector_array_4(
        name: ::unity2::Il2CppString,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    ) -> ();

    #[method(name = "SetGlobalVectorArray", args = 2)]
    pub fn set_global_vector_array_5(
        name_id: i32,
        values: ::unity2::Array<crate::unity_engine::vector4::Vector4>,
    ) -> ();

    #[method(name = "SetGlobalMatrixArray", args = 2)]
    pub fn set_global_matrix_array_2(
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "SetGlobalMatrixArray", args = 2)]
    pub fn set_global_matrix_array_3(
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "SetGlobalMatrixArray", args = 2)]
    pub fn set_global_matrix_array_4(
        name: ::unity2::Il2CppString,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    ) -> ();

    #[method(name = "SetGlobalMatrixArray", args = 2)]
    pub fn set_global_matrix_array_5(
        name_id: i32,
        values: ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>,
    ) -> ();

    #[method(name = "GetGlobalFloat", args = 1)]
    pub fn get_global_float(name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetGlobalFloat", args = 1)]
    pub fn get_global_float_2(name_id: i32) -> f32;

    #[method(name = "GetGlobalInt", args = 1)]
    pub fn get_global_int(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetGlobalInt", args = 1)]
    pub fn get_global_int_2(name_id: i32) -> i32;

    #[method(name = "GetGlobalVector", args = 1)]
    pub fn get_global_vector(name: ::unity2::Il2CppString)
        -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetGlobalVector", args = 1)]
    pub fn get_global_vector_2(name_id: i32) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetGlobalColor", args = 1)]
    pub fn get_global_color(name: ::unity2::Il2CppString) -> crate::unity_engine::color::Color;

    #[method(name = "GetGlobalColor", args = 1)]
    pub fn get_global_color_2(name_id: i32) -> crate::unity_engine::color::Color;

    #[method(name = "GetGlobalMatrix", args = 1)]
    pub fn get_global_matrix(
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "GetGlobalMatrix", args = 1)]
    pub fn get_global_matrix_2(name_id: i32) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "GetGlobalTexture", args = 1)]
    pub fn get_global_texture(
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::texture::Texture;

    #[method(name = "GetGlobalTexture", args = 1)]
    pub fn get_global_texture_2(name_id: i32) -> crate::unity_engine::texture::Texture;

    #[method(name = "GetGlobalFloatArray", args = 1)]
    pub fn get_global_float_array(name: ::unity2::Il2CppString) -> ::unity2::Array<f32>;

    #[method(name = "GetGlobalFloatArray", args = 1)]
    pub fn get_global_float_array_2(name_id: i32) -> ::unity2::Array<f32>;

    #[method(name = "GetGlobalVectorArray", args = 1)]
    pub fn get_global_vector_array(
        name: ::unity2::Il2CppString,
    ) -> ::unity2::Array<crate::unity_engine::vector4::Vector4>;

    #[method(name = "GetGlobalVectorArray", args = 1)]
    pub fn get_global_vector_array_2(
        name_id: i32,
    ) -> ::unity2::Array<crate::unity_engine::vector4::Vector4>;

    #[method(name = "GetGlobalMatrixArray", args = 1)]
    pub fn get_global_matrix_array(
        name: ::unity2::Il2CppString,
    ) -> ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>;

    #[method(name = "GetGlobalMatrixArray", args = 1)]
    pub fn get_global_matrix_array_2(
        name_id: i32,
    ) -> ::unity2::Array<crate::unity_engine::matrix4x4::Matrix4x4>;

    #[method(name = "GetGlobalFloatArray", args = 2)]
    pub fn get_global_float_array_3(
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "GetGlobalFloatArray", args = 2)]
    pub fn get_global_float_array_4(
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<f32>,
    ) -> ();

    #[method(name = "GetGlobalVectorArray", args = 2)]
    pub fn get_global_vector_array_3(
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "GetGlobalVectorArray", args = 2)]
    pub fn get_global_vector_array_4(
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::vector4::Vector4,
        >,
    ) -> ();

    #[method(name = "GetGlobalMatrixArray", args = 2)]
    pub fn get_global_matrix_array_3(
        name: ::unity2::Il2CppString,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = "GetGlobalMatrixArray", args = 2)]
    pub fn get_global_matrix_array_4(
        name_id: i32,
        values: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::matrix4x4::Matrix4x4,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetPropertyName", args = 2)]
    pub fn get_property_name(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetPropertyNameId", args = 2)]
    pub fn get_property_name_id(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
    ) -> i32;

    #[method(name = "GetPropertyType", args = 2)]
    pub fn get_property_type(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
    ) -> crate::unity_engine::rendering::shaderpropertytype::ShaderPropertyType;

    #[method(name = "GetPropertyDescription", args = 2)]
    pub fn get_property_description(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetPropertyFlags", args = 2)]
    pub fn get_property_flags(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
    ) -> crate::unity_engine::rendering::shaderpropertyflags::ShaderPropertyFlags;

    #[method(name = "GetPropertyAttributes", args = 2)]
    pub fn get_property_attributes(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetPropertyDefaultValue", args = 2)]
    pub fn get_property_default_value(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetPropertyTextureDimension", args = 2)]
    pub fn get_property_texture_dimension(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
    ) -> crate::unity_engine::rendering::texturedimension::TextureDimension;

    #[method(name = "GetPropertyTextureDefaultName", args = 2)]
    pub fn get_property_texture_default_name(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FindTextureStackImpl", args = 4)]
    pub fn find_texture_stack_impl(
        s: crate::unity_engine::shader::Shader,
        property_idx: i32,
        stack_name: ::unity2::Il2CppString,
        layer_index: i32,
    ) -> bool;

    #[method(name = "CheckPropertyIndex", args = 2)]
    pub fn check_property_index(s: crate::unity_engine::shader::Shader, property_index: i32) -> ();

    #[method(name = "GetPropertyCount", args = 0)]
    pub fn get_property_count(self) -> i32;

    #[method(name = "FindPropertyIndex", args = 1)]
    pub fn find_property_index(self, property_name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetPropertyName", args = 1)]
    pub fn get_property_name_2(self, property_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetPropertyNameId", args = 1)]
    pub fn get_property_name_id_2(self, property_index: i32) -> i32;

    #[method(name = "GetPropertyType", args = 1)]
    pub fn get_property_type_2(
        self,
        property_index: i32,
    ) -> crate::unity_engine::rendering::shaderpropertytype::ShaderPropertyType;

    #[method(name = "GetPropertyDescription", args = 1)]
    pub fn get_property_description_2(self, property_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetPropertyFlags", args = 1)]
    pub fn get_property_flags_2(
        self,
        property_index: i32,
    ) -> crate::unity_engine::rendering::shaderpropertyflags::ShaderPropertyFlags;

    #[method(name = "GetPropertyAttributes", args = 1)]
    pub fn get_property_attributes_2(
        self,
        property_index: i32,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetPropertyDefaultFloatValue", args = 1)]
    pub fn get_property_default_float_value(self, property_index: i32) -> f32;

    #[method(name = "GetPropertyDefaultVectorValue", args = 1)]
    pub fn get_property_default_vector_value(
        self,
        property_index: i32,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "GetPropertyRangeLimits", args = 1)]
    pub fn get_property_range_limits(
        self,
        property_index: i32,
    ) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetPropertyTextureDimension", args = 1)]
    pub fn get_property_texture_dimension_2(
        self,
        property_index: i32,
    ) -> crate::unity_engine::rendering::texturedimension::TextureDimension;

    #[method(name = "GetPropertyTextureDefaultName", args = 1)]
    pub fn get_property_texture_default_name_2(self, property_index: i32)
        -> ::unity2::Il2CppString;

    #[method(name = "FindTextureStack", args = 3)]
    pub fn find_texture_stack(
        self,
        property_index: i32,
        stack_name: ::unity2::Il2CppString,
        layer_index: i32,
    ) -> bool;

    #[method(name = "SetGlobalVectorImpl_Injected", args = 2)]
    pub fn set_global_vector_impl_injected(
        name: i32,
        value: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "SetGlobalMatrixImpl_Injected", args = 2)]
    pub fn set_global_matrix_impl_injected(
        name: i32,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "GetGlobalVectorImpl_Injected", args = 2)]
    pub fn get_global_vector_impl_injected(
        name: i32,
        ret: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "GetGlobalMatrixImpl_Injected", args = 2)]
    pub fn get_global_matrix_impl_injected(
        name: i32,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "GetPropertyDefaultValue_Injected", args = 3)]
    pub fn get_property_default_value_injected(
        shader: crate::unity_engine::shader::Shader,
        property_index: i32,
        ret: crate::unity_engine::vector4::Vector4,
    ) -> ();
}

#[cfg(feature = "unity_engine-shader")]
impl Shader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Shader),
                ::core::stringify!(new),
            )
        });
        <Self as IShaderMethods>::ctor(this);
        this
    }
}
