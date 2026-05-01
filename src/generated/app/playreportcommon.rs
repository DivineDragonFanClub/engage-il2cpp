
use crate::app::playreportsendbase::IPlayReportSendBase;
use crate::app::playreportsendbase::PlayReportSendBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/playreportcommon/PlayReportCommon.md")))]
#[::unity2::class(namespace = "App", name = "PlayReportCommon")]
#[parent(crate::app::playreportsendbase::PlayReportSendBase)]
pub struct PlayReportCommon {
    #[static_field]
    #[rename(name = "s_ReportVersion")]
    pub s_report_version: i32,
}

#[cfg(feature = "app-playreportcommon")]
#[::unity2::methods]
impl PlayReportCommon {
    #[method(name = "GetBufferSizeImpl", args = 0)]
    pub fn get_buffer_size_impl(self) -> i64;

    #[method(name = "ReportChapterClear", args = 0)]
    pub fn report_chapter_clear(self) -> ();

    #[method(name = "ReportSortie", args = 0)]
    pub fn report_sortie(self) -> ();

    #[method(name = "SendCommon", args = 2)]
    pub fn send_common(self, send_type: ::unity2::Il2CppString, is_send_hubs: bool) -> ();

    #[method(name = "ReportBasic", args = 0)]
    pub fn report_basic(self) -> ();

    #[method(name = "GetProgress", args = 0)]
    pub fn get_progress(self) -> i32;

    #[method(name = "IsGodCleared", args = 1)]
    pub fn is_god_cleared(self, idx: i32) -> bool;

    #[method(name = "GetEvilProgress", args = 0)]
    pub fn get_evil_progress(self) -> i32;

    #[method(name = "GetChapterType", args = 0)]
    pub fn get_chapter_type(self) -> ::unity2::Il2CppString;

    #[method(name = "ReportForce", args = 0)]
    pub fn report_force(self) -> ();

    #[method(name = "ReportConfig", args = 0)]
    pub fn report_config(self) -> ();

    #[method(name = "ReportUnit", args = 0)]
    pub fn report_unit(self) -> ();

    #[method(name = "GetForceMask", args = 0)]
    pub fn get_force_mask(self) -> u32;

    #[method(name = "ReportUnitImpl", args = 3)]
    pub fn report_unit_impl(
        self,
        unit: crate::app::unit::Unit,
        use_report_no: i32,
        in_report_count: i32,
    ) -> ();

    #[method(name = "IsPlayerUnit", args = 1)]
    pub fn is_player_unit(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsPlayerGodPool", args = 1)]
    pub fn is_player_god_pool(self, god_unit: crate::app::godunit::GodUnit) -> bool;

    #[method(name = "GodPoolLevel", args = 2)]
    pub fn god_pool_level(
        self,
        unit: crate::app::unit::Unit,
        god_name: ::unity2::Il2CppString,
    ) -> i32;

    #[method(name = "ReportReliance", args = 0)]
    pub fn report_reliance(self) -> ();

    #[method(name = "ReportHub", args = 0)]
    pub fn report_hub(self) -> ();

    #[method(name = "ReportOther", args = 0)]
    pub fn report_other(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-playreportcommon")]
impl PlayReportCommon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayReportCommon),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayReportCommonMethods>::ctor(this);
        this
    }
}
