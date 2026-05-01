
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/data_structs/multidictionary_2/MultiDictionary_2.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.DataStructs",
    name = "MultiDictionary`2"
)]
pub struct MultiDictionary_2<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[rename(name = "m_Map")]
    pub m_map: crate::system::collections::generic::dictionary_2::Dictionary_2<
        T0,
        crate::system::collections::generic::list_1::List_1<T1>,
    >,
    #[rename(name = "m_DefaultRet")]
    pub m_default_ret: ::unity2::Array<T1>,
}

#[cfg(feature = "moon_sharp-interpreter-data_structs-multidictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> MultiDictionary_2<T0, T1> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        eq_comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(self, key: T0, value: T1) -> bool;

    #[method(name = "Find", args = 1)]
    pub fn find(
        self,
        key: T0,
    ) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<T1>;

    #[method(name = "ContainsKey", args = 1)]
    pub fn contains_key(self, key: T0) -> bool;

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: T0) -> ();

    #[method(name = "RemoveValue", args = 2)]
    pub fn remove_value(self, key: T0, value: T1) -> bool;
}

#[cfg(feature = "moon_sharp-interpreter-data_structs-multidictionary_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> MultiDictionary_2<T0, T1> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MultiDictionary_2),
                ::core::stringify!(new),
            )
        });
        <Self as IMultiDictionary_2Methods<T0, T1>>::ctor(this);
        this
    }

    pub fn new_2(
        eq_comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MultiDictionary_2),
                ::core::stringify!(new_2),
            )
        });
        <Self as IMultiDictionary_2Methods<T0, T1>>::ctor_2(this, eq_comparer);
        this
    }
}
