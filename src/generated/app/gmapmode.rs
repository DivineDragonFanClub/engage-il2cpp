
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmode/GmapMode_Mode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapMode_Mode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapMode_Mode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapMode.Mode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapMode_Mode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapMode_Mode {
    pub fn main() -> Self {
        Self { value: 0 }
    }

    pub fn dlc_god() -> Self {
        Self { value: 1 }
    }

    pub fn dlc_evil() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapmode/GmapMode.md")))]
#[::unity2::class(namespace = "App", name = "GmapMode")]
#[parent(crate::system::object::Object)]
pub struct GmapMode {}

#[cfg(feature = "app-gmapmode")]
#[::unity2::methods]
impl GmapMode {
    #[method(name = "get_IsAlongPath", args = 0)]
    pub fn get_is_along_path() -> bool;

    #[method(name = "set_IsAlongPath", args = 1)]
    pub fn set_is_along_path(value: bool) -> ();

    #[method(name = "GetMode", args = 0)]
    pub fn get_mode() -> crate::app::gmapmode::GmapMode_Mode;

    #[method(name = "SetMode", args = 1)]
    pub fn set_mode(mode: crate::app::gmapmode::GmapMode_Mode) -> ();

    #[method(name = "GoMode", args = 1)]
    pub fn go_mode(mode: crate::app::gmapmode::GmapMode_Mode) -> ();

    #[method(name = "GetSceneName", args = 1)]
    pub fn get_scene_name(mode: crate::app::gmapmode::GmapMode_Mode) -> ::unity2::Il2CppString;

    #[method(name = "SetNowSpotId", args = 1)]
    pub fn set_now_spot_id(spot_id: ::unity2::Il2CppString) -> ();

    #[method(name = "SetNowSpotId", args = 2)]
    pub fn set_now_spot_id_2(
        mode: crate::app::gmapmode::GmapMode_Mode,
        spot_id: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetNowSpotId", args = 0)]
    pub fn get_now_spot_id() -> ::unity2::Il2CppString;

    #[method(name = "CanMoveDlcMap", args = 0)]
    pub fn can_move_dlc_map() -> bool;

    #[method(name = "CanMoveGod", args = 0)]
    pub fn can_move_god() -> bool;

    #[method(name = "CanMoveEvil", args = 0)]
    pub fn can_move_evil() -> bool;

    #[method(name = "CheckSpotModeOnGmap", args = 2)]
    pub fn check_spot_mode_on_gmap(
        spot: crate::app::gmapspot::GmapSpot,
        mode: crate::app::gmapmode::GmapMode_Mode,
    ) -> bool;

    #[method(name = "CheckSpotModeAtChapter", args = 2)]
    pub fn check_spot_mode_at_chapter(
        spot: crate::app::gmapspot::GmapSpot,
        mode: crate::app::gmapmode::GmapMode_Mode,
    ) -> bool;

    #[method(name = "CheckSpotModeOnGmap", args = 2)]
    pub fn check_spot_mode_on_gmap_2(
        spot_id: ::unity2::Il2CppString,
        mode: crate::app::gmapmode::GmapMode_Mode,
    ) -> bool;

    #[method(name = "CheckSpotModeImpl", args = 3)]
    pub fn check_spot_mode_impl(
        spot_id: ::unity2::Il2CppString,
        mode: crate::app::gmapmode::GmapMode_Mode,
        on_gmap: bool,
    ) -> bool;

    #[method(name = "GetDlcGmapPathAssetPath", args = 0)]
    pub fn get_dlc_gmap_path_asset_path() -> ::unity2::Il2CppString;

    #[method(name = "GetDlcGmapSpotAssetPath", args = 0)]
    pub fn get_dlc_gmap_spot_asset_path() -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-gmapmode")]
impl GmapMode {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapMode),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapModeMethods>::ctor(this);
        this
    }
}
