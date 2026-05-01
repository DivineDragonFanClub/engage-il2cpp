
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/webrequestqueue/WebRequestQueue.md")))]
#[::unity2::class(namespace = "UnityEngine.ResourceManagement", name = "WebRequestQueue")]
#[parent(crate::system::object::Object)]
pub struct WebRequestQueue {
# [static_field] # [rename (name = "s_MaxRequest")] pub s_max_request : i32 ,
# [static_field] # [rename (name = "s_QueuedOperations")] pub s_queued_operations : crate :: system :: collections :: generic :: queue_1 :: Queue_1 < crate :: unity_engine :: resource_management :: webrequestqueueoperation :: WebRequestQueueOperation > ,
# [static_field] # [rename (name = "s_ActiveRequests")] pub s_active_requests : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: networking :: unitywebrequestasyncoperation :: UnityWebRequestAsyncOperation > ,
}

#[cfg(feature = "unity_engine-resource_management-webrequestqueue")]
#[::unity2::methods]
impl WebRequestQueue {
    #[method(name = "SetMaxConcurrentRequests", args = 1)]
    pub fn set_max_concurrent_requests(max_requests: i32) -> ();

    #[method(name = "QueueRequest", args = 1)]
    pub fn queue_request(
        request: crate::unity_engine::networking::unitywebrequest::UnityWebRequest,
    ) -> crate::unity_engine::resource_management::webrequestqueueoperation::WebRequestQueueOperation;

    #[method(name = "OnWebAsyncOpComplete", args = 1)]
    pub fn on_web_async_op_complete(
        operation: crate::unity_engine::asyncoperation::AsyncOperation,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
