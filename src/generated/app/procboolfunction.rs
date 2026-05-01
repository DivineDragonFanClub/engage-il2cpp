
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procboolfunction/ProcBoolFunction.md")))]
#[::unity2::class(namespace = "App", name = "ProcBoolFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProcBoolFunction {}

#[cfg(feature = "app-procboolfunction")]
#[::unity2::methods]
impl ProcBoolFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, inst: crate::app::procinst::ProcInst) -> bool;
}

#[cfg(feature = "app-procboolfunction")]
impl ProcBoolFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcBoolFunction),
                ::core::stringify!(new),
            )
        });
        <Self as IProcBoolFunctionMethods>::ctor(this, object, method);
        this
    }
}
