
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerdollycart/CameraControllerDollyCart_State.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct CameraControllerDollyCart_State {
    pub value: i32,
}

impl ::unity2::ClassIdentity for CameraControllerDollyCart_State {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "CameraControllerDollyCart.State";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CameraControllerDollyCart_State {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl CameraControllerDollyCart_State {
    pub fn on_damage() -> Self {
        Self { value: 0 }
    }

    pub fn on_approach() -> Self {
        Self { value: 1 }
    }

    pub fn on_attack() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerdollycart/CameraControllerDollyCart.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerDollyCart")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerDollyCart {
    #[rename(name = "FrameCount")]
    pub frame_count: i32,
    #[rename(name = "m_LastHeight")]
    pub m_last_height: f32,
    #[rename(name = "m_State")]
    pub m_state: crate::combat::cameracontrollerdollycart::CameraControllerDollyCart_State,
}

#[cfg(feature = "combat-cameracontrollerdollycart")]
#[::unity2::methods]
impl CameraControllerDollyCart {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "CheckUsable", args = 1)]
    pub fn check_usable(self, is_routine: bool) -> ();

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollerdollycart")]
impl CameraControllerDollyCart {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerDollyCart),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerDollyCartMethods>::ctor(this);
        this
    }
}
