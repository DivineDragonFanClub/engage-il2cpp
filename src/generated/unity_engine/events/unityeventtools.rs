
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/unityeventtools/UnityEventTools.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "UnityEventTools")]
#[parent(crate::system::object::Object)]
pub struct UnityEventTools {}

#[cfg(feature = "unity_engine-events-unityeventtools")]
#[::unity2::methods]
impl UnityEventTools {
    #[method(name = "TidyAssemblyTypeName", args = 1)]
    pub fn tidy_assembly_type_name(
        assembly_type_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;
}
