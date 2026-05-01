
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/copytexturesupport/CopyTextureSupport.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CopyTextureSupport {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CopyTextureSupport {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "CopyTextureSupport";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CopyTextureSupport {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CopyTextureSupport {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn basic() -> Self {
        Self { value: 1 }
    }

    pub fn copy3_d() -> Self {
        Self { value: 2 }
    }

    pub fn different_types() -> Self {
        Self { value: 4 }
    }

    pub fn texture_to_rt() -> Self {
        Self { value: 8 }
    }

    pub fn rt_to_texture() -> Self {
        Self { value: 16 }
    }
}
