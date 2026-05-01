
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::exceptions::operationexception::IOperationException;
use crate::unity_engine::resource_management::exceptions::operationexception::OperationException;
use crate::unity_engine::resource_management::exceptions::providerexception::IProviderException;
use crate::unity_engine::resource_management::exceptions::providerexception::ProviderException;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/exceptions/remoteproviderexception/RemoteProviderException.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Exceptions",
    name = "RemoteProviderException"
)]
#[parent(
    crate::unity_engine::resource_management::exceptions::providerexception::ProviderException
)]
pub struct RemoteProviderException {}

#[cfg(feature = "unity_engine-resource_management-exceptions-remoteproviderexception")]
#[::unity2::methods]
impl RemoteProviderException {
    #[method(name = "get_WebRequestResult", args = 0)]
    pub fn get_web_request_result(
        self,
    ) -> crate::unity_engine::resource_management::util::unitywebrequestresult::UnityWebRequestResult;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}
