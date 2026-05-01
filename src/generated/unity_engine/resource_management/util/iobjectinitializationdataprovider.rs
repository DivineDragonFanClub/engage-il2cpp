
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/util/iobjectinitializationdataprovider/IObjectInitializationDataProvider.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Util",
    name = "IObjectInitializationDataProvider"
)]
pub struct IObjectInitializationDataProvider {}

#[cfg(feature = "unity_engine-resource_management-util-iobjectinitializationdataprovider")]
#[::unity2::methods]
impl IObjectInitializationDataProvider {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateObjectInitializationData", args = 0)]
    pub fn create_object_initialization_data (self ,) -> crate :: unity_engine :: resource_management :: util :: objectinitializationdata :: ObjectInitializationData ;
}
