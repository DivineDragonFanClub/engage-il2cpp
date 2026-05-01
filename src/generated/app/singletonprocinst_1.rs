
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/singletonprocinst_1/SingletonProcInst_1.md")))]
#[::unity2::class(namespace = "App", name = "SingletonProcInst`1")]
pub struct SingletonProcInst_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "s_Instance")]
    pub s_instance: T0,
    #[rename(name = "m_IsLoaded")]
    pub m_is_loaded: bool,
}

#[cfg(feature = "app-singletonprocinst_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> SingletonProcInst_1<T0> {
    #[method(name = "get_Instance", args = 0)]
    pub fn get_instance() -> T0;

    #[method(name = "OnSingletonCreate", args = 0)]
    pub fn on_singleton_create(self) -> ();

    #[method(name = "OnSingletonDispose", args = 0)]
    pub fn on_singleton_dispose(self) -> ();

    #[method(name = "get_IsResume", args = 0)]
    pub fn get_is_resume(self) -> bool;

    #[method(name = "set_IsResume", args = 1)]
    pub fn set_is_resume(self, value: bool) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "ResumeStart", args = 0)]
    pub fn resume_start(self) -> ();

    #[method(name = "ResumeEnd", args = 0)]
    pub fn resume_end(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "get_GlobalAssetPath", args = 0)]
    pub fn get_global_asset_path(self) -> ::unity2::Il2CppString;

    #[method(name = "LoadGlobalAssetAsync", args = 0)]
    pub fn load_global_asset_async(self) -> ();

    #[method(name = "IsLoadingGlobalAsset", args = 0)]
    pub fn is_loading_global_asset(self) -> bool;

    #[method(name = "ReleaseGlobalAsset", args = 0)]
    pub fn release_global_asset(self) -> ();

    #[method(name = "InstantiateGlobalAsset", args = 1)]
    pub fn instantiate_global_asset(
        self,
        parent: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "InstantiateGlobalAsset", args = 1)]
    pub fn instantiate_global_asset_2(
        self,
        parent: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(exists: crate::app::singletonprocinst_1::SingletonProcInst_1<T0>) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-singletonprocinst_1")]
impl<T0: ::unity2::ClassIdentity> SingletonProcInst_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SingletonProcInst_1),
                ::core::stringify!(new),
            )
        });
        <Self as ISingletonProcInst_1Methods<T0>>::ctor(this);
        this
    }
}
