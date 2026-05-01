
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::AsyncOperationBase_1;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::IAsyncOperationBase_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/initialization/cacheinitialization/CacheInitialization_CacheInitOp.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets.Initialization",
    name = "CacheInitialization.CacheInitOp"
)]
# [parent (crate :: unity_engine :: resource_management :: async_operations :: asyncoperationbase_1 :: AsyncOperationBase_1 < bool >)]
pub struct CacheInitialization_CacheInitOp {
    #[rename(name = "m_Callback")]
    pub m_callback: crate::system::func_1::Func_1<bool>,
}

#[cfg(feature = "unity_engine-addressable_assets-initialization-cacheinitialization")]
#[::unity2::methods]
impl CacheInitialization_CacheInitOp {
    #[method(name = "Init", args = 1)]
    pub fn init(self, callback: crate::system::func_1::Func_1<bool>) -> ();

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = "Update", args = 1)]
    pub fn update(self, unscaled_delta_time: f32) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-initialization-cacheinitialization")]
impl CacheInitialization_CacheInitOp {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CacheInitialization_CacheInitOp),
                ::core::stringify!(new),
            )
        });
        <Self as ICacheInitialization_CacheInitOpMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/initialization/cacheinitialization/CacheInitialization.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets.Initialization",
    name = "CacheInitialization"
)]
#[parent(crate::system::object::Object)]
pub struct CacheInitialization {}

#[cfg(feature = "unity_engine-addressable_assets-initialization-cacheinitialization")]
#[::unity2::methods]
impl CacheInitialization {
    #[method(name = "Initialize", args = 2)]
    pub fn initialize(self, id: ::unity2::Il2CppString, data_str: ::unity2::Il2CppString) -> bool;

    #[method(name = "InitializeAsync", args = 3)]
    pub fn initialize_async (self , rm : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager , id : :: unity2 :: Il2CppString , data : :: unity2 :: Il2CppString) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < bool > ;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-initialization-cacheinitialization")]
impl CacheInitialization {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CacheInitialization),
                ::core::stringify!(new),
            )
        });
        <Self as ICacheInitializationMethods>::ctor(this);
        this
    }
}
