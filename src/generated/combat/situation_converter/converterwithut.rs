
use crate::combat::situation_converter::baseconverter::BaseConverter;
use crate::combat::situation_converter::baseconverter::IBaseConverter;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/situation_converter/converterwithut/ConverterWithUt.md")))]
#[::unity2::class(namespace = "Combat.SituationConverter", name = "ConverterWithUt")]
#[parent(crate::combat::situation_converter::baseconverter::BaseConverter)]
pub struct ConverterWithUt {
    #[rename(name = "m_Chr")]
    pub m_chr: crate::combat::character::Character,
    #[rename(name = "m_Phase")]
    pub m_phase: crate::combat::phase::Phase,
    #[static_field]
    #[rename(name = "AtkSide")]
    pub atk_side: i32,
    #[static_field]
    #[rename(name = "m_LastCam")]
    pub m_last_cam: crate::combat::cameraposition::CameraPosition,
}

#[cfg(feature = "combat-situation_converter-converterwithut")]
#[::unity2::methods]
impl ConverterWithUt {
    #[method(name = "get_IsOnAttack", args = 0)]
    pub fn get_is_on_attack() -> bool;

    #[method(name = "get_IsOnDamage", args = 0)]
    pub fn get_is_on_damage() -> bool;

    #[method(name = "set_IsOnDamage", args = 1)]
    pub fn set_is_on_damage(value: bool) -> ();

    #[method(name = "get_IsPlayerDragon", args = 0)]
    pub fn get_is_player_dragon(self) -> bool;

    #[method(name = "get_IsEnemyDragon", args = 0)]
    pub fn get_is_enemy_dragon(self) -> bool;

    #[method(name = "get_FullFigureCamera", args = 0)]
    pub fn get_full_figure_camera(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "get_IsFullFigure", args = 0)]
    pub fn get_is_full_figure() -> bool;

    #[method(name = "get_CurrentPosition", args = 0)]
    pub fn get_current_position(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::combat::situation_converter::cameradataset::CameraDataSet)
        -> ();

    #[method(name = "Convert", args = 2)]
    pub fn convert(
        self,
        situation: crate::combat::camerasituation::CameraSituation,
        arg: ::unity2::Il2CppString,
    ) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnBegin", args = 0)]
    pub fn on_begin(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnApproach", args = 0)]
    pub fn on_approach(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnAttack", args = 0)]
    pub fn on_attack(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnDamage", args = 0)]
    pub fn on_damage(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnUnusable", args = 0)]
    pub fn on_unusable(self) -> crate::combat::cameraposition::CameraPosition;

    #[method(name = "OnEnd", args = 0)]
    pub fn on_end(self) -> crate::combat::cameraposition::CameraPosition;
}

#[cfg(feature = "combat-situation_converter-converterwithut")]
impl ConverterWithUt {
    pub fn new(data: crate::combat::situation_converter::cameradataset::CameraDataSet) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConverterWithUt),
                ::core::stringify!(new),
            )
        });
        <Self as IConverterWithUtMethods>::ctor(this, data);
        this
    }
}
