
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshvariable/RefreshVariable.md")))]
#[::unity2::class(namespace = "App", name = "RefreshVariable")]
#[parent(crate::system::object::Object)]
pub struct RefreshVariable {
    #[static_field]
    #[rename(name = "m_FlagPrefix")]
    pub m_flag_prefix: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "m_FacilityAid")]
    pub m_facility_aid: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "m_UnitCount")]
    pub m_unit_count: i32,
}

#[cfg(feature = "app-refreshvariable")]
#[::unity2::methods]
impl RefreshVariable {
    #[method(name = "GetFlagName", args = 1)]
    pub fn get_flag_name(aid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetFlagName", args = 2)]
    pub fn get_flag_name_2(aid: ::unity2::Il2CppString, unit_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "RegistGlobalFlags", args = 0)]
    pub fn regist_global_flags() -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "OnCompletedChapter", args = 0)]
    pub fn on_completed_chapter() -> ();

    #[method(name = "HadVisited", args = 1)]
    pub fn had_visited(aid: ::unity2::Il2CppString) -> bool;

    #[method(name = "SetVisited", args = 2)]
    pub fn set_visited(aid: ::unity2::Il2CppString, visited: bool) -> ();

    #[method(name = "GetUnit", args = 2)]
    pub fn get_unit(aid: ::unity2::Il2CppString, index: i32) -> crate::app::unit::Unit;

    #[method(name = "SetUnit", args = 3)]
    pub fn set_unit(aid: ::unity2::Il2CppString, index: i32, unit: crate::app::unit::Unit) -> ();

    #[method(name = "CleanUpUnits", args = 0)]
    pub fn clean_up_units() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refreshvariable")]
impl RefreshVariable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshVariable),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshVariableMethods>::ctor(this);
        this
    }
}
