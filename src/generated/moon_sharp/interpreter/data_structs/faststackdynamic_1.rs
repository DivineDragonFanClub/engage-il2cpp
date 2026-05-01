
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/data_structs/faststackdynamic_1/FastStackDynamic_1.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.DataStructs",
    name = "FastStackDynamic`1"
)]
pub struct FastStackDynamic_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "moon_sharp-interpreter-data_structs-faststackdynamic_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> FastStackDynamic_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, starting_capacity: i32) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set(self, idxofs: i32, item: T0) -> ();

    #[method(name = "Push", args = 1)]
    pub fn push(self, item: T0) -> T0;

    #[method(name = "Expand", args = 1)]
    pub fn expand(self, size: i32) -> ();

    #[method(name = "Zero", args = 1)]
    pub fn zero(self, index: i32) -> ();

    #[method(name = "Peek", args = 1)]
    pub fn peek(self, idxofs: i32) -> T0;

    #[method(name = "CropAtCount", args = 1)]
    pub fn crop_at_count(self, p: i32) -> ();

    #[method(name = "RemoveLast", args = 1)]
    pub fn remove_last(self, cnt: i32) -> ();

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> T0;
}

#[cfg(feature = "moon_sharp-interpreter-data_structs-faststackdynamic_1")]
impl<T0: ::unity2::ClassIdentity> FastStackDynamic_1<T0> {
    pub fn new(starting_capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FastStackDynamic_1),
                ::core::stringify!(new),
            )
        });
        <Self as IFastStackDynamic_1Methods<T0>>::ctor(this, starting_capacity);
        this
    }
}
