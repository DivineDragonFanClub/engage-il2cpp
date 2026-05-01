
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::asyncoperation::AsyncOperation;
use crate::unity_engine::asyncoperation::IAsyncOperation;
use crate::unity_engine::yieldinstruction::IYieldInstruction;
use crate::unity_engine::yieldinstruction::YieldInstruction;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/networking/unitywebrequestasyncoperation/UnityWebRequestAsyncOperation.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Networking",
    name = "UnityWebRequestAsyncOperation"
)]
#[parent(crate::unity_engine::asyncoperation::AsyncOperation)]
pub struct UnityWebRequestAsyncOperation {}

#[cfg(feature = "unity_engine-networking-unitywebrequestasyncoperation")]
#[::unity2::methods]
impl UnityWebRequestAsyncOperation {
    #[method(name = "get_webRequest", args = 0)]
    pub fn get_web_request(
        self,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest;

    #[method(name = "set_webRequest", args = 1)]
    pub fn set_web_request(
        self,
        value: crate::unity_engine::networking::unitywebrequest::UnityWebRequest,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-networking-unitywebrequestasyncoperation")]
impl UnityWebRequestAsyncOperation {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityWebRequestAsyncOperation),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityWebRequestAsyncOperationMethods>::ctor(this);
        this
    }
}
