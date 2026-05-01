
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/async_operations/asyncoperationbase_1/AsyncOperationBase_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.AsyncOperations",
    name = "AsyncOperationBase`1"
)]
pub struct AsyncOperationBase_1 < T0 : :: unity2 :: ClassIdentity > {
# [rename (name = "m_referenceCount")] pub m_reference_count : i32 ,
# [rename (name = "m_Status")] pub m_status : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationstatus :: AsyncOperationStatus ,
# [rename (name = "m_RM")] pub m_rm : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager ,
# [rename (name = "m_Version")] pub m_version : i32 ,
# [rename (name = "m_OnDestroyAction")] pub m_on_destroy_action : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: iasyncoperation :: IAsyncOperation > ,
# [rename (name = "m_dependencyCompleteAction")] pub m_dependency_complete_action : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > ,
# [rename (name = "HasExecuted")] pub has_executed : bool ,
# [rename (name = "m_InDeferredCallbackQueue")] pub m_in_deferred_callback_queue : bool ,
# [rename (name = "m_UpdateCallback")] pub m_update_callback : crate :: system :: action_1 :: Action_1 < f32 > ,
}

#[cfg(feature = "unity_engine-resource_management-async_operations-asyncoperationbase_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> AsyncOperationBase_1<T0> {
    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "get_Progress", args = 0)]
    pub fn get_progress(self) -> f32;

    #[method(name = "get_DebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetDependencies", args = 1)]
    pub fn get_dependencies(
        self,
        dependencies : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "get_Result", args = 0)]
    pub fn get_result(self) -> T0;

    #[method(name = "set_Result", args = 1)]
    pub fn set_result(self, value: T0) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "get_CompletedEventHasListeners", args = 0)]
    pub fn get_completed_event_has_listeners(self) -> bool;

    #[method(name = "get_DestroyedEventHasListeners", args = 0)]
    pub fn get_destroyed_event_has_listeners(self) -> bool;

    #[method(name = "set_OnDestroy", args = 1)]
    pub fn set_on_destroy(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: iasyncoperation :: IAsyncOperation >,
    ) -> ();

    #[method(name = "get_ReferenceCount", args = 0)]
    pub fn get_reference_count(self) -> i32;

    #[method(name = "get_IsRunning", args = 0)]
    pub fn get_is_running(self) -> bool;

    #[method(name = "set_IsRunning", args = 1)]
    pub fn set_is_running(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ShortenPath", args = 2)]
    pub fn shorten_path(p: ::unity2::Il2CppString, keep_extension: bool) -> ::unity2::Il2CppString;

    #[method(name = "IncrementReferenceCount", args = 0)]
    pub fn increment_reference_count(self) -> ();

    #[method(name = "WaitForCompletion", args = 0)]
    pub fn wait_for_completion(self) -> ();

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = "DecrementReferenceCount", args = 0)]
    pub fn decrement_reference_count(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "RegisterForDeferredCallbackEvent", args = 1)]
    pub fn register_for_deferred_callback_event(self, increment_reference_count: bool) -> ();

    #[method(name = "add_Completed", args = 1)]
    pub fn add_completed(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > >,
    ) -> ();

    #[method(name = "remove_Completed", args = 1)]
    pub fn remove_completed(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > >,
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

    #[method(name = "get_Status", args = 0)]
    pub fn get_status (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationstatus :: AsyncOperationStatus ;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "get_IsDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = "get_PercentComplete", args = 0)]
    pub fn get_percent_complete(self) -> f32;

    #[method(name = "InvokeCompletionEvent", args = 0)]
    pub fn invoke_completion_event(self) -> ();

    #[method(name = "get_Handle", args = 0)]
    pub fn get_handle (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > ;

    #[method(name = "UpdateCallback", args = 1)]
    pub fn update_callback(self, unscaled_delta_time: f32) -> ();

    #[method(name = "Complete", args = 3)]
    pub fn complete(self, result: T0, success: bool, error_msg: ::unity2::Il2CppString) -> ();

    #[method(name = "Complete", args = 4)]
    pub fn complete_2(
        self,
        result: T0,
        success: bool,
        error_msg: ::unity2::Il2CppString,
        release_dependencies_on_failure: bool,
    ) -> ();

    #[method(name = "InvokeExecute", args = 0)]
    pub fn invoke_execute(self) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.add_CompletedTypeless",
        args = 1
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_add_completed_typeless(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.remove_CompletedTypeless",
        args = 1
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_remove_completed_typeless(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.add_Destroyed",
        args = 1
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_add_destroyed(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.remove_Destroyed",
        args = 1
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_remove_destroyed(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Version",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_version(
        self,
    ) -> i32;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_ReferenceCount",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_reference_count(
        self,
    ) -> i32;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_PercentComplete",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_percent_complete(
        self,
    ) -> f32;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Status",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_status (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationstatus :: AsyncOperationStatus ;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_IsDone",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_is_done(
        self,
    ) -> bool;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_Handle",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_handle (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.set_OnDestroy",
        args = 1
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_set_on_destroy(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: iasyncoperation :: IAsyncOperation >,
    ) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_DebugName",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_debug_name(
        self,
    ) -> ::unity2::Il2CppString;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.GetResultAsObject",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_result_as_object(
        self,
    ) -> crate::system::object::Object;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.get_ResultType",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_result_type(
        self,
    ) -> ::unity2::SystemType;

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.GetDependencies",
        args = 1
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_dependencies(
        self,
        deps : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.DecrementReferenceCount",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_decrement_reference_count(
        self,
    ) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.IncrementReferenceCount",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_increment_reference_count(
        self,
    ) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.InvokeCompletionEvent",
        args = 0
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_invoke_completion_event(
        self,
    ) -> ();

    #[method(name = "ReleaseDependencies", args = 0)]
    pub fn release_dependencies(self) -> ();

    #[method(
        name = "UnityEngine.ResourceManagement.AsyncOperations.IAsyncOperation.GetDownloadStatus",
        args = 1
    )]
    pub fn unity_engine_resource_management_async_operations_i_async_operation_get_download_status(
        self,
        visited: crate::system::collections::generic::hashset_1::HashSet_1<
            crate::system::object::Object,
        >,
    ) -> crate::unity_engine::resource_management::async_operations::downloadstatus::DownloadStatus;

    #[method(name = "GetDownloadStatus", args = 1)]
    pub fn get_download_status(
        self,
        visited: crate::system::collections::generic::hashset_1::HashSet_1<
            crate::system::object::Object,
        >,
    ) -> crate::unity_engine::resource_management::async_operations::downloadstatus::DownloadStatus;
}

#[cfg(feature = "unity_engine-resource_management-async_operations-asyncoperationbase_1")]
impl<T0: ::unity2::ClassIdentity> AsyncOperationBase_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AsyncOperationBase_1),
                ::core::stringify!(new),
            )
        });
        <Self as IAsyncOperationBase_1Methods<T0>>::ctor(this);
        this
    }
}
