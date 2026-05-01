
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/shadervariantloglevel/ShaderVariantLogLevel.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ShaderVariantLogLevel {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ShaderVariantLogLevel {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";

    const NAME: &'static str = "ShaderVariantLogLevel";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ShaderVariantLogLevel {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ShaderVariantLogLevel {
    pub fn disabled() -> Self {
        Self { value: 0 }
    }

    pub fn only_universal_rp_shaders() -> Self {
        Self { value: 1 }
    }

    pub fn all_shaders() -> Self {
        Self { value: 2 }
    }
}
