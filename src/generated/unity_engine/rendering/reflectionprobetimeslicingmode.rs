
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/reflectionprobetimeslicingmode/ReflectionProbeTimeSlicingMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ReflectionProbeTimeSlicingMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ReflectionProbeTimeSlicingMode {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "ReflectionProbeTimeSlicingMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ReflectionProbeTimeSlicingMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ReflectionProbeTimeSlicingMode {
    pub fn all_faces_at_once() -> Self {
        Self { value: 0 }
    }

    pub fn individual_faces() -> Self {
        Self { value: 1 }
    }

    pub fn no_time_slicing() -> Self {
        Self { value: 2 }
    }
}
