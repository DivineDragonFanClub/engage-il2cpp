
use crate::system::object::IObject;
use crate::system::object::Object;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hublookatcontroller/HubLookAtController.md")))]
#[::unity2::class(namespace = "App", name = "HubLookAtController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubLookAtController {
    #[rename(name = "m_curve")]
    pub m_curve: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_lookAtIKParam")]
    pub m_look_at_ik_param:
        ::unity2::Array<crate::app::hublookatcontroller::HubLookAtController_LookAtIKParam>,
    #[rename(name = "m_lookAtTarget")]
    pub m_look_at_target: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_target")]
    pub m_target: crate::unity_engine::transform::Transform,
    #[rename(name = "m_targetPosition")]
    pub m_target_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_headTransform")]
    pub m_head_transform: crate::unity_engine::transform::Transform,
    #[rename(name = "m_disableFollow")]
    pub m_disable_follow: bool,
    #[rename(name = "m_verticalLimit")]
    pub m_vertical_limit: bool,
    #[rename(name = "m_verticalLimitValue")]
    pub m_vertical_limit_value: f32,
    #[static_field]
    #[rename(name = "DefaultParam")]
    pub default_param: crate::app::hublookatcontroller::HubLookAtController_LookAtIKParam,
}

#[cfg(feature = "app-hublookatcontroller")]
#[::unity2::methods]
impl HubLookAtController {
    #[method(name = "get_FollowTime", args = 0)]
    pub fn get_follow_time(self) -> f32;

    #[method(name = "set_FollowTime", args = 1)]
    pub fn set_follow_time(self, value: f32) -> ();

    #[method(name = "get_IsDisabledParam", args = 0)]
    pub fn get_is_disabled_param(self) -> bool;

    #[method(name = "set_IsDisabledParam", args = 1)]
    pub fn set_is_disabled_param(self, value: bool) -> ();

    #[method(name = "SetTargetTransform", args = 1)]
    pub fn set_target_transform(self, target: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "SetTargetPosition", args = 1)]
    pub fn set_target_position(self, position: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "DisableFollow", args = 0)]
    pub fn disable_follow(self) -> ();

    #[method(name = "EnableFollow", args = 0)]
    pub fn enable_follow(self) -> ();

    #[method(name = "Instant", args = 0)]
    pub fn instant(self) -> ();

    #[method(name = "EnableVerticalLimit", args = 0)]
    pub fn enable_vertical_limit(self) -> ();

    #[method(name = "DisableVerticalLimit", args = 0)]
    pub fn disable_vertical_limit(self) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnDrawGizmos", args = 0)]
    pub fn on_draw_gizmos(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hublookatcontroller")]
impl HubLookAtController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubLookAtController),
                ::core::stringify!(new),
            )
        });
        <Self as IHubLookAtControllerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hublookatcontroller/HubLookAtController_LookAtIKParam.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HubLookAtController_LookAtIKParam {
    pub body_weight: f32,
    pub head_weight: f32,
    pub eyes_weight: f32,
}

impl ::unity2::ClassIdentity for HubLookAtController_LookAtIKParam {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubLookAtController.LookAtIKParam";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubLookAtController_LookAtIKParam {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-hublookatcontroller")]
#[::unity2::methods(value)]
impl HubLookAtController_LookAtIKParam {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, body: f32, head: f32, eyes: f32) -> ();
}
