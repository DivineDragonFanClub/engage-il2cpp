
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/async_operations/asyncoperationhandle/AsyncOperationHandle.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AsyncOperationHandle {
    pub m_internal_op: crate :: unity_engine :: resource_management :: async_operations :: iasyncoperation :: IAsyncOperation,
    pub m_version: i32,
    pub m_location_name: :: unity2 :: Il2CppString,
}

impl ::unity2::ClassIdentity for AsyncOperationHandle {
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.AsyncOperations";

    const NAME: &'static str = "AsyncOperationHandle";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AsyncOperationHandle {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-resource_management-async_operations-asyncoperationhandle")]
#[::unity2::methods(value)]
impl AsyncOperationHandle {
    #[method(name = "get_IsWaitingForCompletion", args = 0)]
    pub fn get_is_waiting_for_completion() -> bool;

    #[method(name = "set_IsWaitingForCompletion", args = 1)]
    pub fn set_is_waiting_for_completion(value: bool) -> ();

    #[method(name = "get_LocationName", args = 0)]
    pub fn get_location_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LocationName", args = 1)]
    pub fn set_location_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        op : crate :: unity_engine :: resource_management :: async_operations :: iasyncoperation :: IAsyncOperation,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        op : crate :: unity_engine :: resource_management :: async_operations :: iasyncoperation :: IAsyncOperation,
        version: i32,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(
        self,
        op : crate :: unity_engine :: resource_management :: async_operations :: iasyncoperation :: IAsyncOperation,
        location_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_4(
        self,
        op : crate :: unity_engine :: resource_management :: async_operations :: iasyncoperation :: IAsyncOperation,
        version: i32,
        location_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Acquire", args = 0)]
    pub fn acquire (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ;

    #[method(name = "add_Completed", args = 1)]
    pub fn add_completed(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "remove_Completed", args = 1)]
    pub fn remove_completed(
        self,
        value : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(
        self,
        other : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle,
    ) -> bool;

    #[method(name = "get_DebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

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

    #[method(name = "GetDependencies", args = 1)]
    pub fn get_dependencies(
        self,
        deps : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "get_InternalOp", args = 0)]
    pub fn get_internal_op(
        self,
    ) -> crate::unity_engine::resource_management::async_operations::iasyncoperation::IAsyncOperation;

    #[method(name = "get_IsDone", args = 0)]
    pub fn get_is_done(self) -> bool;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "get_PercentComplete", args = 0)]
    pub fn get_percent_complete(self) -> f32;

    #[method(name = "GetDownloadStatus", args = 0)]
    pub fn get_download_status(
        self,
    ) -> crate::unity_engine::resource_management::async_operations::downloadstatus::DownloadStatus;

    #[method(name = "InternalGetDownloadStatus", args = 1)]
    pub fn internal_get_download_status(
        self,
        visited: crate::system::collections::generic::hashset_1::HashSet_1<
            crate::system::object::Object,
        >,
    ) -> crate::unity_engine::resource_management::async_operations::downloadstatus::DownloadStatus;

    #[method(name = "get_ReferenceCount", args = 0)]
    pub fn get_reference_count(self) -> i32;

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "get_Result", args = 0)]
    pub fn get_result(self) -> crate::system::object::Object;

    #[method(name = "get_Status", args = 0)]
    pub fn get_status (self ,) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationstatus :: AsyncOperationStatus ;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.IEnumerator.MoveNext", args = 0)]
    pub fn system_collections_i_enumerator_move_next(self) -> bool;

    #[method(name = "System.Collections.IEnumerator.Reset", args = 0)]
    pub fn system_collections_i_enumerator_reset(self) -> ();

    #[method(name = "WaitForCompletion", args = 0)]
    pub fn wait_for_completion(self) -> crate::system::object::Object;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
