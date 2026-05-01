
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/irawassetbundle_1/IRawAssetBundle_1.md")))]
#[::unity2::class(namespace = "App", name = "IRawAssetBundle`1")]
pub struct IRawAssetBundle_1 < T0 : :: unity2 :: ClassIdentity > {
# [rename (name = "m_Handle")] pub m_handle : crate :: unity_engine :: resource_management :: async_operations :: asyncoperationhandle_1 :: AsyncOperationHandle_1 < T0 > ,
# [rename (name = "m_Asset")] pub m_asset : T0 ,
}

#[cfg(feature = "app-irawassetbundle_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> IRawAssetBundle_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, path: ::unity2::Il2CppString) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "OnLoad", args = 0)]
    pub fn on_load(self) -> ();

    #[method(name = "OnUnload", args = 0)]
    pub fn on_unload(self) -> ();

    #[method(name = "Load", args = 1)]
    pub fn load(self, path: ::unity2::Il2CppString) -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "get_Asset", args = 0)]
    pub fn get_asset(self) -> T0;
}

#[cfg(feature = "app-irawassetbundle_1")]
impl<T0: ::unity2::ClassIdentity> IRawAssetBundle_1<T0> {
    pub fn new(path: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(IRawAssetBundle_1),
                ::core::stringify!(new),
            )
        });
        <Self as IIRawAssetBundle_1Methods<T0>>::ctor(this, path);
        this
    }
}
