
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scripthub/ScriptHub.md")))]
#[::unity2::class(namespace = "App", name = "ScriptHub")]
#[parent(crate::app::scriptutil::ScriptUtil)]
pub struct ScriptHub {}

#[cfg(feature = "app-scripthub")]
#[::unity2::methods]
impl ScriptHub {
    #[method(name = "HubNextChapter", args = 1)]
    pub fn hub_next_chapter(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "HubNextGmap", args = 1)]
    pub fn hub_next_gmap(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "HubNextMap", args = 1)]
    pub fn hub_next_map(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "HubDisableLocator", args = 1)]
    pub fn hub_disable_locator(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "HubIsCompleteChapter", args = 1)]
    pub fn hub_is_complete_chapter(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "HubIsCondition", args = 1)]
    pub fn hub_is_condition(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "HubIsFacilityComplete", args = 1)]
    pub fn hub_is_facility_complete(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "HubChangeTalk", args = 1)]
    pub fn hub_change_talk(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "HubIsEnabledLocator", args = 1)]
    pub fn hub_is_enabled_locator(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "CompleteEncountMap", args = 1)]
    pub fn complete_encount_map(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "HubIsEncountMap", args = 1)]
    pub fn hub_is_encount_map(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Regist", args = 1)]
    pub fn regist(script: crate::app::eventscript::EventScript) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-scripthub")]
impl ScriptHub {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptHub),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptHubMethods>::ctor(this);
        this
    }
}
