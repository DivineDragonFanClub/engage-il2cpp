
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::AsyncOperationBase_1;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::IAsyncOperationBase_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/resourceproviderbase/ResourceProviderBase.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "ResourceProviderBase"
)]
#[parent(crate::system::object::Object)]
pub struct ResourceProviderBase {
# [rename (name = "m_ProviderId")] pub m_provider_id : :: unity2 :: Il2CppString ,
# [rename (name = "m_BehaviourFlags")] pub m_behaviour_flags : crate :: unity_engine :: resource_management :: resource_providers :: providerbehaviourflags :: ProviderBehaviourFlags ,
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-resourceproviderbase")]
#[::unity2::methods]
impl ResourceProviderBase {
    #[method(name = "get_ProviderId", args = 0)]
    pub fn get_provider_id(self) -> ::unity2::Il2CppString;

    #[method(name = "Initialize", args = 2)]
    pub fn initialize(self, id: ::unity2::Il2CppString, data: ::unity2::Il2CppString) -> bool;

    #[method(name = "CanProvide", args = 2)]
    pub fn can_provide(
        self,
        t: ::unity2::SystemType,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> bool;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Release", args = 2)]
    pub fn release(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
        obj: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetDefaultType", args = 1)]
    pub fn get_default_type(
        self,
        location : crate :: unity_engine :: resource_management :: resource_locations :: iresourcelocation :: IResourceLocation,
    ) -> ::unity2::SystemType;

    #[method(name = "Provide", args = 1)]
    pub fn provide(
        self,
        provide_handle : crate :: unity_engine :: resource_management :: resource_providers :: providehandle :: ProvideHandle,
    ) -> ();

    #[method(name = "InitializeAsync", args = 3)]
    pub fn initialize_async (self , rm : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager , id : :: unity2 :: Il2CppString , data : :: unity2 :: Il2CppString) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < bool > ;

    #[method(
        name = "UnityEngine.ResourceManagement.ResourceProviders.IResourceProvider.get_BehaviourFlags",
        args = 0
    )]
    pub fn unity_engine_resource_management_resource_providers_i_resource_provider_get_behaviour_flags (self ,) -> crate :: unity_engine :: resource_management :: resource_providers :: providerbehaviourflags :: ProviderBehaviourFlags ;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-resourceproviderbase")]
impl ResourceProviderBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourceProviderBase),
                ::core::stringify!(new),
            )
        });
        <Self as IResourceProviderBaseMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/resource_providers/resourceproviderbase/ResourceProviderBase_BaseInitAsyncOp.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.ResourceProviders",
    name = "ResourceProviderBase.BaseInitAsyncOp"
)]
# [parent (crate :: unity_engine :: resource_management :: async_operations :: asyncoperationbase_1 :: AsyncOperationBase_1 < bool >)]
pub struct ResourceProviderBase_BaseInitAsyncOp {
    #[rename(name = "m_CallBack")]
    pub m_call_back: crate::system::func_1::Func_1<bool>,
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-resourceproviderbase")]
#[::unity2::methods]
impl ResourceProviderBase_BaseInitAsyncOp {
    #[method(name = "Init", args = 1)]
    pub fn init(self, callback: crate::system::func_1::Func_1<bool>) -> ();

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-resource_providers-resourceproviderbase")]
impl ResourceProviderBase_BaseInitAsyncOp {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ResourceProviderBase_BaseInitAsyncOp),
                ::core::stringify!(new),
            )
        });
        <Self as IResourceProviderBase_BaseInitAsyncOpMethods>::ctor(this);
        this
    }
}
