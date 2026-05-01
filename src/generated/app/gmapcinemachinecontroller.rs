
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapcinemachinecontroller/GmapCinemachineController_PointType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapCinemachineController_PointType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapCinemachineController_PointType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapCinemachineController.PointType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapCinemachineController_PointType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapCinemachineController_PointType {
    pub fn none() -> Self {
        Self { value: -1 }
    }

    pub fn start() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapcinemachinecontroller/GmapCinemachineController.md")))]
#[::unity2::class(namespace = "App", name = "GmapCinemachineController")]
#[parent(crate::system::object::Object)]
pub struct GmapCinemachineController {
    #[rename(name = "m_Carrier")]
    pub m_carrier: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-gmapcinemachinecontroller")]
#[::unity2::methods]
impl GmapCinemachineController {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, carrier: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetPosition", args = 1)]
    pub fn set_position(self, pos: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Move", args = 1)]
    pub fn r#move(self, speed: f32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Set", args = 1)]
    pub fn set(self, normalized_pos: f32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "IsMoveFinished", args = 1)]
    pub fn is_move_finished(self, is_forward: bool) -> bool;

    #[method(name = "GetFinishedType", args = 0)]
    pub fn get_finished_type(
        self,
    ) -> crate::app::gmapcinemachinecontroller::GmapCinemachineController_PointType;

    #[method(name = "Enable", args = 0)]
    pub fn enable(self) -> ();

    #[method(name = "Disable", args = 0)]
    pub fn disable(self) -> ();

    #[method(name = "GetNormalizedPathPosition", args = 0)]
    pub fn get_normalized_path_position(self) -> f32;

    #[method(name = "get_PathPosition", args = 0)]
    pub fn get_path_position(self) -> f32;

    #[method(name = "set_PathPosition", args = 1)]
    pub fn set_path_position(self, value: f32) -> ();

    #[method(name = "get_PathLength", args = 0)]
    pub fn get_path_length(self) -> f32;
}

#[cfg(feature = "app-gmapcinemachinecontroller")]
impl GmapCinemachineController {
    pub fn new(carrier: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapCinemachineController),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapCinemachineControllerMethods>::ctor(this, carrier);
        this
    }
}
