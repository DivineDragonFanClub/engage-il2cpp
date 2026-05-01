
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshreliancesetter/RefreshRelianceSetter.md")))]
#[::unity2::class(namespace = "App", name = "RefreshRelianceSetter")]
#[parent(crate::system::object::Object)]
pub struct RefreshRelianceSetter {}

#[cfg(feature = "app-refreshreliancesetter")]
#[::unity2::methods]
impl RefreshRelianceSetter {
    #[method(name = "SetData", args = 7)]
    pub fn set_data(
        unit0: crate::app::unit::Unit,
        unit1: crate::app::unit::Unit,
        rank_c_image: crate::unity_engine::ui::image::Image,
        rank_b_image: crate::unity_engine::ui::image::Image,
        rank_a_image: crate::unity_engine::ui::image::Image,
        rank_s_image: crate::unity_engine::ui::image::Image,
        rank_none_image: crate::unity_engine::ui::image::Image,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refreshreliancesetter")]
impl RefreshRelianceSetter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshRelianceSetter),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshRelianceSetterMethods>::ctor(this);
        this
    }
}
