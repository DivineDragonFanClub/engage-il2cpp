
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/deferredshaderdata/DeferredShaderData_ComputeBufferInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct DeferredShaderData_ComputeBufferInfo {
    pub frame_used: u32,
    pub r#type: crate::unity_engine::computebuffertype::ComputeBufferType,
}

impl ::unity2::ClassIdentity for DeferredShaderData_ComputeBufferInfo {
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";

    const NAME: &'static str = "DeferredShaderData.ComputeBufferInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DeferredShaderData_ComputeBufferInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/universal/deferredshaderdata/DeferredShaderData.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Rendering.Universal",
    name = "DeferredShaderData"
)]
#[parent(crate::system::object::Object)]
pub struct DeferredShaderData {
# [static_field] # [rename (name = "m_Instance")] pub m_instance : crate :: unity_engine :: rendering :: universal :: deferredshaderdata :: DeferredShaderData ,
# [rename (name = "m_Buffers")] pub m_buffers : :: unity2 :: Array < crate :: unity_engine :: computebuffer :: ComputeBuffer > ,
# [rename (name = "m_BufferInfos")] pub m_buffer_infos : :: unity2 :: Array < crate :: unity_engine :: rendering :: universal :: deferredshaderdata :: DeferredShaderData_ComputeBufferInfo > ,
# [rename (name = "m_BufferCount")] pub m_buffer_count : i32 ,
# [rename (name = "m_CachedBufferIndex")] pub m_cached_buffer_index : i32 ,
# [rename (name = "m_FrameIndex")] pub m_frame_index : u32 ,
}

#[cfg(feature = "unity_engine-rendering-universal-deferredshaderdata")]
#[::unity2::methods]
impl DeferredShaderData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_instance", args = 0)]
    pub fn get_instance(
    ) -> crate::unity_engine::rendering::universal::deferredshaderdata::DeferredShaderData;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "ResetBuffers", args = 0)]
    pub fn reset_buffers(self) -> ();

    #[method(name = "GetOrUpdateBuffer", args = 3)]
    pub fn get_or_update_buffer(
        self,
        count: i32,
        stride: i32,
        is_constant_buffer: bool,
    ) -> crate::unity_engine::computebuffer::ComputeBuffer;

    #[method(name = "IsLessCircular", args = 2)]
    pub fn is_less_circular(a: u32, b: u32) -> bool;

    #[method(name = "Align", args = 2)]
    pub fn align(s: i32, alignment: i32) -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-rendering-universal-deferredshaderdata")]
impl DeferredShaderData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DeferredShaderData),
                ::core::stringify!(new),
            )
        });
        <Self as IDeferredShaderDataMethods>::ctor(this);
        this
    }
}
