
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotcolordata/MascotColorData.md")))]
#[::unity2::class(namespace = "App", name = "MascotColorData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: mascotcolordata :: MascotColorData >)]
pub struct MascotColorData {
    #[rename(name = "MaskColor075")]
    pub mask_color075: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-mascotcolordata")]
#[::unity2::methods]
impl MascotColorData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_R", args = 0)]
    pub fn get_r(self) -> i32;

    #[method(name = "set_R", args = 1)]
    pub fn set_r(self, value: i32) -> ();

    #[method(name = "get_G", args = 0)]
    pub fn get_g(self) -> i32;

    #[method(name = "set_G", args = 1)]
    pub fn set_g(self, value: i32) -> ();

    #[method(name = "get_B", args = 0)]
    pub fn get_b(self) -> i32;

    #[method(name = "set_B", args = 1)]
    pub fn set_b(self, value: i32) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mascotcolordata")]
impl MascotColorData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotColorData),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotColorDataMethods>::ctor(this);
        this
    }
}
