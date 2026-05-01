
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/async_operations/iasyncoperation/IAsyncOperation.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.AsyncOperations",
    name = "IAsyncOperation"
)]
pub struct IAsyncOperation {}

#[cfg(feature = "unity_engine-resource_management-async_operations-iasyncoperation")]
#[::unity2::methods]
impl IAsyncOperation {
    #[method(name = "GetResultAsObject", args = 0)]
    pub fn get_result_as_object(self) -> crate::system::object::Object;

    #[method(name = "get_ResultType", args = 0)]
    pub fn get_result_type(self) -> ::unity2::SystemType;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "get_DebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "DecrementReferenceCount", args = 0)]
    pub fn decrement_reference_count(self) -> ();

    #[method(name = "IncrementReferenceCount", args = 0)]
    pub fn increment_reference_count(self) -> ();

    #[method(name = "get_ReferenceCount", args = 0)]
    pub fn get_reference_count(self) -> i32;

    #[method(name = "get_PercentComplete", args = 0)]
    pub fn get_percent_complete(self) -> f32;

    #[method(name = "GetDownloadStatus", args = 1)]
    pub fn get_download_status(
        self,
        visited: crate::system::collections::generic::hashset_1::HashSet_1<
            crate::system::object::Object,
        >,
    ) -> crate::unity_engine::resource_management::async_operations::downloadstatus::DownloadStatus;

    #[method(name = "get_Status", args = 0)]
    pub fn get_status (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationstatus :: AsyncOperationStatus ;

    #[method(name = "get_IsDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = "set_OnDestroy", args = 1)]
    pub fn set_on_destroy(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: iasyncoperation :: IAsyncOperation >,
    ) -> ();

    #[method(name = "GetDependencies", args = 1)]
    pub fn get_dependencies(
        self,
        deps : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "get_IsRunning", args = 0)]
    pub fn get_is_running(self) -> bool;

    #[method(name = "add_CompletedTypeless", args = 1)]
    pub fn add_completed_typeless(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "remove_CompletedTypeless", args = 1)]
    pub fn remove_completed_typeless(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "add_Destroyed", args = 1)]
    pub fn add_destroyed(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "remove_Destroyed", args = 1)]
    pub fn remove_destroyed(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "InvokeCompletionEvent", args = 0)]
    pub fn invoke_completion_event(self) -> ();

    #[method(name = "get_Handle", args = 0)]
    pub fn get_handle (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "WaitForCompletion", args = 0)]
    pub fn wait_for_completion(self) -> ();
}
