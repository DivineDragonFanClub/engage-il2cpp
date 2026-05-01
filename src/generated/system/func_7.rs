
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/func_7/Func_7.md")))]
#[::unity2::class(namespace = "System", name = "Func`7")]
pub struct Func_7<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
    T2: ::unity2::ClassIdentity,
    T3: ::unity2::ClassIdentity,
    T4: ::unity2::ClassIdentity,
    T5: ::unity2::ClassIdentity,
    T6: ::unity2::ClassIdentity,
> {}

#[cfg(feature = "system-func_7")]
#[::unity2::methods]
impl<
        T0: ::unity2::ClassIdentity,
        T1: ::unity2::ClassIdentity,
        T2: ::unity2::ClassIdentity,
        T3: ::unity2::ClassIdentity,
        T4: ::unity2::ClassIdentity,
        T5: ::unity2::ClassIdentity,
        T6: ::unity2::ClassIdentity,
    > Func_7<T0, T1, T2, T3, T4, T5, T6>
{
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 6)]
    pub fn invoke(self, arg1: T0, arg2: T1, arg3: T2, arg4: T3, arg5: T4, arg6: T5) -> T6;
}

#[cfg(feature = "system-func_7")]
impl<
        T0: ::unity2::ClassIdentity,
        T1: ::unity2::ClassIdentity,
        T2: ::unity2::ClassIdentity,
        T3: ::unity2::ClassIdentity,
        T4: ::unity2::ClassIdentity,
        T5: ::unity2::ClassIdentity,
        T6: ::unity2::ClassIdentity,
    > Func_7<T0, T1, T2, T3, T4, T5, T6>
{
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Func_7),
                ::core::stringify!(new),
            )
        });
        <Self as IFunc_7Methods<T0, T1, T2, T3, T4, T5, T6>>::ctor(this, object, method);
        this
    }
}
