
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencehelper/MapSequenceHelper_ProcWaitCameraLoosely.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceHelper.ProcWaitCameraLoosely")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapSequenceHelper_ProcWaitCameraLoosely {}

#[cfg(feature = "app-mapsequencehelper")]
#[::unity2::methods]
impl MapSequenceHelper_ProcWaitCameraLoosely {
    #[method(name = "IsCameraScrolling", args = 0)]
    pub fn is_camera_scrolling() -> bool;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequencehelper")]
impl MapSequenceHelper_ProcWaitCameraLoosely {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceHelper_ProcWaitCameraLoosely),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceHelper_ProcWaitCameraLooselyMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencehelper/MapSequenceHelper.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceHelper")]
#[parent(crate::system::object::Object)]
pub struct MapSequenceHelper {
    #[static_field]
    #[rename(name = "AroundOffsetXs")]
    pub around_offset_xs: ::unity2::Array<i32>,
    #[static_field]
    #[rename(name = "AroundOffsetZs")]
    pub around_offset_zs: ::unity2::Array<i32>,
    #[static_field]
    #[rename(name = "InsideScreenMin")]
    pub inside_screen_min: f32,
    #[static_field]
    #[rename(name = "InsideScreenMax")]
    pub inside_screen_max: f32,
}

#[cfg(feature = "app-mapsequencehelper")]
#[::unity2::methods]
impl MapSequenceHelper {
    #[method(name = "GetCursorTurnFirst", args = 3)]
    pub fn get_cursor_turn_first(force: crate::app::force::Force_Type, x: i32, z: i32) -> bool;

    #[method(name = "GetCursorTurnHero", args = 3)]
    pub fn get_cursor_turn_hero(force: crate::app::force::Force_Type, x: i32, z: i32) -> bool;

    #[method(name = "WaitCamera", args = 1)]
    pub fn wait_camera(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "MoveCamera", args = 3)]
    pub fn move_camera(super_: crate::app::procinst::ProcInst, x: i32, z: i32) -> ();

    #[method(name = "MoveCamera", args = 3)]
    pub fn move_camera_2(super_: crate::app::procinst::ProcInst, x: f32, z: f32) -> ();

    #[method(name = "WaitCameraLoosely", args = 1)]
    pub fn wait_camera_loosely(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "MoveCameraLoosely", args = 3)]
    pub fn move_camera_loosely(super_: crate::app::procinst::ProcInst, x: i32, y: i32) -> ();

    #[method(name = "TryMoveCameraLoosely", args = 2)]
    pub fn try_move_camera_loosely(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "CheckPhaseEnd", args = 0)]
    pub fn check_phase_end() -> bool;

    #[method(name = "CheckRemovable", args = 1)]
    pub fn check_removable(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "CheckRemovableDetail", args = 1)]
    pub fn check_removable_detail(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "SetRemovable", args = 1)]
    pub fn set_removable(unit: crate::app::unit::Unit) -> ();

    #[method(name = "CheckRemagicable", args = 1)]
    pub fn check_remagicable(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "SetRemagicable", args = 1)]
    pub fn set_remagicable(unit: crate::app::unit::Unit) -> ();

    #[method(name = "CheckRerewarp", args = 1)]
    pub fn check_rerewarp(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "SetRerewarp", args = 1)]
    pub fn set_rerewarp(unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetRouteCost", args = 0)]
    pub fn get_route_cost() -> i32;

    #[method(name = "IsInsideScreen", args = 1)]
    pub fn is_inside_screen(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "EngageAttackTelop", args = 1)]
    pub fn engage_attack_telop(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "TryGetEngageAttackPair", args = 2)]
    pub fn try_get_engage_attack_pair(
        main: crate::app::goddata::GodData,
        link: crate::app::goddata::GodData,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapsequencehelper")]
impl MapSequenceHelper {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceHelper),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceHelperMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencehelper/MapSequenceHelper_ProcWaitCamera.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceHelper.ProcWaitCamera")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapSequenceHelper_ProcWaitCamera {}

#[cfg(feature = "app-mapsequencehelper")]
#[::unity2::methods]
impl MapSequenceHelper_ProcWaitCamera {
    #[method(name = "IsCameraScrolling", args = 0)]
    pub fn is_camera_scrolling() -> bool;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequencehelper")]
impl MapSequenceHelper_ProcWaitCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceHelper_ProcWaitCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceHelper_ProcWaitCameraMethods>::ctor(this);
        this
    }
}
