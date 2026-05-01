
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/computebuffer/ComputeBuffer.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ComputeBuffer")]
#[parent(crate::system::object::Object)]
pub struct ComputeBuffer {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-computebuffer")]
#[::unity2::methods]
impl ComputeBuffer {
    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose_2(self, disposing: bool) -> ();

    #[method(name = "InitBuffer", args = 4)]
    pub fn init_buffer(
        count: i32,
        stride: i32,
        r#type: crate::unity_engine::computebuffertype::ComputeBufferType,
        usage: crate::unity_engine::computebuffermode::ComputeBufferMode,
    ) -> ::unity2::IntPtr;

    #[method(name = "DestroyBuffer", args = 1)]
    pub fn destroy_buffer(buf: crate::unity_engine::computebuffer::ComputeBuffer) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, count: i32, stride: i32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        count: i32,
        stride: i32,
        r#type: crate::unity_engine::computebuffertype::ComputeBufferType,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_3(
        self,
        count: i32,
        stride: i32,
        r#type: crate::unity_engine::computebuffertype::ComputeBufferType,
        usage: crate::unity_engine::computebuffermode::ComputeBufferMode,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_4(
        self,
        count: i32,
        stride: i32,
        r#type: crate::unity_engine::computebuffertype::ComputeBufferType,
        usage: crate::unity_engine::computebuffermode::ComputeBufferMode,
        stack_depth: i32,
    ) -> ();

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "get_count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_stride", args = 0)]
    pub fn get_stride(self) -> i32;

    #[method(name = "InternalSetNativeData", args = 5)]
    pub fn internal_set_native_data(
        self,
        data: ::unity2::IntPtr,
        native_buffer_start_index: i32,
        compute_buffer_start_index: i32,
        count: i32,
        elem_size: i32,
    ) -> ();

    #[method(name = "InternalSetData", args = 5)]
    pub fn internal_set_data(
        self,
        data: ::unity2::IlInstance,
        managed_buffer_start_index: i32,
        compute_buffer_start_index: i32,
        count: i32,
        elem_size: i32,
    ) -> ();

    #[method(name = "set_name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "unity_engine-computebuffer")]
impl ComputeBuffer {
    pub fn new(count: i32, stride: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ComputeBuffer),
                ::core::stringify!(new),
            )
        });
        <Self as IComputeBufferMethods>::ctor(this, count, stride);
        this
    }

    pub fn new_2(
        count: i32,
        stride: i32,
        r#type: crate::unity_engine::computebuffertype::ComputeBufferType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ComputeBuffer),
                ::core::stringify!(new_2),
            )
        });
        <Self as IComputeBufferMethods>::ctor_2(this, count, stride, r#type);
        this
    }

    pub fn new_3(
        count: i32,
        stride: i32,
        r#type: crate::unity_engine::computebuffertype::ComputeBufferType,
        usage: crate::unity_engine::computebuffermode::ComputeBufferMode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ComputeBuffer),
                ::core::stringify!(new_3),
            )
        });
        <Self as IComputeBufferMethods>::ctor_3(this, count, stride, r#type, usage);
        this
    }

    pub fn new_4(
        count: i32,
        stride: i32,
        r#type: crate::unity_engine::computebuffertype::ComputeBufferType,
        usage: crate::unity_engine::computebuffermode::ComputeBufferMode,
        stack_depth: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ComputeBuffer),
                ::core::stringify!(new_4),
            )
        });
        <Self as IComputeBufferMethods>::ctor_4(this, count, stride, r#type, usage, stack_depth);
        this
    }
}
