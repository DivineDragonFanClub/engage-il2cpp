
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapinforoot/MapInfoRoot_NextCanvasEnabled.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapInfoRoot_NextCanvasEnabled {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapInfoRoot_NextCanvasEnabled {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapInfoRoot.NextCanvasEnabled";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapInfoRoot_NextCanvasEnabled {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapInfoRoot_NextCanvasEnabled {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn enable() -> Self {
        Self { value: 1 }
    }

    pub fn disable() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapinforoot/MapInfoRoot.md")))]
#[::unity2::class(namespace = "App", name = "MapInfoRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MapInfoRoot {
    #[rename(name = "m_Canvas")]
    pub m_canvas: crate::unity_engine::canvas::Canvas,
    #[rename(name = "m_CanvasGroup")]
    pub m_canvas_group: crate::unity_engine::canvasgroup::CanvasGroup,
    #[rename(name = "m_MapInfoUnitLocatorRoot")]
    pub m_map_info_unit_locator_root: crate::app::mapinfounitlocatorroot::MapInfoUnitLocatorRoot,
    #[rename(name = "m_MapInfoGaugeMainLocatorRoot")]
    pub m_map_info_gauge_main_locator_root:
        crate::app::mapinfogaugemainlocatorroot::MapInfoGaugeMainLocatorRoot,
    #[rename(name = "m_OldAlpha")]
    pub m_old_alpha: f32,
    #[rename(name = "m_IsDisplayOutside")]
    pub m_is_display_outside: bool,
    #[rename(name = "m_IsInfoEnable")]
    pub m_is_info_enable: bool,
    #[rename(name = "m_NextCanvasEnabled")]
    pub m_next_canvas_enabled: crate::app::mapinforoot::MapInfoRoot_NextCanvasEnabled,
}

#[cfg(feature = "app-mapinforoot")]
#[::unity2::methods]
impl MapInfoRoot {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Init", args = 1)]
    pub fn init(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Tick", args = 1)]
    pub fn tick(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdatePosition", args = 1)]
    pub fn update_position(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateCanvas", args = 1)]
    pub fn update_canvas(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateParam", args = 1)]
    pub fn update_param(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateAlpha", args = 1)]
    pub fn update_alpha(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "UpdateSortOrder", args = 1)]
    pub fn update_sort_order(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "ForceUpdate", args = 0)]
    pub fn force_update(self) -> ();

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(self, enable: bool) -> ();

    #[method(name = "PlayEngageEffect", args = 2)]
    pub fn play_engage_effect(self, unit: crate::app::unit::Unit, count: i32) -> ();

    #[method(name = "PlayEngageTurnRecoveryEffect", args = 0)]
    pub fn play_engage_turn_recovery_effect(self) -> ();

    #[method(name = "IsPlayingEngageTurnRecoveryEffect", args = 0)]
    pub fn is_playing_engage_turn_recovery_effect(self) -> bool;

    #[method(name = "GetEngageTurnRecoveryEffectRate", args = 0)]
    pub fn get_engage_turn_recovery_effect_rate(self) -> f32;

    #[method(name = "SetSuppressHpStock", args = 1)]
    pub fn set_suppress_hp_stock(self, enable: bool) -> ();

    #[method(name = "PlayHpStockEffect", args = 1)]
    pub fn play_hp_stock_effect(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "PlayHpStockCreateEffect", args = 1)]
    pub fn play_hp_stock_create_effect(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetHpStockAlpha", args = 2)]
    pub fn set_hp_stock_alpha(self, stock: i32, alpha: f32) -> ();

    #[method(name = "IsPlayingHpStockEffect", args = 0)]
    pub fn is_playing_hp_stock_effect(self) -> bool;

    #[method(name = "GetHpStockEffectRate", args = 0)]
    pub fn get_hp_stock_effect_rate(self) -> f32;

    #[method(name = "SetUnitScreenPosLocalScale", args = 1)]
    pub fn set_unit_screen_pos_local_scale(self, pos: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "SetScreenHalfSize", args = 1)]
    pub fn set_screen_half_size(self, screen_half: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "InitHpForecast", args = 0)]
    pub fn init_hp_forecast(self) -> ();

    #[method(name = "SetHpForecast", args = 2)]
    pub fn set_hp_forecast(self, before: i32, after: i32) -> ();

    #[method(name = "InitBreak", args = 0)]
    pub fn init_break(self) -> ();

    #[method(name = "SetBreak", args = 1)]
    pub fn set_break(self, is_break: bool) -> ();

    #[method(name = "IsCanvasVisibilityChanged", args = 0)]
    pub fn is_canvas_visibility_changed(self) -> bool;

    #[method(name = "IsNextCanvasVisibilityEnable", args = 0)]
    pub fn is_next_canvas_visibility_enable(self) -> bool;

    #[method(name = "IsNextCanvasVisibilityDisable", args = 0)]
    pub fn is_next_canvas_visibility_disable(self) -> bool;

    #[method(name = "CanvasVisibilityChange", args = 0)]
    pub fn canvas_visibility_change(self) -> ();

    #[method(name = "SetInfoEnable", args = 1)]
    pub fn set_info_enable(self, enable: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapinforoot")]
impl MapInfoRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapInfoRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IMapInfoRootMethods>::ctor(this);
        this
    }
}
