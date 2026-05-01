
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/webrequestqueueoperation/WebRequestQueueOperation.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement",
    name = "WebRequestQueueOperation"
)]
#[parent(crate::system::object::Object)]
pub struct WebRequestQueueOperation {
# [rename (name = "Result")] pub result : crate :: unity_engine :: networking :: unitywebrequestasyncoperation :: UnityWebRequestAsyncOperation ,
# [rename (name = "OnComplete")] pub on_complete : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: networking :: unitywebrequestasyncoperation :: UnityWebRequestAsyncOperation > ,
# [rename (name = "m_WebRequest")] pub m_web_request : crate :: unity_engine :: networking :: unitywebrequest :: UnityWebRequest ,
}

#[cfg(feature = "unity_engine-resource_management-webrequestqueueoperation")]
#[::unity2::methods]
impl WebRequestQueueOperation {
    #[method(name = "get_IsDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        request: crate::unity_engine::networking::unitywebrequest::UnityWebRequest,
    ) -> ();

    #[method(name = "Complete", args = 1)]
    pub fn complete(
        self,
        async_op : crate :: unity_engine :: networking :: unitywebrequestasyncoperation :: UnityWebRequestAsyncOperation,
    ) -> ();
}

#[cfg(feature = "unity_engine-resource_management-webrequestqueueoperation")]
impl WebRequestQueueOperation {
    pub fn new(request: crate::unity_engine::networking::unitywebrequest::UnityWebRequest) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WebRequestQueueOperation),
                ::core::stringify!(new),
            )
        });
        <Self as IWebRequestQueueOperationMethods>::ctor(this, request);
        this
    }
}
