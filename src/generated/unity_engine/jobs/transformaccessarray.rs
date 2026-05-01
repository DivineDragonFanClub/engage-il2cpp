
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/jobs/transformaccessarray/TransformAccessArray.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TransformAccessArray {
    pub m_transform_array: ::unity2::IntPtr,
}

impl ::unity2::ClassIdentity for TransformAccessArray {
    const NAMESPACE: &'static str = "UnityEngine.Jobs";

    const NAME: &'static str = "TransformAccessArray";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TransformAccessArray {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-jobs-transformaccessarray")]
#[::unity2::methods(value)]
impl TransformAccessArray {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        transforms: ::unity2::Array<crate::unity_engine::transform::Transform>,
        desired_job_count: i32,
    ) -> ();

    #[method(name = "Allocate", args = 3)]
    pub fn allocate(
        capacity: i32,
        desired_job_count: i32,
        array: crate::unity_engine::jobs::transformaccessarray::TransformAccessArray,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "GetTransformAccessArrayForSchedule", args = 0)]
    pub fn get_transform_access_array_for_schedule(self) -> ::unity2::IntPtr;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "Create", args = 2)]
    pub fn create(capacity: i32, desired_job_count: i32) -> ::unity2::IntPtr;

    #[method(name = "DestroyTransformAccessArray", args = 1)]
    pub fn destroy_transform_access_array(transform_array: ::unity2::IntPtr) -> ();

    #[method(name = "SetTransforms", args = 2)]
    pub fn set_transforms(
        transform_array_int_ptr: ::unity2::IntPtr,
        transforms: ::unity2::Array<crate::unity_engine::transform::Transform>,
    ) -> ();

    #[method(name = "GetSortedTransformAccess", args = 1)]
    pub fn get_sorted_transform_access(
        transform_array_int_ptr: ::unity2::IntPtr,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetSortedToUserIndex", args = 1)]
    pub fn get_sorted_to_user_index(transform_array_int_ptr: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "SetTransform", args = 3)]
    pub fn set_transform(
        transform_array_int_ptr: ::unity2::IntPtr,
        index: i32,
        transform: crate::unity_engine::transform::Transform,
    ) -> ();
}
