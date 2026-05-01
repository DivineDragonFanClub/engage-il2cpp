
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/tmp_vertex/TMP_Vertex.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TMP_Vertex {
    pub position: crate::unity_engine::vector3::Vector3,
    pub uv: crate::unity_engine::vector2::Vector2,
    pub uv2: crate::unity_engine::vector2::Vector2,
    pub uv4: crate::unity_engine::vector2::Vector2,
    pub color: crate::unity_engine::color32::Color32,
}

impl ::unity2::ClassIdentity for TMP_Vertex {
    const NAMESPACE: &'static str = "TMPro";

    const NAME: &'static str = "TMP_Vertex";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TMP_Vertex {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "tm_pro-tmp_vertex")]
#[::unity2::methods(value)]
impl TMP_Vertex {
    #[method(name = "get_zero", args = 0)]
    pub fn get_zero() -> crate::tm_pro::tmp_vertex::TMP_Vertex;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
