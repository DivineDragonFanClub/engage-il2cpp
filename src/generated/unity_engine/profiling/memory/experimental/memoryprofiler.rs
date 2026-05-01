
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/profiling/memory/experimental/memoryprofiler/MemoryProfiler.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Profiling.Memory.Experimental",
    name = "MemoryProfiler"
)]
#[parent(crate::system::object::Object)]
pub struct MemoryProfiler {
    #[static_field]
    #[rename(name = "m_SnapshotFinished")]
    pub m_snapshot_finished: crate::system::action_2::Action_2<::unity2::Il2CppString, bool>,
    #[static_field]
    #[rename(name = "m_SaveScreenshotToDisk")]
    pub m_save_screenshot_to_disk: crate::system::action_3::Action_3<
        ::unity2::Il2CppString,
        bool,
        crate::unity_engine::profiling::experimental::debugscreencapture::DebugScreenCapture,
    >,
    #[static_field]
    #[rename(name = "createMetaData")]
    pub create_meta_data: crate::system::action_1::Action_1<
        crate::unity_engine::profiling::memory::experimental::metadata::MetaData,
    >,
}

#[cfg(feature = "unity_engine-profiling-memory-experimental-memoryprofiler")]
#[::unity2::methods]
impl MemoryProfiler {
    #[method(name = "PrepareMetadata", args = 0)]
    pub fn prepare_metadata() -> ::unity2::Array<u8>;

    #[method(name = "WriteIntToByteArray", args = 3)]
    pub fn write_int_to_byte_array(array: ::unity2::Array<u8>, offset: i32, value: i32) -> i32;

    #[method(name = "WriteStringToByteArray", args = 3)]
    pub fn write_string_to_byte_array(
        array: ::unity2::Array<u8>,
        offset: i32,
        value: ::unity2::Il2CppString,
    ) -> i32;

    #[method(name = "FinalizeSnapshot", args = 2)]
    pub fn finalize_snapshot(path: ::unity2::Il2CppString, result: bool) -> ();

    #[method(name = "SaveScreenshotToDisk", args = 7)]
    pub fn save_screenshot_to_disk(
        path: ::unity2::Il2CppString,
        result: bool,
        pixels_ptr: ::unity2::IntPtr,
        pixels_count: i32,
        format: crate::unity_engine::textureformat::TextureFormat,
        width: i32,
        height: i32,
    ) -> ();
}
