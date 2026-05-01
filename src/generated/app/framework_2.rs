
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/framework_2/Framework_2.md")))]
#[::unity2::class(namespace = "App", name = "Framework")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: framework_2 :: Framework_2 >)]
pub struct Framework_2 {
    #[static_field]
    #[rename(name = "s_PauseCount")]
    pub s_pause_count: i32,
    #[static_field]
    #[rename(name = "s_TotalMemory")]
    pub s_total_memory: i64,
    #[static_field]
    #[rename(name = "s_GCCount")]
    pub s_gc_count: i32,
    #[static_field]
    #[rename(name = "s_GCBind")]
    pub s_gc_bind: crate::app::bindholder::BindHolder,
    #[static_field]
    #[rename(name = "s_DestoryObjects")]
    pub s_destory_objects: crate::system::collections::generic::queue_1::Queue_1<
        crate::unity_engine::object_2::Object_2,
    >,
    #[static_field]
    #[rename(name = "s_LockDestroy")]
    pub s_lock_destroy: ::unity2::IlInstance,
    #[static_field]
    #[rename(name = "LoadingPriority_Default")]
    pub loading_priority_default: crate::unity_engine::threadpriority::ThreadPriority,
    #[static_field]
    #[rename(name = "LoadingPriority_Loading")]
    pub loading_priority_loading: crate::unity_engine::threadpriority::ThreadPriority,
    #[static_field]
    #[rename(name = "AsyncUploadTimeSlice_Default")]
    pub async_upload_time_slice_default: i32,
    #[static_field]
    #[rename(name = "AsyncUploadTimeSlice_Loading")]
    pub async_upload_time_slice_loading: i32,
}

#[cfg(feature = "app-framework_2")]
#[::unity2::methods]
impl Framework_2 {
    #[method(name = "IsBoostMode", args = 0)]
    pub fn is_boost_mode() -> bool;

    #[method(name = "OnBeforeSceneLoadRuntimeMethod", args = 0)]
    pub fn on_before_scene_load_runtime_method() -> ();

    #[method(name = "OnAfterSceneLoadRuntimeMethod", args = 0)]
    pub fn on_after_scene_load_runtime_method() -> ();

    #[method(name = "OnRuntimeMethodLoad", args = 0)]
    pub fn on_runtime_method_load() -> ();

    #[method(name = "UpdateResolution", args = 0)]
    pub fn update_resolution() -> ();

    #[method(name = "NotificationMessageReceived", args = 1)]
    pub fn notification_message_received(
        message: crate::unity_engine::switch::notification::Notification_Message,
    ) -> ();

    #[method(name = "ResetSetting", args = 0)]
    pub fn reset_setting() -> ();

    #[method(name = "InitTotalMemory", args = 0)]
    pub fn init_total_memory() -> ();

    #[method(name = "GetTotalMemory", args = 0)]
    pub fn get_total_memory() -> i64;

    #[method(name = "GetAllocatableMemory", args = 0)]
    pub fn get_allocatable_memory() -> i64;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "UpdatePause", args = 0)]
    pub fn update_pause(self) -> ();

    #[method(name = "CpuBoostOn", args = 0)]
    pub fn cpu_boost_on() -> ();

    #[method(name = "CpuBoostOff", args = 0)]
    pub fn cpu_boost_off() -> ();

    #[method(name = "CpuBoostOnForLoading", args = 0)]
    pub fn cpu_boost_on_for_loading() -> ();

    #[method(name = "CpuBoostOffForLoading", args = 0)]
    pub fn cpu_boost_off_for_loading() -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "IsPausing", args = 0)]
    pub fn is_pausing() -> bool;

    #[method(name = "Shoutdown", args = 0)]
    pub fn shoutdown() -> ();

    #[method(name = "GCBegin", args = 0)]
    pub fn gc_begin() -> ();

    #[method(name = "GCEnd", args = 0)]
    pub fn gc_end() -> ();

    #[method(name = "GCCollect", args = 0)]
    pub fn gc_collect() -> ();

    #[method(name = "GCCollectLow", args = 0)]
    pub fn gc_collect_low() -> ();

    #[method(name = "GCCommit", args = 0)]
    pub fn gc_commit() -> ();

    #[method(name = "GCBind", args = 0)]
    pub fn gc_bind() -> ();

    #[method(name = "GCUnbind", args = 0)]
    pub fn gc_unbind() -> ();

    #[method(name = "CollectCoroutine", args = 1)]
    pub fn collect_coroutine(time: f32) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "AddDestroy", args = 1)]
    pub fn add_destroy(obj: crate::unity_engine::object_2::Object_2) -> ();

    #[method(name = "UpdateDestroy", args = 0)]
    pub fn update_destroy() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-framework_2")]
impl Framework_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Framework_2),
                ::core::stringify!(new),
            )
        });
        <Self as IFramework_2Methods>::ctor(this);
        this
    }
}
