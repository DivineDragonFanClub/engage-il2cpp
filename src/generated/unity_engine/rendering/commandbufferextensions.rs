
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/commandbufferextensions/CommandBufferExtensions.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "CommandBufferExtensions")]
#[parent(crate::system::object::Object)]
pub struct CommandBufferExtensions {}

#[cfg(feature = "unity_engine-rendering-commandbufferextensions")]
#[::unity2::methods]
impl CommandBufferExtensions {
    #[method(name = "Internal_SwitchIntoFastMemory", args = 5)]
    pub fn internal_switch_into_fast_memory(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        fast_memory_flags: crate::unity_engine::rendering::fastmemoryflags::FastMemoryFlags,
        residency: f32,
        copy_contents: bool,
    ) -> ();

    #[method(name = "Internal_SwitchOutOfFastMemory", args = 3)]
    pub fn internal_switch_out_of_fast_memory(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        rt: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        copy_contents: bool,
    ) -> ();

    #[method(name = "SwitchIntoFastMemory", args = 5)]
    pub fn switch_into_fast_memory(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        rid: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        fast_memory_flags: crate::unity_engine::rendering::fastmemoryflags::FastMemoryFlags,
        residency: f32,
        copy_contents: bool,
    ) -> ();

    #[method(name = "SwitchOutOfFastMemory", args = 3)]
    pub fn switch_out_of_fast_memory(
        cmd: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        rid: crate::unity_engine::rendering::rendertargetidentifier::RenderTargetIdentifier,
        copy_contents: bool,
    ) -> ();
}
