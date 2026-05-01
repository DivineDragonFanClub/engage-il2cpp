
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/iscriptprivateresource/IScriptPrivateResource.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "IScriptPrivateResource")]
pub struct IScriptPrivateResource {}

#[cfg(feature = "moon_sharp-interpreter-iscriptprivateresource")]
#[::unity2::methods]
impl IScriptPrivateResource {
    #[method(name = "get_OwnerScript", args = 0)]
    pub fn get_owner_script(self) -> crate::moon_sharp::interpreter::script::Script;
}
