
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/basic_descriptors/iwireabledescriptor/IWireableDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.BasicDescriptors",
    name = "IWireableDescriptor"
)]
pub struct IWireableDescriptor {}

#[cfg(feature = "moon_sharp-interpreter-interop-basic_descriptors-iwireabledescriptor")]
#[::unity2::methods]
impl IWireableDescriptor {
    #[method(name = "PrepareForWiring", args = 1)]
    pub fn prepare_for_wiring(self, t: crate::moon_sharp::interpreter::table::Table) -> ();
}
