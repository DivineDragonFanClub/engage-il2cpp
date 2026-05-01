
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/utility/diagnosticinfo/DiagnosticInfo.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets.Utility",
    name = "DiagnosticInfo"
)]
#[parent(crate::system::object::Object)]
pub struct DiagnosticInfo {
    #[rename(name = "DisplayName")]
    pub display_name: ::unity2::Il2CppString,
    #[rename(name = "ObjectId")]
    pub object_id: i32,
    #[rename(name = "Dependencies")]
    pub dependencies: ::unity2::Array<i32>,
}

#[cfg(feature = "unity_engine-addressable_assets-utility-diagnosticinfo")]
#[::unity2::methods]
impl DiagnosticInfo {
    #[method(name = "CreateEvent", args = 4)]
    pub fn create_event(
        self,
        category: ::unity2::Il2CppString,
        event_type : crate :: unity_engine :: resource_management :: resourcemanager :: ResourceManager_DiagnosticEventType,
        frame: i32,
        val: i32,
    ) -> crate::unity_engine::resource_management::diagnostics::diagnosticevent::DiagnosticEvent;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-addressable_assets-utility-diagnosticinfo")]
impl DiagnosticInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DiagnosticInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IDiagnosticInfoMethods>::ctor(this);
        this
    }
}
