
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/vertexattributeformat/VertexAttributeFormat.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VertexAttributeFormat {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VertexAttributeFormat {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "VertexAttributeFormat";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VertexAttributeFormat {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VertexAttributeFormat {
    pub fn float32() -> Self {
        Self { value: 0 }
    }

    pub fn float16() -> Self {
        Self { value: 1 }
    }

    pub fn u_norm8() -> Self {
        Self { value: 2 }
    }

    pub fn s_norm8() -> Self {
        Self { value: 3 }
    }

    pub fn u_norm16() -> Self {
        Self { value: 4 }
    }

    pub fn s_norm16() -> Self {
        Self { value: 5 }
    }

    pub fn u_int8() -> Self {
        Self { value: 6 }
    }

    pub fn s_int8() -> Self {
        Self { value: 7 }
    }

    pub fn u_int16() -> Self {
        Self { value: 8 }
    }

    pub fn s_int16() -> Self {
        Self { value: 9 }
    }

    pub fn u_int32() -> Self {
        Self { value: 10 }
    }

    pub fn s_int32() -> Self {
        Self { value: 11 }
    }
}
