
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/key3dictionary_2/Key3Dictionary_2.md")))]
#[::unity2::class(namespace = "App", name = "Key3Dictionary`2")]
pub struct Key3Dictionary_2<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[rename(name = "m_Dictionary")]
    pub m_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        T0,
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            T0,
            crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
        >,
    >,
    #[rename(name = "m_Capacity")]
    pub m_capacity: i32,
}

#[cfg(feature = "app-key3dictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> Key3Dictionary_2<T0, T1> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, capacity: i32) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Add", args = 4)]
    pub fn add(self, key_a: T0, key_b: T0, key_c: T0, data: T1) -> ();

    #[method(name = "ContainsKey", args = 3)]
    pub fn contains_key(self, key_a: T0, key_b: T0, key_c: T0) -> bool;

    #[method(name = "TryGetValue", args = 4)]
    pub fn try_get_value(self, key_a: T0, key_b: T0, key_c: T0, data: T1) -> bool;
}

#[cfg(feature = "app-key3dictionary_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> Key3Dictionary_2<T0, T1> {
    pub fn new(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Key3Dictionary_2),
                ::core::stringify!(new),
            )
        });
        <Self as IKey3Dictionary_2Methods<T0, T1>>::ctor(this, capacity);
        this
    }
}
