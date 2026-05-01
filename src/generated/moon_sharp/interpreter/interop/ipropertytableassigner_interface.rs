
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/ipropertytableassigner_interface/IPropertyTableAssigner_Interface.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "IPropertyTableAssigner"
)]
pub struct IPropertyTableAssigner_Interface {}

#[cfg(feature = "moon_sharp-interpreter-interop-ipropertytableassigner_interface")]
#[::unity2::methods]
impl IPropertyTableAssigner_Interface {
    #[method(name = "AssignObjectUnchecked", args = 2)]
    pub fn assign_object_unchecked(
        self,
        o: crate::system::object::Object,
        data: crate::moon_sharp::interpreter::table::Table,
    ) -> ();
}
