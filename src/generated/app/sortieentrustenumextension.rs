
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustenumextension/SortieEntrustEnumExtension.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustEnumExtension")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustEnumExtension {}

#[cfg(feature = "app-sortieentrustenumextension")]
#[::unity2::methods]
impl SortieEntrustEnumExtension {
    #[method(name = "IsBasic", args = 1)]
    pub fn is_basic(progress: crate::app::sortieentrustprogress::SortieEntrustProgress) -> bool;

    #[method(name = "IsEnhancePerson", args = 1)]
    pub fn is_enhance_person(
        progress: crate::app::sortieentrustprogress::SortieEntrustProgress,
    ) -> bool;

    #[method(name = "IsRodLow", args = 1)]
    pub fn is_rod_low(progress: crate::app::sortieentrustprogress::SortieEntrustProgress) -> bool;

    #[method(name = "IsRodHigh", args = 1)]
    pub fn is_rod_high(progress: crate::app::sortieentrustprogress::SortieEntrustProgress) -> bool;

    #[method(name = "IsRange", args = 1)]
    pub fn is_range(progress: crate::app::sortieentrustprogress::SortieEntrustProgress) -> bool;

    #[method(name = "IsSpecial", args = 1)]
    pub fn is_special(progress: crate::app::sortieentrustprogress::SortieEntrustProgress) -> bool;

    #[method(name = "IsSub", args = 1)]
    pub fn is_sub(progress: crate::app::sortieentrustprogress::SortieEntrustProgress) -> bool;

    #[method(name = "IsVulnerary", args = 1)]
    pub fn is_vulnerary(progress: crate::app::sortieentrustprogress::SortieEntrustProgress)
        -> bool;

    #[method(name = "IsEnd", args = 1)]
    pub fn is_end(progress: crate::app::sortieentrustprogress::SortieEntrustProgress) -> bool;
}
