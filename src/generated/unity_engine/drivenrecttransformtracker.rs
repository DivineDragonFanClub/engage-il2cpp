
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/drivenrecttransformtracker/DrivenRectTransformTracker.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct DrivenRectTransformTracker {}

impl ::unity2::ClassIdentity for DrivenRectTransformTracker {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "DrivenRectTransformTracker";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DrivenRectTransformTracker {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-drivenrecttransformtracker")]
#[::unity2::methods(value)]
impl DrivenRectTransformTracker {
    #[method(name = "Add", args = 3)]
    pub fn add(
        self,
        driver: crate::unity_engine::object_2::Object_2,
        rect_transform: crate::unity_engine::recttransform::RectTransform,
        driven_properties : crate :: unity_engine :: driventransformproperties :: DrivenTransformProperties,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}
