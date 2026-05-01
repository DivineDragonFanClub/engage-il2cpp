
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/isceneprovider_interface/ISceneProvider_Interface.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "ISceneProvider"
)]
pub struct ISceneProvider_Interface {}

#[cfg(feature = "unity_engine-resource_management-resource_providers-isceneprovider_interface")]
#[::unity2::methods]
impl ISceneProvider_Interface {
    #[method(name = "ProvideScene", args = 5)]
    pub fn provide_scene (self , resource_manager : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager , location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation , load_mode : crate :: unity_engine :: scene_management :: loadscenemode :: LoadSceneMode , activate_on_load : bool , priority : i32) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "ReleaseScene", args = 2)]
    pub fn release_scene (self , resource_manager : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager , scene_load_handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance >) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;
}
