
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debuguserexceptionhandler/DebugUserExceptionHandler.md")))]
#[::unity2::class(namespace = "App", name = "DebugUserExceptionHandler")]
#[parent(crate::system::object::Object)]
pub struct DebugUserExceptionHandler {}

#[cfg(feature = "app-debuguserexceptionhandler")]
#[::unity2::methods]
impl DebugUserExceptionHandler {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "ExceptionHandler", args = 1)]
    pub fn exception_handler(exception_info: ::unity2::Il2CppString) -> ();
}
