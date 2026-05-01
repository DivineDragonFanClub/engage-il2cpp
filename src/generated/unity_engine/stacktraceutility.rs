
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/stacktraceutility/StackTraceUtility.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "StackTraceUtility")]
#[parent(crate::system::object::Object)]
pub struct StackTraceUtility {}

#[cfg(feature = "unity_engine-stacktraceutility")]
#[::unity2::methods]
impl StackTraceUtility {
    #[method(name = "SetProjectFolder", args = 1)]
    pub fn set_project_folder(folder: ::unity2::Il2CppString) -> ();

    #[method(name = "ExtractStackTrace", args = 0)]
    pub fn extract_stack_trace() -> ::unity2::Il2CppString;

    #[method(name = "ExtractStringFromExceptionInternal", args = 3)]
    pub fn extract_string_from_exception_internal(
        exceptiono: crate::system::object::Object,
        message: ::unity2::Il2CppString,
        stack_trace: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
