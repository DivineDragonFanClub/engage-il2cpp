
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/componentsinchildrenenumerator_1/ComponentsInChildrenEnumerator_1_Func.md")))]
#[::unity2::class(namespace = "App", name = "ComponentsInChildrenEnumerator`1.Func")]
pub struct ComponentsInChildrenEnumerator_1_Func<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-componentsinchildrenenumerator_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ComponentsInChildrenEnumerator_1_Func<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, component: T0) -> ();
}

#[cfg(feature = "app-componentsinchildrenenumerator_1")]
impl<T0: ::unity2::ClassIdentity> ComponentsInChildrenEnumerator_1_Func<T0> {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ComponentsInChildrenEnumerator_1_Func),
                ::core::stringify!(new),
            )
        });
        <Self as IComponentsInChildrenEnumerator_1_FuncMethods<T0>>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/componentsinchildrenenumerator_1/ComponentsInChildrenEnumerator_1.md")))]
#[::unity2::class(namespace = "App", name = "ComponentsInChildrenEnumerator`1")]
pub struct ComponentsInChildrenEnumerator_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-componentsinchildrenenumerator_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ComponentsInChildrenEnumerator_1<T0> {
    #[method(name = "ForEach", args = 2)]
    pub fn for_each(
        go: crate::unity_engine::gameobject::GameObject,
        func: crate::app::componentsinchildrenenumerator_1::ComponentsInChildrenEnumerator_1_Func<
            T0,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-componentsinchildrenenumerator_1")]
impl<T0: ::unity2::ClassIdentity> ComponentsInChildrenEnumerator_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ComponentsInChildrenEnumerator_1),
                ::core::stringify!(new),
            )
        });
        <Self as IComponentsInChildrenEnumerator_1Methods<T0>>::ctor(this);
        this
    }
}
