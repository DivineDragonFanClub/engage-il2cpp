
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/shaderutils/ShaderUtils.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering.Universal", name = "ShaderUtils")]
#[parent(crate::system::object::Object)]
pub struct ShaderUtils {
    #[static_field]
    #[rename(name = "s_ShaderPaths")]
    pub s_shader_paths: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "unity_engine-rendering-universal-shaderutils")]
#[::unity2::methods]
impl ShaderUtils {
    #[method(name = "GetShaderPath", args = 1)]
    pub fn get_shader_path(
        id: crate::unity_engine::rendering::universal::shaderpathid::ShaderPathID,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetEnumFromPath", args = 1)]
    pub fn get_enum_from_path(
        path: ::unity2::Il2CppString,
    ) -> crate::unity_engine::rendering::universal::shaderpathid::ShaderPathID;

    #[method(name = "IsLWShader", args = 1)]
    pub fn is_lw_shader(shader: crate::unity_engine::shader::Shader) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
