
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/builtinrendertexturetype/BuiltinRenderTextureType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BuiltinRenderTextureType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BuiltinRenderTextureType {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "BuiltinRenderTextureType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BuiltinRenderTextureType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BuiltinRenderTextureType {
    pub fn property_name() -> Self {
        Self { value: -4 }
    }

    pub fn buffer_ptr() -> Self {
        Self { value: -3 }
    }

    pub fn render_texture() -> Self {
        Self { value: -2 }
    }

    pub fn bindable_texture() -> Self {
        Self { value: -1 }
    }

    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn current_active() -> Self {
        Self { value: 1 }
    }

    pub fn camera_target() -> Self {
        Self { value: 2 }
    }

    pub fn depth() -> Self {
        Self { value: 3 }
    }

    pub fn depth_normals() -> Self {
        Self { value: 4 }
    }

    pub fn resolved_depth() -> Self {
        Self { value: 5 }
    }

    pub fn prepass_normals_spec() -> Self {
        Self { value: 7 }
    }

    pub fn prepass_light() -> Self {
        Self { value: 8 }
    }

    pub fn prepass_light_spec() -> Self {
        Self { value: 9 }
    }

    pub fn g_buffer0() -> Self {
        Self { value: 10 }
    }

    pub fn g_buffer1() -> Self {
        Self { value: 11 }
    }

    pub fn g_buffer2() -> Self {
        Self { value: 12 }
    }

    pub fn g_buffer3() -> Self {
        Self { value: 13 }
    }

    pub fn reflections() -> Self {
        Self { value: 14 }
    }

    pub fn motion_vectors() -> Self {
        Self { value: 15 }
    }

    pub fn g_buffer4() -> Self {
        Self { value: 16 }
    }

    pub fn g_buffer5() -> Self {
        Self { value: 17 }
    }

    pub fn g_buffer6() -> Self {
        Self { value: 18 }
    }

    pub fn g_buffer7() -> Self {
        Self { value: 19 }
    }
}
