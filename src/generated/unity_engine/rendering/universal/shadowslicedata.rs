
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/shadowslicedata/ShadowSliceData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ShadowSliceData {
    pub view_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    pub projection_matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    pub shadow_transform: crate::unity_engine::matrix4x4::Matrix4x4,
    pub offset_x: i32,
    pub offset_y: i32,
    pub resolution: i32,
}

impl ::unity2::ClassIdentity for ShadowSliceData {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";

    const NAME: &'static str = "ShadowSliceData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ShadowSliceData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-universal-shadowslicedata")]
#[::unity2::methods(value)]
impl ShadowSliceData {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}
