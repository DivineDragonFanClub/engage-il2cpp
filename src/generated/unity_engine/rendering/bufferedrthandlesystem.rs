
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/bufferedrthandlesystem/BufferedRTHandleSystem.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "BufferedRTHandleSystem")]
#[parent(crate::system::object::Object)]
pub struct BufferedRTHandleSystem {
    #[rename(name = "m_RTHandles")]
    pub m_rt_handles: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        ::unity2::Array<crate::unity_engine::rendering::rthandle::RTHandle>,
    >,
    #[rename(name = "m_RTHandleSystem")]
    pub m_rt_handle_system: crate::unity_engine::rendering::rthandlesystem::RTHandleSystem,
    #[rename(name = "m_DisposedValue")]
    pub m_disposed_value: bool,
}

#[cfg(feature = "unity_engine-rendering-bufferedrthandlesystem")]
#[::unity2::methods]
impl BufferedRTHandleSystem {
    #[method(name = "get_maxWidth", args = 0)]
    pub fn get_max_width(self) -> i32;

    #[method(name = "get_maxHeight", args = 0)]
    pub fn get_max_height(self) -> i32;

    #[method(name = "get_rtHandleProperties", args = 0)]
    pub fn get_rt_handle_properties(
        self,
    ) -> crate::unity_engine::rendering::rthandleproperties::RTHandleProperties;

    #[method(name = "GetFrameRT", args = 2)]
    pub fn get_frame_rt(
        self,
        buffer_id: i32,
        frame_index: i32,
    ) -> crate::unity_engine::rendering::rthandle::RTHandle;

    #[method(name = "AllocBuffer", args = 3)]
    pub fn alloc_buffer(
        self,
        buffer_id: i32,
        allocator: crate::system::func_3::Func_3<
            crate::unity_engine::rendering::rthandlesystem::RTHandleSystem,
            i32,
            crate::unity_engine::rendering::rthandle::RTHandle,
        >,
        buffer_count: i32,
    ) -> ();

    #[method(name = "ReleaseBuffer", args = 1)]
    pub fn release_buffer(self, buffer_id: i32) -> ();

    #[method(name = "SwapAndSetReferenceSize", args = 3)]
    pub fn swap_and_set_reference_size(
        self,
        width: i32,
        height: i32,
        msaa_samples: crate::unity_engine::rendering::msaasamples::MSAASamples,
    ) -> ();

    #[method(name = "ResetReferenceSize", args = 2)]
    pub fn reset_reference_size(self, width: i32, height: i32) -> ();

    #[method(name = "Swap", args = 0)]
    pub fn swap(self) -> ();

    #[method(name = "Dispose", args = 1)]
    pub fn dispose(self, disposing: bool) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose_2(self) -> ();

    #[method(name = "ReleaseAll", args = 0)]
    pub fn release_all(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-rendering-bufferedrthandlesystem")]
impl BufferedRTHandleSystem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BufferedRTHandleSystem),
                ::core::stringify!(new),
            )
        });
        <Self as IBufferedRTHandleSystemMethods>::ctor(this);
        this
    }
}
