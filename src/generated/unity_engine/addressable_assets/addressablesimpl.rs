
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::AsyncOperationBase_1;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::IAsyncOperationBase_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/addressablesimpl/AddressablesImpl.md")))]
#[::unity2::class(namespace = "UnityEngine.AddressableAssets", name = "AddressablesImpl")]
#[parent(crate::system::object::Object)]
pub struct AddressablesImpl {
# [rename (name = "m_ResourceManager")] pub m_resource_manager : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager ,
# [rename (name = "m_InstanceProvider")] pub m_instance_provider : crate :: unity_engine :: resource_management :: resource_providers :: iinstanceprovider_interface :: IInstanceProvider_Interface ,
# [rename (name = "m_CatalogRequestsTimeout")] pub m_catalog_requests_timeout : i32 ,
# [static_field] # [rename (name = "kCacheDataFolder")] pub k_cache_data_folder : :: unity2 :: Il2CppString ,
# [rename (name = "SceneProvider")] pub scene_provider : crate :: unity_engine :: resource_management :: resource_providers :: isceneprovider_interface :: ISceneProvider_Interface ,
# [rename (name = "m_ResourceLocators")] pub m_resource_locators : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: addressable_assets :: addressablesimpl :: AddressablesImpl_ResourceLocatorInfo > ,
# [rename (name = "m_InitializationOperation")] pub m_initialization_operation : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator > ,
# [rename (name = "m_ActiveCheckUpdateOperation")] pub m_active_check_update_operation : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: list_1 :: List_1 < :: unity2 :: Il2CppString > > ,
# [rename (name = "m_ActiveUpdateOperation")] pub m_active_update_operation : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator > > ,
# [rename (name = "m_OnHandleCompleteAction")] pub m_on_handle_complete_action : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > ,
# [rename (name = "m_OnSceneHandleCompleteAction")] pub m_on_scene_handle_complete_action : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > ,
# [rename (name = "m_OnHandleDestroyedAction")] pub m_on_handle_destroyed_action : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > ,
# [rename (name = "m_resultToHandle")] pub m_result_to_handle : crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2 < crate :: system :: object :: Object , crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > ,
# [rename (name = "m_SceneInstances")] pub m_scene_instances : crate :: system :: collections :: generic :: hashset_1 :: HashSet_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > ,
# [rename (name = "hasStartedInitialization")] pub has_started_initialization : bool ,
}

#[cfg(feature = "unity_engine-addressable_assets-addressablesimpl")]
#[::unity2::methods]
impl AddressablesImpl {
    #[method(name = "get_InstanceProvider", args = 0)]
    pub fn get_instance_provider (self ,) -> crate :: unity_engine :: resource_management :: resource_providers :: iinstanceprovider_interface :: IInstanceProvider_Interface ;

    #[method(name = "set_InstanceProvider", args = 1)]
    pub fn set_instance_provider(
        self,
        value : crate :: unity_engine :: resource_management :: resource_providers :: iinstanceprovider_interface :: IInstanceProvider_Interface,
    ) -> ();

    #[method(name = "get_ResourceManager", args = 0)]
    pub fn get_resource_manager(
        self,
    ) -> crate::unity_engine::resource_management::resourcemanager::ResourceManager;

    #[method(name = "get_CatalogRequestsTimeout", args = 0)]
    pub fn get_catalog_requests_timeout(self) -> i32;

    #[method(name = "set_CatalogRequestsTimeout", args = 1)]
    pub fn set_catalog_requests_timeout(self, value: i32) -> ();

    #[method(name = "get_SceneOperationCount", args = 0)]
    pub fn get_scene_operation_count(self) -> i32;

    #[method(name = "get_TrackedHandleCount", args = 0)]
    pub fn get_tracked_handle_count(self) -> i32;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        alloc : crate :: unity_engine :: resource_management :: util :: iallocationstrategy :: IAllocationStrategy,
    ) -> ();

    #[method(name = "ReleaseSceneManagerOperation", args = 0)]
    pub fn release_scene_manager_operation(self) -> ();

    #[method(name = "get_InternalIdTransformFunc", args = 0)]
    pub fn get_internal_id_transform_func (self ,) -> crate :: system :: func_2 :: Func_2 < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation , :: unity2 :: Il2CppString > ;

    #[method(name = "set_InternalIdTransformFunc", args = 1)]
    pub fn set_internal_id_transform_func(
        self,
        value : crate :: system :: func_2 :: Func_2 < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation , :: unity2 :: Il2CppString >,
    ) -> ();

    #[method(name = "get_WebRequestOverride", args = 0)]
    pub fn get_web_request_override(
        self,
    ) -> crate::system::action_1::Action_1<
        crate::unity_engine::networking::unitywebrequest::UnityWebRequest,
    >;

    #[method(name = "set_WebRequestOverride", args = 1)]
    pub fn set_web_request_override(
        self,
        value: crate::system::action_1::Action_1<
            crate::unity_engine::networking::unitywebrequest::UnityWebRequest,
        >,
    ) -> ();

    #[method(name = "get_ChainOperation", args = 0)]
    pub fn get_chain_operation (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "get_ShouldChainRequest", args = 0)]
    pub fn get_should_chain_request(self) -> bool;

    #[method(name = "OnSceneUnloaded", args = 1)]
    pub fn on_scene_unloaded(
        self,
        scene: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();

    #[method(name = "get_StreamingAssetsSubFolder", args = 0)]
    pub fn get_streaming_assets_sub_folder(self) -> ::unity2::Il2CppString;

    #[method(name = "get_BuildPath", args = 0)]
    pub fn get_build_path(self) -> ::unity2::Il2CppString;

    #[method(name = "get_PlayerBuildDataPath", args = 0)]
    pub fn get_player_build_data_path(self) -> ::unity2::Il2CppString;

    #[method(name = "get_RuntimePath", args = 0)]
    pub fn get_runtime_path(self) -> ::unity2::Il2CppString;

    #[method(name = "Log", args = 1)]
    pub fn log(self, msg: ::unity2::Il2CppString) -> ();

    #[method(name = "LogFormat", args = 2)]
    pub fn log_format(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "LogWarning", args = 1)]
    pub fn log_warning(self, msg: ::unity2::Il2CppString) -> ();

    #[method(name = "LogWarningFormat", args = 2)]
    pub fn log_warning_format(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "LogError", args = 1)]
    pub fn log_error(self, msg: ::unity2::Il2CppString) -> ();

    #[method(name = "LogErrorFormat", args = 2)]
    pub fn log_error_format(
        self,
        format: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "ResolveInternalId", args = 1)]
    pub fn resolve_internal_id(self, id: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "get_ResourceLocators", args = 0)]
    pub fn get_resource_locators (self ,) -> crate :: system :: collections :: generic :: ienumerable_1 :: IEnumerable_1 < crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator > ;

    #[method(name = "AddResourceLocator", args = 3)]
    pub fn add_resource_locator(
        self,
        loc : crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator,
        local_catalog_hash: ::unity2::Il2CppString,
        remote_catalog_location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> ();

    #[method(name = "RemoveResourceLocator", args = 1)]
    pub fn remove_resource_locator(
        self,
        loc : crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator,
    ) -> ();

    #[method(name = "ClearResourceLocators", args = 0)]
    pub fn clear_resource_locators(self) -> ();

    #[method(name = "GetResourceLocations", args = 3)]
    pub fn get_resource_locations(
        self,
        key: crate::system::object::Object,
        r#type: ::unity2::SystemType,
        locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation >,
    ) -> bool;

    #[method(name = "GetResourceLocations", args = 4)]
    pub fn get_resource_locations_2(
        self,
        keys: crate::system::collections::ienumerable::IEnumerable,
        r#type: ::unity2::SystemType,
        merge: crate::unity_engine::addressable_assets::addressables::Addressables_MergeMode,
        locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation >,
    ) -> bool;

    #[method(name = "InitializeAsync", args = 3)]
    pub fn initialize_async (self , runtime_data_path : :: unity2 :: Il2CppString , provider_suffix : :: unity2 :: Il2CppString , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator > ;

    #[method(name = "InitializeAsync", args = 0)]
    pub fn initialize_async_2 (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator > ;

    #[method(name = "CreateCatalogLocationWithHashDependencies", args = 2)]
    pub fn create_catalog_location_with_hash_dependencies (self , catalog_path : :: unity2 :: Il2CppString , hash_file_path : :: unity2 :: Il2CppString) -> crate :: unity_engine :: resource_management :: resource_locations :: resourcelocationbase :: ResourceLocationBase ;

    #[method(name = "LoadContentCatalogAsync", args = 3)]
    pub fn load_content_catalog_async (self , catalog_path : :: unity2 :: Il2CppString , auto_release_handle : bool , provider_suffix : :: unity2 :: Il2CppString) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator > ;

    #[method(name = "TrackHandle", args = 1)]
    pub fn track_handle (self , handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance >) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "TrackHandle", args = 1)]
    pub fn track_handle_2 (self , handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "ClearTrackHandles", args = 0)]
    pub fn clear_track_handles(self) -> ();

    #[method(name = "LoadResourceLocationsWithChain", args = 4)]
    pub fn load_resource_locations_with_chain (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , keys : crate :: system :: collections :: ienumerable :: IEnumerable , mode : crate :: unity_engine :: addressable_assets :: addressables :: Addressables_MergeMode , r#type : :: unity2 :: SystemType) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > > ;

    #[method(name = "LoadResourceLocationsAsync", args = 3)]
    pub fn load_resource_locations_async (self , keys : crate :: system :: collections :: ienumerable :: IEnumerable , mode : crate :: unity_engine :: addressable_assets :: addressables :: Addressables_MergeMode , r#type : :: unity2 :: SystemType) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > > ;

    #[method(name = "LoadResourceLocationsWithChain", args = 3)]
    pub fn load_resource_locations_with_chain_2 (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , key : crate :: system :: object :: Object , r#type : :: unity2 :: SystemType) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > > ;

    #[method(name = "LoadResourceLocationsAsync", args = 2)]
    pub fn load_resource_locations_async_2 (self , key : crate :: system :: object :: Object , r#type : :: unity2 :: SystemType) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > > ;

    #[method(name = "OnHandleDestroyed", args = 1)]
    pub fn on_handle_destroyed(
        self,
        handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle,
    ) -> ();

    #[method(name = "OnSceneHandleCompleted", args = 1)]
    pub fn on_scene_handle_completed(
        self,
        handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle,
    ) -> ();

    #[method(name = "OnHandleCompleted", args = 1)]
    pub fn on_handle_completed(
        self,
        handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle,
    ) -> ();

    #[method(name = "Release", args = 1)]
    pub fn release(
        self,
        handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle,
    ) -> ();

    #[method(name = "GetDownloadSizeWithChain", args = 2)]
    pub fn get_download_size_with_chain (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , key : crate :: system :: object :: Object) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < i64 > ;

    #[method(name = "GetDownloadSizeWithChain", args = 2)]
    pub fn get_download_size_with_chain_2 (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , keys : crate :: system :: collections :: ienumerable :: IEnumerable) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < i64 > ;

    #[method(name = "GetDownloadSizeAsync", args = 1)]
    pub fn get_download_size_async (self , key : crate :: system :: object :: Object) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < i64 > ;

    #[method(name = "GetDownloadSizeAsync", args = 1)]
    pub fn get_download_size_async_2 (self , keys : crate :: system :: collections :: ienumerable :: IEnumerable) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < i64 > ;

    #[method(name = "DownloadDependenciesAsyncWithChain", args = 3)]
    pub fn download_dependencies_async_with_chain (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , key : crate :: system :: object :: Object , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "GatherDependenciesFromLocations", args = 1)]
    pub fn gather_dependencies_from_locations (locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation >) -> crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > ;

    #[method(name = "DownloadDependenciesAsync", args = 2)]
    pub fn download_dependencies_async (self , key : crate :: system :: object :: Object , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "DownloadDependenciesAsyncWithChain", args = 3)]
    pub fn download_dependencies_async_with_chain_2 (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "DownloadDependenciesAsync", args = 2)]
    pub fn download_dependencies_async_2 (self , locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "DownloadDependenciesAsyncWithChain", args = 4)]
    pub fn download_dependencies_async_with_chain_3 (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , keys : crate :: system :: collections :: ienumerable :: IEnumerable , mode : crate :: unity_engine :: addressable_assets :: addressables :: Addressables_MergeMode , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "DownloadDependenciesAsync", args = 3)]
    pub fn download_dependencies_async_3 (self , keys : crate :: system :: collections :: ienumerable :: IEnumerable , mode : crate :: unity_engine :: addressable_assets :: addressables :: Addressables_MergeMode , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "ClearDependencyCacheForKey", args = 1)]
    pub fn clear_dependency_cache_for_key(self, key: crate::system::object::Object) -> bool;

    #[method(name = "AutoReleaseHandleOnCompletion", args = 1)]
    pub fn auto_release_handle_on_completion(
        self,
        handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle,
    ) -> ();

    #[method(name = "ClearDependencyCacheAsync", args = 2)]
    pub fn clear_dependency_cache_async (self , key : crate :: system :: object :: Object , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < bool > ;

    #[method(name = "ClearDependencyCacheAsync", args = 2)]
    pub fn clear_dependency_cache_async_2 (self , locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < bool > ;

    #[method(name = "ClearDependencyCacheAsync", args = 2)]
    pub fn clear_dependency_cache_async_3 (self , keys : crate :: system :: collections :: ienumerable :: IEnumerable , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < bool > ;

    #[method(name = "InstantiateAsync", args = 4)]
    pub fn instantiate_async (self , location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation , parent : crate :: unity_engine :: transform :: Transform , instantiate_in_world_space : bool , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "InstantiateAsync", args = 5)]
    pub fn instantiate_async_2 (self , location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation , position : crate :: unity_engine :: vector3 :: Vector3 , rotation : crate :: unity_engine :: quaternion :: Quaternion , parent : crate :: unity_engine :: transform :: Transform , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "InstantiateAsync", args = 4)]
    pub fn instantiate_async_3 (self , key : crate :: system :: object :: Object , parent : crate :: unity_engine :: transform :: Transform , instantiate_in_world_space : bool , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "InstantiateAsync", args = 5)]
    pub fn instantiate_async_4 (self , key : crate :: system :: object :: Object , position : crate :: unity_engine :: vector3 :: Vector3 , rotation : crate :: unity_engine :: quaternion :: Quaternion , parent : crate :: unity_engine :: transform :: Transform , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "InstantiateWithChain", args = 4)]
    pub fn instantiate_with_chain (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , key : crate :: system :: object :: Object , instantiate_parameters : crate :: unity_engine :: resource_management :: resource_providers :: instantiationparameters :: InstantiationParameters , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "InstantiateAsync", args = 3)]
    pub fn instantiate_async_5 (self , key : crate :: system :: object :: Object , instantiate_parameters : crate :: unity_engine :: resource_management :: resource_providers :: instantiationparameters :: InstantiationParameters , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "InstantiateWithChain", args = 4)]
    pub fn instantiate_with_chain_2 (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation , instantiate_parameters : crate :: unity_engine :: resource_management :: resource_providers :: instantiationparameters :: InstantiationParameters , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "InstantiateAsync", args = 3)]
    pub fn instantiate_async_6 (self , location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation , instantiate_parameters : crate :: unity_engine :: resource_management :: resource_providers :: instantiationparameters :: InstantiationParameters , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: gameobject :: GameObject > ;

    #[method(name = "ReleaseInstance", args = 1)]
    pub fn release_instance(self, instance: crate::unity_engine::gameobject::GameObject) -> bool;

    #[method(name = "LoadSceneWithChain", args = 5)]
    pub fn load_scene_with_chain (self , dep : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , key : crate :: system :: object :: Object , load_mode : crate :: unity_engine :: scene_management :: loadscenemode :: LoadSceneMode , activate_on_load : bool , priority : i32) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "LoadSceneAsync", args = 5)]
    pub fn load_scene_async (self , key : crate :: system :: object :: Object , load_mode : crate :: unity_engine :: scene_management :: loadscenemode :: LoadSceneMode , activate_on_load : bool , priority : i32 , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "LoadSceneAsync", args = 5)]
    pub fn load_scene_async_2 (self , location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation , load_mode : crate :: unity_engine :: scene_management :: loadscenemode :: LoadSceneMode , activate_on_load : bool , priority : i32 , track_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "UnloadSceneAsync", args = 2)]
    pub fn unload_scene_async (self , scene : crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "UnloadSceneAsync", args = 2)]
    pub fn unload_scene_async_2 (self , handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "UnloadSceneAsync", args = 2)]
    pub fn unload_scene_async_3 (self , handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "CreateUnloadSceneWithChain", args = 2)]
    pub fn create_unload_scene_with_chain (self , handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "CreateUnloadSceneWithChain", args = 2)]
    pub fn create_unload_scene_with_chain_2 (self , handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "InternalUnloadScene", args = 2)]
    pub fn internal_unload_scene (self , handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "EvaluateKey", args = 1)]
    pub fn evaluate_key(self, obj: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "CheckForCatalogUpdates", args = 1)]
    pub fn check_for_catalog_updates (self , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: list_1 :: List_1 < :: unity2 :: Il2CppString > > ;

    #[method(name = "GetLocatorInfo", args = 1)]
    pub fn get_locator_info (self , c : :: unity2 :: Il2CppString) -> crate :: unity_engine :: addressable_assets :: addressablesimpl :: AddressablesImpl_ResourceLocatorInfo ;

    #[method(name = "get_CatalogsWithAvailableUpdates", args = 0)]
    pub fn get_catalogs_with_available_updates(
        self,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<::unity2::Il2CppString>;

    #[method(name = "UpdateCatalogs", args = 2)]
    pub fn update_catalogs (self , catalog_ids : crate :: system :: collections :: generic :: ienumerable_1 :: IEnumerable_1 < :: unity2 :: Il2CppString > , auto_release_handle : bool) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator > > ;

    #[method(name = "Equals", args = 2)]
    pub fn equals(
        self,
        x : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        y : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> bool;

    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(
        self,
        loc : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> i32;
}

#[cfg(feature = "unity_engine-addressable_assets-addressablesimpl")]
impl AddressablesImpl {
    pub fn new(
        alloc : crate :: unity_engine :: resource_management :: util :: iallocationstrategy :: IAllocationStrategy,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AddressablesImpl),
                ::core::stringify!(new),
            )
        });
        <Self as IAddressablesImplMethods>::ctor(this, alloc);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/addressablesimpl/AddressablesImpl_LoadResourceLocationKeysOp.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets",
    name = "AddressablesImpl.LoadResourceLocationKeysOp"
)]
# [parent (crate :: unity_engine :: resource_management :: async_operations :: asyncoperationbase_1 :: AsyncOperationBase_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > >)]
pub struct AddressablesImpl_LoadResourceLocationKeysOp {
# [rename (name = "m_Key")] pub m_key : crate :: system :: collections :: ienumerable :: IEnumerable ,
# [rename (name = "m_MergeMode")] pub m_merge_mode : crate :: unity_engine :: addressable_assets :: addressables :: Addressables_MergeMode ,
# [rename (name = "m_locations")] pub m_locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > ,
# [rename (name = "m_Addressables")] pub m_addressables : crate :: unity_engine :: addressable_assets :: addressablesimpl :: AddressablesImpl ,
# [rename (name = "m_ResourceType")] pub m_resource_type : :: unity2 :: SystemType ,
}

#[cfg(feature = "unity_engine-addressable_assets-addressablesimpl")]
#[::unity2::methods]
impl AddressablesImpl_LoadResourceLocationKeysOp {
    #[method(name = "get_DebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Init", args = 4)]
    pub fn init(
        self,
        aa: crate::unity_engine::addressable_assets::addressablesimpl::AddressablesImpl,
        t: ::unity2::SystemType,
        key: crate::system::collections::ienumerable::IEnumerable,
        merge_mode: crate::unity_engine::addressable_assets::addressables::Addressables_MergeMode,
    ) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-addressablesimpl")]
impl AddressablesImpl_LoadResourceLocationKeysOp {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AddressablesImpl_LoadResourceLocationKeysOp),
                ::core::stringify!(new),
            )
        });
        <Self as IAddressablesImpl_LoadResourceLocationKeysOpMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/addressablesimpl/AddressablesImpl_LoadResourceLocationKeyOp.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets",
    name = "AddressablesImpl.LoadResourceLocationKeyOp"
)]
# [parent (crate :: unity_engine :: resource_management :: async_operations :: asyncoperationbase_1 :: AsyncOperationBase_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > >)]
pub struct AddressablesImpl_LoadResourceLocationKeyOp {
# [rename (name = "m_Keys")] pub m_keys : :: unity2 :: IlInstance ,
# [rename (name = "m_locations")] pub m_locations : crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation > ,
# [rename (name = "m_Addressables")] pub m_addressables : crate :: unity_engine :: addressable_assets :: addressablesimpl :: AddressablesImpl ,
# [rename (name = "m_ResourceType")] pub m_resource_type : :: unity2 :: SystemType ,
}

#[cfg(feature = "unity_engine-addressable_assets-addressablesimpl")]
#[::unity2::methods]
impl AddressablesImpl_LoadResourceLocationKeyOp {
    #[method(name = "get_DebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Init", args = 3)]
    pub fn init(
        self,
        aa: crate::unity_engine::addressable_assets::addressablesimpl::AddressablesImpl,
        t: ::unity2::SystemType,
        keys: crate::system::object::Object,
    ) -> ();

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-addressablesimpl")]
impl AddressablesImpl_LoadResourceLocationKeyOp {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AddressablesImpl_LoadResourceLocationKeyOp),
                ::core::stringify!(new),
            )
        });
        <Self as IAddressablesImpl_LoadResourceLocationKeyOpMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/addressablesimpl/AddressablesImpl_ResourceLocatorInfo.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets",
    name = "AddressablesImpl.ResourceLocatorInfo"
)]
#[parent(crate::system::object::Object)]
pub struct AddressablesImpl_ResourceLocatorInfo {}

#[cfg(feature = "unity_engine-addressable_assets-addressablesimpl")]
#[::unity2::methods]
impl AddressablesImpl_ResourceLocatorInfo {
    #[method(name = "get_Locator", args = 0)]
    pub fn get_locator (self ,) -> crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator ;

    #[method(name = "set_Locator", args = 1)]
    pub fn set_locator(
        self,
        value : crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator,
    ) -> ();

    #[method(name = "get_LocalHash", args = 0)]
    pub fn get_local_hash(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LocalHash", args = 1)]
    pub fn set_local_hash(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CatalogLocation", args = 0)]
    pub fn get_catalog_location (self ,) -> crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ;

    #[method(name = "set_CatalogLocation", args = 1)]
    pub fn set_catalog_location(
        self,
        value : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> ();

    #[method(name = "get_ContentUpdateAvailable", args = 0)]
    pub fn get_content_update_available(self) -> bool;

    #[method(name = "set_ContentUpdateAvailable", args = 1)]
    pub fn set_content_update_available(self, value: bool) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        loc : crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator,
        local_hash: ::unity2::Il2CppString,
        remote_catalog_location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> ();

    #[method(name = "get_HashLocation", args = 0)]
    pub fn get_hash_location (self ,) -> crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ;

    #[method(name = "get_CanUpdateContent", args = 0)]
    pub fn get_can_update_content(self) -> bool;

    #[method(name = "UpdateContent", args = 3)]
    pub fn update_content(
        self,
        locator : crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator,
        hash: ::unity2::Il2CppString,
        loc : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-addressablesimpl")]
impl AddressablesImpl_ResourceLocatorInfo {
    pub fn new(
        loc : crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator,
        local_hash: ::unity2::Il2CppString,
        remote_catalog_location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AddressablesImpl_ResourceLocatorInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IAddressablesImpl_ResourceLocatorInfoMethods>::ctor(
            this,
            loc,
            local_hash,
            remote_catalog_location,
        );
        this
    }
}
