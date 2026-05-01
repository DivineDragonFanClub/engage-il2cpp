
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::AsyncOperationBase_1;
use crate::unity_engine::resource_management::async_operations::asyncoperationbase_1::IAsyncOperationBase_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/updatecatalogsoperation/UpdateCatalogsOperation.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets",
    name = "UpdateCatalogsOperation"
)]
# [parent (crate :: unity_engine :: resource_management :: async_operations :: asyncoperationbase_1 :: AsyncOperationBase_1 < crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator > >)]
pub struct UpdateCatalogsOperation {
# [rename (name = "m_Addressables")] pub m_addressables : crate :: unity_engine :: addressable_assets :: addressablesimpl :: AddressablesImpl ,
# [rename (name = "m_LocatorInfos")] pub m_locator_infos : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: addressable_assets :: addressablesimpl :: AddressablesImpl_ResourceLocatorInfo > ,
# [rename (name = "m_DepOp")] pub m_dep_op : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: ilist_1_interface :: IList_1_Interface < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle > > ,
}

#[cfg(feature = "unity_engine-addressable_assets-updatecatalogsoperation")]
#[::unity2::methods]
impl UpdateCatalogsOperation {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        aa: crate::unity_engine::addressable_assets::addressablesimpl::AddressablesImpl,
    ) -> ();

    #[method(name = "Start", args = 1)]
    pub fn start (self , catalog_ids : crate :: system :: collections :: generic :: ienumerable_1 :: IEnumerable_1 < :: unity2 :: Il2CppString >) -> crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: addressable_assets :: resource_locators :: iresourcelocator :: IResourceLocator > > ;

    #[method(name = "InvokeWaitForCompletion", args = 0)]
    pub fn invoke_wait_for_completion(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "GetDependencies", args = 1)]
    pub fn get_dependencies(
        self,
        dependencies : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle :: AsyncOperationHandle >,
    ) -> ();

    #[method(name = "Execute", args = 0)]
    pub fn execute(self) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-updatecatalogsoperation")]
impl UpdateCatalogsOperation {
    pub fn new(
        aa: crate::unity_engine::addressable_assets::addressablesimpl::AddressablesImpl,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UpdateCatalogsOperation),
                ::core::stringify!(new),
            )
        });
        <Self as IUpdateCatalogsOperationMethods>::ctor(this, aa);
        this
    }
}
