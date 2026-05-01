
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/standard_descriptors/hardwired_descriptors/defaultvalue/DefaultValue.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop.StandardDescriptors.HardwiredDescriptors",
    name = "DefaultValue"
)]
#[parent(crate::system::object::Object)]
pub struct DefaultValue {
# [static_field] # [rename (name = "Instance")] pub instance : crate :: moon_sharp :: interpreter :: interop :: standard_descriptors :: hardwired_descriptors :: defaultvalue :: DefaultValue ,
}

#[cfg(feature = "moon_sharp-interpreter-interop-standard_descriptors-hardwired_descriptors-defaultvalue")]
#[::unity2::methods]
impl DefaultValue {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-standard_descriptors-hardwired_descriptors-defaultvalue")]
impl DefaultValue {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DefaultValue),
                ::core::stringify!(new),
            )
        });
        <Self as IDefaultValueMethods>::ctor(this);
        this
    }
}
