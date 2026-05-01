
use crate::combat::basecameracontroller::BaseCameraController;
use crate::combat::basecameracontroller::IBaseCameraController;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerbullet/CameraControllerBullet.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerBullet")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerBullet {
    #[rename(name = "StartVector")]
    pub start_vector: crate::unity_engine::vector3::Vector3,
    #[rename(name = "HoldVector")]
    pub hold_vector: crate::unity_engine::vector3::Vector3,
    #[rename(name = "BulletVector")]
    pub bullet_vector: crate::unity_engine::vector3::Vector3,
    #[rename(name = "SpeedBrake")]
    pub speed_brake: f32,
    #[rename(name = "m_LastSpeed")]
    pub m_last_speed: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "combat-cameracontrollerbullet")]
#[::unity2::methods]
impl CameraControllerBullet {
    #[method(name = "get_ShootState", args = 0)]
    pub fn get_shoot_state(
        self,
    ) -> crate::combat::cameracontrollerbullet::CameraControllerBullet_State;

    #[method(name = "set_ShootState", args = 1)]
    pub fn set_shoot_state(
        self,
        value: crate::combat::cameracontrollerbullet::CameraControllerBullet_State,
    ) -> ();

    #[method(name = "get_Lancher", args = 0)]
    pub fn get_lancher(self) -> crate::combat::launchbehaviour::LaunchBehaviour;

    #[method(name = "set_Lancher", args = 1)]
    pub fn set_lancher(self, value: crate::combat::launchbehaviour::LaunchBehaviour) -> ();

    #[method(name = "CheckState", args = 0)]
    pub fn check_state(self) -> ();

    #[method(name = "GetCombatVector", args = 1)]
    pub fn get_combat_vector(
        self,
        look_at_vector: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollerbullet")]
impl CameraControllerBullet {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerBullet),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerBulletMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerbullet/CameraControllerBullet_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CameraControllerBullet_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CameraControllerBullet_State {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "CameraControllerBullet.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CameraControllerBullet_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CameraControllerBullet_State {
    pub fn ready() -> Self {
        Self { value: 0 }
    }

    pub fn before_shoot() -> Self {
        Self { value: 1 }
    }

    pub fn shooting() -> Self {
        Self { value: 2 }
    }

    pub fn after_shoot() -> Self {
        Self { value: 3 }
    }
}
