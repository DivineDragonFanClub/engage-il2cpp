
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::AsyncOperationBase_1;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::IAsyncOperationBase_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/sceneprovider/SceneProvider.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "SceneProvider"
)]
#[parent(crate::system::object::Object)]
pub struct SceneProvider {}

#[cfg(feature = "unity_engine-resource_management-resource_providers-sceneprovider")]
#[::unity2::methods]
impl SceneProvider {
    #[method(name = "ProvideScene", args = 5)]
    pub fn provide_scene (self , resource_manager : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager , location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation , load_mode : crate :: unity_engine :: scene_management :: loadscenemode :: LoadSceneMode , activate_on_load : bool , priority : i32) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = "ReleaseScene", args = 2)]
    pub fn release_scene (self , resource_manager : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager , scene_load_handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance >) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-sceneprovider")]
impl SceneProvider {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SceneProvider),
                ::core::stringify!(new),
            )
        });
        <Self as ISceneProviderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/sceneprovider/SceneProvider_SceneOp.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "SceneProvider.SceneOp"
)]
# [parent (crate :: unity_engine :: resource_management :: async_operations :: asyncoperationbase_1 :: AsyncOperationBase_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance >)]
pub struct SceneProvider_SceneOp {
# [rename (name = "m_ActivateOnLoad")] pub m_activate_on_load : bool ,
# [rename (name = "m_Inst")] pub m_inst : crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance ,
# [rename (name = "m_Location")] pub m_location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation ,
# [rename (name = "m_LoadMode")] pub m_load_mode : crate :: unity_engine :: scene_management :: loadscenemode :: LoadSceneMode ,
# [rename (name = "m_Priority")] pub m_priority : i32 ,
# [rename (name = "m_DepOp")] pub m_dep_op : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > > ,
# [rename (name = "m_ResourceManager")] pub m_resource_manager : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager ,
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-sceneprovider")]
#[::unity2::methods]
impl SceneProvider_SceneOp {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        rm: crate::unity_engine::resource_management::resourcemanager::ResourceManager,
    ) -> ();

    #[method(name = "GetDownloadStatus", args = 1)]
    pub fn get_download_status(
        self,
        visited: crate::system::collections::generic::hashset_1::HashSet_1<
            crate::system::object::Object,
        >,
    ) -> crate::unity_engine::resource_management::async_operations::downloadstatus::DownloadStatus;

    #[method(name = "Init", args = 5)]
    pub fn init(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        load_mode: crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
        activate_on_load: bool,
        priority: i32,
        dep_op : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > >,
    ) -> ();

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = "GetDependencies", args = 1)]
    pub fn get_dependencies(
        self,
        deps : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "get_DebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "InternalLoadScene", args = 5)]
    pub fn internal_load_scene(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        loading_from_bundle: bool,
        load_mode: crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
        activate_on_load: bool,
        priority: i32,
    ) -> crate::unity_engine::resource_management::resource_providers::sceneinstance::SceneInstance;

    #[method(name = "InternalLoad", args = 3)]
    pub fn internal_load(
        self,
        path: ::unity2::Il2CppString,
        loading_from_bundle: bool,
        mode: crate::unity_engine::scene_management::loadscenemode::LoadSceneMode,
    ) -> crate::unity_engine::asyncoperation::AsyncOperation;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "get_Progress", args = 0)]
    pub fn get_progress(self) -> f32;

    #[method(
        name = "UnityEngine.ResourceManagement.IUpdateReceiver.Update",
        args = 1
    )]
    pub fn unity_engine_resource_management_i_update_receiver_update(
        self,
        unscaled_delta_time: f32,
    ) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-sceneprovider")]
impl SceneProvider_SceneOp {
    pub fn new(
        rm: crate::unity_engine::resource_management::resourcemanager::ResourceManager,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SceneProvider_SceneOp),
                ::core::stringify!(new),
            )
        });
        <Self as ISceneProvider_SceneOpMethods>::ctor(this, rm);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/sceneprovider/SceneProvider_UnloadSceneOp.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "SceneProvider.UnloadSceneOp"
)]
# [parent (crate :: unity_engine :: resource_management :: async_operations :: asyncoperationbase_1 :: AsyncOperationBase_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance >)]
pub struct SceneProvider_UnloadSceneOp {
# [rename (name = "m_Instance")] pub m_instance : crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance ,
# [rename (name = "m_sceneLoadHandle")] pub m_scene_load_handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance > ,
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-sceneprovider")]
#[::unity2::methods]
impl SceneProvider_UnloadSceneOp {
    #[method(name = "Init", args = 1)]
    pub fn init(
        self,
        scene_load_handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: unity_engine :: resource_management :: resource_providers :: sceneinstance :: SceneInstance >,
    ) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = "UnloadSceneCompleted", args = 1)]
    pub fn unload_scene_completed(
        self,
        obj: crate::unity_engine::asyncoperation::AsyncOperation,
    ) -> ();

    #[method(name = "UnloadSceneCompletedNoRelease", args = 1)]
    pub fn unload_scene_completed_no_release(
        self,
        obj: crate::unity_engine::asyncoperation::AsyncOperation,
    ) -> ();

    #[method(name = "get_Progress", args = 0)]
    pub fn get_progress(self) -> f32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-sceneprovider")]
impl SceneProvider_UnloadSceneOp {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SceneProvider_UnloadSceneOp),
                ::core::stringify!(new),
            )
        });
        <Self as ISceneProvider_UnloadSceneOpMethods>::ctor(this);
        this
    }
}
