
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/action_8/Action_8.md")))]
#[::unity2::class(namespace = "System", name = "Action`8")]
pub struct Action_8<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
    T2: ::unity2::ClassIdentity,
    T3: ::unity2::ClassIdentity,
    T4: ::unity2::ClassIdentity,
    T5: ::unity2::ClassIdentity,
    T6: ::unity2::ClassIdentity,
    T7: ::unity2::ClassIdentity,
> {}

#[cfg(feature = "system-action_8")]
#[::unity2::methods]
impl<
        T0: ::unity2::ClassIdentity,
        T1: ::unity2::ClassIdentity,
        T2: ::unity2::ClassIdentity,
        T3: ::unity2::ClassIdentity,
        T4: ::unity2::ClassIdentity,
        T5: ::unity2::ClassIdentity,
        T6: ::unity2::ClassIdentity,
        T7: ::unity2::ClassIdentity,
    > Action_8<T0, T1, T2, T3, T4, T5, T6, T7>
{
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 8)]
    pub fn invoke(
        self,
        arg1: T0,
        arg2: T1,
        arg3: T2,
        arg4: T3,
        arg5: T4,
        arg6: T5,
        arg7: T6,
        arg8: T7,
    ) -> ();
}

#[cfg(feature = "system-action_8")]
impl<
        T0: ::unity2::ClassIdentity,
        T1: ::unity2::ClassIdentity,
        T2: ::unity2::ClassIdentity,
        T3: ::unity2::ClassIdentity,
        T4: ::unity2::ClassIdentity,
        T5: ::unity2::ClassIdentity,
        T6: ::unity2::ClassIdentity,
        T7: ::unity2::ClassIdentity,
    > Action_8<T0, T1, T2, T3, T4, T5, T6, T7>
{
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Action_8),
                ::core::stringify!(new),
            )
        });
        <Self as IAction_8Methods<T0, T1, T2, T3, T4, T5, T6, T7>>::ctor(this, object, method);
        this
    }
}
