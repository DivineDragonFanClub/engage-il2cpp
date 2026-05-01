
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcDead.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcDead")]
#[parent(crate::app::mapaction::MapAction_ProcUnitAction)]
pub struct MapAction_ProcDead {
    #[static_field]
    #[rename(name = "FadeTime")]
    pub fade_time: f32,
}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcDead {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, actor: crate::app::unitactor::UnitActor) -> ();

    #[method(name = "Executed", args = 0)]
    pub fn executed(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
    ) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcDead {
    pub fn new(actor: crate::app::unitactor::UnitActor) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcDead),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcDeadMethods>::ctor(this, actor);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcBlow.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcBlow")]
#[parent(crate::app::mapaction::MapAction_ProcUnitAction)]
pub struct MapAction_ProcBlow {}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcBlow {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> ();

    #[method(name = "get_Speed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
        to_x: i32,
        to_z: i32,
    ) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcBlow {
    pub fn new(actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcBlow),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcBlowMethods>::ctor(this, actor, to_x, to_z);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcChangePos.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcChangePos")]
#[parent(crate::app::mapaction::MapAction_ProcUnitAction)]
pub struct MapAction_ProcChangePos {}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcChangePos {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
        to_x: i32,
        to_z: i32,
    ) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcChangePos {
    pub fn new(actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcChangePos),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcChangePosMethods>::ctor(this, actor, to_x, to_z);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcRouteMove_Spline.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcRouteMove.Spline")]
#[parent(crate::system::object::Object)]
pub struct MapAction_ProcRouteMove_Spline {
    #[static_field]
    #[rename(name = "CurveMax")]
    pub curve_max: i32,
    #[rename(name = "From")]
    pub from: crate::unity_engine::vector3::Vector3,
    #[rename(name = "To")]
    pub to: crate::unity_engine::vector3::Vector3,
    #[rename(name = "Curves")]
    pub curves: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcRouteMove_Spline {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcRouteMove_Spline {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcRouteMove_Spline),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcRouteMove_SplineMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcUnitAction.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcUnitAction")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapAction_ProcUnitAction {
    #[rename(name = "m_Actor")]
    pub m_actor: crate::app::unitactor::UnitActor,
    #[rename(name = "m_From")]
    pub m_from: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_To")]
    pub m_to: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Dist")]
    pub m_dist: f32,
    #[rename(name = "m_Range")]
    pub m_range: f32,
    #[rename(name = "m_Time")]
    pub m_time: f32,
}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcUnitAction {
    #[method(name = "get_CanWaitSkip", args = 0)]
    pub fn get_can_wait_skip(self) -> bool;

    #[method(name = "get_Speed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, actor: crate::app::unitactor::UnitActor) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(self, actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> ();

    #[method(name = "CalcRange", args = 2)]
    pub fn calc_range(self, dx: f32, dz: f32) -> ();

    #[method(name = "AddTime", args = 0)]
    pub fn add_time(self) -> bool;

    #[method(name = "GetRatio", args = 0)]
    pub fn get_ratio(self) -> f32;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcUnitAction {
    pub fn new(actor: crate::app::unitactor::UnitActor) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcUnitAction),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcUnitActionMethods>::ctor(this, actor);
        this
    }

    pub fn new_2(actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcUnitAction),
                ::core::stringify!(new_2),
            )
        });
        <Self as IMapAction_ProcUnitActionMethods>::ctor_2(this, actor, to_x, to_z);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcRevive.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcRevive")]
#[parent(crate::app::mapaction::MapAction_ProcUnitAction)]
pub struct MapAction_ProcRevive {
    #[static_field]
    #[rename(name = "FadeTime")]
    pub fade_time: f32,
}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcRevive {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, actor: crate::app::unitactor::UnitActor) -> ();

    #[method(name = "Executed", args = 0)]
    pub fn executed(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
    ) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcRevive {
    pub fn new(actor: crate::app::unitactor::UnitActor) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcRevive),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcReviveMethods>::ctor(this, actor);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction.md")))]
#[::unity2::class(namespace = "App", name = "MapAction")]
#[parent(crate::system::object::Object)]
pub struct MapAction {}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction {
    #[method(name = "ChangePosBind", args = 4)]
    pub fn change_pos_bind(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
        to_x: i32,
        to_z: i32,
    ) -> ();

    #[method(name = "WarpBind", args = 4)]
    pub fn warp_bind(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
        to_x: i32,
        to_z: i32,
    ) -> ();

    #[method(name = "WarpInBind", args = 4)]
    pub fn warp_in_bind(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
        to_x: i32,
        to_z: i32,
    ) -> ();

    #[method(name = "WarpOutBind", args = 4)]
    pub fn warp_out_bind(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
        to_x: i32,
        to_z: i32,
    ) -> ();

    #[method(name = "DeadBind", args = 2)]
    pub fn dead_bind(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
    ) -> ();

    #[method(name = "ReviveBind", args = 2)]
    pub fn revive_bind(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
    ) -> ();

    #[method(name = "BlowBind", args = 4)]
    pub fn blow_bind(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
        to_x: i32,
        to_z: i32,
    ) -> ();

    #[method(name = "BounceBind", args = 5)]
    pub fn bounce_bind(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
        dx: i32,
        dz: i32,
        distance: i32,
    ) -> ();

    #[method(name = "JumpCreate", args = 5)]
    pub fn jump_create(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
        to_x: i32,
        to_z: i32,
        range: f32,
    ) -> ();

    #[method(name = "TranslationBind", args = 4)]
    pub fn translation_bind(
        actor: crate::app::unitactor::UnitActor,
        super_: crate::app::procinst::ProcInst,
        to_x: i32,
        to_z: i32,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcRouteMove_Result.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapAction_ProcRouteMove_Result {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapAction_ProcRouteMove_Result {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapAction.ProcRouteMove.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapAction_ProcRouteMove_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapAction_ProcRouteMove_Result {
    pub fn r#move() -> Self {
        Self { value: 0 }
    }

    pub fn next() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcTranslation.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcTranslation")]
#[parent(crate::app::mapaction::MapAction_ProcUnitAction)]
pub struct MapAction_ProcTranslation {}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcTranslation {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> ();

    #[method(name = "get_Speed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
        to_x: i32,
        to_z: i32,
    ) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcTranslation {
    pub fn new(actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcTranslation),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcTranslationMethods>::ctor(this, actor, to_x, to_z);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcSyncSkyCastle.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcSyncSkyCastle")]
#[parent(crate::app::mapaction::MapAction_ProcUnitAction)]
pub struct MapAction_ProcSyncSkyCastle {
    #[rename(name = "m_IsUpdate")]
    pub m_is_update: bool,
}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcSyncSkyCastle {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcBounce.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcBounce")]
#[parent(crate::app::mapaction::MapAction_ProcUnitAction)]
pub struct MapAction_ProcBounce {
    #[rename(name = "m_impacted")]
    pub m_impacted: bool,
    #[rename(name = "m_Hit")]
    pub m_hit: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcBounce {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        actor: crate::app::unitactor::UnitActor,
        dx: i32,
        dz: i32,
        distance: i32,
    ) -> ();

    #[method(name = "get_Speed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
        dx: i32,
        dz: i32,
        distance: i32,
    ) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcBounce {
    pub fn new(actor: crate::app::unitactor::UnitActor, dx: i32, dz: i32, distance: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcBounce),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcBounceMethods>::ctor(this, actor, dx, dz, distance);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcWarp.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcWarp")]
#[parent(crate::app::mapaction::MapAction_ProcUnitAction)]
pub struct MapAction_ProcWarp {
    #[static_field]
    #[rename(name = "FadeTime")]
    pub fade_time: f32,
}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcWarp {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> ();

    #[method(name = "GetWarpOutEffect", args = 1)]
    pub fn get_warp_out_effect(unit: crate::app::unit::Unit) -> ::unity2::Il2CppString;

    #[method(name = "GetWarpInEffect", args = 1)]
    pub fn get_warp_in_effect(unit: crate::app::unit::Unit) -> ::unity2::Il2CppString;

    #[method(name = "WarpOut", args = 0)]
    pub fn warp_out(self) -> ();

    #[method(name = "WarpIn", args = 0)]
    pub fn warp_in(self) -> ();

    #[method(name = "FadeOut", args = 0)]
    pub fn fade_out(self) -> ();

    #[method(name = "FadeIn", args = 0)]
    pub fn fade_in(self) -> ();

    #[method(name = "WaitTick", args = 0)]
    pub fn wait_tick(self) -> ();

    #[method(name = "WarpBind", args = 4)]
    pub fn warp_bind(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
        to_x: i32,
        to_z: i32,
    ) -> ();

    #[method(name = "WarpInBind", args = 4)]
    pub fn warp_in_bind(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
        to_x: i32,
        to_z: i32,
    ) -> ();

    #[method(name = "WarpOutBind", args = 4)]
    pub fn warp_out_bind(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
        to_x: i32,
        to_z: i32,
    ) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcWarp {
    pub fn new(actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcWarp),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcWarpMethods>::ctor(this, actor, to_x, to_z);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcRouteMove.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcRouteMove")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapAction_ProcRouteMove {
    #[static_field]
    #[rename(name = "SmokeCount")]
    pub smoke_count: i32,
    #[static_field]
    #[rename(name = "MoveSpeed")]
    pub move_speed: f32,
    #[rename(name = "m_Actor")]
    pub m_actor: crate::app::unitactor::UnitActor,
    #[rename(name = "m_MoveFlag")]
    pub m_move_flag: crate::app::mapmoveflag::MapMoveFlag,
    #[rename(name = "m_Routes")]
    pub m_routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
    #[rename(name = "m_RouteCount")]
    pub m_route_count: i32,
    #[rename(name = "m_Position")]
    pub m_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_FromX")]
    pub m_from_x: i32,
    #[rename(name = "m_FromZ")]
    pub m_from_z: i32,
    #[rename(name = "m_ToX")]
    pub m_to_x: i32,
    #[rename(name = "m_ToZ")]
    pub m_to_z: i32,
    #[rename(name = "m_Spline")]
    pub m_spline: crate::app::mapaction::MapAction_ProcRouteMove_Spline,
    #[rename(name = "m_Fraction")]
    pub m_fraction: f32,
    #[rename(name = "m_Distance")]
    pub m_distance: f32,
    #[rename(name = "m_IsPass")]
    pub m_is_pass: bool,
    #[rename(name = "m_IsStay")]
    pub m_is_stay: bool,
}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcRouteMove {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        actor: crate::app::unitactor::UnitActor,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
        from_x: i32,
        from_z: i32,
        move_flag: crate::app::mapmoveflag::MapMoveFlag,
    ) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "GetTimeScale", args = 0)]
    pub fn get_time_scale(self) -> i32;

    #[method(name = "IsPass", args = 3)]
    pub fn is_pass(
        self,
        x: i32,
        z: i32,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
    ) -> bool;

    #[method(name = "IsStay", args = 0)]
    pub fn is_stay(self) -> bool;

    #[method(name = "InitRoute", args = 1)]
    pub fn init_route(self, routes: ::unity2::Array<crate::app::dir_2::Dir_Type>) -> ();

    #[method(name = "InitPosition", args = 0)]
    pub fn init_position(self) -> ();

    #[method(name = "CalcPosition", args = 0)]
    pub fn calc_position(self) -> bool;

    #[method(name = "MoveRoute", args = 1)]
    pub fn move_route(self, speed: f32) -> crate::app::mapaction::MapAction_ProcRouteMove_Result;

    #[method(name = "Create", args = 6)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
        from_x: i32,
        from_z: i32,
        move_flag: crate::app::mapmoveflag::MapMoveFlag,
    ) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcRouteMove {
    pub fn new(
        actor: crate::app::unitactor::UnitActor,
        routes: ::unity2::Array<crate::app::dir_2::Dir_Type>,
        from_x: i32,
        from_z: i32,
        move_flag: crate::app::mapmoveflag::MapMoveFlag,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcRouteMove),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcRouteMoveMethods>::ctor(
            this, actor, routes, from_x, from_z, move_flag,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapaction/MapAction_ProcJump.md")))]
#[::unity2::class(namespace = "App", name = "MapAction.ProcJump")]
#[parent(crate::app::mapaction::MapAction_ProcUnitAction)]
pub struct MapAction_ProcJump {}

#[cfg(feature = "app-mapaction")]
#[::unity2::methods]
impl MapAction_ProcJump {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        actor: crate::app::unitactor::UnitActor,
        to_x: i32,
        to_z: i32,
        range: f32,
    ) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "Create", args = 5)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        actor: crate::app::unitactor::UnitActor,
        to_x: i32,
        to_z: i32,
        range: f32,
    ) -> ();
}

#[cfg(feature = "app-mapaction")]
impl MapAction_ProcJump {
    pub fn new(actor: crate::app::unitactor::UnitActor, to_x: i32, to_z: i32, range: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapAction_ProcJump),
                ::core::stringify!(new),
            )
        });
        <Self as IMapAction_ProcJumpMethods>::ctor(this, actor, to_x, to_z, range);
        this
    }
}
