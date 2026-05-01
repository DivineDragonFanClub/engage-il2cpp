
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/uisystemprofilerapi/UISystemProfilerApi_SampleType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct UISystemProfilerApi_SampleType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for UISystemProfilerApi_SampleType {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "UISystemProfilerApi.SampleType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UISystemProfilerApi_SampleType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl UISystemProfilerApi_SampleType {
    pub fn layout() -> Self {
        Self { value: 0 }
    }

    pub fn render() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/uisystemprofilerapi/UISystemProfilerApi.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "UISystemProfilerApi")]
#[parent(crate::system::object::Object)]
pub struct UISystemProfilerApi {}

#[cfg(feature = "unity_engine-uisystemprofilerapi")]
#[::unity2::methods]
impl UISystemProfilerApi {
    #[method(name = "BeginSample", args = 1)]
    pub fn begin_sample(
        r#type: crate::unity_engine::uisystemprofilerapi::UISystemProfilerApi_SampleType,
    ) -> ();

    #[method(name = "EndSample", args = 1)]
    pub fn end_sample(
        r#type: crate::unity_engine::uisystemprofilerapi::UISystemProfilerApi_SampleType,
    ) -> ();

    #[method(name = "AddMarker", args = 2)]
    pub fn add_marker(
        name: ::unity2::Il2CppString,
        obj: crate::unity_engine::object_2::Object_2,
    ) -> ();
}
