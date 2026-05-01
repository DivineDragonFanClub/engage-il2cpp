
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/procvoidmethod/ProcVoidMethod.md")))]
#[::unity2::class(namespace = "App", name = "ProcVoidMethod")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProcVoidMethod {}

#[cfg(feature = "app-procvoidmethod")]
#[::unity2::methods]
impl ProcVoidMethod {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-procvoidmethod")]
impl ProcVoidMethod {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProcVoidMethod),
                ::core::stringify!(new),
            )
        });
        <Self as IProcVoidMethodMethods>::ctor(this, object, method);
        this
    }
}
