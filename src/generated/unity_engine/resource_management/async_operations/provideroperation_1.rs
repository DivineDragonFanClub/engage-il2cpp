
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::AsyncOperationBase_1;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::IAsyncOperationBase_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/async_operations/provideroperation_1/ProviderOperation_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.AsyncOperations",
    name = "ProviderOperation`1"
)]
pub struct ProviderOperation_1 < T0 : :: unity2 :: ClassIdentity > {
# [rename (name = "m_ReleaseDependenciesOnFailure")] pub m_release_dependencies_on_failure : bool ,
# [rename (name = "m_GetDepCallback")] pub m_get_dep_callback : crate :: system :: action_2 :: Action_2 < i32 , crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: system :: object :: Object > > ,
# [rename (name = "m_GetProgressCallback")] pub m_get_progress_callback : crate :: system :: func_1 :: Func_1 < f32 > ,
# [rename (name = "m_GetDownloadProgressCallback")] pub m_get_download_progress_callback : crate :: system :: func_1 :: Func_1 < crate :: unity_engine :: resource_management :: async_operations :: downloadstatus :: DownloadStatus > ,
# [rename (name = "m_WaitForCompletionCallback")] pub m_wait_for_completion_callback : crate :: system :: func_1 :: Func_1 < bool > ,
# [rename (name = "m_DownloadStatus")] pub m_download_status : crate :: unity_engine :: resource_management :: async_operations :: downloadstatus :: DownloadStatus ,
# [rename (name = "m_Provider")] pub m_provider : crate :: unity_engine :: resource_management :: resource_providers :: iresourceprovider :: IResourceProvider ,
# [rename (name = "m_DepOp")] pub m_dep_op : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > > ,
# [rename (name = "m_Location")] pub m_location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ,
# [rename (name = "m_ProvideHandleVersion")] pub m_provide_handle_version : i32 ,
# [rename (name = "m_NeedsRelease")] pub m_needs_release : bool ,
# [rename (name = "m_ResourceManager")] pub m_resource_manager : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager ,
# [static_field] # [rename (name = "k_OperationWaitingToCompletePercentComplete")] pub k_operation_waiting_to_complete_percent_complete : f32 ,
# [static_field] # [rename (name = "kInvalidHandleMsg")] pub k_invalid_handle_msg : :: unity2 :: Il2CppString ,
}

#[cfg(feature = "unity_engine-resource_management-async_operations-provideroperation_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ProviderOperation_1<T0> {
    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.ICachable.get_Key",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_cachable_get_key(
        self,
    ) -> crate::unity_engine::resource_management::util::ioperationcachekey::IOperationCacheKey;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.ICachable.set_Key",
        args = 1
    )]
    pub fn unity_engine_resource_management_async_operations_i_cachable_set_key(
        self,
        value : crate :: unity_engine :: resource_management :: util :: ioperationcachekey :: IOperationCacheKey,
    ) -> ();

    #[method(name = "get_ProvideHandleVersion", args = 0)]
    pub fn get_provide_handle_version(self) -> i32;

    #[method(name = "get_Location", args = 0)]
    pub fn get_location (self ,) -> crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ;

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

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = "GetDownloadStatus", args = 1)]
    pub fn get_download_status(
        self,
        visited: crate::system::collections::generic::hashset_1::HashSet_1<
            crate::system::object::Object,
        >,
    ) -> crate::unity_engine::resource_management::async_operations::downloadstatus::DownloadStatus;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetDependencies", args = 1)]
    pub fn get_dependencies(
        self,
        deps : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "ReleaseDependencies", args = 0)]
    pub fn release_dependencies(self) -> ();

    #[method(name = "get_DebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetDependencies", args = 1)]
    pub fn get_dependencies_2(
        self,
        dst_list: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<
            crate::system::object::Object,
        >,
    ) -> ();

    #[method(name = "get_RequestedType", args = 0)]
    pub fn get_requested_type(self) -> ::unity2::SystemType;

    #[method(name = "get_DependencyCount", args = 0)]
    pub fn get_dependency_count(self) -> i32;

    #[method(name = "SetProgressCallback", args = 1)]
    pub fn set_progress_callback(self, callback: crate::system::func_1::Func_1<f32>) -> ();

    #[method(name = "get_Progress", args = 0)]
    pub fn get_progress(self) -> f32;

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

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

    #[method(name = "WaitForCompletionHandler", args = 0)]
    pub fn wait_for_completion_handler(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-async_operations-provideroperation_1")]
impl<T0: ::unity2::ClassIdentity> ProviderOperation_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProviderOperation_1),
                ::core::stringify!(new),
            )
        });
        <Self as IProviderOperation_1Methods<T0>>::ctor(this);
        this
    }
}
