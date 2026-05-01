
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/util/unitywebrequestutilities/UnityWebRequestUtilities.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Util",
    name = "UnityWebRequestUtilities"
)]
#[parent(crate::system::object::Object)]
pub struct UnityWebRequestUtilities {}

#[cfg(feature = "unity_engine-resource_management-util-unitywebrequestutilities")]
#[::unity2::methods]
impl UnityWebRequestUtilities {
    #[method(name = "RequestHasErrors", args = 2)]
    pub fn request_has_errors(
        web_req: crate::unity_engine::networking::unitywebrequest::UnityWebRequest,
        result : crate :: unity_engine :: resource_management :: util :: unitywebrequestresult :: UnityWebRequestResult,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-util-unitywebrequestutilities")]
impl UnityWebRequestUtilities {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityWebRequestUtilities),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityWebRequestUtilitiesMethods>::ctor(this);
        this
    }
}
