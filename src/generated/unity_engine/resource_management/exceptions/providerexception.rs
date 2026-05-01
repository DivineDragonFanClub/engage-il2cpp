
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::exceptions::operationexception::IOperationException;
use crate::unity_engine::resource_management::exceptions::operationexception::OperationException;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/exceptions/providerexception/ProviderException.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Exceptions",
    name = "ProviderException"
)]
#[parent(
    crate::unity_engine::resource_management::exceptions::operationexception::OperationException
)]
pub struct ProviderException {}

#[cfg(feature = "unity_engine-resource_management-exceptions-providerexception")]
#[::unity2::methods]
impl ProviderException {
    #[method(name = "get_Location", args = 0)]
    pub fn get_location (self ,) -> crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ;
}
