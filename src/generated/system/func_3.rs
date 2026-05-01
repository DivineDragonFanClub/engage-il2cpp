
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/func_3/Func_3.md")))]
#[::unity2::class(namespace = "System", name = "Func`3")]
pub struct Func_3<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
    T2: ::unity2::ClassIdentity,
> {}

#[cfg(feature = "system-func_3")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity, T2: ::unity2::ClassIdentity>
    Func_3<T0, T1, T2>
{
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, arg1: T0, arg2: T1) -> T2;
}

#[cfg(feature = "system-func_3")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity, T2: ::unity2::ClassIdentity>
    Func_3<T0, T1, T2>
{
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Func_3),
                ::core::stringify!(new),
            )
        });
        <Self as IFunc_3Methods<T0, T1, T2>>::ctor(this, object, method);
        this
    }
}
