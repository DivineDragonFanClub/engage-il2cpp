
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/async_operations/igenericprovideroperation/IGenericProviderOperation.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.AsyncOperations",
    name = "IGenericProviderOperation"
)]
pub struct IGenericProviderOperation {}

#[cfg(feature = "unity_engine-resource_management-async_operations-igenericprovideroperation")]
#[::unity2::methods]
impl IGenericProviderOperation {
    #[method(name = "Init", args = 4)]
    pub fn init(
        self,
        rm: crate::unity_engine::resource_management::resourcemanager::ResourceManager,
        provider : crate :: unity_engine :: resource_management :: resource_providers :: iresourceprovider :: IResourceProvider,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        dep_op : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > >,
    ) -> ();

    #[method(name = "Init", args = 5)]
    pub fn init_2(
        self,
        rm: crate::unity_engine::resource_management::resourcemanager::ResourceManager,
        provider : crate :: unity_engine :: resource_management :: resource_providers :: iresourceprovider :: IResourceProvider,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        dep_op : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > >,
        release_dependencies_on_failure: bool,
    ) -> ();

    #[method(name = "get_ProvideHandleVersion", args = 0)]
    pub fn get_provide_handle_version(self) -> i32;

    #[method(name = "get_Location", args = 0)]
    pub fn get_location (self ,) -> crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ;

    #[method(name = "get_DependencyCount", args = 0)]
    pub fn get_dependency_count(self) -> i32;

    #[method(name = "GetDependencies", args = 1)]
    pub fn get_dependencies(
        self,
        dst_list: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            crate::system::object::Object,
        >,
    ) -> ();

    #[method(name = "SetProgressCallback", args = 1)]
    pub fn set_progress_callback(self, callback: crate::system::func_1::Func_1<f32>) -> ();

    #[method(name = "get_RequestedType", args = 0)]
    pub fn get_requested_type(self) -> ::unity2::SystemType;

    #[method(name = "SetDownloadProgressCallback", args = 1)]
    pub fn set_download_progress_callback(
        self,
        callback : crate :: system :: func_1 :: Func_1 < crate :: unity_engine :: resource_management :: async_operations :: downloadstatus :: DownloadStatus >,
    ) -> ();

    #[method(name = "SetWaitForCompletionCallback", args = 1)]
    pub fn set_wait_for_completion_callback(
        self,
        callback: crate::system::func_1::Func_1<bool>,
    ) -> ();
}
