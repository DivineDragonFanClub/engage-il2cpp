
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/action_14/Action_14.md")))]
#[::unity2::class(namespace = "System", name = "Action`14")]
pub struct Action_14<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
    T2: ::unity2::ClassIdentity,
    T3: ::unity2::ClassIdentity,
    T4: ::unity2::ClassIdentity,
    T5: ::unity2::ClassIdentity,
    T6: ::unity2::ClassIdentity,
    T7: ::unity2::ClassIdentity,
    T8: ::unity2::ClassIdentity,
    T9: ::unity2::ClassIdentity,
    T10: ::unity2::ClassIdentity,
    T11: ::unity2::ClassIdentity,
    T12: ::unity2::ClassIdentity,
    T13: ::unity2::ClassIdentity,
> {}

#[cfg(feature = "system-action_14")]
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
        T8: ::unity2::ClassIdentity,
        T9: ::unity2::ClassIdentity,
        T10: ::unity2::ClassIdentity,
        T11: ::unity2::ClassIdentity,
        T12: ::unity2::ClassIdentity,
        T13: ::unity2::ClassIdentity,
    > Action_14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
{
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 14)]
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
        arg9: T8,
        arg10: T9,
        arg11: T10,
        arg12: T11,
        arg13: T12,
        arg14: T13,
    ) -> ();
}

#[cfg(feature = "system-action_14")]
impl<
        T0: ::unity2::ClassIdentity,
        T1: ::unity2::ClassIdentity,
        T2: ::unity2::ClassIdentity,
        T3: ::unity2::ClassIdentity,
        T4: ::unity2::ClassIdentity,
        T5: ::unity2::ClassIdentity,
        T6: ::unity2::ClassIdentity,
        T7: ::unity2::ClassIdentity,
        T8: ::unity2::ClassIdentity,
        T9: ::unity2::ClassIdentity,
        T10: ::unity2::ClassIdentity,
        T11: ::unity2::ClassIdentity,
        T12: ::unity2::ClassIdentity,
        T13: ::unity2::ClassIdentity,
    > Action_14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
{
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Action_14),
                ::core::stringify!(new),
            )
        });
        < Self as IAction_14Methods < T0 , T1 , T2 , T3 , T4 , T5 , T6 , T7 , T8 , T9 , T10 , T11 , T12 , T13 > > :: ctor (this , object , method) ;
        this
    }
}
