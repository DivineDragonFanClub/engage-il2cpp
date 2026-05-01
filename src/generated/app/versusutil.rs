
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusutil/VersusUtil.md")))]
#[::unity2::class(namespace = "App", name = "VersusUtil")]
#[parent(crate::system::object::Object)]
pub struct VersusUtil {}

#[cfg(feature = "app-versusutil")]
#[::unity2::methods]
impl VersusUtil {
    #[method(name = "AddEncountCounter", args = 0)]
    pub fn add_encount_counter() -> ();

    #[method(name = "AddAchieveCount", args = 0)]
    pub fn add_achieve_count() -> ();

    #[method(name = "AddPlayReportStartCount", args = 0)]
    pub fn add_play_report_start_count() -> ();

    #[method(name = "AddPlayReportResultCount", args = 0)]
    pub fn add_play_report_result_count() -> ();

    #[method(name = "AddPlayReportResultCountDefense", args = 1)]
    pub fn add_play_report_result_count_defense(result: crate::app::versus::Versus_MapResult)
        -> ();

    #[method(name = "AddProfilePlayCount", args = 0)]
    pub fn add_profile_play_count() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-versusutil")]
impl VersusUtil {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusUtil),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusUtilMethods>::ctor(this);
        this
    }
}
