
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/xrgraphicsautomatedtests/XRGraphicsAutomatedTests.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "XRGraphicsAutomatedTests")]
#[parent(crate::system::object::Object)]
pub struct XRGraphicsAutomatedTests {
    #[static_field]
    #[rename(name = "running")]
    pub running: bool,
}

#[cfg(feature = "unity_engine-rendering-xrgraphicsautomatedtests")]
#[::unity2::methods]
impl XRGraphicsAutomatedTests {
    #[method(name = "get_activatedFromCommandLine", args = 0)]
    pub fn get_activated_from_command_line() -> bool;

    #[method(name = "get_enabled", args = 0)]
    pub fn get_enabled() -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
