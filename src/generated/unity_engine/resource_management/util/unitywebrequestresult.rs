
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/util/unitywebrequestresult/UnityWebRequestResult.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Util",
    name = "UnityWebRequestResult"
)]
#[parent(crate::system::object::Object)]
pub struct UnityWebRequestResult {}

#[cfg(feature = "unity_engine-resource_management-util-unitywebrequestresult")]
#[::unity2::methods]
impl UnityWebRequestResult {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        request: crate::unity_engine::networking::unitywebrequest::UnityWebRequest,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Error", args = 0)]
    pub fn get_error(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ResponseCode", args = 0)]
    pub fn get_response_code(self) -> i64;

    #[method(name = "get_Result", args = 0)]
    pub fn get_result(
        self,
    ) -> crate::unity_engine::networking::unitywebrequest::UnityWebRequest_Result;

    #[method(name = "get_Method", args = 0)]
    pub fn get_method(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Url", args = 0)]
    pub fn get_url(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "unity_engine-resource_management-util-unitywebrequestresult")]
impl UnityWebRequestResult {
    pub fn new(request: crate::unity_engine::networking::unitywebrequest::UnityWebRequest) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityWebRequestResult),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityWebRequestResultMethods>::ctor(this, request);
        this
    }
}
