
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitfor/UnitFor.md")))]
#[::unity2::class(namespace = "App", name = "UnitFor")]
#[parent(crate::system::object::Object)]
pub struct UnitFor {}

#[cfg(feature = "app-unitfor")]
#[::unity2::methods]
impl UnitFor {
    #[method(name = "Each", args = 2)]
    pub fn each(
        r#type: crate::app::force::Force_Type,
        func: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    ) -> ();

    #[method(name = "Reverse", args = 2)]
    pub fn reverse(
        r#type: crate::app::force::Force_Type,
        func: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    ) -> ();

    #[method(name = "Each", args = 2)]
    pub fn each_2(
        force_mask: u32,
        func: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    ) -> ();

    #[method(name = "Reverse", args = 2)]
    pub fn reverse_2(
        force_mask: u32,
        func: crate::system::action_1::Action_1<crate::app::unit::Unit>,
    ) -> ();

    #[method(name = "Find", args = 2)]
    pub fn find(
        r#type: crate::app::force::Force_Type,
        pred: crate::system::predicate_1::Predicate_1<crate::app::unit::Unit>,
    ) -> crate::app::unit::Unit;

    #[method(name = "FindLast", args = 2)]
    pub fn find_last(
        r#type: crate::app::force::Force_Type,
        pred: crate::system::predicate_1::Predicate_1<crate::app::unit::Unit>,
    ) -> crate::app::unit::Unit;

    #[method(name = "Find", args = 2)]
    pub fn find_2(
        force_mask: u32,
        pred: crate::system::predicate_1::Predicate_1<crate::app::unit::Unit>,
    ) -> crate::app::unit::Unit;

    #[method(name = "FindLast", args = 2)]
    pub fn find_last_2(
        force_mask: u32,
        pred: crate::system::predicate_1::Predicate_1<crate::app::unit::Unit>,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetPrev", args = 1)]
    pub fn get_prev(unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "GetNext", args = 1)]
    pub fn get_next(unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "GetPrev", args = 2)]
    pub fn get_prev_2(unit: crate::app::unit::Unit, force_mask: u32) -> crate::app::unit::Unit;

    #[method(name = "GetNext", args = 2)]
    pub fn get_next_2(unit: crate::app::unit::Unit, force_mask: u32) -> crate::app::unit::Unit;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-unitfor")]
impl UnitFor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitFor),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitForMethods>::ctor(this);
        this
    }
}
