
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/scriptfunctiondelegate/ScriptFunctionDelegate.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "ScriptFunctionDelegate")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ScriptFunctionDelegate {}

#[cfg(feature = "moon_sharp-interpreter-scriptfunctiondelegate")]
#[::unity2::methods]
impl ScriptFunctionDelegate {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> crate::system::object::Object;
}

#[cfg(feature = "moon_sharp-interpreter-scriptfunctiondelegate")]
impl ScriptFunctionDelegate {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptFunctionDelegate),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptFunctionDelegateMethods>::ctor(this, object, method);
        this
    }
}
