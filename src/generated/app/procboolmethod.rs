
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procboolmethod/ProcBoolMethod.md")))]
#[::unity2::class(namespace = "App", name = "ProcBoolMethod")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProcBoolMethod {}

#[cfg(feature = "app-procboolmethod")]
#[::unity2::methods]
impl ProcBoolMethod {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> bool;
}

#[cfg(feature = "app-procboolmethod")]
impl ProcBoolMethod {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcBoolMethod),
                ::core::stringify!(new),
            )
        });
        <Self as IProcBoolMethodMethods>::ctor(this, object, method);
        this
    }
}
