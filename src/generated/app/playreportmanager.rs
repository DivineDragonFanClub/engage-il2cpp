
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/playreportmanager/PlayReportManager.md")))]
#[::unity2::class(namespace = "App", name = "PlayReportManager")]
#[parent(crate::system::object::Object)]
pub struct PlayReportManager {
    #[static_field]
    #[rename(name = "s_ReportCommon")]
    pub s_report_common: crate::app::playreportcommon::PlayReportCommon,
    #[static_field]
    #[rename(name = "s_ReportID")]
    pub s_report_id: u64,
    #[static_field]
    #[rename(name = "s_ReportValue")]
    pub s_report_value: u64,
}

#[cfg(feature = "app-playreportmanager")]
#[::unity2::methods]
impl PlayReportManager {
    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "ReportChapterClear", args = 0)]
    pub fn report_chapter_clear() -> ();

    #[method(name = "ReportSortie", args = 0)]
    pub fn report_sortie() -> ();

    #[method(name = "RequestImmediateTransmission", args = 0)]
    pub fn request_immediate_transmission() -> ();

    #[method(name = "GetReportID", args = 0)]
    pub fn get_report_id() -> u64;

    #[method(name = "UpdateReportID", args = 0)]
    pub fn update_report_id() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-playreportmanager")]
impl PlayReportManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayReportManager),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayReportManagerMethods>::ctor(this);
        this
    }
}
