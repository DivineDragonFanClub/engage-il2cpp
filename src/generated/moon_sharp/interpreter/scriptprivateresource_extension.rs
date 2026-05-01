
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/scriptprivateresource_extension/ScriptPrivateResource_Extension.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter",
    name = "ScriptPrivateResource_Extension"
)]
#[parent(crate::system::object::Object)]
pub struct ScriptPrivateResource_Extension {}

#[cfg(feature = "moon_sharp-interpreter-scriptprivateresource_extension")]
#[::unity2::methods]
impl ScriptPrivateResource_Extension {
    #[method(name = "CheckScriptOwnership", args = 2)]
    pub fn check_script_ownership(
        containing_resource : crate :: moon_sharp :: interpreter :: iscriptprivateresource :: IScriptPrivateResource,
        values: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "CheckScriptOwnership", args = 2)]
    pub fn check_script_ownership_2(
        containing_resource : crate :: moon_sharp :: interpreter :: iscriptprivateresource :: IScriptPrivateResource,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "CheckScriptOwnership", args = 2)]
    pub fn check_script_ownership_3(
        resource: crate::moon_sharp::interpreter::iscriptprivateresource::IScriptPrivateResource,
        script: crate::moon_sharp::interpreter::script::Script,
    ) -> ();

    #[method(name = "CheckScriptOwnership", args = 2)]
    pub fn check_script_ownership_4(
        containing_resource : crate :: moon_sharp :: interpreter :: iscriptprivateresource :: IScriptPrivateResource,
        item_resource : crate :: moon_sharp :: interpreter :: iscriptprivateresource :: IScriptPrivateResource,
    ) -> ();
}
