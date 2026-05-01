
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::AsyncOperationBase_1;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::IAsyncOperationBase_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/chainoperationtypelessdepedency_1/ChainOperationTypelessDepedency_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement",
    name = "ChainOperationTypelessDepedency`1"
)]
pub struct ChainOperationTypelessDepedency_1 < T0 : :: unity2 :: ClassIdentity > {
# [rename (name = "m_DepOp")] pub m_dep_op : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle ,
# [rename (name = "m_WrappedOp")] pub m_wrapped_op : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > ,
# [rename (name = "m_depStatus")] pub m_dep_status : crate :: unity_engine :: resource_management :: async_operations :: downloadstatus :: DownloadStatus ,
# [rename (name = "m_wrapStatus")] pub m_wrap_status : crate :: unity_engine :: resource_management :: async_operations :: downloadstatus :: DownloadStatus ,
# [rename (name = "m_Callback")] pub m_callback : crate :: system :: func_2 :: Func_2 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > > ,
# [rename (name = "m_CachedOnWrappedCompleted")] pub m_cached_on_wrapped_completed : crate :: system :: action_1 :: Action_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > > ,
# [rename (name = "m_ReleaseDependenciesOnFailure")] pub m_release_dependencies_on_failure : bool ,
}

#[cfg(feature = "unity_engine-resource_management-chainoperationtypelessdepedency_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ChainOperationTypelessDepedency_1<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_DebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetDependencies", args = 1)]
    pub fn get_dependencies(
        self,
        deps : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "Init", args = 3)]
    pub fn init(
        self,
        dependent_op : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle,
        callback : crate :: system :: func_2 :: Func_2 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle , crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > >,
        release_dependencies_on_failure: bool,
    ) -> ();

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "OnWrappedCompleted", args = 1)]
    pub fn on_wrapped_completed(
        self,
        x : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 >,
    ) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "ReleaseDependencies", args = 0)]
    pub fn release_dependencies(self) -> ();

    #[method(name = "GetDownloadStatus", args = 1)]
    pub fn get_download_status(
        self,
        visited: crate::system::collections::generic::hashset_1::HashSet_1<
            crate::system::object::Object,
        >,
    ) -> crate::unity_engine::resource_management::async_operations::downloadstatus::DownloadStatus;

    #[method(name = "RefreshDownloadStatus", args = 1)]
    pub fn refresh_download_status(
        self,
        visited: crate::system::collections::generic::hashset_1::HashSet_1<
            crate::system::object::Object,
        >,
    ) -> ();

    #[method(name = "get_Progress", args = 0)]
    pub fn get_progress(self) -> f32;
}

#[cfg(feature = "unity_engine-resource_management-chainoperationtypelessdepedency_1")]
impl<T0: ::unity2::ClassIdentity> ChainOperationTypelessDepedency_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChainOperationTypelessDepedency_1),
                ::core::stringify!(new),
            )
        });
        <Self as IChainOperationTypelessDepedency_1Methods<T0>>::ctor(this);
        this
    }
}
